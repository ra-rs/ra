#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DTC Control Register"]
    pub dtccr: DTCCR,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DTC Vector Base Register"]
    pub dtcvbr: DTCVBR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - DTC Module Start Register"]
    pub dtcst: DTCST,
    _reserved3: [u8; 0x01],
    #[doc = "0x0e - DTC Status Register"]
    pub dtcsts: DTCSTS,
    #[doc = "0x10 - DTC Control Register for secure Region"]
    pub dtccr_sec: DTCCR_SEC,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - DTC Vector Base Register for secure Region"]
    pub dtcvbr_sec: DTCVBR_SEC,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - DTC Error Vector Register"]
    pub dtevr: DTEVR,
}
#[doc = "DTCCR (rw) register accessor: an alias for `Reg<DTCCR_SPEC>`"]
pub type DTCCR = crate::Reg<dtccr::DTCCR_SPEC>;
#[doc = "DTC Control Register"]
pub mod dtccr;
#[doc = "DTCVBR (rw) register accessor: an alias for `Reg<DTCVBR_SPEC>`"]
pub type DTCVBR = crate::Reg<dtcvbr::DTCVBR_SPEC>;
#[doc = "DTC Vector Base Register"]
pub mod dtcvbr;
#[doc = "DTCST (rw) register accessor: an alias for `Reg<DTCST_SPEC>`"]
pub type DTCST = crate::Reg<dtcst::DTCST_SPEC>;
#[doc = "DTC Module Start Register"]
pub mod dtcst;
#[doc = "DTCSTS (r) register accessor: an alias for `Reg<DTCSTS_SPEC>`"]
pub type DTCSTS = crate::Reg<dtcsts::DTCSTS_SPEC>;
#[doc = "DTC Status Register"]
pub mod dtcsts;
#[doc = "DTCCR_SEC (rw) register accessor: an alias for `Reg<DTCCR_SEC_SPEC>`"]
pub type DTCCR_SEC = crate::Reg<dtccr_sec::DTCCR_SEC_SPEC>;
#[doc = "DTC Control Register for secure Region"]
pub mod dtccr_sec;
#[doc = "DTCVBR_SEC (rw) register accessor: an alias for `Reg<DTCVBR_SEC_SPEC>`"]
pub type DTCVBR_SEC = crate::Reg<dtcvbr_sec::DTCVBR_SEC_SPEC>;
#[doc = "DTC Vector Base Register for secure Region"]
pub mod dtcvbr_sec;
#[doc = "DTEVR (rw) register accessor: an alias for `Reg<DTEVR_SPEC>`"]
pub type DTEVR = crate::Reg<dtevr::DTEVR_SPEC>;
#[doc = "DTC Error Vector Register"]
pub mod dtevr;
