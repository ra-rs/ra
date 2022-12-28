#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Output Delay Control Register"]
    pub gtdlycr: GTDLYCR,
    #[doc = "0x02 - PWM Output Delay Control Register2"]
    pub gtdlycr2: GTDLYCR2,
    _reserved2: [u8; 0x14],
    #[doc = "0x18 - GTIOC%sA Rising Output Delay Register"]
    pub gtdlyr0a: GTDLYRA,
    #[doc = "0x1a - GTIOC%sB Rising Output Delay Register"]
    pub gtdlyr0b: GTDLYRB,
    #[doc = "0x1c - GTIOC%sA Rising Output Delay Register"]
    pub gtdlyr1a: GTDLYRA,
    #[doc = "0x1e - GTIOC%sB Rising Output Delay Register"]
    pub gtdlyr1b: GTDLYRB,
    #[doc = "0x20 - GTIOC%sA Rising Output Delay Register"]
    pub gtdlyr2a: GTDLYRA,
    #[doc = "0x22 - GTIOC%sB Rising Output Delay Register"]
    pub gtdlyr2b: GTDLYRB,
    #[doc = "0x24 - GTIOC%sA Rising Output Delay Register"]
    pub gtdlyr3a: GTDLYRA,
    #[doc = "0x26 - GTIOC%sB Rising Output Delay Register"]
    pub gtdlyr3b: GTDLYRB,
    #[doc = "0x28 - GTIOC%sA Falling Output Delay Register"]
    pub gtdlyf0a: GTDLYFA,
    #[doc = "0x2a - GTIOC%sB Falling Output Delay Register"]
    pub gtdlyf0b: GTDLYFB,
    #[doc = "0x2c - GTIOC%sA Falling Output Delay Register"]
    pub gtdlyf1a: GTDLYFA,
    #[doc = "0x2e - GTIOC%sB Falling Output Delay Register"]
    pub gtdlyf1b: GTDLYFB,
    #[doc = "0x30 - GTIOC%sA Falling Output Delay Register"]
    pub gtdlyf2a: GTDLYFA,
    #[doc = "0x32 - GTIOC%sB Falling Output Delay Register"]
    pub gtdlyf2b: GTDLYFB,
    #[doc = "0x34 - GTIOC%sA Falling Output Delay Register"]
    pub gtdlyf3a: GTDLYFA,
    #[doc = "0x36 - GTIOC%sB Falling Output Delay Register"]
    pub gtdlyf3b: GTDLYFB,
}
#[doc = "GTDLYCR (rw) register accessor: an alias for `Reg<GTDLYCR_SPEC>`"]
pub type GTDLYCR = crate::Reg<gtdlycr::GTDLYCR_SPEC>;
#[doc = "PWM Output Delay Control Register"]
pub mod gtdlycr;
#[doc = "GTDLYCR2 (rw) register accessor: an alias for `Reg<GTDLYCR2_SPEC>`"]
pub type GTDLYCR2 = crate::Reg<gtdlycr2::GTDLYCR2_SPEC>;
#[doc = "PWM Output Delay Control Register2"]
pub mod gtdlycr2;
#[doc = "GTDLYRA (rw) register accessor: an alias for `Reg<GTDLYRA_SPEC>`"]
pub type GTDLYRA = crate::Reg<gtdlyra::GTDLYRA_SPEC>;
#[doc = "GTIOC%sA Rising Output Delay Register"]
pub mod gtdlyra;
#[doc = "GTDLYRB (rw) register accessor: an alias for `Reg<GTDLYRB_SPEC>`"]
pub type GTDLYRB = crate::Reg<gtdlyrb::GTDLYRB_SPEC>;
#[doc = "GTIOC%sB Rising Output Delay Register"]
pub mod gtdlyrb;
#[doc = "GTDLYFA (rw) register accessor: an alias for `Reg<GTDLYFA_SPEC>`"]
pub type GTDLYFA = crate::Reg<gtdlyfa::GTDLYFA_SPEC>;
#[doc = "GTIOC%sA Falling Output Delay Register"]
pub mod gtdlyfa;
#[doc = "GTDLYFB (rw) register accessor: an alias for `Reg<GTDLYFB_SPEC>`"]
pub type GTDLYFB = crate::Reg<gtdlyfb::GTDLYFB_SPEC>;
#[doc = "GTIOC%sB Falling Output Delay Register"]
pub mod gtdlyfb;
