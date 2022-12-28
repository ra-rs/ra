#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POEG Group A Setting Register"]
    pub poegga: POEGGA,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40 - GPT Output Stopping Control Group A Write Protection Register"]
    pub gtoncwpa: GTONCWPA,
    _reserved2: [u8; 0x02],
    #[doc = "0x44 - GPT Output Stopping Control Group A Controlling Register"]
    pub gtonccra: GTONCCRA,
    _reserved3: [u8; 0xba],
    #[doc = "0x100 - POEG Group B Setting Register"]
    pub poeggb: POEGGB,
    _reserved4: [u8; 0x3c],
    #[doc = "0x140 - GPT Output Stopping Control Group B Write Protection Register"]
    pub gtoncwpb: GTONCWPB,
    _reserved5: [u8; 0x02],
    #[doc = "0x144 - GPT Output Stopping Control Group B Controlling Register"]
    pub gtonccrb: GTONCCRB,
    _reserved6: [u8; 0xba],
    #[doc = "0x200 - POEG Group C Setting Register"]
    pub poeggc: POEGGC,
    _reserved7: [u8; 0x3c],
    #[doc = "0x240 - GPT Output Stopping Control Group C Write Protection Register"]
    pub gtoncwpc: GTONCWPC,
    _reserved8: [u8; 0x02],
    #[doc = "0x244 - GPT Output Stopping Control Group C Controlling Register"]
    pub gtonccrc: GTONCCRC,
    _reserved9: [u8; 0xba],
    #[doc = "0x300 - POEG Group D Setting Register"]
    pub poeggd: POEGGD,
    _reserved10: [u8; 0x3c],
    #[doc = "0x340 - GPT Output Stopping Control Group D Write Protection Register"]
    pub gtoncwpd: GTONCWPD,
    _reserved11: [u8; 0x02],
    #[doc = "0x344 - GPT Output Stopping Control Group D Controlling Register"]
    pub gtonccrd: GTONCCRD,
}
#[doc = "POEGGA (rw) register accessor: an alias for `Reg<POEGGA_SPEC>`"]
pub type POEGGA = crate::Reg<poegga::POEGGA_SPEC>;
#[doc = "POEG Group A Setting Register"]
pub mod poegga;
#[doc = "GTONCWPA (rw) register accessor: an alias for `Reg<GTONCWPA_SPEC>`"]
pub type GTONCWPA = crate::Reg<gtoncwpa::GTONCWPA_SPEC>;
#[doc = "GPT Output Stopping Control Group A Write Protection Register"]
pub mod gtoncwpa;
#[doc = "GTONCCRA (rw) register accessor: an alias for `Reg<GTONCCRA_SPEC>`"]
pub type GTONCCRA = crate::Reg<gtonccra::GTONCCRA_SPEC>;
#[doc = "GPT Output Stopping Control Group A Controlling Register"]
pub mod gtonccra;
#[doc = "POEGGB (rw) register accessor: an alias for `Reg<POEGGB_SPEC>`"]
pub type POEGGB = crate::Reg<poeggb::POEGGB_SPEC>;
#[doc = "POEG Group B Setting Register"]
pub mod poeggb;
#[doc = "GTONCWPB (rw) register accessor: an alias for `Reg<GTONCWPB_SPEC>`"]
pub type GTONCWPB = crate::Reg<gtoncwpb::GTONCWPB_SPEC>;
#[doc = "GPT Output Stopping Control Group B Write Protection Register"]
pub mod gtoncwpb;
#[doc = "GTONCCRB (rw) register accessor: an alias for `Reg<GTONCCRB_SPEC>`"]
pub type GTONCCRB = crate::Reg<gtonccrb::GTONCCRB_SPEC>;
#[doc = "GPT Output Stopping Control Group B Controlling Register"]
pub mod gtonccrb;
#[doc = "POEGGC (rw) register accessor: an alias for `Reg<POEGGC_SPEC>`"]
pub type POEGGC = crate::Reg<poeggc::POEGGC_SPEC>;
#[doc = "POEG Group C Setting Register"]
pub mod poeggc;
#[doc = "GTONCWPC (rw) register accessor: an alias for `Reg<GTONCWPC_SPEC>`"]
pub type GTONCWPC = crate::Reg<gtoncwpc::GTONCWPC_SPEC>;
#[doc = "GPT Output Stopping Control Group C Write Protection Register"]
pub mod gtoncwpc;
#[doc = "GTONCCRC (rw) register accessor: an alias for `Reg<GTONCCRC_SPEC>`"]
pub type GTONCCRC = crate::Reg<gtonccrc::GTONCCRC_SPEC>;
#[doc = "GPT Output Stopping Control Group C Controlling Register"]
pub mod gtonccrc;
#[doc = "POEGGD (rw) register accessor: an alias for `Reg<POEGGD_SPEC>`"]
pub type POEGGD = crate::Reg<poeggd::POEGGD_SPEC>;
#[doc = "POEG Group D Setting Register"]
pub mod poeggd;
#[doc = "GTONCWPD (rw) register accessor: an alias for `Reg<GTONCWPD_SPEC>`"]
pub type GTONCWPD = crate::Reg<gtoncwpd::GTONCWPD_SPEC>;
#[doc = "GPT Output Stopping Control Group D Write Protection Register"]
pub mod gtoncwpd;
#[doc = "GTONCCRD (rw) register accessor: an alias for `Reg<GTONCCRD_SPEC>`"]
pub type GTONCCRD = crate::Reg<gtonccrd::GTONCCRD_SPEC>;
#[doc = "GPT Output Stopping Control Group D Controlling Register"]
pub mod gtonccrd;
