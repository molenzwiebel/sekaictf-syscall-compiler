use crate::insn::{BlockId, IReg, IRegOrImm, Insn, ProgramBuilder};

impl ProgramBuilder {
    pub fn stdout(&mut self) -> IReg {
        let reg = self.reg();
        self.add_insn(Insn::GetStdout(reg));
        reg
    }

    pub fn stdin(&mut self) -> IReg {
        let reg = self.reg();
        self.add_insn(Insn::GetStdin(reg));
        reg
    }

    pub fn os_build_number(&mut self) -> IReg {
        let reg = self.reg();
        self.add_insn(Insn::GetOSBuildNumber(reg));
        reg
    }

    pub fn arg(&mut self, idx: usize) -> IReg {
        let reg = self.reg();
        self.add_insn(Insn::GetArg(reg, idx));
        reg
    }

    pub fn comment(&mut self, comment: impl Into<String>) {
        self.add_insn(Insn::Comment(comment.into()));
    }

    pub fn imm(&mut self, imm: u64) -> IReg {
        let reg = self.reg();
        self.add_insn(Insn::MovImm64(reg, imm));
        reg
    }

    pub fn write_imm32(&mut self, addr: u64, val: impl Into<IRegOrImm<u32>>) {
        self.add_insn(Insn::WriteMemAbs32(addr, val.into()));
    }

    pub fn mov(&mut self, tgt: IReg, src: IReg) {
        self.add_insn(Insn::MovReg(tgt, src));
    }

    pub fn clone(&mut self, reg: IReg) -> IReg {
        let new_reg = self.reg();
        self.mov(new_reg, reg);
        new_reg
    }

    pub fn jz(&mut self, reg: IReg, tgt: BlockId) {
        self.add_insn(Insn::Jz(reg, tgt));
        self.add_tgt(tgt);
    }

    pub fn jmp(&mut self, tgt: BlockId) {
        self.add_insn(Insn::Jmp(tgt));
        self.add_tgt(tgt);
    }

    pub fn exit(&mut self, reg: IReg) {
        self.add_insn(Insn::Exit(reg));
    }

    pub fn print(&mut self, msg: &str) {
        let msg = msg.as_bytes();
        let data_addr = self.append_data(msg, 1);
        let ios = self.append_random_data(16, 8);

        let stdout = self.stdout();
        self.ZwWriteFile(stdout, 0, 0, 0, ios, data_addr, msg.len() as u64, 0, 0);
    }

    pub fn read(&mut self, addr: u64, len: u64) {
        let ios = self.append_random_data(16, 8);
        let stdout = self.stdin();
        self.ZwReadFile(stdout, 0, 0, 0, ios, addr, len, 0, 0);
    }
}
