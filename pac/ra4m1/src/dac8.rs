#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Conversion Value Setting Register %s"]
    pub dacs: [DACS; 2],
    _reserved1: [u8; 0x01],
    #[doc = "0x03 - D/A Converter Mode Register"]
    pub dam: DAM,
}
#[doc = "DACS (rw) register accessor: an alias for `Reg<DACS_SPEC>`"]
pub type DACS = crate::Reg<dacs::DACS_SPEC>;
#[doc = "D/A Conversion Value Setting Register %s"]
pub mod dacs;
#[doc = "DAM (rw) register accessor: an alias for `Reg<DAM_SPEC>`"]
pub type DAM = crate::Reg<dam::DAM_SPEC>;
#[doc = "D/A Converter Mode Register"]
pub mod dam;
