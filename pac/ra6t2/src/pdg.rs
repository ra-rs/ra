#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Output Delay Control Register"]
    pub gtdlycr: GTDLYCR,
    #[doc = "0x02 - PWM Output Delay Control Register 2"]
    pub gtdlycr2: GTDLYCR2,
    _reserved2: [u8; 0x14],
    #[doc = "0x18 - GTIOCnA Rising Output Delay Register"]
    pub gtdlyr0a: GTDLYRA,
    #[doc = "0x1a - GTIOCnB Rising Output Delay Register"]
    pub gtdlyr0b: GTDLYRB,
    #[doc = "0x1c - GTIOCnA Rising Output Delay Register"]
    pub gtdlyr1a: GTDLYRA,
    #[doc = "0x1e - GTIOCnB Rising Output Delay Register"]
    pub gtdlyr1b: GTDLYRB,
    #[doc = "0x20 - GTIOCnA Rising Output Delay Register"]
    pub gtdlyr2a: GTDLYRA,
    #[doc = "0x22 - GTIOCnB Rising Output Delay Register"]
    pub gtdlyr2b: GTDLYRB,
    #[doc = "0x24 - GTIOCnA Rising Output Delay Register"]
    pub gtdlyr3a: GTDLYRA,
    #[doc = "0x26 - GTIOCnB Rising Output Delay Register"]
    pub gtdlyr3b: GTDLYRB,
    #[doc = "0x28 - GTIOCnA Falling Output Delay Register"]
    pub gtdlyf0a: GTDLYFA,
    #[doc = "0x2a - GTIOCnB Falling Output Delay Register"]
    pub gtdlyf0b: GTDLYFB,
    #[doc = "0x2c - GTIOCnA Falling Output Delay Register"]
    pub gtdlyf1a: GTDLYFA,
    #[doc = "0x2e - GTIOCnB Falling Output Delay Register"]
    pub gtdlyf1b: GTDLYFB,
    #[doc = "0x30 - GTIOCnA Falling Output Delay Register"]
    pub gtdlyf2a: GTDLYFA,
    #[doc = "0x32 - GTIOCnB Falling Output Delay Register"]
    pub gtdlyf2b: GTDLYFB,
    #[doc = "0x34 - GTIOCnA Falling Output Delay Register"]
    pub gtdlyf3a: GTDLYFA,
    #[doc = "0x36 - GTIOCnB Falling Output Delay Register"]
    pub gtdlyf3b: GTDLYFB,
}
#[doc = "GTDLYCR (rw) register accessor: an alias for `Reg<GTDLYCR_SPEC>`"]
pub type GTDLYCR = crate::Reg<gtdlycr::GTDLYCR_SPEC>;
#[doc = "PWM Output Delay Control Register"]
pub mod gtdlycr;
#[doc = "GTDLYCR2 (rw) register accessor: an alias for `Reg<GTDLYCR2_SPEC>`"]
pub type GTDLYCR2 = crate::Reg<gtdlycr2::GTDLYCR2_SPEC>;
#[doc = "PWM Output Delay Control Register 2"]
pub mod gtdlycr2;
#[doc = "GTDLYRA (rw) register accessor: an alias for `Reg<GTDLYRA_SPEC>`"]
pub type GTDLYRA = crate::Reg<gtdlyra::GTDLYRA_SPEC>;
#[doc = "GTIOCnA Rising Output Delay Register"]
pub mod gtdlyra;
#[doc = "GTDLYRB (rw) register accessor: an alias for `Reg<GTDLYRB_SPEC>`"]
pub type GTDLYRB = crate::Reg<gtdlyrb::GTDLYRB_SPEC>;
#[doc = "GTIOCnB Rising Output Delay Register"]
pub mod gtdlyrb;
#[doc = "GTDLYFA (rw) register accessor: an alias for `Reg<GTDLYFA_SPEC>`"]
pub type GTDLYFA = crate::Reg<gtdlyfa::GTDLYFA_SPEC>;
#[doc = "GTIOCnA Falling Output Delay Register"]
pub mod gtdlyfa;
#[doc = "GTDLYFB (rw) register accessor: an alias for `Reg<GTDLYFB_SPEC>`"]
pub type GTDLYFB = crate::Reg<gtdlyfb::GTDLYFB_SPEC>;
#[doc = "GTIOCnB Falling Output Delay Register"]
pub mod gtdlyfb;
