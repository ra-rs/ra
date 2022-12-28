#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x017c],
    #[doc = "0x17c - Temperature Sensor Calibration Data Register"]
    pub tscdr: TSCDR,
}
#[doc = "TSCDR (r) register accessor: an alias for `Reg<TSCDR_SPEC>`"]
pub type TSCDR = crate::Reg<tscdr::TSCDR_SPEC>;
#[doc = "Temperature Sensor Calibration Data Register"]
pub mod tscdr;
