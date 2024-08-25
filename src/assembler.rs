use std::{cell::RefCell, collections::HashMap, ffi::CString, rc::Rc, u64};

use crate::{
    consts::{
        iced_reg_to_16_bit, iced_reg_to_32_bit, preg_to_iced_reg, ARGS, BASE_ADDRESS_DATA,
        BASE_ADDRESS_EXECUTABLE, BASE_ADDRESS_TEXT, BUILD_AS_EXE, DATA_SECTION_RVA, DEBUG,
        FILE_ALIGN, REAL_JUMPS, SECT_ALIGN, SHUFFLE_BLOCKS, TEXT_SECTION_RVA, TEXT_SIZE,
    },
    insn::{BlockId, IRegOrImm, Insn},
    random_order,
    regalloc::RegAllocProgram,
    syscalls::SyscallIndices,
    util,
};
use iced_x86::{
    code_asm::{qword_ptr, CodeAssembler, CodeLabel},
    BlockEncoderOptions,
};
use object::{
    pe::{
        IMAGE_FILE_MACHINE_AMD64, IMAGE_SCN_CNT_CODE, IMAGE_SCN_CNT_INITIALIZED_DATA,
        IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE,
    },
    write::pe::{NtHeaders, Writer},
};
use regalloc2::{Allocation, Block, Edit, Function, InstOrEdit, Output};

pub type SharedLabel = Rc<RefCell<CodeLabel>>;

pub struct AssemblerContext {
    a: CodeAssembler,
    idx: SyscallIndices,
    data: Vec<u8>,

    jmp_context_addresses: HashMap<CodeLabel, usize>,
    data_fixups: Vec<(SharedLabel, usize)>, // u32 offsets to be fixed up after assembling
}

impl AssemblerContext {
    pub fn new(idx: SyscallIndices) -> Self {
        let assembler = CodeAssembler::new(64).unwrap();

        Self {
            a: assembler,
            idx,
            data: Vec::new(),
            jmp_context_addresses: HashMap::new(),
            data_fixups: Vec::new(),
        }
    }

    fn append_data(&mut self, data: &[u8], align: usize) -> usize {
        // align to alignment first
        let offset = self.data.len();
        let aligned_offset = (offset + align - 1) & !(align - 1);

        self.data.resize(aligned_offset + data.len(), 0);
        self.data[aligned_offset..aligned_offset + data.len()].copy_from_slice(data);

        aligned_offset + BASE_ADDRESS_DATA as usize
    }

    fn create_label(&mut self) -> SharedLabel {
        Rc::new(RefCell::new(self.a.create_label()))
    }

    fn create_context_for_jump_to_label(&mut self, label: SharedLabel, zero_rsp: bool) -> usize {
        if let Some(addr) = self.jmp_context_addresses.get(&label.borrow()) {
            return *addr;
        }

        // build context
        let mut data = [0u8; 1271];
        data[0x030..][..4].copy_from_slice(&0x100001u32.to_le_bytes()); // ContextFlags
        data[0x038..][..2].copy_from_slice(&0x33u16.to_le_bytes()); // cs
        data[0x042..][..2].copy_from_slice(&0x2bu16.to_le_bytes()); // ss

        // randomize parts
        util::fill_random(&mut data[0..0x30]);
        util::fill_random(&mut data[0x46..0xfc]); // leave upper 4 bytes zeroed
        util::fill_random(&mut data[0x100..]);

        if zero_rsp {
            data[0x98..][..8].fill(0);
        }

        let data_addr = self.append_data(&data, 64);
        let rip_address = data_addr + 0x0f8;

        // fixup rip later after codegen is finished
        self.data_fixups.push((label.clone(), rip_address));

        self.jmp_context_addresses
            .insert(*label.borrow(), data_addr);
        data_addr
    }

    fn assemble_insn(
        &mut self,
        insn: &Insn,
        allocated_operands: &[Allocation],
        block_labels: &HashMap<usize, SharedLabel>,
    ) {
        use iced_x86::code_asm::*;

        match insn {
            Insn::MovImm64(_, imm) => {
                self.a
                    .mov(
                        preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()),
                        *imm as i64,
                    )
                    .unwrap();
            }

            Insn::MovReg(_, _) => {
                let reg_a = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());
                let reg_b = preg_to_iced_reg(allocated_operands[1].as_reg().unwrap());

                if reg_a == reg_b {
                    return;
                }

                self.a.mov(reg_a, reg_b).unwrap();
            }

            Insn::GetArg(_, idx) => {
                let tgt_reg = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());
                self.a.mov(tgt_reg, preg_to_iced_reg(ARGS[*idx])).unwrap();
            }

            Insn::BackUpRegs(addr) => {
                // rbx, rcx, rdx, rsi, rdi, rbp, r8, r9, r10, r11, r12, r13, r14, r15
                for reg in &[
                    rbx, rcx, rdx, rsi, rdi, rbp, r8, r9, r10, r11, r12, r13, r14, r15,
                ] {
                    self.a.push(*reg).unwrap();
                }
                self.a.mov(qword_ptr(*addr), rsp).unwrap();
            }

            Insn::RestoreRegsAndReturn(addr, _) => {
                // set return value
                self.a
                    .mov(
                        rax,
                        preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()),
                    )
                    .unwrap();

                self.a.mov(rsp, qword_ptr(*addr)).unwrap();
                for reg in &[
                    r15, r14, r13, r12, r11, r10, r9, r8, rbp, rdi, rsi, rdx, rcx, rbx,
                ] {
                    self.a.pop(*reg).unwrap();
                }

                // return value already in the correct register
                self.a.ret().unwrap();
            }

            Insn::ReadMem8(_, _, reloff) => {
                self.a
                    .mov(
                        preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()),
                        byte_ptr(
                            preg_to_iced_reg(allocated_operands[1].as_reg().unwrap()) + *reloff,
                        ),
                    )
                    .unwrap();
            }

            Insn::ReadMem32(_, _, reloff) => {
                self.a
                    .mov(
                        preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()),
                        dword_ptr(
                            preg_to_iced_reg(allocated_operands[1].as_reg().unwrap()) + *reloff,
                        ),
                    )
                    .unwrap();
            }

            Insn::ReadMem64(_, _, reloff) => {
                self.a
                    .mov(
                        preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()),
                        qword_ptr(
                            preg_to_iced_reg(allocated_operands[1].as_reg().unwrap()) + *reloff,
                        ),
                    )
                    .unwrap();
            }

            Insn::WriteMem8(_, _, reloff) => {
                self.a
                    .mov(
                        word_ptr(
                            preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()) + *reloff,
                        ),
                        preg_to_iced_reg(allocated_operands[1].as_reg().unwrap()),
                    )
                    .unwrap();
            }

            Insn::WriteMem32(_, _, reloff) => {
                self.a
                    .mov(
                        dword_ptr(
                            preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()) + *reloff,
                        ),
                        preg_to_iced_reg(allocated_operands[1].as_reg().unwrap()),
                    )
                    .unwrap();
            }

            Insn::WriteMem64(_, _, reloff) => {
                self.a
                    .mov(
                        qword_ptr(
                            preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()) + *reloff,
                        ),
                        preg_to_iced_reg(allocated_operands[1].as_reg().unwrap()),
                    )
                    .unwrap();
            }

            Insn::ReadMemAbs16(_, addr) => {
                let iced_reg = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());
                let low_16 = iced_reg_to_16_bit(iced_reg);
                self.a.mov(iced_reg, 0u64).unwrap(); // ensure top bits are zero
                self.a.mov(low_16, word_ptr(*addr)).unwrap();
            }

            Insn::ReadMemAbs32(_, addr) => {
                self.a
                    .mov(
                        iced_reg_to_32_bit(preg_to_iced_reg(
                            allocated_operands[0].as_reg().unwrap(),
                        )),
                        dword_ptr(*addr),
                    )
                    .unwrap();
            }

            Insn::ReadMemAbs64(_, addr) => {
                self.a
                    .mov(
                        preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()),
                        qword_ptr(*addr),
                    )
                    .unwrap();
            }

            Insn::WriteMemAbs32(addr, arg) => match arg {
                IRegOrImm::IReg(_) => self
                    .a
                    .mov(
                        dword_ptr(*addr),
                        iced_reg_to_32_bit(preg_to_iced_reg(
                            allocated_operands[0].as_reg().unwrap(),
                        )),
                    )
                    .unwrap(),
                IRegOrImm::Imm(imm) => self.a.mov(dword_ptr(*addr), *imm as u32).unwrap(),
            },

            Insn::ReadMemIndirect(_, _) => {
                self.assemble_indirect_read(insn, allocated_operands, block_labels);
            }

            Insn::GetStdout(_) => {
                let reg = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());
                self.a.mov(reg, ptr(0x60).gs()).unwrap(); // PEB
                self.a.mov(reg, qword_ptr(reg + 0x20)).unwrap(); // ProcessParameters
                self.a.mov(reg, qword_ptr(reg + 0x28)).unwrap(); // StandardInput
            }

            Insn::GetStdin(_) => {
                let reg = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());
                self.a.mov(reg, ptr(0x60).gs()).unwrap(); // PEB
                self.a.mov(reg, qword_ptr(reg + 0x20)).unwrap(); // ProcessParameters
                self.a.mov(reg, qword_ptr(reg + 0x20)).unwrap(); // StandardOutput
            }

            Insn::GetOSBuildNumber(_) => {
                let reg = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());
                self.a.mov(rsp, ptr(0x60).gs()).unwrap(); // PEB
                self.a.mov(reg, 0u64).unwrap(); // zero out reg
                self.a
                    .mov(iced_reg_to_16_bit(reg), word_ptr(rsp + 0x120))
                    .unwrap(); // OSBuildNumber
            }

            Insn::Comment(cmt) => {
                if !DEBUG {
                    return;
                }
                let comment_addr =
                    self.append_data(CString::new(cmt.as_bytes()).unwrap().as_bytes_with_nul(), 1);

                // jmp after
                // lea rax, [comment_addr]
                // after:
                let temp = self.create_label();
                self.a.jmp(*temp.borrow()).unwrap();
                self.a.lea(rax, qword_ptr(comment_addr as u64)).unwrap();
                self.a.set_label(&mut temp.borrow_mut()).unwrap();
            }

            Insn::Syscall(_, _, _) => {
                self.assemble_syscall(insn, allocated_operands, block_labels);
            }
            Insn::Jz(_, _) => {
                self.assemble_jz(insn, allocated_operands, block_labels);
            }
            Insn::Jmp(_) => {
                self.assemble_jmp(insn, allocated_operands, block_labels);
            }
            Insn::Exit(_) => {
                self.assemble_exit(insn, allocated_operands, block_labels);
            }
        }
    }

    fn assemble_syscall(
        &mut self,
        insn: &Insn,
        allocated_operands: &[Allocation],
        _block_labels: &HashMap<usize, SharedLabel>,
    ) {
        use iced_x86::code_asm::*;
        let Insn::Syscall(_, syscall_name, syscall_args) = insn else {
            panic!("expected syscall insn");
        };

        // handle first four arguments uniformly as they will already be in the correct register
        // only need to write the literals
        let mut operand_idx = 0;
        let mut queued_args = syscall_args[..4]
            .iter()
            .zip(&[r10, rdx, r8, r9])
            .map(|(a, b)| match a {
                IRegOrImm::IReg(_) => {
                    operand_idx += 1;
                    (*a, *b, operand_idx)
                }
                IRegOrImm::Imm(_) => (*a, *b, 0),
            })
            .collect::<Vec<_>>();
        util::shuffle(&mut queued_args);

        random_order!(
            {
                for (arg, reg, operand_idx) in queued_args {
                    match arg {
                        IRegOrImm::IReg(_) => {
                            self.a
                                .mov(
                                    reg,
                                    preg_to_iced_reg(allocated_operands[operand_idx].as_reg().unwrap()),
                                )
                                .unwrap();
                        }
                        IRegOrImm::Imm(imm) => {
                            self.a.mov(reg, imm).unwrap();
                        }
                    }
                }
            }

            {
                self.a
                    .mov(rax, self.idx.get_randomized(syscall_name))
                    .unwrap();
            }
        );

        if syscall_args.len() > 4 {
            // keep track of which ireg allocation index we're at
            let mut ireg_counter = 1 + syscall_args[..4]
                .iter()
                .filter(|x| matches!(x, IRegOrImm::IReg(_)))
                .count();

            // we'll have to build a stack frame for the 5th+ syscall args
            let mut frame = vec![0u8; (syscall_args.len() + 1) * 8];
            let mut queued_writes = vec![];
            for i in 5..=syscall_args.len() {
                let arg = &syscall_args[i - 1];
                match arg {
                    IRegOrImm::IReg(_) => {
                        let target_reg =
                            preg_to_iced_reg(allocated_operands[ireg_counter].as_reg().unwrap());
                        queued_writes.push((i, target_reg));
                        ireg_counter += 1;
                    }
                    IRegOrImm::Imm(imm) => {
                        frame[i * 8..][..8].copy_from_slice(&imm.to_le_bytes());
                    }
                }
            }

            // need to write out rsp first
            let frame_addr = self.append_data(&frame, 8);
            self.a.mov(rsp, frame_addr as u64).unwrap();

            // do extra arguments in a random order
            util::shuffle(&mut queued_writes);
            for (i, reg) in queued_writes {
                self.a
                    .mov(qword_ptr(rsp + (i as i64 * 8) as u64), reg)
                    .unwrap();
            }
        }

        self.a.syscall().unwrap();

        // move rax to correct register
        self.a
            .mov(
                preg_to_iced_reg(allocated_operands[0].as_reg().unwrap()),
                rax,
            )
            .unwrap();
    }

    fn assemble_jz(
        &mut self,
        insn: &Insn,
        allocated_operands: &[Allocation],
        block_labels: &HashMap<usize, SharedLabel>,
    ) {
        use iced_x86::code_asm::*;
        let Insn::Jz(_, tgt) = insn else {
            panic!("expected jz insn");
        };

        let label = block_labels.get(&(tgt.0 as usize)).unwrap();
        let reg = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());

        if REAL_JUMPS {
            self.a.test(reg, reg).unwrap();
            self.a.jz(*label.borrow()).unwrap();
            return;
        }

        // build context
        let data_addr = self.create_context_for_jump_to_label(label.clone(), false);

        // build KCONTINUE_ARGUMENT
        let arg_data = [0u8; 8 * 4];
        let arg_data_addr = self.append_data(&arg_data, 8);

        let a = &mut self.a;

        random_order! {
            {
                a.mov(rdx, arg_data_addr as u64).unwrap();
                a.mov(qword_ptr(rdx + 0x10), reg).unwrap();
            }

            {
                a.mov(r10, data_addr as u64).unwrap();
            }

            {
                a.mov(rax, self.idx.get_randomized("ZwContinueEx")).unwrap();
            }
        }

        a.syscall().unwrap();
    }

    fn assemble_jmp(
        &mut self,
        insn: &Insn,
        _allocated_operands: &[Allocation],
        block_labels: &HashMap<usize, SharedLabel>,
    ) {
        use iced_x86::code_asm::*;
        let Insn::Jmp(tgt) = insn else {
            panic!("expected jmp insn");
        };
        let label = block_labels.get(&(tgt.0 as usize)).unwrap();

        if REAL_JUMPS {
            self.a.jmp(*label.borrow()).unwrap();
            return;
        }

        // build context
        let data_addr = self.create_context_for_jump_to_label(label.clone(), false);

        let a = &mut self.a;

        random_order! {
            { a.mov(r10, data_addr as u64).unwrap() }
            { a.mov(rdx, 0u64).unwrap() }
            { a.mov(rax, self.idx.get_randomized("ZwContinue")).unwrap() }
        };
        a.syscall().unwrap();
    }

    fn assemble_exit(
        &mut self,
        insn: &Insn,
        allocated_operands: &[Allocation],
        _block_labels: &HashMap<usize, SharedLabel>,
    ) {
        use iced_x86::code_asm::*;
        let Insn::Exit(_) = insn else {
            panic!("expected exit insn");
        };

        // exit code is already in the correct register
        random_order! {
            { self.a.mov(rdx, preg_to_iced_reg(allocated_operands[0].as_reg().unwrap())).unwrap(); }
            { self.a.mov(r10, u64::MAX).unwrap(); }
            { self.a.mov(rax, self.idx.get_randomized("ZwTerminateProcess")).unwrap(); }
        };

        self.a.syscall().unwrap();
    }

    fn assemble_indirect_read(
        &mut self,
        insn: &Insn,
        allocated_operands: &[Allocation],
        _block_labels: &HashMap<usize, SharedLabel>,
    ) {
        use iced_x86::code_asm::*;
        let Insn::ReadMemIndirect(_, addrs) = insn else {
            panic!("expected read mem insn");
        };

        let next_label = self.create_label();
        let context_addr = self.create_context_for_jump_to_label(next_label.clone(), true);
        let target_addr = context_addr + 0x98;

        // build args for the syscall, can just be all zeros
        let frame = vec![0u8; 6 * 8];
        let frame_addr = self.append_data(&frame, 8);

        let mut read = [false; 8];

        let mut i = 1;
        for (addr, size, offset) in addrs {
            assert!(matches!(size, 1 | 2 | 4 | 8));
            for i in *offset..(*offset + *size) {
                let i = i as usize;
                assert!(!read[i]);
                read[i] = true;
            }

            if util::random_bool() {
                // ZwReadVirtualMemory to read value into rsp field of ZwContinue context
                random_order! {
                    { self.a.mov(r10, u64::MAX).unwrap(); }
                    {
                        match addr {
                            IRegOrImm::IReg(_) => {
                                self.a.mov(rdx, preg_to_iced_reg(allocated_operands[i].as_reg().unwrap())).unwrap();
                                i += 1;
                            }
                            IRegOrImm::Imm(addr) => {
                                self.a.mov(rdx, *addr).unwrap();
                            }
                        }
                    }
                    { self.a.mov(r8, target_addr as u64 + *offset as u64).unwrap(); }
                    { self.a.mov(r9, *size as u64).unwrap(); }
                    { self.a.mov(rsp, frame_addr as u64).unwrap(); }
                    { self.a.mov(rax, self.idx.get_randomized("ZwReadVirtualMemory")).unwrap(); }
                };
                self.a.syscall().unwrap();
            } else {
                // ZwWriteVirtualMemory to read value into rsp field of ZwContinue context
                random_order! {
                    { self.a.mov(r10, u64::MAX).unwrap(); }
                    { self.a.mov(rdx, target_addr as u64 + *offset as u64).unwrap(); }
                    {
                        match addr {
                            IRegOrImm::IReg(_) => {
                                self.a.mov(r8, preg_to_iced_reg(allocated_operands[i].as_reg().unwrap())).unwrap();
                                i += 1;
                            }
                            IRegOrImm::Imm(addr) => {
                                self.a.mov(r8, *addr).unwrap();
                            }
                        }
                    }
                    { self.a.mov(r9, *size as u64).unwrap(); }
                    { self.a.mov(rsp, frame_addr as u64).unwrap(); }
                    { self.a.mov(rax, self.idx.get_randomized("ZwWriteVirtualMemory")).unwrap(); }
                };
                self.a.syscall().unwrap();
            }
        }

        // ZwContinue to write value into RSP
        random_order! {
            { self.a.mov(r10, context_addr as u64).unwrap(); }
            { self.a.mov(rdx, 0u64).unwrap(); }
            { self.a.mov(rax, self.idx.get_randomized("ZwContinue")).unwrap(); }
        };
        self.a.syscall().unwrap();

        self.a.set_label(&mut next_label.borrow_mut()).unwrap();

        // move rsp into target reg
        let target_reg = preg_to_iced_reg(allocated_operands[0].as_reg().unwrap());
        self.a.mov(target_reg, rsp).unwrap();
    }

    pub fn assemble_insns(&mut self, program: &RegAllocProgram, alloc_result: &Output) {
        // copy data section
        self.data.extend_from_slice(&program.data);

        // create addresses for spill slots
        let mut spill_slots = vec![];
        for _ in 0..alloc_result.num_spillslots {
            spill_slots.push(self.append_data(&[0u8; 8], 8));
        }
        eprintln!("allocated {} spill slots", spill_slots.len());

        let mut block_labels = HashMap::new();

        for block in 0..program.num_blocks() {
            block_labels.insert(block, self.create_label());
        }

        // find which blocks are intermediate blocks that immediately branch to another
        // these are the results of critical edge breakups and can be collapsed
        let mut direct_jmp_blocks = HashMap::new(); // block -> dest
        for block in 0..program.num_blocks() {
            let mut significant_instructions = alloc_result
                .block_insts_and_edits(program, Block(block as u32))
                .skip_while(|x| match x {
                    InstOrEdit::Inst(inst_id) => {
                        let insn = program.insn_from_inst(*inst_id);
                        matches!(insn.0, Insn::Comment(_))
                    }
                    _ => false,
                });

            let first_insn = significant_instructions.next();
            let second_insn = significant_instructions.next();

            if let Some(InstOrEdit::Inst(inst_id)) = first_insn {
                let insn = program.insn_from_inst(inst_id);
                if let Insn::Jmp(BlockId(dest)) = insn.0 {
                    if second_insn.is_none() {
                        direct_jmp_blocks.insert(block, *dest as usize);
                    }
                }
            }
        }

        // rewrite target labels of direct jumps to the target block
        for (block, mut dest) in &direct_jmp_blocks {
            while let Some(new_dest) = direct_jmp_blocks.get(dest) {
                dest = new_dest;
            }

            block_labels.insert(*block, block_labels.get(dest).unwrap().clone());
        }

        let block_it = if SHUFFLE_BLOCKS {
            let mut blocks = (1..program.num_blocks()).collect::<Vec<_>>();
            util::shuffle(&mut blocks);
            blocks.insert(0, 0);
            blocks
        } else {
            (0..program.num_blocks()).collect::<Vec<_>>()
        };

        for block in block_it {
            // we'll be skipping codegen for these
            if direct_jmp_blocks.contains_key(&block) {
                continue;
            }

            self.a
                .set_label(&mut block_labels.get(&block).unwrap().borrow_mut())
                .unwrap();

            for inst_or_edit in alloc_result.block_insts_and_edits(program, Block(block as u32)) {
                match inst_or_edit {
                    InstOrEdit::Inst(inst_id) => {
                        let insn = program.insn_from_inst(inst_id);
                        let allocated_operands = alloc_result.inst_allocs(inst_id);
                        self.assemble_insn(insn.0, allocated_operands, &block_labels);
                    }

                    InstOrEdit::Edit(Edit::Move { from, to }) => {
                        if from.is_reg() && to.is_reg() {
                            self.a
                                .mov(
                                    preg_to_iced_reg(to.as_reg().unwrap()),
                                    preg_to_iced_reg(from.as_reg().unwrap()),
                                )
                                .unwrap();
                        } else if from.is_reg() && to.is_stack() {
                            self.a
                                .mov(
                                    qword_ptr(spill_slots[to.as_stack().unwrap().index()] as u64),
                                    preg_to_iced_reg(from.as_reg().unwrap()),
                                )
                                .unwrap();
                        } else if from.is_stack() && to.is_reg() {
                            self.a
                                .mov(
                                    preg_to_iced_reg(to.as_reg().unwrap()),
                                    qword_ptr(spill_slots[from.as_stack().unwrap().index()] as u64),
                                )
                                .unwrap();
                        } else {
                            todo!("unhandled move case: {:?} -> {:?}", from, to);
                        }
                    }
                }
            }
        }
    }

    pub fn finalize(mut self) -> (Vec<u8>, Vec<u8>) {
        let assembled = self
            .a
            .assemble_options(
                BASE_ADDRESS_TEXT as u64,
                BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS,
            )
            .unwrap();

        for (label, offset) in self.data_fixups {
            let label_target = assembled.label_ip(&label.borrow()).unwrap();
            let label_target = label_target as u32;

            let offset = offset - BASE_ADDRESS_DATA as usize;
            self.data[offset..offset + 4].copy_from_slice(&label_target.to_le_bytes());
        }

        (assembled.inner.code_buffer, self.data)
    }

    pub fn finalize_and_build_pe(mut self) -> Vec<u8> {
        self.a.ret().unwrap();

        let (code, data) = self.finalize();
        assert!(
            code.len() < TEXT_SIZE as usize,
            "code too large ({} > {})",
            code.len(),
            TEXT_SIZE
        );

        let mut pe_object = vec![];
        let mut pe = Writer::new(true, SECT_ALIGN, FILE_ALIGN, &mut pe_object);

        pe.reserve_dos_header_and_stub();
        pe.reserve_nt_headers(16);
        pe.reserve_section_headers(2);

        let text_sec = pe.reserve_section(
            *b".text\0\0\0",
            IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_CNT_CODE,
            DATA_SECTION_RVA - TEXT_SECTION_RVA,
            code.len() as u32,
        );
        let data_sec = pe.reserve_section(
            *b".data\0\0\0",
            IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE | IMAGE_SCN_CNT_INITIALIZED_DATA,
            data.len() as u32,
            data.len() as u32,
        );

        pe.write_dos_header_and_stub().unwrap();
        pe.write_nt_headers(NtHeaders {
            machine: IMAGE_FILE_MACHINE_AMD64,
            time_date_stamp: 0,
            characteristics: 3,
            major_linker_version: 0x0E,
            minor_linker_version: 0x24,
            address_of_entry_point: if BUILD_AS_EXE { TEXT_SECTION_RVA } else { 0 },
            image_base: BASE_ADDRESS_EXECUTABLE as u64,
            major_operating_system_version: 6,
            minor_operating_system_version: 0,
            major_image_version: 0,
            minor_image_version: 0,
            major_subsystem_version: 6,
            minor_subsystem_version: 0,
            subsystem: 3,                // console
            dll_characteristics: 0x8100, // no relocate
            size_of_stack_reserve: 0x100000,
            size_of_stack_commit: 0x1000,
            size_of_heap_reserve: 0x100000,
            size_of_heap_commit: 0x1000,
        });

        pe.write_section_headers(); // need to fix the rvas up later

        pe.write_section(text_sec.file_offset, &code);
        pe.write_section(data_sec.file_offset, &data);

        // adjust virtual addresses
        pe_object[0x194..][..4].copy_from_slice(&(TEXT_SECTION_RVA as u32).to_le_bytes());
        pe_object[0x1bc..][..4].copy_from_slice(&(DATA_SECTION_RVA as u32).to_le_bytes());

        // adjust sizeofimage
        let min_addr = BASE_ADDRESS_EXECUTABLE;
        let max_addr = BASE_ADDRESS_DATA + data.len() as u32;
        let max_addr = (max_addr + SECT_ALIGN - 1) & !(SECT_ALIGN - 1);
        let real_sizeofimage = max_addr - min_addr;
        pe_object[0xd0..][..4].copy_from_slice(&real_sizeofimage.to_le_bytes());

        pe_object
    }
}
