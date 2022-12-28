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
    _reserved7: [u8; 0x02],
    #[doc = "0x20 - Event Link Setting Register %s"]
    pub elsr4: ELSR,
    _reserved8: [u8; 0x02],
    #[doc = "0x24 - Event Link Setting Register %s"]
    pub elsr5: ELSR,
    _reserved9: [u8; 0x02],
    #[doc = "0x28 - Event Link Setting Register %s"]
    pub elsr6: ELSR,
    _reserved10: [u8; 0x02],
    #[doc = "0x2c - Event Link Setting Register %s"]
    pub elsr7: ELSR,
    _reserved11: [u8; 0x02],
    #[doc = "0x30 - Event Link Setting Register %s"]
    pub elsr8: ELSR,
    _reserved12: [u8; 0x02],
    #[doc = "0x34 - Event Link Setting Register %s"]
    pub elsr9: ELSR,
    _reserved13: [u8; 0x02],
    #[doc = "0x38 - Event Link Setting Register %s"]
    pub elsr10: ELSR,
    _reserved14: [u8; 0x02],
    #[doc = "0x3c - Event Link Setting Register %s"]
    pub elsr11: ELSR,
    _reserved15: [u8; 0x02],
    #[doc = "0x40 - Event Link Setting Register %s"]
    pub elsr12: ELSR,
    _reserved16: [u8; 0x02],
    #[doc = "0x44 - Event Link Setting Register %s"]
    pub elsr13: ELSR,
    _reserved17: [u8; 0x02],
    #[doc = "0x48 - Event Link Setting Register %s"]
    pub elsr14: ELSR,
    _reserved18: [u8; 0x02],
    #[doc = "0x4c - Event Link Setting Register %s"]
    pub elsr15: ELSR,
    _reserved19: [u8; 0x02],
    #[doc = "0x50 - Event Link Setting Register %s"]
    pub elsr16: ELSR,
    _reserved20: [u8; 0x02],
    #[doc = "0x54 - Event Link Setting Register %s"]
    pub elsr17: ELSR,
    _reserved21: [u8; 0x02],
    #[doc = "0x58 - Event Link Setting Register %s"]
    pub elsr18: ELSR,
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
