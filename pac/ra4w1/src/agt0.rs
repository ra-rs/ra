#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AGT Counter Register"]
    pub agt: AGT,
    #[doc = "0x02 - AGT Compare Match A Register"]
    pub agtcma: AGTCMA,
    #[doc = "0x04 - AGT Compare Match B Register"]
    pub agtcmb: AGTCMB,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - AGT Control Register"]
    pub agtcr: AGTCR,
    #[doc = "0x09 - AGT Mode Register 1"]
    pub agtmr1: AGTMR1,
    #[doc = "0x0a - AGT Mode Register 2"]
    pub agtmr2: AGTMR2,
    _reserved6: [u8; 0x01],
    #[doc = "0x0c - AGT I/O Control Register"]
    pub agtioc: AGTIOC,
    #[doc = "0x0d - AGT Event Pin Select Register"]
    pub agtisr: AGTISR,
    #[doc = "0x0e - AGT Compare Match Function Select Register"]
    pub agtcmsr: AGTCMSR,
    #[doc = "0x0f - AGT Pin Select Register"]
    pub agtiosel: AGTIOSEL,
}
#[doc = "AGT (rw) register accessor: an alias for `Reg<AGT_SPEC>`"]
pub type AGT = crate::Reg<agt::AGT_SPEC>;
#[doc = "AGT Counter Register"]
pub mod agt;
#[doc = "AGTCMA (rw) register accessor: an alias for `Reg<AGTCMA_SPEC>`"]
pub type AGTCMA = crate::Reg<agtcma::AGTCMA_SPEC>;
#[doc = "AGT Compare Match A Register"]
pub mod agtcma;
#[doc = "AGTCMB (rw) register accessor: an alias for `Reg<AGTCMB_SPEC>`"]
pub type AGTCMB = crate::Reg<agtcmb::AGTCMB_SPEC>;
#[doc = "AGT Compare Match B Register"]
pub mod agtcmb;
#[doc = "AGTCR (rw) register accessor: an alias for `Reg<AGTCR_SPEC>`"]
pub type AGTCR = crate::Reg<agtcr::AGTCR_SPEC>;
#[doc = "AGT Control Register"]
pub mod agtcr;
#[doc = "AGTMR1 (rw) register accessor: an alias for `Reg<AGTMR1_SPEC>`"]
pub type AGTMR1 = crate::Reg<agtmr1::AGTMR1_SPEC>;
#[doc = "AGT Mode Register 1"]
pub mod agtmr1;
#[doc = "AGTMR2 (rw) register accessor: an alias for `Reg<AGTMR2_SPEC>`"]
pub type AGTMR2 = crate::Reg<agtmr2::AGTMR2_SPEC>;
#[doc = "AGT Mode Register 2"]
pub mod agtmr2;
#[doc = "AGTIOC (rw) register accessor: an alias for `Reg<AGTIOC_SPEC>`"]
pub type AGTIOC = crate::Reg<agtioc::AGTIOC_SPEC>;
#[doc = "AGT I/O Control Register"]
pub mod agtioc;
#[doc = "AGTISR (rw) register accessor: an alias for `Reg<AGTISR_SPEC>`"]
pub type AGTISR = crate::Reg<agtisr::AGTISR_SPEC>;
#[doc = "AGT Event Pin Select Register"]
pub mod agtisr;
#[doc = "AGTCMSR (rw) register accessor: an alias for `Reg<AGTCMSR_SPEC>`"]
pub type AGTCMSR = crate::Reg<agtcmsr::AGTCMSR_SPEC>;
#[doc = "AGT Compare Match Function Select Register"]
pub mod agtcmsr;
#[doc = "AGTIOSEL (rw) register accessor: an alias for `Reg<AGTIOSEL_SPEC>`"]
pub type AGTIOSEL = crate::Reg<agtiosel::AGTIOSEL_SPEC>;
#[doc = "AGT Pin Select Register"]
pub mod agtiosel;
