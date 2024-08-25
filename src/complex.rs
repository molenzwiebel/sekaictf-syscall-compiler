use std::u64;

use crate::{
    insn::{BlockId, IReg, Insn, ProgramBuilder},
    random_order,
};

impl ProgramBuilder {
    pub fn jz_or_positive(&mut self, cond: IReg, tgt: BlockId) {
        let handle_addr = self.append_random_data(8, 8);
        let succeeded = self.ZwCreateSemaphore(handle_addr, 0x1F0003u64, 0, cond, 0x7FFFFFFF);
        self.jz(succeeded, tgt);
    }

    fn read_worker_factory(&mut self, handle: IReg, addr: u64, out: IReg) {
        self.ZwQueryInformationWorkerFactory(
            handle, 7, /* WorkerFactoryBasicInformation */
            addr, 120, 0,
        );
        self.add_insn(Insn::ReadMemAbs32(out, addr + 0x20));
    }

    pub fn add16(&mut self, a: IReg, b: IReg) -> IReg {
        let ret_block = self.new_block();
        let ret_block_crit_breaker = self.new_block();
        let retval = self.reg();

        let ret_a = self.new_block();
        let ret_b = self.new_block();
        let b_nonzero = self.new_block();
        let both_nonzero = self.new_block();

        // can immediately do addition
        let scratch_addr = self.append_random_data(8, 8);
        let wbi_addr = self.append_random_data(120, 8);

        let iocp_handle = self.reg();
        let wf_handle = self.reg();

        self.comment("add16: start");
        self.jz(b, ret_a);
        self.jmp(b_nonzero);

        self.switch_block(b_nonzero);
        self.comment("b is nonzero");
        self.jz(a, ret_b);
        self.jmp(both_nonzero);

        self.switch_block(both_nonzero);
        self.comment("both integers are nonzero");
        self.ZwCreateIoCompletion(scratch_addr, 0x1F0003u64, 0, 0);
        self.add_insn(Insn::ReadMemAbs64(iocp_handle, scratch_addr));

        // as well as at least one worker factory
        self.ZwCreateWorkerFactory(
            scratch_addr,
            0xF00FF,
            0,
            iocp_handle,
            u64::MAX,
            0,
            0,
            u64::MAX,
            0x1000,
            0x1000,
        );
        self.add_insn(Insn::ReadMemAbs64(wf_handle, scratch_addr));

        // cur == -1
        self.write_imm32(scratch_addr, a);
        self.ZwSetInformationWorkerFactory(wf_handle, 3, scratch_addr, 4);

        // cur == a + b
        self.write_imm32(scratch_addr, b);
        self.ZwSetInformationWorkerFactory(wf_handle, 3, scratch_addr, 4);

        // read result as u16
        self.ZwQueryInformationWorkerFactory(
            wf_handle, 7, /* WorkerFactoryBasicInformation */
            wbi_addr, 120, 0,
        );
        self.add_insn(Insn::ReadMemAbs16(retval, wbi_addr + 0x20));
        self.jmp(ret_block_crit_breaker);

        self.switch_block(ret_block_crit_breaker);
        self.comment("critical edge breaker");
        self.jmp(ret_block);

        self.switch_block(ret_a);
        self.comment("return a");
        self.mov(retval, a);
        self.jmp(ret_block);

        self.switch_block(ret_b);
        self.comment("return b");
        self.mov(retval, b);
        self.jmp(ret_block);

        self.switch_block(ret_block);
        retval
    }

    pub fn add32(&mut self, a: IReg, b: IReg) -> IReg {
        let ret_block = self.new_block();
        let retval = self.reg();

        let ret_a = self.new_block();
        let ret_b = self.new_block();
        let ret_zero = self.new_block();
        let both_nonzero = self.new_block();

        let a_positive = self.new_block();
        let a_negative = self.new_block();
        let a_and_b_negative = self.new_block();
        let a_and_b_positive = self.new_block();
        let a_negative_b_positive = self.new_block();
        let a_positive_b_negative = self.new_block();
        let complex_case = self.new_block();

        let iocp_handle_addr = self.append_random_data(8, 8);
        let wf_handle_addr_1 = self.append_random_data(8, 8);
        let value_addr = self.append_random_data(8, 8);
        let wbi_addr = self.append_random_data(120, 8);

        let iocp_handle = self.reg();
        let wf_handle_1 = self.reg();

        let make_branch_to = |zelf: &mut ProgramBuilder, block: BlockId| {
            let cur = zelf.current_block();
            let intermediate = zelf.new_block();
            zelf.switch_block(intermediate);
            zelf.comment("critical edge breaker");
            zelf.jmp(block);
            zelf.switch_block(cur);
            intermediate
        };

        let a_nonzero = self.new_block();

        // if either is zero, return the other
        self.comment("add: begin");
        self.jz(a, ret_b);
        self.jmp(a_nonzero);

        self.switch_block(a_nonzero);
        self.jz(b, ret_a);
        self.jmp(both_nonzero);

        // both are nonzero
        {
            self.switch_block(both_nonzero);
            self.comment("both integers are nonzero");

            // we'll need an iocp for sure
            self.ZwCreateIoCompletion(iocp_handle_addr, 0x1F0003u64, 0, 0);
            self.add_insn(Insn::ReadMemAbs64(iocp_handle, iocp_handle_addr));

            // as well as at least one worker factory
            self.ZwCreateWorkerFactory(
                wf_handle_addr_1,
                0xF00FF,
                0,
                iocp_handle,
                u64::MAX,
                0,
                0,
                u64::MAX,
                0x1000,
                0x1000,
            );
            self.add_insn(Insn::ReadMemAbs64(wf_handle_1, wf_handle_addr_1));

            self.jz_or_positive(a, a_positive);
            self.jmp(a_negative);
        }

        // a is negative, b is nonzero
        {
            self.switch_block(a_negative);
            self.comment("a is negative, b is nonzero");

            {
                let a_negative_b_positive = make_branch_to(self, a_negative_b_positive);
                self.jz_or_positive(b, a_negative_b_positive);
            }
            {
                let a_and_b_negative = make_branch_to(self, a_and_b_negative);
                self.jmp(a_and_b_negative);
            }
        }

        // both a and b are negative
        {
            // // case 2a: b is also negative
            // // a < 0, b < 0
            // int cur = cursed_add(0, 0); // cur == -1
            // cur = cursed_add(cur, a);   // cur == a - 1
            // cur = cursed_add(cur, 1);   // cur == a
            // cur = cursed_add(cur, b);   // cur == a + b
            self.switch_block(a_and_b_negative);
            self.comment("both integers are negative");

            // cur == -1
            self.write_imm32(value_addr, 0);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // cur == -1 + a
            self.write_imm32(value_addr, a);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // cur == -1 + a + 1 == a
            self.write_imm32(value_addr, 1);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // cur == a + b
            self.write_imm32(value_addr, b);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // read result
            self.read_worker_factory(wf_handle_1, wbi_addr, retval);

            self.jmp(ret_block);
        }

        // a is negative, b is positive
        {
            self.switch_block(a_negative_b_positive);
            self.comment("a is negative, b is positive");

            // we'll want to try for the easy case first
            // cur == b
            self.write_imm32(value_addr, b);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // cur == b + a, or 0, depending on overflow behavior
            self.write_imm32(value_addr, a);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // read result
            self.read_worker_factory(wf_handle_1, wbi_addr, retval);

            self.jz(retval, complex_case);
            let ret_block = make_branch_to(self, ret_block);
            self.jmp(ret_block);
        }

        // a is negative, b is positive, and a + b <= 0
        // this is the complex case
        {
            self.switch_block(complex_case);
            self.comment("a is negative, b is positive, and a + b <= 0 (complex case)");

            // get a into the worker factory first
            // cur == -1
            self.write_imm32(value_addr, 0);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // cur == a - 1
            self.write_imm32(value_addr, a);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // cur == a
            self.write_imm32(value_addr, 1);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // now, allocate a new second worker factory that we'll unfortunately need
            let wf_handle_addr_2 = self.append_random_data(8, 8);
            let wf_handle_2 = self.reg();
            self.ZwCreateWorkerFactory(
                wf_handle_addr_2,
                0xF00FF,
                0,
                iocp_handle,
                u64::MAX,
                0,
                0,
                u64::MAX,
                0x1000,
                0x1000,
            );
            self.add_insn(Insn::ReadMemAbs64(wf_handle_2, wf_handle_addr_2));

            // try b + -1
            // cur == b
            self.write_imm32(value_addr, b);
            self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

            // cur == b - 1
            self.write_imm32(value_addr, u32::MAX);
            self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

            let b_minus_1 = self.reg();
            let b_minus_2 = self.reg();

            self.read_worker_factory(wf_handle_2, wbi_addr, b_minus_1);

            let b_minus_one_is_zero = self.new_block();
            let b_minus_one_is_nonzero = self.new_block();

            let b_minus_two_is_zero = self.new_block();
            let b_minus_two_is_nonzero = self.new_block();

            self.jz(b_minus_1, b_minus_one_is_zero);
            self.jmp(b_minus_one_is_nonzero);

            let add_b_to_cur_and_return = self.new_block();

            {
                self.switch_block(b_minus_one_is_zero);
                self.comment("b - 1 is zero");

                // attempt 2 + a; if that's nonzero, the result is zero; else, the result is cur + b
                // first, reset the worker factory
                self.ZwCreateWorkerFactory(
                    wf_handle_addr_2,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_2, wf_handle_addr_2));

                // cur == 2
                self.write_imm32(value_addr, 2);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // cur == 2 + a
                self.write_imm32(value_addr, a);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // read result
                let two_plus_a = self.reg();
                self.read_worker_factory(wf_handle_2, wbi_addr, two_plus_a);

                // if (tmp) return 0; else return cursed_add(cur, b);
                {
                    let add_b_to_cur_and_return = make_branch_to(self, add_b_to_cur_and_return);
                    self.jz(two_plus_a, add_b_to_cur_and_return);
                }
                {
                    let ret_zero = make_branch_to(self, ret_zero);
                    self.jmp(ret_zero);
                }
            }

            {
                self.switch_block(b_minus_one_is_nonzero);
                self.comment("b - 1 is nonzero");

                // add another -1 to it, then check again
                // wf_handle_2 still contains b - 1
                // cur == b - 2
                self.write_imm32(value_addr, u32::MAX);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // read result
                self.read_worker_factory(wf_handle_2, wbi_addr, b_minus_2);

                self.jz(b_minus_2, b_minus_two_is_zero);
                self.jmp(b_minus_two_is_nonzero);
            }

            {
                // b_minus_two is zero
                self.switch_block(b_minus_two_is_zero);
                self.comment("b - 2 is zero");

                // attempt 3 + a; if that's nonzero, the result is zero; else, the result is cur + b
                // first, reset the worker factory
                self.ZwCreateWorkerFactory(
                    wf_handle_addr_2,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_2, wf_handle_addr_2));

                // cur == 3
                self.write_imm32(value_addr, 3);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // cur == 3 + a
                self.write_imm32(value_addr, a);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // read result
                let three_plus_a = self.reg();
                self.read_worker_factory(wf_handle_2, wbi_addr, three_plus_a);

                // if (tmp) return 0; else return cursed_add(cur, b);
                {
                    let add_b_to_cur_and_return = make_branch_to(self, add_b_to_cur_and_return);
                    self.jz(three_plus_a, add_b_to_cur_and_return);
                }
                {
                    let ret_zero = make_branch_to(self, ret_zero);
                    self.jmp(ret_zero);
                }
            }

            {
                // b minus two is nonzero
                self.switch_block(b_minus_two_is_nonzero);
                self.comment("b - 2 is nonzero");

                // add b_minus_two to cur, then add 1 twice; wf_handle_1 still contains a
                // cur == a + b - 2
                self.write_imm32(value_addr, b_minus_2);
                self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

                // cur == a + b - 1
                self.write_imm32(value_addr, 1);
                self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

                let x = self.reg();
                self.read_worker_factory(wf_handle_1, wbi_addr, x);

                // cur == a + b
                self.write_imm32(value_addr, 1);
                self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

                let y = retval;
                self.read_worker_factory(wf_handle_1, wbi_addr, retval);

                // if 2 + x is zero, and 2 + y is zero, return 0; else return x + y

                // first, reset the worker factory
                self.ZwCreateWorkerFactory(
                    wf_handle_addr_2,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_2, wf_handle_addr_2));

                // cur == 2
                self.write_imm32(value_addr, 2);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // cur == 2 + x
                self.write_imm32(value_addr, x);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // read result
                let two_plus_x = self.reg();
                self.read_worker_factory(wf_handle_2, wbi_addr, two_plus_x);

                let check_two_plus_y = self.new_block();

                {
                    let ret_block = make_branch_to(self, ret_block);
                    self.jz(two_plus_x, ret_block); // if this is zero, return y (cur)
                }
                self.jmp(check_two_plus_y);

                self.switch_block(check_two_plus_y);
                self.comment("check if 2 + y is zero");

                // first, reset the worker factory again
                self.ZwCreateWorkerFactory(
                    wf_handle_addr_2,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_2, wf_handle_addr_2));

                // cur == 2
                self.write_imm32(value_addr, 2);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // cur == 2 + x
                self.write_imm32(value_addr, y);
                self.ZwSetInformationWorkerFactory(wf_handle_2, 3, value_addr, 4);

                // read result
                let two_plus_y = self.reg();
                self.read_worker_factory(wf_handle_2, wbi_addr, two_plus_y);

                {
                    let ret_block = make_branch_to(self, ret_block);
                    self.jz(two_plus_y, ret_block); // if this is zero, return x (cur)
                }
                let ret_zero = make_branch_to(self, ret_zero);
                self.jmp(ret_zero);
            }

            {
                // add_b_to_cur_and_return
                self.switch_block(add_b_to_cur_and_return);
                self.comment("add b to cur and return");

                // cur == a + b
                self.write_imm32(value_addr, b);
                self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

                // read result
                self.read_worker_factory(wf_handle_1, wbi_addr, retval);
                {
                    let ret_block = make_branch_to(self, ret_block);
                    self.jmp(ret_block);
                }
            }
        }

        // a is positive, b is nonzero
        {
            self.switch_block(a_positive);
            self.comment("a is positive, b is nonzero");

            self.jz_or_positive(b, a_and_b_positive);
            self.jmp(a_positive_b_negative);
        }

        // a is positive, b is negative
        {
            self.switch_block(a_positive_b_negative);
            self.comment("a is positive, b is negative");

            // swap a and b, then jump to a_negative_b_positive
            let tmp = self.reg();
            self.mov(tmp, a);
            self.mov(a, b);
            self.mov(b, tmp);
            self.jmp(a_negative_b_positive);
        }

        // both are positive
        {
            self.switch_block(a_and_b_positive);
            self.comment("both integers are positive");

            // we can do a normal add here
            // cur == a
            self.write_imm32(value_addr, a);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // cur == a + b
            self.write_imm32(value_addr, b);
            self.ZwSetInformationWorkerFactory(wf_handle_1, 3, value_addr, 4);

            // read result
            self.ZwQueryInformationWorkerFactory(
                wf_handle_1,
                7, /* WorkerFactoryBasicInformation */
                wbi_addr,
                120,
                0,
            );
            self.add_insn(Insn::ReadMemAbs32(retval, wbi_addr + 0x20));

            self.jmp(ret_block);
        }

        // b == 0, return a
        {
            // // case 1: b == 0
            // if (!b)
            //     return a;
            self.switch_block(ret_a);
            self.comment("b is zero, return a");
            self.mov(retval, a);
            self.jmp(ret_block);
        }

        // a == 0, return b
        {
            // // case 0: a == 0
            // if (!a)
            //     return b;
            self.switch_block(ret_b);
            self.comment("a is zero, return b");
            self.mov(retval, b);
            self.jmp(ret_block);
        }

        {
            // // case 3: a == 0 && b == 0
            // return 0;
            self.switch_block(ret_zero);
            self.comment("return zero");
            self.add_insn(Insn::MovImm64(retval, 0));
            self.jmp(ret_block);
        }

        self.switch_block(ret_block);
        retval
    }

    pub fn rot16_imm(&mut self, val: IReg, is_left: bool, by: i32) -> IReg {
        let val = self.clone(val);

        // we'll need two adders
        let scratch_addr = self.append_random_data(8, 8);
        let wbi_addr = self.append_random_data(120, 8);

        let iocp_handle = self.reg();
        let wf_handle_out = self.reg();
        let wf_handle_cur = self.reg();

        // we'll need an iocp for sure
        self.ZwCreateIoCompletion(scratch_addr, 0x1F0003u64, 0, 0);
        self.add_insn(Insn::ReadMemAbs64(iocp_handle, scratch_addr));

        // as well at two worker factories
        random_order!(
            {
                self.ZwCreateWorkerFactory(
                    scratch_addr,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_out, scratch_addr));
            }

            {
                self.ZwCreateWorkerFactory(
                    scratch_addr,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_cur, scratch_addr));
            }
        );

        let initialize_cur = self.new_block();
        let mut next_block = self.new_block();
        let next_block_crit_breaker = self.new_block();

        // set cur == val
        self.jz(val, next_block_crit_breaker);
        self.jmp(initialize_cur);

        self.switch_block(next_block_crit_breaker);
        self.jmp(next_block);

        self.switch_block(initialize_cur);
        self.comment("initialize cur");

        // cur == val
        self.write_imm32(scratch_addr, val);
        self.ZwSetInformationWorkerFactory(wf_handle_cur, 3, scratch_addr, 4);
        self.jmp(next_block);

        for i in (0..=15).into_iter().rev() {
            let mut out_bit = if is_left { i + by } else { i - by };
            if out_bit < 0 {
                out_bit += 16;
            }
            out_bit %= 16;

            self.switch_block(next_block);
            self.comment(&format!("rotate bit {}", i));

            let next_check = self.new_block();
            let next_check_crit_breaker = self.new_block();
            let was_set = self.new_block();

            // check whether the bit is set by seeing whether it is more than the given value
            if i != 0 {
                let was_more =
                    self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, val, (1 << i) - 1);
                self.jz(was_more, next_check_crit_breaker);
                self.jmp(was_set);
            } else {
                self.jz(val, next_check_crit_breaker);
                self.jmp(was_set);
            }

            self.switch_block(next_check_crit_breaker);
            self.jmp(next_check);

            // the bit was set
            self.switch_block(was_set);
            self.comment("bit was set");

            random_order!(
                {
                    // set out bit in out
                    self.write_imm32(scratch_addr, 1u32 << out_bit);
                    self.ZwSetInformationWorkerFactory(wf_handle_out, 3, scratch_addr, 4);
                }

                {
                    // unset bit in cur
                    self.write_imm32(scratch_addr, -(1i32 << i) as u32 & 0xFFFF);
                    self.ZwSetInformationWorkerFactory(wf_handle_cur, 3, scratch_addr, 4);

                    // re-read cur
                    self.ZwQueryInformationWorkerFactory(
                        wf_handle_cur,
                        7, /* WorkerFactoryBasicInformation */
                        wbi_addr,
                        120,
                        0,
                    );
                    self.add_insn(Insn::ReadMemAbs16(val, wbi_addr + 0x20));
                }
            );

            self.jmp(next_check);

            next_block = next_check;
        }

        // final value is in out
        self.switch_block(next_block);
        self.comment("final value is in out");

        // read result
        self.ZwQueryInformationWorkerFactory(
            wf_handle_out,
            7, /* WorkerFactoryBasicInformation */
            wbi_addr,
            120,
            0,
        );

        let out = self.reg();
        self.add_insn(Insn::ReadMemAbs16(out, wbi_addr + 0x20));

        out
    }

    // xor with a known immediate
    pub fn xor16_imm(&mut self, a: IReg, b: u16) -> IReg {
        let a = self.clone(a);

        // we'll need two adders
        let scratch_addr = self.append_random_data(8, 8);
        let wbi_addr = self.append_random_data(120, 8);

        let iocp_handle = self.reg();
        let wf_handle_out = self.reg();
        let wf_handle_cur_a = self.reg();

        // we'll need an iocp for sure
        self.ZwCreateIoCompletion(scratch_addr, 0x1F0003u64, 0, 0);
        self.add_insn(Insn::ReadMemAbs64(iocp_handle, scratch_addr));

        // as well as two worker factories
        random_order!(
            {
                self.ZwCreateWorkerFactory(
                    scratch_addr,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_out, scratch_addr));
            }

            {
                self.ZwCreateWorkerFactory(
                    scratch_addr,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_cur_a, scratch_addr));
            }
        );

        let initialize_a = self.new_block();
        let mut next_block = self.new_block();
        let next_block_crit_breaker = self.new_block();

        // initialize `a` adder to current values
        self.comment("maybe initialize a");
        self.jz(a, next_block_crit_breaker);
        self.jmp(initialize_a);

        self.switch_block(initialize_a);
        self.comment("initialize a");
        self.write_imm32(scratch_addr, a);
        self.ZwSetInformationWorkerFactory(wf_handle_cur_a, 3, scratch_addr, 4);
        self.jmp(next_block);

        self.switch_block(next_block_crit_breaker);
        self.jmp(next_block);

        for i in (0..=15).into_iter().rev() {
            self.switch_block(next_block);

            // block for next check and/or return
            let next_check = self.new_block();

            // check if bit a is set
            let a_bit_set = self.new_block();
            let a_bit_not_set = self.new_block();

            if i != 0 {
                let was_more =
                    self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, a, (1 << i) - 1);
                self.jz(was_more, a_bit_not_set);
                self.jmp(a_bit_set);
            } else {
                self.jz(a, a_bit_not_set);
                self.jmp(a_bit_set);
            }

            let is_bit_set_in_b = (b & (1 << i)) != 0;

            {
                // the bit in a is set
                self.switch_block(a_bit_set);

                // unset the bit in a
                self.write_imm32(scratch_addr, -(1i32 << i) as u32 & 0xFFFF);
                self.ZwSetInformationWorkerFactory(wf_handle_cur_a, 3, scratch_addr, 4);

                // read a
                self.ZwQueryInformationWorkerFactory(
                    wf_handle_cur_a,
                    7, /* WorkerFactoryBasicInformation */
                    wbi_addr,
                    120,
                    0,
                );
                self.add_insn(Insn::ReadMemAbs16(a, wbi_addr + 0x20));

                if !is_bit_set_in_b {
                    // the bit in a is set, the bit in b is not set -> we should set the bit in out
                    self.write_imm32(scratch_addr, 1u32 << i);
                    self.ZwSetInformationWorkerFactory(wf_handle_out, 3, scratch_addr, 4);
                } else {
                    // both bits are set -> no need to set any bit in out
                }

                self.jmp(next_check);
            }

            {
                // the bit in a is not set
                self.switch_block(a_bit_not_set);

                if is_bit_set_in_b {
                    // the bit in a is not set, the bit in b is set -> we should set the bit in out
                    self.write_imm32(scratch_addr, 1u32 << i);
                    self.ZwSetInformationWorkerFactory(wf_handle_out, 3, scratch_addr, 4);
                } else {
                    // both bits are not set -> no need to set any bit in out
                }

                self.jmp(next_check);
            }

            next_block = next_check;
        }

        // final value is in out
        self.switch_block(next_block);

        let out = self.reg();

        // read result
        self.ZwQueryInformationWorkerFactory(
            wf_handle_out,
            7, /* WorkerFactoryBasicInformation */
            wbi_addr,
            120,
            0,
        );
        self.add_insn(Insn::ReadMemAbs16(out, wbi_addr + 0x20));

        out
    }

    pub fn xor16(&mut self, a: IReg, b: IReg) -> IReg {
        let a = self.clone(a);
        let b = self.clone(b);

        // we'll need three adders
        let scratch_addr = self.append_random_data(8, 8);
        let wbi_addr = self.append_random_data(120, 8);

        let iocp_handle = self.reg();
        let wf_handle_out = self.reg();
        let wf_handle_cur_a = self.reg();
        let wf_handle_cur_b = self.reg();

        // we'll need an iocp for sure
        self.ZwCreateIoCompletion(scratch_addr, 0x1F0003u64, 0, 0);
        self.add_insn(Insn::ReadMemAbs64(iocp_handle, scratch_addr));

        // as well at three worker factories
        random_order!(
            {
                self.ZwCreateWorkerFactory(
                    scratch_addr,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_out, scratch_addr));
            }

            {
                self.ZwCreateWorkerFactory(
                    scratch_addr,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_cur_a, scratch_addr));
            }

            {
                self.ZwCreateWorkerFactory(
                    scratch_addr,
                    0xF00FF,
                    0,
                    iocp_handle,
                    u64::MAX,
                    0,
                    0,
                    u64::MAX,
                    0x1000,
                    0x1000,
                );
                self.add_insn(Insn::ReadMemAbs64(wf_handle_cur_b, scratch_addr));
            }
        );

        let initialize_a = self.new_block();
        let initialize_b = self.new_block();
        let maybe_initialize_b = self.new_block();
        let mut next_block = self.new_block();

        let maybe_initialize_b_crit_breaker = self.new_block();
        let next_block_crit_breaker = self.new_block();

        // initialize adders to current values
        self.comment("maybe initialize a");
        self.jz(a, maybe_initialize_b_crit_breaker);
        self.jmp(initialize_a);

        self.switch_block(initialize_a);
        self.comment("initialize a");
        self.write_imm32(scratch_addr, a);
        self.ZwSetInformationWorkerFactory(wf_handle_cur_a, 3, scratch_addr, 4);
        self.jmp(maybe_initialize_b);

        self.switch_block(maybe_initialize_b_crit_breaker);
        self.jmp(maybe_initialize_b);

        self.switch_block(maybe_initialize_b);
        self.comment("maybe initialize b");
        self.jz(b, next_block_crit_breaker);
        self.jmp(initialize_b);

        self.switch_block(initialize_b);
        self.comment("initialize b");
        self.write_imm32(scratch_addr, b);
        self.ZwSetInformationWorkerFactory(wf_handle_cur_b, 3, scratch_addr, 4);
        self.jmp(next_block);

        self.switch_block(next_block_crit_breaker);
        self.jmp(next_block);

        for i in (0..=16).into_iter().rev() {
            self.switch_block(next_block);

            // block for next check and/or return
            let next_check = self.new_block();

            // check if bit a is set
            let a_set_b_unk = self.new_block();
            let a_not_set_b_unk = self.new_block();

            if i != 0 {
                let was_more =
                    self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, a, (1 << i) - 1);
                self.jz(was_more, a_not_set_b_unk);
                self.jmp(a_set_b_unk);
            } else {
                self.jz(a, a_not_set_b_unk);
                self.jmp(a_set_b_unk);
            }

            {
                // the bit in a is set; we don't know about the bit in b
                self.switch_block(a_set_b_unk);

                // first, unset the bit in a
                self.write_imm32(scratch_addr, -(1i32 << i) as u32 & 0xFFFF);
                self.ZwSetInformationWorkerFactory(wf_handle_cur_a, 3, scratch_addr, 4);

                // read a
                self.ZwQueryInformationWorkerFactory(
                    wf_handle_cur_a,
                    7, /* WorkerFactoryBasicInformation */
                    wbi_addr,
                    120,
                    0,
                );
                self.add_insn(Insn::ReadMemAbs16(a, wbi_addr + 0x20));

                // next, check b
                let a_set_b_set = self.new_block();
                let a_set_b_not_set = self.new_block();

                if i != 0 {
                    let was_more =
                        self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, b, (1 << i) - 1);
                    self.jz(was_more, a_set_b_not_set);
                    self.jmp(a_set_b_set);
                } else {
                    self.jz(b, a_set_b_not_set);
                    self.jmp(a_set_b_set);
                }

                {
                    // both bits are set; unset b
                    self.switch_block(a_set_b_set);

                    self.write_imm32(scratch_addr, -(1i32 << i) as u32 & 0xFFFF);
                    self.ZwSetInformationWorkerFactory(wf_handle_cur_b, 3, scratch_addr, 4);

                    // read b
                    self.ZwQueryInformationWorkerFactory(
                        wf_handle_cur_b,
                        7, /* WorkerFactoryBasicInformation */
                        wbi_addr,
                        120,
                        0,
                    );
                    self.add_insn(Insn::ReadMemAbs16(b, wbi_addr + 0x20));

                    // no need to set bit, both are set
                    self.jmp(next_check);
                }

                {
                    // the bit in a is set; the bit in b is not set
                    // set the bit in out
                    self.switch_block(a_set_b_not_set);

                    self.write_imm32(scratch_addr, 1u32 << i);
                    self.ZwSetInformationWorkerFactory(wf_handle_out, 3, scratch_addr, 4);

                    self.jmp(next_check);
                }
            }

            {
                // the bit in a is not set; we don't know about the bit in b
                self.switch_block(a_not_set_b_unk);

                // check the b bit
                let a_not_set_b_set = self.new_block();
                let a_not_set_b_not_set = self.new_block();

                if i != 0 {
                    let was_more =
                        self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, b, (1 << i) - 1);
                    self.jz(was_more, a_not_set_b_not_set);
                    self.jmp(a_not_set_b_set);
                } else {
                    self.jz(b, a_not_set_b_not_set);
                    self.jmp(a_not_set_b_set);
                }

                {
                    // the bit in a is not set; the bit in b is set
                    // set the bit in out
                    // unset the bit in b
                    self.switch_block(a_not_set_b_set);

                    self.write_imm32(scratch_addr, 1u32 << i);
                    self.ZwSetInformationWorkerFactory(wf_handle_out, 3, scratch_addr, 4);

                    self.write_imm32(scratch_addr, -(1i32 << i) as u32 & 0xFFFF);
                    self.ZwSetInformationWorkerFactory(wf_handle_cur_b, 3, scratch_addr, 4);

                    // read b
                    self.ZwQueryInformationWorkerFactory(
                        wf_handle_cur_b,
                        7, /* WorkerFactoryBasicInformation */
                        wbi_addr,
                        120,
                        0,
                    );
                    self.add_insn(Insn::ReadMemAbs16(b, wbi_addr + 0x20));

                    self.jmp(next_check);
                }

                {
                    // neither bit is set
                    self.switch_block(a_not_set_b_not_set);
                    self.jmp(next_check);
                }
            }

            next_block = next_check;
        }

        // final value is in out
        self.switch_block(next_block);

        let out = self.reg();

        // read result
        self.ZwQueryInformationWorkerFactory(
            wf_handle_out,
            7, /* WorkerFactoryBasicInformation */
            wbi_addr,
            120,
            0,
        );
        self.add_insn(Insn::ReadMemAbs16(out, wbi_addr + 0x20));

        out
    }
}
