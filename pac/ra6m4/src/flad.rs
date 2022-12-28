#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - Data Flash Access Frequency Register"]
    pub fckmhz: FCKMHZ,
}
#[doc = "FCKMHZ (rw) register accessor: an alias for `Reg<FCKMHZ_SPEC>`"]
pub type FCKMHZ = crate::Reg<fckmhz::FCKMHZ_SPEC>;
#[doc = "Data Flash Access Frequency Register"]
pub mod fckmhz;
