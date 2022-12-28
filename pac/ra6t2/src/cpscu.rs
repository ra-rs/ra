#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Security Attribution Register"]
    pub csar: CSAR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - SRAM Security Attribution Register"]
    pub sramsar: SRAMSAR,
    #[doc = "0x14 - Standby RAM memory Security Attribution Register"]
    pub stbramsar: STBRAMSAR,
    _reserved3: [u8; 0x18],
    #[doc = "0x30 - DTC Controller Security Attribution Register"]
    pub dtcsar: DTCSAR,
    #[doc = "0x34 - DMAC Controller Security Attribution Register"]
    pub dmacsar: DMACSAR,
    _reserved5: [u8; 0x08],
    #[doc = "0x40 - Interrupt Controller Unit Security Attribution Register A"]
    pub icusara: ICUSARA,
    #[doc = "0x44 - Interrupt Controller Unit Security Attribution Register B"]
    pub icusarb: ICUSARB,
    #[doc = "0x48 - Interrupt Controller Unit Security Attribution Register C"]
    pub icusarc: ICUSARC,
    #[doc = "0x4c - Interrupt Controller Unit Security Attribution Register D"]
    pub icusard: ICUSARD,
    #[doc = "0x50 - Interrupt Controller Unit Security Attribution Register E"]
    pub icusare: ICUSARE,
    _reserved10: [u8; 0x1c],
    #[doc = "0x70 - Interrupt Controller Unit Security Attribution Register G"]
    pub icusarg: ICUSARG,
    #[doc = "0x74 - Interrupt Controller Unit Security Attribution Register H"]
    pub icusarh: ICUSARH,
    #[doc = "0x78 - Interrupt Controller Unit Security Attribution Register I"]
    pub icusari: ICUSARI,
    _reserved13: [u8; 0x84],
    #[doc = "0x100 - BUS Security Attribution Register A"]
    pub bussara: BUSSARA,
    #[doc = "0x104 - BUS Security Attribution Register B"]
    pub bussarb: BUSSARB,
    _reserved15: [u8; 0x28],
    #[doc = "0x130 - Master Memory Protection Unit Security Attribution Register A"]
    pub mmpusara: MMPUSARA,
    #[doc = "0x134 - Master Memory Protection Unit Security Attribution Register B"]
    pub mmpusarb: MMPUSARB,
    _reserved17: [u8; 0x48],
    #[doc = "0x180 - TrustZone Filter Security Attribution Register"]
    pub tzfsar: TZFSAR,
    _reserved18: [u8; 0x2c],
    #[doc = "0x1b0 - CPU Debug Security Attribution Register"]
    pub cpudsar: CPUDSAR,
}
#[doc = "CSAR (rw) register accessor: an alias for `Reg<CSAR_SPEC>`"]
pub type CSAR = crate::Reg<csar::CSAR_SPEC>;
#[doc = "Cache Security Attribution Register"]
pub mod csar;
#[doc = "SRAMSAR (rw) register accessor: an alias for `Reg<SRAMSAR_SPEC>`"]
pub type SRAMSAR = crate::Reg<sramsar::SRAMSAR_SPEC>;
#[doc = "SRAM Security Attribution Register"]
pub mod sramsar;
#[doc = "STBRAMSAR (rw) register accessor: an alias for `Reg<STBRAMSAR_SPEC>`"]
pub type STBRAMSAR = crate::Reg<stbramsar::STBRAMSAR_SPEC>;
#[doc = "Standby RAM memory Security Attribution Register"]
pub mod stbramsar;
#[doc = "DTCSAR (rw) register accessor: an alias for `Reg<DTCSAR_SPEC>`"]
pub type DTCSAR = crate::Reg<dtcsar::DTCSAR_SPEC>;
#[doc = "DTC Controller Security Attribution Register"]
pub mod dtcsar;
#[doc = "DMACSAR (rw) register accessor: an alias for `Reg<DMACSAR_SPEC>`"]
pub type DMACSAR = crate::Reg<dmacsar::DMACSAR_SPEC>;
#[doc = "DMAC Controller Security Attribution Register"]
pub mod dmacsar;
#[doc = "ICUSARA (rw) register accessor: an alias for `Reg<ICUSARA_SPEC>`"]
pub type ICUSARA = crate::Reg<icusara::ICUSARA_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register A"]
pub mod icusara;
#[doc = "ICUSARB (rw) register accessor: an alias for `Reg<ICUSARB_SPEC>`"]
pub type ICUSARB = crate::Reg<icusarb::ICUSARB_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register B"]
pub mod icusarb;
#[doc = "ICUSARC (rw) register accessor: an alias for `Reg<ICUSARC_SPEC>`"]
pub type ICUSARC = crate::Reg<icusarc::ICUSARC_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register C"]
pub mod icusarc;
#[doc = "ICUSARD (rw) register accessor: an alias for `Reg<ICUSARD_SPEC>`"]
pub type ICUSARD = crate::Reg<icusard::ICUSARD_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register D"]
pub mod icusard;
#[doc = "ICUSARE (rw) register accessor: an alias for `Reg<ICUSARE_SPEC>`"]
pub type ICUSARE = crate::Reg<icusare::ICUSARE_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register E"]
pub mod icusare;
#[doc = "ICUSARG (rw) register accessor: an alias for `Reg<ICUSARG_SPEC>`"]
pub type ICUSARG = crate::Reg<icusarg::ICUSARG_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register G"]
pub mod icusarg;
#[doc = "ICUSARH (rw) register accessor: an alias for `Reg<ICUSARH_SPEC>`"]
pub type ICUSARH = crate::Reg<icusarh::ICUSARH_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register H"]
pub mod icusarh;
#[doc = "ICUSARI (rw) register accessor: an alias for `Reg<ICUSARI_SPEC>`"]
pub type ICUSARI = crate::Reg<icusari::ICUSARI_SPEC>;
#[doc = "Interrupt Controller Unit Security Attribution Register I"]
pub mod icusari;
#[doc = "BUSSARA (rw) register accessor: an alias for `Reg<BUSSARA_SPEC>`"]
pub type BUSSARA = crate::Reg<bussara::BUSSARA_SPEC>;
#[doc = "BUS Security Attribution Register A"]
pub mod bussara;
#[doc = "BUSSARB (rw) register accessor: an alias for `Reg<BUSSARB_SPEC>`"]
pub type BUSSARB = crate::Reg<bussarb::BUSSARB_SPEC>;
#[doc = "BUS Security Attribution Register B"]
pub mod bussarb;
#[doc = "MMPUSARA (rw) register accessor: an alias for `Reg<MMPUSARA_SPEC>`"]
pub type MMPUSARA = crate::Reg<mmpusara::MMPUSARA_SPEC>;
#[doc = "Master Memory Protection Unit Security Attribution Register A"]
pub mod mmpusara;
#[doc = "MMPUSARB (rw) register accessor: an alias for `Reg<MMPUSARB_SPEC>`"]
pub type MMPUSARB = crate::Reg<mmpusarb::MMPUSARB_SPEC>;
#[doc = "Master Memory Protection Unit Security Attribution Register B"]
pub mod mmpusarb;
#[doc = "TZFSAR (rw) register accessor: an alias for `Reg<TZFSAR_SPEC>`"]
pub type TZFSAR = crate::Reg<tzfsar::TZFSAR_SPEC>;
#[doc = "TrustZone Filter Security Attribution Register"]
pub mod tzfsar;
#[doc = "CPUDSAR (rw) register accessor: an alias for `Reg<CPUDSAR_SPEC>`"]
pub type CPUDSAR = crate::Reg<cpudsar::CPUDSAR_SPEC>;
#[doc = "CPU Debug Security Attribution Register"]
pub mod cpudsar;
