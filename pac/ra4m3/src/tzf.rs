#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TrustZone Filter Operation After Detection Register"]
    pub tzfoad: TZFOAD,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TrustZone Filter Protect Register"]
    pub tzfpt: TZFPT,
}
#[doc = "TZFOAD (rw) register accessor: an alias for `Reg<TZFOAD_SPEC>`"]
pub type TZFOAD = crate::Reg<tzfoad::TZFOAD_SPEC>;
#[doc = "TrustZone Filter Operation After Detection Register"]
pub mod tzfoad;
#[doc = "TZFPT (rw) register accessor: an alias for `Reg<TZFPT_SPEC>`"]
pub type TZFPT = crate::Reg<tzfpt::TZFPT_SPEC>;
#[doc = "TrustZone Filter Protect Register"]
pub mod tzfpt;
