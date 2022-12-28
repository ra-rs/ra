#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Data Register"]
    pub srcid: SRCID,
    #[doc = "0x04 - Output Data Register"]
    pub srcod: SRCOD,
    #[doc = "0x08 - Input Data Control Register"]
    pub srcidctrl: SRCIDCTRL,
    #[doc = "0x0a - Output Data Control Register"]
    pub srcodctrl: SRCODCTRL,
    #[doc = "0x0c - Control Register"]
    pub srcctrl: SRCCTRL,
    #[doc = "0x0e - Status Register"]
    pub srcstat: SRCSTAT,
}
#[doc = "SRCID (w) register accessor: an alias for `Reg<SRCID_SPEC>`"]
pub type SRCID = crate::Reg<srcid::SRCID_SPEC>;
#[doc = "Input Data Register"]
pub mod srcid;
#[doc = "SRCOD (r) register accessor: an alias for `Reg<SRCOD_SPEC>`"]
pub type SRCOD = crate::Reg<srcod::SRCOD_SPEC>;
#[doc = "Output Data Register"]
pub mod srcod;
#[doc = "SRCIDCTRL (rw) register accessor: an alias for `Reg<SRCIDCTRL_SPEC>`"]
pub type SRCIDCTRL = crate::Reg<srcidctrl::SRCIDCTRL_SPEC>;
#[doc = "Input Data Control Register"]
pub mod srcidctrl;
#[doc = "SRCODCTRL (rw) register accessor: an alias for `Reg<SRCODCTRL_SPEC>`"]
pub type SRCODCTRL = crate::Reg<srcodctrl::SRCODCTRL_SPEC>;
#[doc = "Output Data Control Register"]
pub mod srcodctrl;
#[doc = "SRCCTRL (rw) register accessor: an alias for `Reg<SRCCTRL_SPEC>`"]
pub type SRCCTRL = crate::Reg<srcctrl::SRCCTRL_SPEC>;
#[doc = "Control Register"]
pub mod srcctrl;
#[doc = "SRCSTAT (rw) register accessor: an alias for `Reg<SRCSTAT_SPEC>`"]
pub type SRCSTAT = crate::Reg<srcstat::SRCSTAT_SPEC>;
#[doc = "Status Register"]
pub mod srcstat;
