#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Link Controller Register"]
    pub elcr: ELCR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Event Link Software Event Generation Register %s"]
    pub elsegr0: ELSEGR,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Event Link Software Event Generation Register %s"]
    pub elsegr1: ELSEGR,
    _reserved3: [u8; 0x0b],
    #[doc = "0x10 - Event Link Setting Register %s"]
    pub elsr0: ELSR,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Event Link Setting Register %s"]
    pub elsr1: ELSR,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - Event Link Setting Register %s"]
    pub elsr2: ELSR,
    _reserved6: [u8; 0x02],
    #[doc = "0x1c - Event Link Setting Register %s"]
    pub elsr3: ELSR,
    _reserved7: [u8; 0x12],
    #[doc = "0x30 - Event Link Setting Register %s"]
    pub elsr8: ELSR8,
    _reserved8: [u8; 0x02],
    #[doc = "0x34 - Event Link Setting Register %s"]
    pub elsr9: ELSR8,
    _reserved9: [u8; 0x12],
    #[doc = "0x48 - Event Link Setting Register %s"]
    pub elsr14: ELSR14,
    _reserved10: [u8; 0x02],
    #[doc = "0x4c - Event Link Setting Register %s"]
    pub elsr15: ELSR14,
}
#[doc = "ELCR (rw) register accessor: an alias for `Reg<ELCR_SPEC>`"]
pub type ELCR = crate::Reg<elcr::ELCR_SPEC>;
#[doc = "Event Link Controller Register"]
pub mod elcr;
#[doc = "ELSEGR (rw) register accessor: an alias for `Reg<ELSEGR_SPEC>`"]
pub type ELSEGR = crate::Reg<elsegr::ELSEGR_SPEC>;
#[doc = "Event Link Software Event Generation Register %s"]
pub mod elsegr;
#[doc = "ELSR (rw) register accessor: an alias for `Reg<ELSR_SPEC>`"]
pub type ELSR = crate::Reg<elsr::ELSR_SPEC>;
#[doc = "Event Link Setting Register %s"]
pub mod elsr;
pub use elsr as elsr8;
pub use elsr as elsr14;
pub use ELSR as ELSR8;
pub use ELSR as ELSR14;
