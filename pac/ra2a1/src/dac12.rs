#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Data Register 0"]
    pub dadr0: DADR0,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - D/A Control Register"]
    pub dacr: DACR,
    #[doc = "0x05 - DADR0 Format Select Register"]
    pub dadpr: DADPR,
    #[doc = "0x06 - D/A-A/D Synchronous Start Control Register"]
    pub daadscr: DAADSCR,
    #[doc = "0x07 - D/A VREF Control Register"]
    pub davrefcr: DAVREFCR,
    _reserved5: [u8; 0x01],
    #[doc = "0x09 - D/A Switch Charge Pump Control Register"]
    pub dapc: DAPC,
}
#[doc = "DADR0 (rw) register accessor: an alias for `Reg<DADR0_SPEC>`"]
pub type DADR0 = crate::Reg<dadr0::DADR0_SPEC>;
#[doc = "D/A Data Register 0"]
pub mod dadr0;
#[doc = "DACR (rw) register accessor: an alias for `Reg<DACR_SPEC>`"]
pub type DACR = crate::Reg<dacr::DACR_SPEC>;
#[doc = "D/A Control Register"]
pub mod dacr;
#[doc = "DADPR (rw) register accessor: an alias for `Reg<DADPR_SPEC>`"]
pub type DADPR = crate::Reg<dadpr::DADPR_SPEC>;
#[doc = "DADR0 Format Select Register"]
pub mod dadpr;
#[doc = "DAADSCR (rw) register accessor: an alias for `Reg<DAADSCR_SPEC>`"]
pub type DAADSCR = crate::Reg<daadscr::DAADSCR_SPEC>;
#[doc = "D/A-A/D Synchronous Start Control Register"]
pub mod daadscr;
#[doc = "DAVREFCR (rw) register accessor: an alias for `Reg<DAVREFCR_SPEC>`"]
pub type DAVREFCR = crate::Reg<davrefcr::DAVREFCR_SPEC>;
#[doc = "D/A VREF Control Register"]
pub mod davrefcr;
#[doc = "DAPC (rw) register accessor: an alias for `Reg<DAPC_SPEC>`"]
pub type DAPC = crate::Reg<dapc::DAPC_SPEC>;
#[doc = "D/A Switch Charge Pump Control Register"]
pub mod dapc;
