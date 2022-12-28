#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Temperature Sensor Calibration Data Register"]
    pub tscdr: TSCDR,
}
#[doc = "TSCDR (r) register accessor: an alias for `Reg<TSCDR_SPEC>`"]
pub type TSCDR = crate::Reg<tscdr::TSCDR_SPEC>;
#[doc = "Temperature Sensor Calibration Data Register"]
pub mod tscdr;
