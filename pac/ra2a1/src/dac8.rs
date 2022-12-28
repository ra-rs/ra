#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Conversion Value Setting Register %s"]
    pub dacs: [DACS; 2],
    _reserved1: [u8; 0x01],
    #[doc = "0x03 - D/A Converter Mode Register"]
    pub dam: DAM,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - D/A A/D Synchronous Start Control Register"]
    pub dacadscr: DACADSCR,
    #[doc = "0x07 - D/A SW Charge Pump Control Register"]
    pub dacpc: DACPC,
}
#[doc = "DACS (rw) register accessor: an alias for `Reg<DACS_SPEC>`"]
pub type DACS = crate::Reg<dacs::DACS_SPEC>;
#[doc = "D/A Conversion Value Setting Register %s"]
pub mod dacs;
#[doc = "DAM (rw) register accessor: an alias for `Reg<DAM_SPEC>`"]
pub type DAM = crate::Reg<dam::DAM_SPEC>;
#[doc = "D/A Converter Mode Register"]
pub mod dam;
#[doc = "DACADSCR (rw) register accessor: an alias for `Reg<DACADSCR_SPEC>`"]
pub type DACADSCR = crate::Reg<dacadscr::DACADSCR_SPEC>;
#[doc = "D/A A/D Synchronous Start Control Register"]
pub mod dacadscr;
#[doc = "DACPC (rw) register accessor: an alias for `Reg<DACPC_SPEC>`"]
pub type DACPC = crate::Reg<dacpc::DACPC_SPEC>;
#[doc = "D/A SW Charge Pump Control Register"]
pub mod dacpc;
