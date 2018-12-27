#![allow(non_snake_case)]

use super::seL4_Word;

// adapted from https://github.com/seL4/seL4/blob/ffbb278305d0ee322f6bea25739c5a4e08b7d807/include/util.h

#[allow(dead_code)]
pub fn BIT(bits: u8) -> seL4_Word {
    1 << bits
}
