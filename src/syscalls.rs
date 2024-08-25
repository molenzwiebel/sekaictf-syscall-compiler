use std::collections::HashMap;

use object::Object;
use rand::RngCore;

use crate::RAND;

pub struct SyscallIndices {
    indices: HashMap<String, u32>,
}

// number of random bits in the syscall index
const FIXED_BITS: u64 = 12 + 2;
const RANDOM_BITS: u64 = 64 - FIXED_BITS; // 12 bits for the syscall index, 2 bits for the syscall type
const RANDOM_BITS_MASK: u64 = (1 << RANDOM_BITS) - 1;

impl SyscallIndices {
    pub fn from_ntdll(path: &str) -> SyscallIndices {
        let contents = std::fs::read(path).unwrap();
        let contents: &[u8] = contents.as_ref();
        let pe = object::read::pe::PeFile64::parse(contents).unwrap();

        let mut exports = pe.exports().unwrap();
        exports.sort_by_cached_key(|x| x.address());

        let mut i = 0;
        let mut out = HashMap::new();

        for export in exports {
            let name = export.name();
            let name = std::str::from_utf8(name).unwrap();
            if !name.starts_with("Zw") {
                continue;
            }

            out.insert(name.to_string(), i);
            i += 1;
        }

        SyscallIndices { indices: out }
    }

    pub fn from_table(path: &str) -> SyscallIndices {
        let contents = std::fs::read_to_string(path).unwrap();

        let mut out = HashMap::new();

        for line in contents.lines() {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap();
            let num = parts.next().unwrap().parse().unwrap();

            out.insert(name.to_string().replace("Nt", "Zw"), num);
        }

        SyscallIndices { indices: out }
    }

    pub fn get(&self, name: &str) -> u32 {
        self.indices
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("unknown syscall: {}", name))
    }

    pub fn get_randomized(&self, name: &str) -> u64 {
        let mut rng = RAND.get().unwrap().lock().unwrap();
        let random_bits = rng.next_u64() & RANDOM_BITS_MASK;

        self.get(name) as u64 | (random_bits << FIXED_BITS)
    }
}
