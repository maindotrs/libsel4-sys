#![allow(non_snake_case)]

use super::{seL4_CapRights, seL4_CapRights_new};

// adapted from https://github.com/seL4/seL4/blob/2f43788b824c9760a64b68c25452dc1ec0426a1f/libsel4/include/sel4/shared_types.h

#[allow(dead_code)]
pub unsafe fn seL4_AllRights() -> seL4_CapRights {
    seL4_CapRights_new(1, 1, 1)
}

#[allow(dead_code)]
pub unsafe fn seL4_CanRead() -> seL4_CapRights {
    seL4_CapRights_new(0, 1, 0)
}

#[allow(dead_code)]
pub unsafe fn seL4_CanWrite() -> seL4_CapRights {
    seL4_CapRights_new(0, 0, 1)
}

#[allow(dead_code)]
pub unsafe fn seL4_CanGrant() -> seL4_CapRights {
    seL4_CapRights_new(1, 0, 0)
}

#[allow(dead_code)]
pub unsafe fn seL4_NoWrite() -> seL4_CapRights {
    seL4_CapRights_new(1, 1, 0)
}

#[allow(dead_code)]
pub unsafe fn seL4_NoRead() -> seL4_CapRights {
    seL4_CapRights_new(1, 0, 1)
}

#[allow(dead_code)]
pub unsafe fn seL4_NoRights() -> seL4_CapRights {
    seL4_CapRights_new(0, 0, 0)
}
