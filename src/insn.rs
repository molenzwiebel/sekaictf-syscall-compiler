use std::collections::HashMap;

use crate::{
    consts::*,
    regalloc::{RegAllocInsn, RegAllocProgram},
    util,
};
use regalloc2::{Block, Operand, PRegSet, VReg};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IReg(pub usize);

impl std::fmt::Debug for IReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BlockId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ParamId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(private_bounds)]
pub enum IRegOrImm<T: Immediate> {
    IReg(IReg),
    Imm(T),
}

trait Immediate {}
impl Immediate for u8 {}
impl Immediate for u32 {}
impl Immediate for u64 {}

impl<T: Immediate> From<IReg> for IRegOrImm<T> {
    fn from(ireg: IReg) -> Self {
        IRegOrImm::IReg(ireg)
    }
}

impl<T: Immediate> From<T> for IRegOrImm<T> {
    fn from(imm: T) -> Self {
        IRegOrImm::Imm(imm)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Insn {
    MovImm64(IReg, u64), // $0 = $1
    MovReg(IReg, IReg),  // $0 = $1

    // testing purposes
    GetArg(IReg, usize),             // $0 = <nth argument>
    BackUpRegs(u64),                 // store RSP to $0
    RestoreRegsAndReturn(u64, IReg), // restore RSP from $0; return $1

    ReadMem8(IReg, IReg, usize),  // $0 = [$1 + $2]
    ReadMem32(IReg, IReg, usize), // $0 = [$1 + $2]
    ReadMem64(IReg, IReg, usize), // $0 = [$1 + $2]

    WriteMem8(IReg, IReg, usize),  // [$0 + $1] = $2
    WriteMem32(IReg, IReg, usize), // [$0 + $1] = $2
    WriteMem64(IReg, IReg, usize), // [$0 + $1] = $2

    ReadMemAbs16(IReg, u64),            // $0 = [$1]
    ReadMemAbs32(IReg, u64),            // $0 = [$1]
    ReadMemAbs64(IReg, u64),            // $0 = [$1]
    WriteMemAbs32(u64, IRegOrImm<u32>), // [$0] = $1

    ReadMemIndirect(IReg, Vec<(IRegOrImm<u64>, u8, u8)>), // $0 = <(addr, size, offset)>* combined

    GetStdout(IReg),        // $0 = stdout
    GetStdin(IReg),         // $0 = stdin
    GetOSBuildNumber(IReg), // $0 = OS build number

    Syscall(IReg, &'static str, Vec<IRegOrImm<u64>>), // $0 = syscall $1($2, $3, ...); this is never allowed to diverge/terminate

    Jz(IReg, BlockId), // if $0 == 0 { goto $1 }
    Jmp(BlockId),      // goto $0

    Comment(String),

    Exit(IReg), // exit($0)
}

impl Insn {
    fn operands(&self) -> Vec<Operand> {
        match self {
            Insn::MovImm64(to, _) => {
                vec![Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int))]
            }

            Insn::MovReg(to, from) => {
                vec![
                    Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int)),
                    Operand::reg_use(VReg::new(from.0, regalloc2::RegClass::Int)),
                ]
            }

            Insn::GetArg(to, _idx) => {
                vec![Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int))]
            }

            Insn::RestoreRegsAndReturn(_, from) => {
                vec![Operand::reg_use(VReg::new(
                    from.0,
                    regalloc2::RegClass::Int,
                ))]
            }

            Insn::BackUpRegs(_) => {
                vec![]
            }

            Insn::ReadMem8(to, from, _offs)
            | Insn::ReadMem32(to, from, _offs)
            | Insn::ReadMem64(to, from, _offs)
            | Insn::WriteMem8(to, from, _offs)
            | Insn::WriteMem32(to, from, _offs)
            | Insn::WriteMem64(to, from, _offs) => {
                vec![
                    Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int)),
                    Operand::reg_use(VReg::new(from.0, regalloc2::RegClass::Int)),
                ]
            }

            Insn::ReadMemAbs16(to, _) | Insn::ReadMemAbs32(to, _) | Insn::ReadMemAbs64(to, _) => {
                vec![Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int))]
            }

            Insn::WriteMemAbs32(_, IRegOrImm::IReg(from)) => {
                vec![Operand::reg_use(VReg::new(
                    from.0,
                    regalloc2::RegClass::Int,
                ))]
            }
            Insn::WriteMemAbs32(_, _) => vec![],

            Insn::ReadMemIndirect(to, args) => {
                let mut out = vec![Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int))];
                for (addr, _, _) in args {
                    if let IRegOrImm::IReg(vreg) = addr {
                        out.push(Operand::reg_use(VReg::new(
                            vreg.0,
                            regalloc2::RegClass::Int,
                        )));
                    }
                }

                out
            }

            Insn::GetStdout(to) => {
                vec![Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int))]
            }

            Insn::GetStdin(to) => {
                vec![Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int))]
            }

            Insn::GetOSBuildNumber(to) => {
                vec![Operand::reg_def(VReg::new(to.0, regalloc2::RegClass::Int))]
            }

            Insn::Syscall(tgt, _, args) => {
                let mut ret = vec![Operand::reg_def(VReg::new(tgt.0, regalloc2::RegClass::Int))];

                // R10, RDX, R8, R9; RSP if more than 4 arguments
                for vreg in args {
                    if let IRegOrImm::IReg(vreg) = vreg {
                        ret.push(Operand::reg_use(VReg::new(
                            vreg.0,
                            regalloc2::RegClass::Int,
                        )));
                    }
                }

                // don't depend on RSP here, we'll just not assign it to anything so
                // its always available for us to clobber

                ret
            }

            Insn::Jz(val, _) => {
                // this is SetContextEx which uses two registers; but as we need to construct a context
                // for this we don't particularly care what the registers are; the registers we clobber
                // are defined as clobbers anyway
                vec![Operand::reg_use(VReg::new(val.0, regalloc2::RegClass::Int))]
            }

            Insn::Jmp(_) => vec![], // nothing used; clobbers only
            Insn::Comment(_) => vec![],

            Insn::Exit(code) => vec![
                // needs to be r10 since it's the first argument to ExitProcess
                Operand::reg_use(VReg::new(code.0, regalloc2::RegClass::Int)),
            ],
        }
    }

    pub fn clobbers(&self) -> PRegSet {
        PRegSet::empty()
    }
}

pub struct ProgramBuilder {
    blocks: Vec<Vec<Insn>>,
    block_params: HashMap<BlockId, HashMap<ParamId, IReg>>,

    data: Vec<u8>,

    current_block: BlockId,

    block_succs: HashMap<BlockId, Vec<BlockId>>,
    block_preds: HashMap<BlockId, Vec<BlockId>>,

    outgoing_param_values: HashMap<BlockId, HashMap<(BlockId, ParamId), IReg>>,

    next_ireg: IReg,
    next_param_id: ParamId,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            blocks: vec![vec![]],
            block_params: HashMap::new(),
            data: vec![],
            current_block: BlockId(0),
            block_succs: HashMap::new(),
            block_preds: HashMap::new(),
            outgoing_param_values: HashMap::new(),
            next_ireg: IReg(0),
            next_param_id: ParamId(0),
        }
    }

    pub fn current_block(&self) -> BlockId {
        self.current_block
    }

    pub fn new_block(&mut self) -> BlockId {
        self.blocks.push(vec![]);
        BlockId(self.blocks.len() - 1)
    }

    pub fn reg(&mut self) -> IReg {
        let ret = self.next_ireg;
        self.next_ireg.0 += 1;
        ret
    }

    pub fn append_data(&mut self, data: &[u8], align: usize) -> u64 {
        // align to alignment first
        let offset = self.data.len();
        let aligned_offset = (offset + align - 1) & !(align - 1);

        self.data.resize(aligned_offset + data.len(), 0);
        self.data[aligned_offset..aligned_offset + data.len()].copy_from_slice(data);

        aligned_offset as u64 + BASE_ADDRESS_DATA as u64
    }

    pub fn append_random_data(&mut self, size: usize, align: usize) -> u64 {
        let mut data = vec![0; size];
        util::fill_random(&mut data);

        self.append_data(&data, align)
    }

    pub fn param_for_block(&mut self, block: BlockId) -> (ParamId, IReg) {
        let ret = self.next_param_id;
        self.next_param_id.0 += 1;

        let ret_ireg = self.reg();

        self.block_params
            .entry(block)
            .or_insert_with(HashMap::new)
            .insert(ret, ret_ireg);

        (ret, ret_ireg)
    }

    pub fn set_param_ireg(&mut self, next_block: BlockId, param: ParamId, ireg: IReg) {
        self.outgoing_param_values
            .entry(self.current_block)
            .or_insert_with(HashMap::new)
            .insert((next_block, param), ireg);
    }

    pub fn switch_block(&mut self, block: BlockId) {
        self.current_block = block;
    }

    pub fn add_insn(&mut self, insn: Insn) {
        self.blocks[self.current_block.0].push(insn);
    }

    pub fn add_tgt(&mut self, tgt: BlockId) {
        self.block_succs
            .entry(self.current_block)
            .or_insert_with(Vec::new)
            .push(tgt);
        self.block_preds
            .entry(tgt)
            .or_insert_with(Vec::new)
            .push(self.current_block);
    }

    pub fn build(&mut self) -> RegAllocProgram {
        let mut blocks = vec![];
        let mut block_start_indices = vec![];
        let mut count = 0;
        for (_i, block) in self.blocks.iter_mut().enumerate() {
            block_start_indices.push(count);
            let mut regalloc_insns = vec![];

            let mut new_block = block.clone();
            block.clear();
            block.append(&mut new_block);

            for insn in block {
                let operands = insn.operands();
                regalloc_insns.push(RegAllocInsn(insn, operands));
                count += 1;
            }
            blocks.push(regalloc_insns);
        }

        let mut block_succs = HashMap::new();
        for (block, succs) in &self.block_succs {
            block_succs.insert(
                Block(block.0 as u32),
                succs.iter().map(|&x| Block(x.0 as u32)).collect(),
            );
        }

        let mut block_preds = HashMap::new();
        for (block, preds) in &self.block_preds {
            block_preds.insert(
                Block(block.0 as u32),
                preds.iter().map(|&x| Block(x.0 as u32)).collect(),
            );
        }

        let mut block_params = HashMap::new();
        for (block, params) in &self.block_params {
            // ensure params are sorted by their ID to have a consistent order
            let mut params: Vec<_> = params.iter().collect();
            params.sort_by_cached_key(|x| x.0);

            block_params.insert(
                Block(block.0 as u32),
                params
                    .iter()
                    .map(|(_, &v)| VReg::new(v.0, regalloc2::RegClass::Int))
                    .collect(),
            );
        }

        let mut outgoing_block_params = HashMap::new();
        for (block, params) in &self.outgoing_param_values {
            let params: Vec<_> = params.iter().map(|(a, b)| (a.0, a.1, *b)).collect();

            for (outgoing_idx, outgoing_block_id) in self.block_succs[block].iter().enumerate() {
                let mut relevant_outgoing: Vec<_> = params
                    .iter()
                    .filter(|(block_id, _, _)| *block_id == *outgoing_block_id)
                    .map(|(_, param_id, ireg)| (*param_id, *ireg))
                    .collect();
                relevant_outgoing.sort_by_cached_key(|x| x.0);

                outgoing_block_params
                    .entry(Block(block.0 as u32))
                    .or_insert_with(HashMap::new)
                    .insert(
                        outgoing_idx,
                        relevant_outgoing
                            .iter()
                            .map(|(_, v)| VReg::new(v.0, regalloc2::RegClass::Int))
                            .collect(),
                    );
            }
        }

        let mut new_data = vec![];
        new_data.append(&mut self.data);

        RegAllocProgram {
            blocks,
            block_params,
            data: new_data,
            outgoing_param_values: outgoing_block_params,
            block_start_indices,
            block_succs,
            block_preds,
            next_unused_ireg: self.next_ireg,
        }
    }
}
