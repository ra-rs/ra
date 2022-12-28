#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Operational amplifier mode control register"]
    pub ampmc: AMPMC,
    #[doc = "0x09 - Operational amplifier trigger mode control register"]
    pub amptrm: AMPTRM,
    #[doc = "0x0a - Operational Amplifier Activation Trigger Select Register"]
    pub amptrs: AMPTRS,
    #[doc = "0x0b - Operational amplifier control register"]
    pub ampc: AMPC,
    #[doc = "0x0c - Operational amplifier monitor register"]
    pub ampmon: AMPMON,
}
#[doc = "AMPMC (rw) register accessor: an alias for `Reg<AMPMC_SPEC>`"]
pub type AMPMC = crate::Reg<ampmc::AMPMC_SPEC>;
#[doc = "Operational amplifier mode control register"]
pub mod ampmc;
#[doc = "AMPTRM (rw) register accessor: an alias for `Reg<AMPTRM_SPEC>`"]
pub type AMPTRM = crate::Reg<amptrm::AMPTRM_SPEC>;
#[doc = "Operational amplifier trigger mode control register"]
pub mod amptrm;
#[doc = "AMPTRS (rw) register accessor: an alias for `Reg<AMPTRS_SPEC>`"]
pub type AMPTRS = crate::Reg<amptrs::AMPTRS_SPEC>;
#[doc = "Operational Amplifier Activation Trigger Select Register"]
pub mod amptrs;
#[doc = "AMPC (rw) register accessor: an alias for `Reg<AMPC_SPEC>`"]
pub type AMPC = crate::Reg<ampc::AMPC_SPEC>;
#[doc = "Operational amplifier control register"]
pub mod ampc;
#[doc = "AMPMON (r) register accessor: an alias for `Reg<AMPMON_SPEC>`"]
pub type AMPMON = crate::Reg<ampmon::AMPMON_SPEC>;
#[doc = "Operational amplifier monitor register"]
pub mod ampmon;
