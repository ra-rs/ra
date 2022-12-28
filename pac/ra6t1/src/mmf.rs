#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MemMirror Special Function Register"]
    pub mmsfr: MMSFR,
    #[doc = "0x04 - MemMirror Enable Register"]
    pub mmen: MMEN,
}
#[doc = "MMSFR (rw) register accessor: an alias for `Reg<MMSFR_SPEC>`"]
pub type MMSFR = crate::Reg<mmsfr::MMSFR_SPEC>;
#[doc = "MemMirror Special Function Register"]
pub mod mmsfr;
#[doc = "MMEN (rw) register accessor: an alias for `Reg<MMEN_SPEC>`"]
pub type MMEN = crate::Reg<mmen::MMEN_SPEC>;
#[doc = "MemMirror Enable Register"]
pub mod mmen;
