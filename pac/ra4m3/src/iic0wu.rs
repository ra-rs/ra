#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    #[doc = "0x02 - I2C Bus Wakeup Unit Register"]
    pub icwur: ICWUR,
    #[doc = "0x03 - I2C Bus Wakeup Unit Register 2"]
    pub icwur2: ICWUR2,
}
#[doc = "ICWUR (rw) register accessor: an alias for `Reg<ICWUR_SPEC>`"]
pub type ICWUR = crate::Reg<icwur::ICWUR_SPEC>;
#[doc = "I2C Bus Wakeup Unit Register"]
pub mod icwur;
#[doc = "ICWUR2 (rw) register accessor: an alias for `Reg<ICWUR2_SPEC>`"]
pub type ICWUR2 = crate::Reg<icwur2::ICWUR2_SPEC>;
#[doc = "I2C Bus Wakeup Unit Register 2"]
pub mod icwur2;
