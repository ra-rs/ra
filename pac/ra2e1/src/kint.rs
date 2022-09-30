#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key Return Control Register"]
    pub krctl: KRCTL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Key Return Flag Register"]
    pub krf: KRF,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Key Return Mode Register"]
    pub krm: KRM,
}
#[doc = "KRCTL (rw) register accessor: an alias for `Reg<KRCTL_SPEC>`"]
pub type KRCTL = crate::Reg<krctl::KRCTL_SPEC>;
#[doc = "Key Return Control Register"]
pub mod krctl;
#[doc = "KRF (rw) register accessor: an alias for `Reg<KRF_SPEC>`"]
pub type KRF = crate::Reg<krf::KRF_SPEC>;
#[doc = "Key Return Flag Register"]
pub mod krf;
#[doc = "KRM (rw) register accessor: an alias for `Reg<KRM_SPEC>`"]
pub type KRM = crate::Reg<krm::KRM_SPEC>;
#[doc = "Key Return Mode Register"]
pub mod krm;
