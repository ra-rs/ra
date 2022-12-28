#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - Flash Cache Enable Register"]
    pub fcachee: FCACHEE,
    _reserved1: [u8; 0x02],
    #[doc = "0x104 - Flash Cache Invalidate Register"]
    pub fcacheiv: FCACHEIV,
}
#[doc = "FCACHEE (rw) register accessor: an alias for `Reg<FCACHEE_SPEC>`"]
pub type FCACHEE = crate::Reg<fcachee::FCACHEE_SPEC>;
#[doc = "Flash Cache Enable Register"]
pub mod fcachee;
#[doc = "FCACHEIV (rw) register accessor: an alias for `Reg<FCACHEIV_SPEC>`"]
pub type FCACHEIV = crate::Reg<fcacheiv::FCACHEIV_SPEC>;
#[doc = "Flash Cache Invalidate Register"]
pub mod fcacheiv;
