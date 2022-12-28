#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT Refresh Register"]
    pub wdtrr: WDTRR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - WDT Control Register"]
    pub wdtcr: WDTCR,
    #[doc = "0x04 - WDT Status Register"]
    pub wdtsr: WDTSR,
    #[doc = "0x06 - WDT Reset Control Register"]
    pub wdtrcr: WDTRCR,
    _reserved4: [u8; 0x01],
    #[doc = "0x08 - WDT Count Stop Control Register"]
    pub wdtcstpr: WDTCSTPR,
}
#[doc = "WDTRR (rw) register accessor: an alias for `Reg<WDTRR_SPEC>`"]
pub type WDTRR = crate::Reg<wdtrr::WDTRR_SPEC>;
#[doc = "WDT Refresh Register"]
pub mod wdtrr;
#[doc = "WDTCR (rw) register accessor: an alias for `Reg<WDTCR_SPEC>`"]
pub type WDTCR = crate::Reg<wdtcr::WDTCR_SPEC>;
#[doc = "WDT Control Register"]
pub mod wdtcr;
#[doc = "WDTSR (rw) register accessor: an alias for `Reg<WDTSR_SPEC>`"]
pub type WDTSR = crate::Reg<wdtsr::WDTSR_SPEC>;
#[doc = "WDT Status Register"]
pub mod wdtsr;
#[doc = "WDTRCR (rw) register accessor: an alias for `Reg<WDTRCR_SPEC>`"]
pub type WDTRCR = crate::Reg<wdtrcr::WDTRCR_SPEC>;
#[doc = "WDT Reset Control Register"]
pub mod wdtrcr;
#[doc = "WDTCSTPR (rw) register accessor: an alias for `Reg<WDTCSTPR_SPEC>`"]
pub type WDTCSTPR = crate::Reg<wdtcstpr::WDTCSTPR_SPEC>;
#[doc = "WDT Count Stop Control Register"]
pub mod wdtcstpr;
