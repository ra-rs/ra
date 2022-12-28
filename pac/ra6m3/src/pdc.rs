#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDC Control Register 0"]
    pub pccr0: PCCR0,
    #[doc = "0x04 - PDC Control Register 1"]
    pub pccr1: PCCR1,
    #[doc = "0x08 - PDC Status Register"]
    pub pcsr: PCSR,
    #[doc = "0x0c - PDC Pin Monitor Register"]
    pub pcmonr: PCMONR,
    #[doc = "0x10 - PDC Receive Data Register"]
    pub pcdr: PCDR,
    #[doc = "0x14 - Vertical Capture Register"]
    pub vcr: VCR,
    #[doc = "0x18 - Horizontal Capture Register"]
    pub hcr: HCR,
}
#[doc = "PCCR0 (rw) register accessor: an alias for `Reg<PCCR0_SPEC>`"]
pub type PCCR0 = crate::Reg<pccr0::PCCR0_SPEC>;
#[doc = "PDC Control Register 0"]
pub mod pccr0;
#[doc = "PCCR1 (rw) register accessor: an alias for `Reg<PCCR1_SPEC>`"]
pub type PCCR1 = crate::Reg<pccr1::PCCR1_SPEC>;
#[doc = "PDC Control Register 1"]
pub mod pccr1;
#[doc = "PCSR (rw) register accessor: an alias for `Reg<PCSR_SPEC>`"]
pub type PCSR = crate::Reg<pcsr::PCSR_SPEC>;
#[doc = "PDC Status Register"]
pub mod pcsr;
#[doc = "PCMONR (r) register accessor: an alias for `Reg<PCMONR_SPEC>`"]
pub type PCMONR = crate::Reg<pcmonr::PCMONR_SPEC>;
#[doc = "PDC Pin Monitor Register"]
pub mod pcmonr;
#[doc = "PCDR (r) register accessor: an alias for `Reg<PCDR_SPEC>`"]
pub type PCDR = crate::Reg<pcdr::PCDR_SPEC>;
#[doc = "PDC Receive Data Register"]
pub mod pcdr;
#[doc = "VCR (rw) register accessor: an alias for `Reg<VCR_SPEC>`"]
pub type VCR = crate::Reg<vcr::VCR_SPEC>;
#[doc = "Vertical Capture Register"]
pub mod vcr;
#[doc = "HCR (rw) register accessor: an alias for `Reg<HCR_SPEC>`"]
pub type HCR = crate::Reg<hcr::HCR_SPEC>;
#[doc = "Horizontal Capture Register"]
pub mod hcr;
