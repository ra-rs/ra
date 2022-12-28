#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Status Register"]
    pub dbgstr: DBGSTR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Debug Stop Control Register"]
    pub dbgstopcr: DBGSTOPCR,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Trace Control Register"]
    pub tracectr: TRACECTR,
}
#[doc = "DBGSTR (r) register accessor: an alias for `Reg<DBGSTR_SPEC>`"]
pub type DBGSTR = crate::Reg<dbgstr::DBGSTR_SPEC>;
#[doc = "Debug Status Register"]
pub mod dbgstr;
#[doc = "DBGSTOPCR (rw) register accessor: an alias for `Reg<DBGSTOPCR_SPEC>`"]
pub type DBGSTOPCR = crate::Reg<dbgstopcr::DBGSTOPCR_SPEC>;
#[doc = "Debug Stop Control Register"]
pub mod dbgstopcr;
#[doc = "TRACECTR (rw) register accessor: an alias for `Reg<TRACECTR_SPEC>`"]
pub type TRACECTR = crate::Reg<tracectr::TRACECTR_SPEC>;
#[doc = "Trace Control Register"]
pub mod tracectr;
