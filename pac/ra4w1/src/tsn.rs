#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0228],
    #[doc = "0x228 - Temperature Sensor Calibration Data Register H"]
    pub tscdrh: TSCDRH,
    #[doc = "0x229 - Temperature Sensor Calibration Data Register L"]
    pub tscdrl: TSCDRL,
}
#[doc = "TSCDRH (r) register accessor: an alias for `Reg<TSCDRH_SPEC>`"]
pub type TSCDRH = crate::Reg<tscdrh::TSCDRH_SPEC>;
#[doc = "Temperature Sensor Calibration Data Register H"]
pub mod tscdrh;
#[doc = "TSCDRL (r) register accessor: an alias for `Reg<TSCDRL_SPEC>`"]
pub type TSCDRL = crate::Reg<tscdrl::TSCDRL_SPEC>;
#[doc = "Temperature Sensor Calibration Data Register L"]
pub mod tscdrl;
