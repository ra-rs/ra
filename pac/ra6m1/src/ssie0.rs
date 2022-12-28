#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ssicr: SSICR,
    #[doc = "0x04 - Status Register"]
    pub ssisr: SSISR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - FIFO Control Register"]
    pub ssifcr: SSIFCR,
    #[doc = "0x14 - FIFO Status Register"]
    pub ssifsr: SSIFSR,
    #[doc = "0x18 - Transmit FIFO Data Register"]
    pub ssiftdr: SSIFTDR,
    #[doc = "0x1c - Receive FIFO Data Register"]
    pub ssifrdr: SSIFRDR,
    #[doc = "0x20 - Audio Format Register"]
    pub ssiofr: SSIOFR,
    #[doc = "0x24 - Status Control Register"]
    pub ssiscr: SSISCR,
}
#[doc = "SSICR (rw) register accessor: an alias for `Reg<SSICR_SPEC>`"]
pub type SSICR = crate::Reg<ssicr::SSICR_SPEC>;
#[doc = "Control Register"]
pub mod ssicr;
#[doc = "SSISR (rw) register accessor: an alias for `Reg<SSISR_SPEC>`"]
pub type SSISR = crate::Reg<ssisr::SSISR_SPEC>;
#[doc = "Status Register"]
pub mod ssisr;
#[doc = "SSIFCR (rw) register accessor: an alias for `Reg<SSIFCR_SPEC>`"]
pub type SSIFCR = crate::Reg<ssifcr::SSIFCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod ssifcr;
#[doc = "SSIFSR (rw) register accessor: an alias for `Reg<SSIFSR_SPEC>`"]
pub type SSIFSR = crate::Reg<ssifsr::SSIFSR_SPEC>;
#[doc = "FIFO Status Register"]
pub mod ssifsr;
#[doc = "SSIFTDR (w) register accessor: an alias for `Reg<SSIFTDR_SPEC>`"]
pub type SSIFTDR = crate::Reg<ssiftdr::SSIFTDR_SPEC>;
#[doc = "Transmit FIFO Data Register"]
pub mod ssiftdr;
#[doc = "SSIFRDR (r) register accessor: an alias for `Reg<SSIFRDR_SPEC>`"]
pub type SSIFRDR = crate::Reg<ssifrdr::SSIFRDR_SPEC>;
#[doc = "Receive FIFO Data Register"]
pub mod ssifrdr;
#[doc = "SSIOFR (rw) register accessor: an alias for `Reg<SSIOFR_SPEC>`"]
pub type SSIOFR = crate::Reg<ssiofr::SSIOFR_SPEC>;
#[doc = "Audio Format Register"]
pub mod ssiofr;
#[doc = "SSISCR (rw) register accessor: an alias for `Reg<SSISCR_SPEC>`"]
pub type SSISCR = crate::Reg<ssiscr::SSISCR_SPEC>;
#[doc = "Status Control Register"]
pub mod ssiscr;
