use std::collections::HashMap;

use regalloc2::{
    Block, Function, Inst, InstRange, MachineEnv, Operand, Output, RegallocOptions, VReg,
};

use crate::{
    consts::*,
    insn::{IReg, Insn},
};

#[derive(Debug)]
pub struct RegAllocInsn<'a>(pub &'a Insn, pub Vec<Operand>);

pub struct RegAllocProgram<'a> {
    pub blocks: Vec<Vec<RegAllocInsn<'a>>>,
    pub block_start_indices: Vec<u32>,

    pub data: Vec<u8>,

    pub block_params: HashMap<Block, Vec<VReg>>,
    pub outgoing_param_values: HashMap<Block, HashMap<usize, Vec<VReg>>>,

    pub block_succs: HashMap<Block, Vec<Block>>,
    pub block_preds: HashMap<Block, Vec<Block>>,

    pub next_unused_ireg: IReg,
}

impl Function for RegAllocProgram<'_> {
    fn num_insts(&self) -> usize {
        self.blocks.iter().map(|block| block.len()).sum()
    }

    fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    fn entry_block(&self) -> regalloc2::Block {
        Block(0)
    }

    fn block_insns(&self, block: regalloc2::Block) -> regalloc2::InstRange {
        let begin = Inst(self.block_start_indices[block.0 as usize]);
        let end = Inst(
            self.block_start_indices[block.0 as usize] + self.blocks[block.0 as usize].len() as u32,
        );

        InstRange::forward(begin, end)
    }

    fn block_succs(&self, block: regalloc2::Block) -> &[regalloc2::Block] {
        self.block_succs
            .get(&block)
            .map(|x| &x[..])
            .unwrap_or_else(|| &[])
    }

    fn block_preds(&self, block: regalloc2::Block) -> &[regalloc2::Block] {
        self.block_preds
            .get(&block)
            .map(|x| &x[..])
            .unwrap_or_else(|| &[])
    }

    fn block_params(&self, _block: regalloc2::Block) -> &[regalloc2::VReg] {
        self.block_params
            .get(&_block)
            .map(|x| &x[..])
            .unwrap_or_else(|| &[])
    }

    fn is_ret(&self, insn: regalloc2::Inst) -> bool {
        matches!(self.insn_from_inst(insn).0, Insn::Exit(_))
    }

    fn is_branch(&self, insn: regalloc2::Inst) -> bool {
        matches!(self.insn_from_inst(insn).0, Insn::Jz(_, _) | Insn::Jmp(_))
    }

    fn branch_blockparams(
        &self,
        block: regalloc2::Block,
        _insn: regalloc2::Inst,
        succ_idx: usize,
    ) -> &[regalloc2::VReg] {
        self.outgoing_param_values
            .get(&block)
            .and_then(|x| x.get(&succ_idx))
            .map(|x| &x[..])
            .unwrap_or_else(|| &[])
    }

    fn inst_operands(&self, insn: regalloc2::Inst) -> &[regalloc2::Operand] {
        let insn = self.insn_from_inst(insn);
        &insn.1
    }

    fn inst_clobbers(&self, insn: regalloc2::Inst) -> regalloc2::PRegSet {
        let insn = self.insn_from_inst(insn);
        insn.0.clobbers()
    }

    fn num_vregs(&self) -> usize {
        self.next_unused_ireg.0
    }

    fn spillslot_size(&self, _regclass: regalloc2::RegClass) -> usize {
        1
    }
}

impl RegAllocProgram<'_> {
    pub fn insn_from_inst(&self, inst: Inst) -> &RegAllocInsn {
        let block_idx = self
            .block_start_indices
            .iter()
            .position(|&x| x > inst.0)
            .unwrap_or_else(|| self.blocks.len())
            - 1;
        let insn_idx = inst.0 - self.block_start_indices[block_idx];
        &self.blocks[block_idx][insn_idx as usize]
    }

    pub fn run(&self) -> Output {
        let env = MachineEnv {
            preferred_regs_by_class: [vec![RBX, RDI, RSI, RBP, R12, R13, R14, R15], vec![], vec![]],
            non_preferred_regs_by_class: [vec![], vec![], vec![]],
            scratch_by_class: [None, None, None],
            fixed_stack_slots: vec![],
        };

        let opts = RegallocOptions {
            verbose_log: true,
            validate_ssa: false,
        };

        regalloc2::run(self, &env, &opts).unwrap()
    }
}
