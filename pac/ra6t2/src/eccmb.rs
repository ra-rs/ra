#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ECC Control Register"]
    pub ec710ctl: EC710CTL,
    #[doc = "0x04 - ECC Test Mode Control Register"]
    pub ec710tmc: EC710TMC,
    _reserved2: [u8; 0x06],
    #[doc = "0x0c - ECC Test Substitute Data Register"]
    pub ec710ted: EC710TED,
    #[doc = "0x10 - ECC Error Address Register"]
    pub ec710ead0: EC710EAD0,
}
#[doc = "EC710CTL (rw) register accessor: an alias for `Reg<EC710CTL_SPEC>`"]
pub type EC710CTL = crate::Reg<ec710ctl::EC710CTL_SPEC>;
#[doc = "ECC Control Register"]
pub mod ec710ctl;
#[doc = "EC710TMC (rw) register accessor: an alias for `Reg<EC710TMC_SPEC>`"]
pub type EC710TMC = crate::Reg<ec710tmc::EC710TMC_SPEC>;
#[doc = "ECC Test Mode Control Register"]
pub mod ec710tmc;
#[doc = "EC710TED (rw) register accessor: an alias for `Reg<EC710TED_SPEC>`"]
pub type EC710TED = crate::Reg<ec710ted::EC710TED_SPEC>;
#[doc = "ECC Test Substitute Data Register"]
pub mod ec710ted;
#[doc = "EC710EAD0 (r) register accessor: an alias for `Reg<EC710EAD0_SPEC>`"]
pub type EC710EAD0 = crate::Reg<ec710ead0::EC710EAD0_SPEC>;
#[doc = "ECC Error Address Register"]
pub mod ec710ead0;
