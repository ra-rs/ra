#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Cache Enable Register"]
    pub fcachee: FCACHEE,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Flash Cache Invalidate Register"]
    pub fcacheiv: FCACHEIV,
    _reserved2: [u8; 0x16],
    #[doc = "0x1c - Flash Wait Cycle Register"]
    pub flwt: FLWT,
    _reserved3: [u8; 0x23],
    #[doc = "0x40 - Flash Security Attribution Register"]
    pub fsar: FSAR,
}
#[doc = "FCACHEE (rw) register accessor: an alias for `Reg<FCACHEE_SPEC>`"]
pub type FCACHEE = crate::Reg<fcachee::FCACHEE_SPEC>;
#[doc = "Flash Cache Enable Register"]
pub mod fcachee;
#[doc = "FCACHEIV (rw) register accessor: an alias for `Reg<FCACHEIV_SPEC>`"]
pub type FCACHEIV = crate::Reg<fcacheiv::FCACHEIV_SPEC>;
#[doc = "Flash Cache Invalidate Register"]
pub mod fcacheiv;
#[doc = "FLWT (rw) register accessor: an alias for `Reg<FLWT_SPEC>`"]
pub type FLWT = crate::Reg<flwt::FLWT_SPEC>;
#[doc = "Flash Wait Cycle Register"]
pub mod flwt;
#[doc = "FSAR (rw) register accessor: an alias for `Reg<FSAR_SPEC>`"]
pub type FSAR = crate::Reg<fsar::FSAR_SPEC>;
#[doc = "Flash Security Attribution Register"]
pub mod fsar;
