#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operational amplifier mode control register"]
    pub ampmc: AMPMC,
    #[doc = "0x01 - Operational amplifier trigger mode control register"]
    pub amptrm: AMPTRM,
    #[doc = "0x02 - Operational Amplifier Activation Trigger Select Register"]
    pub amptrs: AMPTRS,
    #[doc = "0x03 - Operational amplifier control register"]
    pub ampc: AMPC,
    #[doc = "0x04 - Operational amplifier monitor register"]
    pub ampmon: AMPMON,
    _reserved5: [u8; 0x01],
    #[doc = "0x06 - Operational Amplifier 0 Output Select Register"]
    pub amp0os: AMP0OS,
    #[doc = "0x07 - Operational Amplifier 0 Minus Input Select Register"]
    pub amp0ms: AMP0MS,
    #[doc = "0x08 - Operational Amplifier 0 Plus Input Select Register"]
    pub amp0ps: AMP0PS,
    _reserved8: [u8; 0x01],
    #[doc = "0x0a - Operational Amplifier 1 Minus Input Select Register"]
    pub amp1ms: AMP1MS,
    #[doc = "0x0b - Operational Amplifier 1 Plus Input Select Register"]
    pub amp1ps: AMP1PS,
    _reserved10: [u8; 0x01],
    #[doc = "0x0d - Operational Amplifier 2 Minus Input Select Register"]
    pub amp2ms: AMP2MS,
    #[doc = "0x0e - Operational Amplifier 2 Plus Input Select Register"]
    pub amp2ps: AMP2PS,
    _reserved12: [u8; 0x03],
    #[doc = "0x12 - Operational Amplifier Switch Charge Pump Control Register"]
    pub ampcpc: AMPCPC,
    _reserved13: [u8; 0x04],
    #[doc = "0x17 - Operational Amplifier User Offset Trimming Enable Register"]
    pub ampuote: AMPUOTE,
    #[doc = "0x18 - Operational Amplifier 0 Offset Trimming Pch Register"]
    pub amp0otp: AMP0OTP,
    #[doc = "0x19 - Operational Amplifier 0 Offset Trimming Nch Register"]
    pub amp0otn: AMP0OTN,
    #[doc = "0x1a - Operational Amplifier 1 Offset Trimming Pch Register"]
    pub amp1otp: AMP1OTP,
    #[doc = "0x1b - Operational Amplifier 1 Offset Trimming Nch Register"]
    pub amp1otn: AMP1OTN,
    #[doc = "0x1c - Operational Amplifier 2 Offset Trimming Pch Register"]
    pub amp2otp: AMP2OTP,
    #[doc = "0x1d - Operational Amplifier 2 Offset Trimming Nch Register"]
    pub amp2otn: AMP2OTN,
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
#[doc = "AMP0OS (rw) register accessor: an alias for `Reg<AMP0OS_SPEC>`"]
pub type AMP0OS = crate::Reg<amp0os::AMP0OS_SPEC>;
#[doc = "Operational Amplifier 0 Output Select Register"]
pub mod amp0os;
#[doc = "AMP0MS (rw) register accessor: an alias for `Reg<AMP0MS_SPEC>`"]
pub type AMP0MS = crate::Reg<amp0ms::AMP0MS_SPEC>;
#[doc = "Operational Amplifier 0 Minus Input Select Register"]
pub mod amp0ms;
#[doc = "AMP0PS (rw) register accessor: an alias for `Reg<AMP0PS_SPEC>`"]
pub type AMP0PS = crate::Reg<amp0ps::AMP0PS_SPEC>;
#[doc = "Operational Amplifier 0 Plus Input Select Register"]
pub mod amp0ps;
#[doc = "AMP1MS (rw) register accessor: an alias for `Reg<AMP1MS_SPEC>`"]
pub type AMP1MS = crate::Reg<amp1ms::AMP1MS_SPEC>;
#[doc = "Operational Amplifier 1 Minus Input Select Register"]
pub mod amp1ms;
#[doc = "AMP1PS (rw) register accessor: an alias for `Reg<AMP1PS_SPEC>`"]
pub type AMP1PS = crate::Reg<amp1ps::AMP1PS_SPEC>;
#[doc = "Operational Amplifier 1 Plus Input Select Register"]
pub mod amp1ps;
#[doc = "AMP2MS (rw) register accessor: an alias for `Reg<AMP2MS_SPEC>`"]
pub type AMP2MS = crate::Reg<amp2ms::AMP2MS_SPEC>;
#[doc = "Operational Amplifier 2 Minus Input Select Register"]
pub mod amp2ms;
#[doc = "AMP2PS (rw) register accessor: an alias for `Reg<AMP2PS_SPEC>`"]
pub type AMP2PS = crate::Reg<amp2ps::AMP2PS_SPEC>;
#[doc = "Operational Amplifier 2 Plus Input Select Register"]
pub mod amp2ps;
#[doc = "AMPCPC (rw) register accessor: an alias for `Reg<AMPCPC_SPEC>`"]
pub type AMPCPC = crate::Reg<ampcpc::AMPCPC_SPEC>;
#[doc = "Operational Amplifier Switch Charge Pump Control Register"]
pub mod ampcpc;
#[doc = "AMPUOTE (rw) register accessor: an alias for `Reg<AMPUOTE_SPEC>`"]
pub type AMPUOTE = crate::Reg<ampuote::AMPUOTE_SPEC>;
#[doc = "Operational Amplifier User Offset Trimming Enable Register"]
pub mod ampuote;
#[doc = "AMP0OTP (rw) register accessor: an alias for `Reg<AMP0OTP_SPEC>`"]
pub type AMP0OTP = crate::Reg<amp0otp::AMP0OTP_SPEC>;
#[doc = "Operational Amplifier 0 Offset Trimming Pch Register"]
pub mod amp0otp;
#[doc = "AMP0OTN (rw) register accessor: an alias for `Reg<AMP0OTN_SPEC>`"]
pub type AMP0OTN = crate::Reg<amp0otn::AMP0OTN_SPEC>;
#[doc = "Operational Amplifier 0 Offset Trimming Nch Register"]
pub mod amp0otn;
#[doc = "AMP1OTP (rw) register accessor: an alias for `Reg<AMP1OTP_SPEC>`"]
pub type AMP1OTP = crate::Reg<amp1otp::AMP1OTP_SPEC>;
#[doc = "Operational Amplifier 1 Offset Trimming Pch Register"]
pub mod amp1otp;
#[doc = "AMP1OTN (rw) register accessor: an alias for `Reg<AMP1OTN_SPEC>`"]
pub type AMP1OTN = crate::Reg<amp1otn::AMP1OTN_SPEC>;
#[doc = "Operational Amplifier 1 Offset Trimming Nch Register"]
pub mod amp1otn;
#[doc = "AMP2OTP (rw) register accessor: an alias for `Reg<AMP2OTP_SPEC>`"]
pub type AMP2OTP = crate::Reg<amp2otp::AMP2OTP_SPEC>;
#[doc = "Operational Amplifier 2 Offset Trimming Pch Register"]
pub mod amp2otp;
#[doc = "AMP2OTN (rw) register accessor: an alias for `Reg<AMP2OTN_SPEC>`"]
pub type AMP2OTN = crate::Reg<amp2otn::AMP2OTN_SPEC>;
#[doc = "Operational Amplifier 2 Offset Trimming Nch Register"]
pub mod amp2otn;
