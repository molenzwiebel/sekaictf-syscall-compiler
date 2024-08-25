use crate::{
    insn::{BlockId, IReg, IRegOrImm, Insn, ProgramBuilder},
    random_order, util, RAND,
};
use iced_x86::code_asm::ch;
use rand::Rng;

macro_rules! speck_round {
    ($zelf:ident, $x:expr, $y:expr, $k:expr) => {
        $x = $zelf.rot16_imm($x, false, 7); // x = rotr(x, 7)
        $x = $zelf.add16($x, $y); // x += y
        $x = $zelf.xor16($x, $k); // x ^= k
        $y = $zelf.rot16_imm($y, true, 2); // y = rotl(y, 2)
        $y = $zelf.xor16($y, $x); // y ^= x
    };
}

impl ProgramBuilder {
    pub fn speck_block(&mut self, plaintext: [IReg; 2], key: [IReg; 4]) -> [IReg; 2] {
        let mut ciphertext = [plaintext[0], plaintext[1]];

        let mut b = key[0];
        let mut a0 = key[1];
        let mut a1 = key[2];
        let mut a2 = key[3];

        const ROUNDS: usize = 22;
        for i in 0..ROUNDS {
            speck_round!(self, ciphertext[1], ciphertext[0], b);

            let mut a = self.clone(a0);
            let i_k = self.imm(i as u64);
            speck_round!(self, a, b, i_k);

            a0 = a1;
            a1 = a2;
            a2 = a;
        }

        ciphertext
    }

    // same as speck block, but with immediate keys so the xor can be inlined
    pub fn speck_block_imm(&mut self, plaintext: [IReg; 2], key: [u16; 4]) -> [IReg; 2] {
        let mut ciphertext = [plaintext[0], plaintext[1]];

        let mut b = key[0];
        let mut a0 = key[1];
        let mut a1 = key[2];
        let mut a2 = key[3];

        const ROUNDS: usize = 22;
        for i in 0..ROUNDS {
            let round_key = b;

            // do round
            ciphertext[1] = self.rot16_imm(ciphertext[1], false, 7); // x = rotr(x, 7)
            ciphertext[1] = self.add16(ciphertext[1], ciphertext[0]); // x += y
            ciphertext[1] = self.xor16_imm(ciphertext[1], round_key); // x ^= k
            ciphertext[0] = self.rot16_imm(ciphertext[0], true, 2); // y = rotl(y, 2)
            ciphertext[0] = self.xor16(ciphertext[0], ciphertext[1]); // y ^= x

            // compute round key for next round
            let mut a = a0;
            speck32_64_round(&mut a, &mut b, i as u16);

            a0 = a1;
            a1 = a2;
            a2 = a;
        }

        ciphertext
    }

    pub fn speck_block_it(&mut self, plaintext: [IReg; 2], key: [IReg; 4]) -> [IReg; 2] {
        let ciphertext = [self.reg(), self.reg()];
        self.mov(ciphertext[0], plaintext[0]);
        self.mov(ciphertext[1], plaintext[1]);

        let b = self.clone(key[0]);
        let a0 = self.clone(key[1]);
        let a1 = self.clone(key[2]);
        let a2 = self.clone(key[3]);

        let i = self.imm(0);
        let scratch_addr = self.append_random_data(8, 8);

        let body = self.new_block();
        let body_crit_break = self.new_block();

        let after = self.new_block();
        let after_crit_break = self.new_block();

        self.jmp(body);

        self.switch_block(body_crit_break);
        self.jmp(body);

        const ROUNDS: usize = 22;
        {
            self.switch_block(body);

            // speck_round!(self, ciphertext[1], ciphertext[0], b);
            {
                let x = ciphertext[1];
                let y = ciphertext[0];
                let k = b;

                let x = self.rot16_imm(x, false, 7); // x = rotr(x, 7)
                let x = self.add16(x, y); // x += y
                let x = self.xor16(x, k); // x ^= k
                let y = self.rot16_imm(y, true, 2); // y = rotl(y, 2)
                let y = self.xor16(y, x); // y ^= x

                self.mov(ciphertext[1], x);
                self.mov(ciphertext[0], y);
            }

            let a = self.clone(a0);

            {
                let x = a;
                let y = b;
                let k = i;

                let x = self.rot16_imm(x, false, 7); // x = rotr(x, 7)
                let x = self.add16(x, y); // x += y
                let x = self.xor16(x, k); // x ^= k
                let y = self.rot16_imm(y, true, 2); // y = rotl(y, 2)
                let y = self.xor16(y, x); // y ^= x

                self.mov(a, x);
                self.mov(b, y);
            }

            self.mov(a0, a1);
            self.mov(a1, a2);
            self.mov(a2, a);

            let one = self.imm(1);
            let new_i = self.add16(i, one);
            self.mov(i, new_i);

            let was_less = self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, ROUNDS as u64, i);
            self.jz(was_less, after_crit_break);
            self.jmp(body_crit_break);
        }

        self.switch_block(after_crit_break);
        self.jmp(after);

        self.switch_block(after);

        ciphertext
    }

    pub fn check_eq(&mut self, a: IReg, b: impl Into<IRegOrImm<u64>> + Copy) -> (BlockId, BlockId) {
        let cur = self.current_block();

        let equal = self.new_block();
        let not_equal = self.new_block();

        let next_check = self.new_block();

        let equal_crit_breaker = self.new_block();
        let not_equal_crit_breaker1 = self.new_block();
        let not_equal_crit_breaker2 = self.new_block();

        let scratch_addr = self.append_random_data(8, 8);

        self.comment("check_eq: a <= b");
        let check_a = self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, a, b);
        self.jz(check_a, next_check);
        self.jmp(not_equal_crit_breaker1);

        self.switch_block(next_check);
        self.comment("check_eq: b <= a");
        let check_b = self.ZwCreateSemaphore(scratch_addr, 0x1F0003u64, 0, b, a);
        self.jz(check_b, equal_crit_breaker);
        self.jmp(not_equal_crit_breaker2);

        self.switch_block(not_equal_crit_breaker1);
        self.jmp(not_equal);

        self.switch_block(not_equal_crit_breaker2);
        self.jmp(not_equal);

        self.switch_block(equal_crit_breaker);
        self.jmp(equal);

        self.switch_block(cur);
        (equal, not_equal)
    }

    pub fn challenge(&mut self, flag: &str, version: &str) {
        assert!(flag.len() % 4 == 0);

        let chunks = {
            let mut check_indices = (0..flag.len()).into_iter().collect::<Vec<_>>();
            util::shuffle(&mut check_indices);

            check_indices
                .chunks_exact(4)
                .map(|x| x.into_iter().copied().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        };

        let input_addr = self.append_random_data(flag.len(), 1);

        self.print(&format!("Welcome to my secure crackme, now powered by secure ring-0 encryption services (patent pending).\nThis executable requires {version} to work correctly. Check your OS version with `winver` and download a different version from the SekaiCTF website if necessary.\n\nEnter the secret code:\n"));
        self.read(input_addr, flag.len() as u64);

        self.print("\nVerifying...\n");

        // create single counter to check for number of incorrect blocks
        let scratch_addr = self.append_random_data(8, 8);
        let wbi_addr = self.append_random_data(120, 8);
        let one = self.append_data(&1u32.to_le_bytes(), 4);

        let iocp_handle = self.reg();
        let incorrect_block_handle = self.reg();

        self.ZwCreateIoCompletion(scratch_addr, 0x1F0003u64, 0, 0);
        self.add_insn(Insn::ReadMemAbs64(iocp_handle, scratch_addr));

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
        self.add_insn(Insn::ReadMemAbs64(incorrect_block_handle, scratch_addr));

        // check chunks in order
        for char_indices in chunks {
            let plaintext = [
                u16::from_le_bytes([
                    flag.as_bytes()[char_indices[0]],
                    flag.as_bytes()[char_indices[1]],
                ]),
                u16::from_le_bytes([
                    flag.as_bytes()[char_indices[2]],
                    flag.as_bytes()[char_indices[3]],
                ]),
            ];

            let key = {
                let mut rand = RAND.get().unwrap().lock().unwrap();

                [
                    rand.gen::<u16>(),
                    rand.gen::<u16>(),
                    rand.gen::<u16>(),
                    rand.gen::<u16>(),
                ]
            };

            let expected = speck32_64(plaintext, key);
            println!("checking chunk of character indices {:?} (plaintext = {:x?}) with key {:x?}, expected result is {:x?}", char_indices, plaintext, key, expected);

            let input = [self.reg(), self.reg()];

            // write addresses to the addresses in memory so you can't just xref for the flag buffer address (lmao)
            let obfuscated_addrs = char_indices
                .iter()
                .map(|&x| {
                    let addr = input_addr + x as u64;
                    // swap addr half-half
                    let addr = addr << 32 | addr >> 32;

                    // random bytes, put the real address at an unaligned address
                    let mut data = [0u8; 17];
                    util::fill_random(&mut data);
                    data[5..][..8].copy_from_slice(&addr.to_le_bytes());

                    let chunk_addr = self.append_data(&data, 1);
                    chunk_addr + 5 // lmao unaligned
                })
                .collect::<Vec<_>>();
            println!("{:x?}", obfuscated_addrs);

            random_order!(
                {
                    let reg1 = self.reg();
                    let reg2 = self.reg();

                    self.add_insn(Insn::ReadMemIndirect(reg1, vec![
                        (IRegOrImm::Imm(obfuscated_addrs[0]), 4, 4),
                        (IRegOrImm::Imm(obfuscated_addrs[0] + 4), 4, 0),
                    ]));

                    self.add_insn(Insn::ReadMemIndirect(reg2, vec![
                        (IRegOrImm::Imm(obfuscated_addrs[1]), 4, 4),
                        (IRegOrImm::Imm(obfuscated_addrs[1] + 4), 4, 0),
                    ]));

                    self.add_insn(Insn::ReadMemIndirect(input[0], vec![
                        (IRegOrImm::IReg(reg1), 1, 0),
                        (IRegOrImm::IReg(reg2), 1, 1),
                    ]));
                }

                {
                    let reg1 = self.reg();
                    let reg2 = self.reg();

                    self.add_insn(Insn::ReadMemIndirect(reg1, vec![
                        (IRegOrImm::Imm(obfuscated_addrs[2]), 4, 4),
                        (IRegOrImm::Imm(obfuscated_addrs[2] + 4), 4, 0),
                    ]));

                    self.add_insn(Insn::ReadMemIndirect(reg2, vec![
                        (IRegOrImm::Imm(obfuscated_addrs[3]), 4, 4),
                        (IRegOrImm::Imm(obfuscated_addrs[3] + 4), 4, 0),
                    ]));

                    self.add_insn(Insn::ReadMemIndirect(input[1], vec![
                        (IRegOrImm::IReg(reg1), 1, 0),
                        (IRegOrImm::IReg(reg2), 1, 1),
                    ]));
                }
            );

            let output = self.speck_block_imm(input, key);

            let mut next_check = self.new_block();
            self.jmp(next_check);

            for j in 0..2 {
                self.switch_block(next_check);

                let check_next_block = self.new_block();
                let (equal, not_equal) = self.check_eq(output[j], expected[j] as u64);

                let add_one = self.new_block();

                self.switch_block(not_equal);
                self.comment("block is invalid");
                self.jmp(add_one);

                self.switch_block(add_one);
                self.ZwSetInformationWorkerFactory(incorrect_block_handle, 3, one, 4);
                self.jmp(check_next_block);

                self.switch_block(equal);
                self.comment("block is equal");
                self.jmp(check_next_block);

                next_check = check_next_block;
            }

            self.switch_block(next_check);
        }

        let correct = self.new_block();
        let incorrect = self.new_block();

        // sum is zero if all blocks match
        let sum = self.reg();
        self.ZwQueryInformationWorkerFactory(
            incorrect_block_handle,
            7, /* WorkerFactoryBasicInformation */
            wbi_addr,
            120,
            0,
        );
        self.add_insn(Insn::ReadMemAbs16(sum, wbi_addr + 0x20));
        self.jz(sum, correct);
        self.jmp(incorrect);

        self.switch_block(incorrect);
        self.print("That's incorrect! Sorry :(\n");
        let code = self.imm(1);
        self.exit(code);

        self.switch_block(correct);
        self.print("Congratulations! Submit as SEKAI{<your input here>} to receive your reward!\n");
        let code = self.imm(0);
        self.exit(code);
    }
}

fn speck32_64_round(x: &mut u16, y: &mut u16, k: u16) {
    *x = x.rotate_right(7);
    *x = x.wrapping_add(*y);
    *x ^= k;
    *y = y.rotate_left(2);
    *y ^= *x;
}

fn speck32_64(plaintext: [u16; 2], key: [u16; 4]) -> [u16; 2] {
    let mut ciphertext0 = plaintext[0];
    let mut ciphertext1 = plaintext[1];

    let mut b = key[0];
    let mut a0 = key[1];
    let mut a1 = key[2];
    let mut a2 = key[3];

    const ROUNDS: usize = 22;
    for i in 0..ROUNDS {
        speck32_64_round(&mut ciphertext1, &mut ciphertext0, b);

        let mut a = a0;
        speck32_64_round(&mut a, &mut b, i as u16);

        a0 = a1;
        a1 = a2;
        a2 = a;
    }

    [ciphertext0, ciphertext1]
}
