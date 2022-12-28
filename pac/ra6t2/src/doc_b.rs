#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DOC Control Register"]
    pub docr: DOCR,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DOC Flag Status Register"]
    pub dosr: DOSR,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - DOC Flag Status Clear Register"]
    pub doscr: DOSCR,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - DOC Data Input Register"]
    pub dodir: DODIR,
    #[doc = "0x10 - DOC Data Setting Register 0"]
    pub dodsr0: DODSR0,
    #[doc = "0x14 - DOC Data Setting Register 1"]
    pub dodsr1: DODSR1,
}
#[doc = "DOCR (rw) register accessor: an alias for `Reg<DOCR_SPEC>`"]
pub type DOCR = crate::Reg<docr::DOCR_SPEC>;
#[doc = "DOC Control Register"]
pub mod docr;
#[doc = "DOSR (r) register accessor: an alias for `Reg<DOSR_SPEC>`"]
pub type DOSR = crate::Reg<dosr::DOSR_SPEC>;
#[doc = "DOC Flag Status Register"]
pub mod dosr;
#[doc = "DOSCR (w) register accessor: an alias for `Reg<DOSCR_SPEC>`"]
pub type DOSCR = crate::Reg<doscr::DOSCR_SPEC>;
#[doc = "DOC Flag Status Clear Register"]
pub mod doscr;
#[doc = "DODIR (rw) register accessor: an alias for `Reg<DODIR_SPEC>`"]
pub type DODIR = crate::Reg<dodir::DODIR_SPEC>;
#[doc = "DOC Data Input Register"]
pub mod dodir;
#[doc = "DODSR0 (rw) register accessor: an alias for `Reg<DODSR0_SPEC>`"]
pub type DODSR0 = crate::Reg<dodsr0::DODSR0_SPEC>;
#[doc = "DOC Data Setting Register 0"]
pub mod dodsr0;
#[doc = "DODSR1 (rw) register accessor: an alias for `Reg<DODSR1_SPEC>`"]
pub type DODSR1 = crate::Reg<dodsr1::DODSR1_SPEC>;
#[doc = "DOC Data Setting Register 1"]
pub mod dodsr1;
