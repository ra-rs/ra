#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Trigonometric Status Register"]
    pub trgsts: TRGSTS,
    _reserved1: [u8; 0x07],
    #[doc = "0x10 - Sine Cosine Data Register 0"]
    pub scdt0: SCDT0,
    #[doc = "0x14 - Sine Cosine Data Register 1"]
    pub scdt1: SCDT1,
    #[doc = "0x18 - Arctangent Data Register 0"]
    pub atdt0: ATDT0,
    #[doc = "0x1c - Arctangent Data Register 1"]
    pub atdt1: ATDT1,
}
#[doc = "TRGSTS (r) register accessor: an alias for `Reg<TRGSTS_SPEC>`"]
pub type TRGSTS = crate::Reg<trgsts::TRGSTS_SPEC>;
#[doc = "Trigonometric Status Register"]
pub mod trgsts;
#[doc = "SCDT0 (rw) register accessor: an alias for `Reg<SCDT0_SPEC>`"]
pub type SCDT0 = crate::Reg<scdt0::SCDT0_SPEC>;
#[doc = "Sine Cosine Data Register 0"]
pub mod scdt0;
#[doc = "SCDT1 (rw) register accessor: an alias for `Reg<SCDT1_SPEC>`"]
pub type SCDT1 = crate::Reg<scdt1::SCDT1_SPEC>;
#[doc = "Sine Cosine Data Register 1"]
pub mod scdt1;
#[doc = "ATDT0 (rw) register accessor: an alias for `Reg<ATDT0_SPEC>`"]
pub type ATDT0 = crate::Reg<atdt0::ATDT0_SPEC>;
#[doc = "Arctangent Data Register 0"]
pub mod atdt0;
#[doc = "ATDT1 (rw) register accessor: an alias for `Reg<ATDT1_SPEC>`"]
pub type ATDT1 = crate::Reg<atdt1::ATDT1_SPEC>;
#[doc = "Arctangent Data Register 1"]
pub mod atdt1;
