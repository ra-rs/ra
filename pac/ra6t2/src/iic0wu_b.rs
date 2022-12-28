#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wake Up Unit Control Register"]
    pub wuctl: WUCTL,
    _reserved1: [u8; 0x017c],
    #[doc = "0x180 - Wake Up Unit Operating Status Register"]
    pub wust: WUST,
}
#[doc = "WUCTL (rw) register accessor: an alias for `Reg<WUCTL_SPEC>`"]
pub type WUCTL = crate::Reg<wuctl::WUCTL_SPEC>;
#[doc = "Wake Up Unit Control Register"]
pub mod wuctl;
#[doc = "WUST (r) register accessor: an alias for `Reg<WUST_SPEC>`"]
pub type WUST = crate::Reg<wust::WUST_SPEC>;
#[doc = "Wake Up Unit Operating Status Register"]
pub mod wust;
