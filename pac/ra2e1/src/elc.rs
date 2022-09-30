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
    #[doc = "0x10 - Event Link Setting Register 0"]
    pub elsr0: ELSR0,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Event Link Setting Register 1"]
    pub elsr1: ELSR1,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - Event Link Setting Register 2"]
    pub elsr2: ELSR2,
    _reserved6: [u8; 0x02],
    #[doc = "0x1c - Event Link Setting Register 3"]
    pub elsr3: ELSR3,
    _reserved7: [u8; 0x12],
    #[doc = "0x30 - Event Link Setting Register 8"]
    pub elsr8: ELSR8,
    _reserved8: [u8; 0x02],
    #[doc = "0x34 - Event Link Setting Register 9"]
    pub elsr9: ELSR9,
    _reserved9: [u8; 0x0a],
    #[doc = "0x40 - Event Link Setting Register 12"]
    pub elsr12: ELSR12,
    _reserved10: [u8; 0x06],
    #[doc = "0x48 - Event Link Setting Register 14"]
    pub elsr14: ELSR14,
    _reserved11: [u8; 0x02],
    #[doc = "0x4c - Event Link Setting Register 15"]
    pub elsr15: ELSR15,
    _reserved12: [u8; 0x0a],
    #[doc = "0x58 - Event Link Setting Register 18"]
    pub elsr18: ELSR18,
}
#[doc = "ELCR (rw) register accessor: an alias for `Reg<ELCR_SPEC>`"]
pub type ELCR = crate::Reg<elcr::ELCR_SPEC>;
#[doc = "Event Link Controller Register"]
pub mod elcr;
#[doc = "ELSEGR (rw) register accessor: an alias for `Reg<ELSEGR_SPEC>`"]
pub type ELSEGR = crate::Reg<elsegr::ELSEGR_SPEC>;
#[doc = "Event Link Software Event Generation Register %s"]
pub mod elsegr;
#[doc = "ELSR0 (rw) register accessor: an alias for `Reg<ELSR0_SPEC>`"]
pub type ELSR0 = crate::Reg<elsr0::ELSR0_SPEC>;
#[doc = "Event Link Setting Register 0"]
pub mod elsr0;
#[doc = "ELSR1 (rw) register accessor: an alias for `Reg<ELSR1_SPEC>`"]
pub type ELSR1 = crate::Reg<elsr1::ELSR1_SPEC>;
#[doc = "Event Link Setting Register 1"]
pub mod elsr1;
#[doc = "ELSR2 (rw) register accessor: an alias for `Reg<ELSR2_SPEC>`"]
pub type ELSR2 = crate::Reg<elsr2::ELSR2_SPEC>;
#[doc = "Event Link Setting Register 2"]
pub mod elsr2;
#[doc = "ELSR3 (rw) register accessor: an alias for `Reg<ELSR3_SPEC>`"]
pub type ELSR3 = crate::Reg<elsr3::ELSR3_SPEC>;
#[doc = "Event Link Setting Register 3"]
pub mod elsr3;
#[doc = "ELSR8 (rw) register accessor: an alias for `Reg<ELSR8_SPEC>`"]
pub type ELSR8 = crate::Reg<elsr8::ELSR8_SPEC>;
#[doc = "Event Link Setting Register 8"]
pub mod elsr8;
#[doc = "ELSR9 (rw) register accessor: an alias for `Reg<ELSR9_SPEC>`"]
pub type ELSR9 = crate::Reg<elsr9::ELSR9_SPEC>;
#[doc = "Event Link Setting Register 9"]
pub mod elsr9;
#[doc = "ELSR12 (rw) register accessor: an alias for `Reg<ELSR12_SPEC>`"]
pub type ELSR12 = crate::Reg<elsr12::ELSR12_SPEC>;
#[doc = "Event Link Setting Register 12"]
pub mod elsr12;
#[doc = "ELSR14 (rw) register accessor: an alias for `Reg<ELSR14_SPEC>`"]
pub type ELSR14 = crate::Reg<elsr14::ELSR14_SPEC>;
#[doc = "Event Link Setting Register 14"]
pub mod elsr14;
#[doc = "ELSR15 (rw) register accessor: an alias for `Reg<ELSR15_SPEC>`"]
pub type ELSR15 = crate::Reg<elsr15::ELSR15_SPEC>;
#[doc = "Event Link Setting Register 15"]
pub mod elsr15;
#[doc = "ELSR18 (rw) register accessor: an alias for `Reg<ELSR18_SPEC>`"]
pub type ELSR18 = crate::Reg<elsr18::ELSR18_SPEC>;
#[doc = "Event Link Setting Register 18"]
pub mod elsr18;
