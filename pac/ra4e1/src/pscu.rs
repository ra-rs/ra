#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Peripheral Security Attribution Register B"]
    pub psarb: PSARB,
    #[doc = "0x08 - Peripheral Security Attribution Register C"]
    pub psarc: PSARC,
    #[doc = "0x0c - Peripheral Security Attribution Register D"]
    pub psard: PSARD,
    #[doc = "0x10 - Peripheral Security Attribution Register E"]
    pub psare: PSARE,
    #[doc = "0x14 - Module Stop Security Attribution Register"]
    pub mssar: MSSAR,
    #[doc = "0x18 - Code Flash Security Attribution Monitor Register A"]
    pub cfsamona: CFSAMONA,
    #[doc = "0x1c - Code Flash Security Attribution Monitor Register B"]
    pub cfsamonb: CFSAMONB,
    #[doc = "0x20 - Data Flash Security Attribution Monitor Register"]
    pub dfsamon: DFSAMON,
    #[doc = "0x24 - SRAM Security Attribution Monitor Register A"]
    pub ssamona: SSAMONA,
    #[doc = "0x28 - SRAM Security Attribution Monitor Register B"]
    pub ssamonb: SSAMONB,
    #[doc = "0x2c - Device Lifecycle Management State Monitor Register"]
    pub dlmmon: DLMMON,
}
#[doc = "PSARB (rw) register accessor: an alias for `Reg<PSARB_SPEC>`"]
pub type PSARB = crate::Reg<psarb::PSARB_SPEC>;
#[doc = "Peripheral Security Attribution Register B"]
pub mod psarb;
#[doc = "PSARC (rw) register accessor: an alias for `Reg<PSARC_SPEC>`"]
pub type PSARC = crate::Reg<psarc::PSARC_SPEC>;
#[doc = "Peripheral Security Attribution Register C"]
pub mod psarc;
#[doc = "PSARD (rw) register accessor: an alias for `Reg<PSARD_SPEC>`"]
pub type PSARD = crate::Reg<psard::PSARD_SPEC>;
#[doc = "Peripheral Security Attribution Register D"]
pub mod psard;
#[doc = "PSARE (rw) register accessor: an alias for `Reg<PSARE_SPEC>`"]
pub type PSARE = crate::Reg<psare::PSARE_SPEC>;
#[doc = "Peripheral Security Attribution Register E"]
pub mod psare;
#[doc = "MSSAR (rw) register accessor: an alias for `Reg<MSSAR_SPEC>`"]
pub type MSSAR = crate::Reg<mssar::MSSAR_SPEC>;
#[doc = "Module Stop Security Attribution Register"]
pub mod mssar;
#[doc = "CFSAMONA (r) register accessor: an alias for `Reg<CFSAMONA_SPEC>`"]
pub type CFSAMONA = crate::Reg<cfsamona::CFSAMONA_SPEC>;
#[doc = "Code Flash Security Attribution Monitor Register A"]
pub mod cfsamona;
#[doc = "CFSAMONB (r) register accessor: an alias for `Reg<CFSAMONB_SPEC>`"]
pub type CFSAMONB = crate::Reg<cfsamonb::CFSAMONB_SPEC>;
#[doc = "Code Flash Security Attribution Monitor Register B"]
pub mod cfsamonb;
#[doc = "DFSAMON (r) register accessor: an alias for `Reg<DFSAMON_SPEC>`"]
pub type DFSAMON = crate::Reg<dfsamon::DFSAMON_SPEC>;
#[doc = "Data Flash Security Attribution Monitor Register"]
pub mod dfsamon;
#[doc = "SSAMONA (r) register accessor: an alias for `Reg<SSAMONA_SPEC>`"]
pub type SSAMONA = crate::Reg<ssamona::SSAMONA_SPEC>;
#[doc = "SRAM Security Attribution Monitor Register A"]
pub mod ssamona;
#[doc = "SSAMONB (r) register accessor: an alias for `Reg<SSAMONB_SPEC>`"]
pub type SSAMONB = crate::Reg<ssamonb::SSAMONB_SPEC>;
#[doc = "SRAM Security Attribution Monitor Register B"]
pub mod ssamonb;
#[doc = "DLMMON (r) register accessor: an alias for `Reg<DLMMON_SPEC>`"]
pub type DLMMON = crate::Reg<dlmmon::DLMMON_SPEC>;
#[doc = "Device Lifecycle Management State Monitor Register"]
pub mod dlmmon;
