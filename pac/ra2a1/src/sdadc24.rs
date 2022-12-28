#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Startup Control Register 1"]
    pub stc1: STC1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Startup Control Register 2"]
    pub stc2: STC2,
    _reserved2: [u8; 0x03],
    #[doc = "0x08..0x1c - Input Multiplexer %s Setting Register"]
    pub pgac: [PGAC; 5],
    #[doc = "0x1c - Sigma-delta A/D Converter Control Register 1"]
    pub adc1: ADC1,
    #[doc = "0x20 - Sigma-delta A/D Converter Control Register 2"]
    pub adc2: ADC2,
    _reserved5: [u8; 0x03],
    #[doc = "0x24 - Sigma-delta A/D Converter Conversion Result Register"]
    pub adcr: ADCR,
    #[doc = "0x28 - Sigma-delta A/D Converter Average Value Register"]
    pub adar: ADAR,
    _reserved7: [u8; 0x04],
    #[doc = "0x30 - Calibration Control Register"]
    pub clbc: CLBC,
    _reserved8: [u8; 0x03],
    #[doc = "0x34 - Calibration Start Control Register"]
    pub clbstr: CLBSTR,
    _reserved9: [u8; 0x07],
    #[doc = "0x3c - Calibration Status Register"]
    pub clbssr: CLBSSR,
}
#[doc = "STC1 (rw) register accessor: an alias for `Reg<STC1_SPEC>`"]
pub type STC1 = crate::Reg<stc1::STC1_SPEC>;
#[doc = "Startup Control Register 1"]
pub mod stc1;
#[doc = "STC2 (rw) register accessor: an alias for `Reg<STC2_SPEC>`"]
pub type STC2 = crate::Reg<stc2::STC2_SPEC>;
#[doc = "Startup Control Register 2"]
pub mod stc2;
#[doc = "PGAC (rw) register accessor: an alias for `Reg<PGAC_SPEC>`"]
pub type PGAC = crate::Reg<pgac::PGAC_SPEC>;
#[doc = "Input Multiplexer %s Setting Register"]
pub mod pgac;
#[doc = "ADC1 (rw) register accessor: an alias for `Reg<ADC1_SPEC>`"]
pub type ADC1 = crate::Reg<adc1::ADC1_SPEC>;
#[doc = "Sigma-delta A/D Converter Control Register 1"]
pub mod adc1;
#[doc = "ADC2 (rw) register accessor: an alias for `Reg<ADC2_SPEC>`"]
pub type ADC2 = crate::Reg<adc2::ADC2_SPEC>;
#[doc = "Sigma-delta A/D Converter Control Register 2"]
pub mod adc2;
#[doc = "ADCR (rw) register accessor: an alias for `Reg<ADCR_SPEC>`"]
pub type ADCR = crate::Reg<adcr::ADCR_SPEC>;
#[doc = "Sigma-delta A/D Converter Conversion Result Register"]
pub mod adcr;
#[doc = "ADAR (r) register accessor: an alias for `Reg<ADAR_SPEC>`"]
pub type ADAR = crate::Reg<adar::ADAR_SPEC>;
#[doc = "Sigma-delta A/D Converter Average Value Register"]
pub mod adar;
#[doc = "CLBC (rw) register accessor: an alias for `Reg<CLBC_SPEC>`"]
pub type CLBC = crate::Reg<clbc::CLBC_SPEC>;
#[doc = "Calibration Control Register"]
pub mod clbc;
#[doc = "CLBSTR (rw) register accessor: an alias for `Reg<CLBSTR_SPEC>`"]
pub type CLBSTR = crate::Reg<clbstr::CLBSTR_SPEC>;
#[doc = "Calibration Start Control Register"]
pub mod clbstr;
#[doc = "CLBSSR (r) register accessor: an alias for `Reg<CLBSSR_SPEC>`"]
pub type CLBSSR = crate::Reg<clbssr::CLBSSR_SPEC>;
#[doc = "Calibration Status Register"]
pub mod clbssr;
