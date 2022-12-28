#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MMPU Operation After Detection Register"]
    pub mmpuoad: MMPUOAD,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - MMPU Operation After Detection Protect Register"]
    pub mmpuoadpt: MMPUOADPT,
    _reserved2: [u8; 0xfa],
    #[doc = "0x100 - MMPU Enable Register for DMAC"]
    pub mmpuendmac: MMPUENDMAC,
    _reserved3: [u8; 0x02],
    #[doc = "0x104 - MMPU Enable Protect Register for DMAC"]
    pub mmpuenptdmac: MMPUENPTDMAC,
    _reserved4: [u8; 0x02],
    #[doc = "0x108 - MMPU Regions Protect Register for DMAC"]
    pub mmpurptdmac: MMPURPTDMAC,
    _reserved5: [u8; 0x02],
    #[doc = "0x10c - MMPU Regions Protect register for DMAC Secure"]
    pub mmpurptdmac_sec: MMPURPTDMAC_SEC,
    _reserved6: [u8; 0xf2],
    #[doc = "0x200 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac0: MMPUACDMAC,
    _reserved7: [u8; 0x02],
    #[doc = "0x204 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac0: MMPUSDMAC,
    #[doc = "0x208 - MMPU End Address Register for DMAC"]
    pub mmpuedmac0: MMPUEDMAC,
    _reserved9: [u8; 0x04],
    #[doc = "0x210 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac1: MMPUACDMAC,
    _reserved10: [u8; 0x02],
    #[doc = "0x214 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac1: MMPUSDMAC,
    #[doc = "0x218 - MMPU End Address Register for DMAC"]
    pub mmpuedmac1: MMPUEDMAC,
    _reserved12: [u8; 0x04],
    #[doc = "0x220 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac2: MMPUACDMAC,
    _reserved13: [u8; 0x02],
    #[doc = "0x224 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac2: MMPUSDMAC,
    #[doc = "0x228 - MMPU End Address Register for DMAC"]
    pub mmpuedmac2: MMPUEDMAC,
    _reserved15: [u8; 0x04],
    #[doc = "0x230 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac3: MMPUACDMAC,
    _reserved16: [u8; 0x02],
    #[doc = "0x234 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac3: MMPUSDMAC,
    #[doc = "0x238 - MMPU End Address Register for DMAC"]
    pub mmpuedmac3: MMPUEDMAC,
    _reserved18: [u8; 0x04],
    #[doc = "0x240 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac4: MMPUACDMAC,
    _reserved19: [u8; 0x02],
    #[doc = "0x244 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac4: MMPUSDMAC,
    #[doc = "0x248 - MMPU End Address Register for DMAC"]
    pub mmpuedmac4: MMPUEDMAC,
    _reserved21: [u8; 0x04],
    #[doc = "0x250 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac5: MMPUACDMAC,
    _reserved22: [u8; 0x02],
    #[doc = "0x254 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac5: MMPUSDMAC,
    #[doc = "0x258 - MMPU End Address Register for DMAC"]
    pub mmpuedmac5: MMPUEDMAC,
    _reserved24: [u8; 0x04],
    #[doc = "0x260 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac6: MMPUACDMAC,
    _reserved25: [u8; 0x02],
    #[doc = "0x264 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac6: MMPUSDMAC,
    #[doc = "0x268 - MMPU End Address Register for DMAC"]
    pub mmpuedmac6: MMPUEDMAC,
    _reserved27: [u8; 0x04],
    #[doc = "0x270 - MMPU Access Control Register for DMAC"]
    pub mmpuacdmac7: MMPUACDMAC,
    _reserved28: [u8; 0x02],
    #[doc = "0x274 - MMPU Start Address Register for DMAC"]
    pub mmpusdmac7: MMPUSDMAC,
    #[doc = "0x278 - MMPU End Address Register for DMAC"]
    pub mmpuedmac7: MMPUEDMAC,
    _reserved30: [u8; 0x0284],
    #[doc = "0x500 - MMPU Enable Register for EDMAC"]
    pub mmpuenedmac: MMPUENEDMAC,
    _reserved31: [u8; 0x02],
    #[doc = "0x504 - MMPU Enable Protect Register for EDMAC"]
    pub mmpuenptedmac: MMPUENPTEDMAC,
    _reserved32: [u8; 0x02],
    #[doc = "0x508 - MMPU Regions Protect Register for EDMAC"]
    pub mmpurptedmac: MMPURPTEDMAC,
    _reserved33: [u8; 0xf6],
    #[doc = "0x600 - MMPU Access Control Register for EDMAC"]
    pub mmpuacedmac0: MMPUACEDMAC,
    _reserved34: [u8; 0x02],
    #[doc = "0x604 - MMPU Start Address Register for EDMAC"]
    pub mmpusedmac0: MMPUSEDMAC,
    #[doc = "0x608 - MMPU End Address Register for EDMAC"]
    pub mmpueedmac0: MMPUEEDMAC,
    _reserved36: [u8; 0x04],
    #[doc = "0x610 - MMPU Access Control Register for EDMAC"]
    pub mmpuacedmac1: MMPUACEDMAC,
    _reserved37: [u8; 0x02],
    #[doc = "0x614 - MMPU Start Address Register for EDMAC"]
    pub mmpusedmac1: MMPUSEDMAC,
    #[doc = "0x618 - MMPU End Address Register for EDMAC"]
    pub mmpueedmac1: MMPUEEDMAC,
    _reserved39: [u8; 0x04],
    #[doc = "0x620 - MMPU Access Control Register for EDMAC"]
    pub mmpuacedmac2: MMPUACEDMAC,
    _reserved40: [u8; 0x02],
    #[doc = "0x624 - MMPU Start Address Register for EDMAC"]
    pub mmpusedmac2: MMPUSEDMAC,
    #[doc = "0x628 - MMPU End Address Register for EDMAC"]
    pub mmpueedmac2: MMPUEEDMAC,
    _reserved42: [u8; 0x04],
    #[doc = "0x630 - MMPU Access Control Register for EDMAC"]
    pub mmpuacedmac3: MMPUACEDMAC,
    _reserved43: [u8; 0x02],
    #[doc = "0x634 - MMPU Start Address Register for EDMAC"]
    pub mmpusedmac3: MMPUSEDMAC,
    #[doc = "0x638 - MMPU End Address Register for EDMAC"]
    pub mmpueedmac3: MMPUEEDMAC,
}
#[doc = "MMPUOAD (rw) register accessor: an alias for `Reg<MMPUOAD_SPEC>`"]
pub type MMPUOAD = crate::Reg<mmpuoad::MMPUOAD_SPEC>;
#[doc = "MMPU Operation After Detection Register"]
pub mod mmpuoad;
#[doc = "MMPUOADPT (rw) register accessor: an alias for `Reg<MMPUOADPT_SPEC>`"]
pub type MMPUOADPT = crate::Reg<mmpuoadpt::MMPUOADPT_SPEC>;
#[doc = "MMPU Operation After Detection Protect Register"]
pub mod mmpuoadpt;
#[doc = "MMPUENDMAC (rw) register accessor: an alias for `Reg<MMPUENDMAC_SPEC>`"]
pub type MMPUENDMAC = crate::Reg<mmpuendmac::MMPUENDMAC_SPEC>;
#[doc = "MMPU Enable Register for DMAC"]
pub mod mmpuendmac;
#[doc = "MMPUENPTDMAC (rw) register accessor: an alias for `Reg<MMPUENPTDMAC_SPEC>`"]
pub type MMPUENPTDMAC = crate::Reg<mmpuenptdmac::MMPUENPTDMAC_SPEC>;
#[doc = "MMPU Enable Protect Register for DMAC"]
pub mod mmpuenptdmac;
#[doc = "MMPURPTDMAC (rw) register accessor: an alias for `Reg<MMPURPTDMAC_SPEC>`"]
pub type MMPURPTDMAC = crate::Reg<mmpurptdmac::MMPURPTDMAC_SPEC>;
#[doc = "MMPU Regions Protect Register for DMAC"]
pub mod mmpurptdmac;
#[doc = "MMPURPTDMAC_SEC (rw) register accessor: an alias for `Reg<MMPURPTDMAC_SEC_SPEC>`"]
pub type MMPURPTDMAC_SEC = crate::Reg<mmpurptdmac_sec::MMPURPTDMAC_SEC_SPEC>;
#[doc = "MMPU Regions Protect register for DMAC Secure"]
pub mod mmpurptdmac_sec;
#[doc = "MMPUACDMAC (rw) register accessor: an alias for `Reg<MMPUACDMAC_SPEC>`"]
pub type MMPUACDMAC = crate::Reg<mmpuacdmac::MMPUACDMAC_SPEC>;
#[doc = "MMPU Access Control Register for DMAC"]
pub mod mmpuacdmac;
#[doc = "MMPUSDMAC (rw) register accessor: an alias for `Reg<MMPUSDMAC_SPEC>`"]
pub type MMPUSDMAC = crate::Reg<mmpusdmac::MMPUSDMAC_SPEC>;
#[doc = "MMPU Start Address Register for DMAC"]
pub mod mmpusdmac;
#[doc = "MMPUEDMAC (rw) register accessor: an alias for `Reg<MMPUEDMAC_SPEC>`"]
pub type MMPUEDMAC = crate::Reg<mmpuedmac::MMPUEDMAC_SPEC>;
#[doc = "MMPU End Address Register for DMAC"]
pub mod mmpuedmac;
#[doc = "MMPUENEDMAC (rw) register accessor: an alias for `Reg<MMPUENEDMAC_SPEC>`"]
pub type MMPUENEDMAC = crate::Reg<mmpuenedmac::MMPUENEDMAC_SPEC>;
#[doc = "MMPU Enable Register for EDMAC"]
pub mod mmpuenedmac;
#[doc = "MMPUENPTEDMAC (rw) register accessor: an alias for `Reg<MMPUENPTEDMAC_SPEC>`"]
pub type MMPUENPTEDMAC = crate::Reg<mmpuenptedmac::MMPUENPTEDMAC_SPEC>;
#[doc = "MMPU Enable Protect Register for EDMAC"]
pub mod mmpuenptedmac;
#[doc = "MMPURPTEDMAC (rw) register accessor: an alias for `Reg<MMPURPTEDMAC_SPEC>`"]
pub type MMPURPTEDMAC = crate::Reg<mmpurptedmac::MMPURPTEDMAC_SPEC>;
#[doc = "MMPU Regions Protect Register for EDMAC"]
pub mod mmpurptedmac;
#[doc = "MMPUACEDMAC (rw) register accessor: an alias for `Reg<MMPUACEDMAC_SPEC>`"]
pub type MMPUACEDMAC = crate::Reg<mmpuacedmac::MMPUACEDMAC_SPEC>;
#[doc = "MMPU Access Control Register for EDMAC"]
pub mod mmpuacedmac;
#[doc = "MMPUSEDMAC (rw) register accessor: an alias for `Reg<MMPUSEDMAC_SPEC>`"]
pub type MMPUSEDMAC = crate::Reg<mmpusedmac::MMPUSEDMAC_SPEC>;
#[doc = "MMPU Start Address Register for EDMAC"]
pub mod mmpusedmac;
#[doc = "MMPUEEDMAC (rw) register accessor: an alias for `Reg<MMPUEEDMAC_SPEC>`"]
pub type MMPUEEDMAC = crate::Reg<mmpueedmac::MMPUEEDMAC_SPEC>;
#[doc = "MMPU End Address Register for EDMAC"]
pub mod mmpueedmac;
