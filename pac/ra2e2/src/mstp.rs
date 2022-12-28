#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Stop Control Register B"]
    pub mstpcrb: MSTPCRB,
    #[doc = "0x04 - Module Stop Control Register C"]
    pub mstpcrc: MSTPCRC,
    #[doc = "0x08 - Module Stop Control Register D"]
    pub mstpcrd: MSTPCRD,
    #[doc = "0x0c - Low Speed Module R/W Disable Control Register"]
    pub lsmrwdis: LSMRWDIS,
}
#[doc = "MSTPCRB (rw) register accessor: an alias for `Reg<MSTPCRB_SPEC>`"]
pub type MSTPCRB = crate::Reg<mstpcrb::MSTPCRB_SPEC>;
#[doc = "Module Stop Control Register B"]
pub mod mstpcrb;
#[doc = "MSTPCRC (rw) register accessor: an alias for `Reg<MSTPCRC_SPEC>`"]
pub type MSTPCRC = crate::Reg<mstpcrc::MSTPCRC_SPEC>;
#[doc = "Module Stop Control Register C"]
pub mod mstpcrc;
#[doc = "MSTPCRD (rw) register accessor: an alias for `Reg<MSTPCRD_SPEC>`"]
pub type MSTPCRD = crate::Reg<mstpcrd::MSTPCRD_SPEC>;
#[doc = "Module Stop Control Register D"]
pub mod mstpcrd;
#[doc = "LSMRWDIS (rw) register accessor: an alias for `Reg<LSMRWDIS_SPEC>`"]
pub type LSMRWDIS = crate::Reg<lsmrwdis::LSMRWDIS_SPEC>;
#[doc = "Low Speed Module R/W Disable Control Register"]
pub mod lsmrwdis;
