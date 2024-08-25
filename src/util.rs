use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

use crate::RAND;

#[macro_export]
macro_rules! random_order {
    ($fst:tt $snd:tt $($tail:tt)*) => {
        if $crate::util::random_bool() {
            $fst;
            random_order!($snd $($tail)*);
        } else {
            $snd;
            random_order!($fst $($tail)*);
        }
    };
    ($head:tt) => { $head };
    () => {};
}

pub fn random_bool() -> bool {
    let mut rng = RAND.get().unwrap().lock().unwrap();
    rng.gen_bool(0.5)
}

pub fn shuffle<T>(slice: &mut [T]) {
    let mut rng = RAND.get().unwrap().lock().unwrap();
    slice.shuffle(&mut *rng);
}

pub fn fill_random(slice: &mut [u8]) {
    let mut rng = RAND.get().unwrap().lock().unwrap();
    rng.fill_bytes(slice);
}
