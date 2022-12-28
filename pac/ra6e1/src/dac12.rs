#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Data Register n"]
    pub dadrn: DADRN,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - D/A Control Register"]
    pub dacr: DACR,
    #[doc = "0x05 - DADR Format Select Register"]
    pub dadpr: DADPR,
    #[doc = "0x06 - D/A A/D Synchronous Start Control Register"]
    pub daadscr: DAADSCR,
    _reserved4: [u8; 0x01],
    #[doc = "0x08 - D/A Output Amplifier Control Register"]
    pub daampcr: DAAMPCR,
    _reserved5: [u8; 0x13],
    #[doc = "0x1c - D/A Amplifier Stabilization Wait Control Register"]
    pub daaswcr: DAASWCR,
    _reserved6: [u8; 0x10a3],
    #[doc = "0x10c0 - D/A A/D Synchronous Unit Select Register"]
    pub daadusr: DAADUSR,
}
#[doc = "DADRn (rw) register accessor: an alias for `Reg<DADRN_SPEC>`"]
pub type DADRN = crate::Reg<dadrn::DADRN_SPEC>;
#[doc = "D/A Data Register n"]
pub mod dadrn;
#[doc = "DACR (rw) register accessor: an alias for `Reg<DACR_SPEC>`"]
pub type DACR = crate::Reg<dacr::DACR_SPEC>;
#[doc = "D/A Control Register"]
pub mod dacr;
#[doc = "DADPR (rw) register accessor: an alias for `Reg<DADPR_SPEC>`"]
pub type DADPR = crate::Reg<dadpr::DADPR_SPEC>;
#[doc = "DADR Format Select Register"]
pub mod dadpr;
#[doc = "DAADSCR (rw) register accessor: an alias for `Reg<DAADSCR_SPEC>`"]
pub type DAADSCR = crate::Reg<daadscr::DAADSCR_SPEC>;
#[doc = "D/A A/D Synchronous Start Control Register"]
pub mod daadscr;
#[doc = "DAAMPCR (rw) register accessor: an alias for `Reg<DAAMPCR_SPEC>`"]
pub type DAAMPCR = crate::Reg<daampcr::DAAMPCR_SPEC>;
#[doc = "D/A Output Amplifier Control Register"]
pub mod daampcr;
#[doc = "DAASWCR (rw) register accessor: an alias for `Reg<DAASWCR_SPEC>`"]
pub type DAASWCR = crate::Reg<daaswcr::DAASWCR_SPEC>;
#[doc = "D/A Amplifier Stabilization Wait Control Register"]
pub mod daaswcr;
#[doc = "DAADUSR (rw) register accessor: an alias for `Reg<DAADUSR_SPEC>`"]
pub type DAADUSR = crate::Reg<daadusr::DAADUSR_SPEC>;
#[doc = "D/A A/D Synchronous Unit Select Register"]
pub mod daadusr;
