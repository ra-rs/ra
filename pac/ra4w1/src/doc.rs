#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DOC Control Register"]
    pub docr: DOCR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - DOC Data Input Register"]
    pub dodir: DODIR,
    #[doc = "0x04 - DOC Data Setting Register"]
    pub dodsr: DODSR,
}
#[doc = "DOCR (rw) register accessor: an alias for `Reg<DOCR_SPEC>`"]
pub type DOCR = crate::Reg<docr::DOCR_SPEC>;
#[doc = "DOC Control Register"]
pub mod docr;
#[doc = "DODIR (rw) register accessor: an alias for `Reg<DODIR_SPEC>`"]
pub type DODIR = crate::Reg<dodir::DODIR_SPEC>;
#[doc = "DOC Data Input Register"]
pub mod dodir;
#[doc = "DODSR (rw) register accessor: an alias for `Reg<DODSR_SPEC>`"]
pub type DODSR = crate::Reg<dodsr::DODSR_SPEC>;
#[doc = "DOC Data Setting Register"]
pub mod dodsr;
