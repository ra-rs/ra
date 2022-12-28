#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Link Controller Register"]
    pub elcr: ELCR,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Event Link Software Event Generation Register %s"]
    pub elsegr0: ELSEGR,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Event Link Software Event Generation Register %s"]
    pub elsegr1: ELSEGR,
    _reserved3: [u8; 0x17],
    #[doc = "0x20 - Event Link Setting Register %s"]
    pub elsr0: ELSR,
    _reserved4: [u8; 0x02],
    #[doc = "0x24 - Event Link Setting Register %s"]
    pub elsr1: ELSR,
    _reserved5: [u8; 0x02],
    #[doc = "0x28 - Event Link Setting Register %s"]
    pub elsr2: ELSR,
    _reserved6: [u8; 0x02],
    #[doc = "0x2c - Event Link Setting Register %s"]
    pub elsr3: ELSR,
    _reserved7: [u8; 0x02],
    #[doc = "0x30 - Event Link Setting Register %s"]
    pub elsr4: ELSR,
    _reserved8: [u8; 0x02],
    #[doc = "0x34 - Event Link Setting Register %s"]
    pub elsr5: ELSR,
    _reserved9: [u8; 0x02],
    #[doc = "0x38 - Event Link Setting Register %s"]
    pub elsr6: ELSR,
    _reserved10: [u8; 0x02],
    #[doc = "0x3c - Event Link Setting Register %s"]
    pub elsr7: ELSR,
    _reserved11: [u8; 0x12],
    #[doc = "0x50 - Event Link Setting Register %s"]
    pub elsr12: ELSR12,
    _reserved12: [u8; 0x02],
    #[doc = "0x54 - Event Link Setting Register %s"]
    pub elsr13: ELSR12,
    _reserved13: [u8; 0x02],
    #[doc = "0x58 - Event Link Setting Register %s"]
    pub elsr14: ELSR12,
    _reserved14: [u8; 0x02],
    #[doc = "0x5c - Event Link Setting Register %s"]
    pub elsr15: ELSR12,
    _reserved15: [u8; 0x02],
    #[doc = "0x60 - Event Link Setting Register %s"]
    pub elsr16: ELSR12,
    _reserved16: [u8; 0x02],
    #[doc = "0x64 - Event Link Setting Register %s"]
    pub elsr17: ELSR12,
    _reserved17: [u8; 0x06],
    #[doc = "0x6c - Event Link Setting Register %s"]
    pub elsr19: ELSR19,
    _reserved18: [u8; 0x02],
    #[doc = "0x70 - Event Link Setting Register %s"]
    pub elsr20: ELSR19,
    _reserved19: [u8; 0x02],
    #[doc = "0x74 - Event Link Setting Register %s"]
    pub elsr21: ELSR19,
    _reserved20: [u8; 0x02],
    #[doc = "0x78 - Event Link Setting Register %s"]
    pub elsr22: ELSR19,
    _reserved21: [u8; 0x02],
    #[doc = "0x7c - Event Link Setting Register %s"]
    pub elsr23: ELSR19,
    _reserved22: [u8; 0x02],
    #[doc = "0x80 - Event Link Setting Register %s"]
    pub elsr24: ELSR19,
    _reserved23: [u8; 0x0e],
    #[doc = "0x90 - Event Link Setting Register %s"]
    pub elsr28: ELSR28,
    _reserved24: [u8; 0x02],
    #[doc = "0x94 - Event Link Setting Register %s"]
    pub elsr29: ELSR28,
    _reserved25: [u8; 0x4a],
    #[doc = "0xe0 - Event Link Controller Security Attribution Register A"]
    pub elcsara: ELCSARA,
    #[doc = "0xe4 - Event Link Controller Security Attribution Register B"]
    pub elcsarb: ELCSARB,
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
pub use elsr as elsr12;
pub use elsr as elsr19;
pub use elsr as elsr28;
pub use ELSR as ELSR12;
pub use ELSR as ELSR19;
pub use ELSR as ELSR28;
#[doc = "ELCSARA (rw) register accessor: an alias for `Reg<ELCSARA_SPEC>`"]
pub type ELCSARA = crate::Reg<elcsara::ELCSARA_SPEC>;
#[doc = "Event Link Controller Security Attribution Register A"]
pub mod elcsara;
#[doc = "ELCSARB (rw) register accessor: an alias for `Reg<ELCSARB_SPEC>`"]
pub type ELCSARB = crate::Reg<elcsarb::ELCSARB_SPEC>;
#[doc = "Event Link Controller Security Attribution Register B"]
pub mod elcsarb;
