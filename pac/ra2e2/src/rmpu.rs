#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bus Master MPU Control Register"]
    pub mmpuctla: MMPUCTLA,
    _reserved1: [u8; 0x0100],
    #[doc = "0x102 - Group A Protection of Register"]
    pub mmpupta: MMPUPTA,
    _reserved2: [u8; 0xfc],
    #[doc = "0x200 - Group A Region %s access control register"]
    pub mmpuaca0: MMPUACA,
    _reserved3: [u8; 0x02],
    #[doc = "0x204 - Group A Region %s Start Address Register"]
    pub mmpusa0: MMPUSA,
    #[doc = "0x208 - Group A Region %s End Address Register"]
    pub mmpuea0: MMPUEA,
    _reserved5: [u8; 0x04],
    #[doc = "0x210 - Group A Region %s access control register"]
    pub mmpuaca1: MMPUACA,
    _reserved6: [u8; 0x02],
    #[doc = "0x214 - Group A Region %s Start Address Register"]
    pub mmpusa1: MMPUSA,
    #[doc = "0x218 - Group A Region %s End Address Register"]
    pub mmpuea1: MMPUEA,
    _reserved8: [u8; 0x04],
    #[doc = "0x220 - Group A Region %s access control register"]
    pub mmpuaca2: MMPUACA,
    _reserved9: [u8; 0x02],
    #[doc = "0x224 - Group A Region %s Start Address Register"]
    pub mmpusa2: MMPUSA,
    #[doc = "0x228 - Group A Region %s End Address Register"]
    pub mmpuea2: MMPUEA,
    _reserved11: [u8; 0x04],
    #[doc = "0x230 - Group A Region %s access control register"]
    pub mmpuaca3: MMPUACA,
    _reserved12: [u8; 0x02],
    #[doc = "0x234 - Group A Region %s Start Address Register"]
    pub mmpusa3: MMPUSA,
    #[doc = "0x238 - Group A Region %s End Address Register"]
    pub mmpuea3: MMPUEA,
    _reserved14: [u8; 0x09c4],
    #[doc = "0xc00 - Slave MPU Control Register"]
    pub smpuctl: SMPUCTL,
    _reserved15: [u8; 0x0e],
    #[doc = "0xc10 - Access Control Register for Memory Bus 1"]
    pub smpumbiu: SMPUMBIU,
    _reserved16: [u8; 0x02],
    #[doc = "0xc14 - Access Control Register for Internal Peripheral Bus 9"]
    pub smpufbiu: SMPUFBIU,
    _reserved17: [u8; 0x02],
    #[doc = "0xc18 - Access Control Register for Memory Bus 4"]
    pub smpusram0: SMPUSRAM0,
    _reserved18: [u8; 0x06],
    #[doc = "0xc20 - Access Control Register for Internal Peripheral Bus 1"]
    pub smpup0biu: SMPUP0BIU,
    _reserved19: [u8; 0x02],
    #[doc = "0xc24 - Access Control Register for Internal Peripheral Bus 3"]
    pub smpup2biu: SMPUP2BIU,
    _reserved20: [u8; 0x02],
    #[doc = "0xc28 - Access Control Register for Internal Peripheral Bus 7"]
    pub smpup6biu: SMPUP6BIU,
    _reserved21: [u8; 0xd6],
    #[doc = "0xd00 - Stack Pointer Monitor Operation After Detection Register"]
    pub mspmpuoad: MSPMPUOAD,
    _reserved22: [u8; 0x02],
    #[doc = "0xd04 - Stack Pointer Monitor Access Control Register"]
    pub mspmpuctl: MSPMPUCTL,
    #[doc = "0xd06 - Stack Pointer Monitor Protection Register"]
    pub mspmpupt: MSPMPUPT,
    #[doc = "0xd08 - Main Stack Pointer (MSP) Monitor Start Address Register"]
    pub mspmpusa: MSPMPUSA,
    #[doc = "0xd0c - Main Stack Pointer (MSP) Monitor End Address Register"]
    pub mspmpuea: MSPMPUEA,
    #[doc = "0xd10 - Stack Pointer Monitor Operation After Detection Register"]
    pub pspmpuoad: PSPMPUOAD,
    _reserved27: [u8; 0x02],
    #[doc = "0xd14 - Stack Pointer Monitor Access Control Register"]
    pub pspmpuctl: PSPMPUCTL,
    #[doc = "0xd16 - Stack Pointer Monitor Protection Register"]
    pub pspmpupt: PSPMPUPT,
    #[doc = "0xd18 - Process Stack Pointer (PSP) Monitor Start Address Register"]
    pub pspmpusa: PSPMPUSA,
    #[doc = "0xd1c - Process Stack Pointer (PSP) Monitor End Address Register"]
    pub pspmpuea: PSPMPUEA,
}
#[doc = "MMPUCTLA (rw) register accessor: an alias for `Reg<MMPUCTLA_SPEC>`"]
pub type MMPUCTLA = crate::Reg<mmpuctla::MMPUCTLA_SPEC>;
#[doc = "Bus Master MPU Control Register"]
pub mod mmpuctla;
#[doc = "MMPUPTA (rw) register accessor: an alias for `Reg<MMPUPTA_SPEC>`"]
pub type MMPUPTA = crate::Reg<mmpupta::MMPUPTA_SPEC>;
#[doc = "Group A Protection of Register"]
pub mod mmpupta;
#[doc = "MMPUACA (rw) register accessor: an alias for `Reg<MMPUACA_SPEC>`"]
pub type MMPUACA = crate::Reg<mmpuaca::MMPUACA_SPEC>;
#[doc = "Group A Region %s access control register"]
pub mod mmpuaca;
#[doc = "MMPUSA (rw) register accessor: an alias for `Reg<MMPUSA_SPEC>`"]
pub type MMPUSA = crate::Reg<mmpusa::MMPUSA_SPEC>;
#[doc = "Group A Region %s Start Address Register"]
pub mod mmpusa;
#[doc = "MMPUEA (rw) register accessor: an alias for `Reg<MMPUEA_SPEC>`"]
pub type MMPUEA = crate::Reg<mmpuea::MMPUEA_SPEC>;
#[doc = "Group A Region %s End Address Register"]
pub mod mmpuea;
#[doc = "SMPUCTL (rw) register accessor: an alias for `Reg<SMPUCTL_SPEC>`"]
pub type SMPUCTL = crate::Reg<smpuctl::SMPUCTL_SPEC>;
#[doc = "Slave MPU Control Register"]
pub mod smpuctl;
#[doc = "SMPUMBIU (rw) register accessor: an alias for `Reg<SMPUMBIU_SPEC>`"]
pub type SMPUMBIU = crate::Reg<smpumbiu::SMPUMBIU_SPEC>;
#[doc = "Access Control Register for Memory Bus 1"]
pub mod smpumbiu;
#[doc = "SMPUFBIU (rw) register accessor: an alias for `Reg<SMPUFBIU_SPEC>`"]
pub type SMPUFBIU = crate::Reg<smpufbiu::SMPUFBIU_SPEC>;
#[doc = "Access Control Register for Internal Peripheral Bus 9"]
pub mod smpufbiu;
#[doc = "SMPUSRAM0 (rw) register accessor: an alias for `Reg<SMPUSRAM0_SPEC>`"]
pub type SMPUSRAM0 = crate::Reg<smpusram0::SMPUSRAM0_SPEC>;
#[doc = "Access Control Register for Memory Bus 4"]
pub mod smpusram0;
#[doc = "SMPUP0BIU (rw) register accessor: an alias for `Reg<SMPUP0BIU_SPEC>`"]
pub type SMPUP0BIU = crate::Reg<smpup0biu::SMPUP0BIU_SPEC>;
#[doc = "Access Control Register for Internal Peripheral Bus 1"]
pub mod smpup0biu;
#[doc = "SMPUP2BIU (rw) register accessor: an alias for `Reg<SMPUP2BIU_SPEC>`"]
pub type SMPUP2BIU = crate::Reg<smpup2biu::SMPUP2BIU_SPEC>;
#[doc = "Access Control Register for Internal Peripheral Bus 3"]
pub mod smpup2biu;
#[doc = "SMPUP6BIU (rw) register accessor: an alias for `Reg<SMPUP6BIU_SPEC>`"]
pub type SMPUP6BIU = crate::Reg<smpup6biu::SMPUP6BIU_SPEC>;
#[doc = "Access Control Register for Internal Peripheral Bus 7"]
pub mod smpup6biu;
#[doc = "MSPMPUOAD (rw) register accessor: an alias for `Reg<MSPMPUOAD_SPEC>`"]
pub type MSPMPUOAD = crate::Reg<mspmpuoad::MSPMPUOAD_SPEC>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod mspmpuoad;
#[doc = "MSPMPUCTL (rw) register accessor: an alias for `Reg<MSPMPUCTL_SPEC>`"]
pub type MSPMPUCTL = crate::Reg<mspmpuctl::MSPMPUCTL_SPEC>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod mspmpuctl;
#[doc = "MSPMPUPT (rw) register accessor: an alias for `Reg<MSPMPUPT_SPEC>`"]
pub type MSPMPUPT = crate::Reg<mspmpupt::MSPMPUPT_SPEC>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod mspmpupt;
#[doc = "MSPMPUSA (rw) register accessor: an alias for `Reg<MSPMPUSA_SPEC>`"]
pub type MSPMPUSA = crate::Reg<mspmpusa::MSPMPUSA_SPEC>;
#[doc = "Main Stack Pointer (MSP) Monitor Start Address Register"]
pub mod mspmpusa;
#[doc = "MSPMPUEA (rw) register accessor: an alias for `Reg<MSPMPUEA_SPEC>`"]
pub type MSPMPUEA = crate::Reg<mspmpuea::MSPMPUEA_SPEC>;
#[doc = "Main Stack Pointer (MSP) Monitor End Address Register"]
pub mod mspmpuea;
#[doc = "PSPMPUOAD (rw) register accessor: an alias for `Reg<PSPMPUOAD_SPEC>`"]
pub type PSPMPUOAD = crate::Reg<pspmpuoad::PSPMPUOAD_SPEC>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod pspmpuoad;
#[doc = "PSPMPUCTL (rw) register accessor: an alias for `Reg<PSPMPUCTL_SPEC>`"]
pub type PSPMPUCTL = crate::Reg<pspmpuctl::PSPMPUCTL_SPEC>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod pspmpuctl;
#[doc = "PSPMPUPT (rw) register accessor: an alias for `Reg<PSPMPUPT_SPEC>`"]
pub type PSPMPUPT = crate::Reg<pspmpupt::PSPMPUPT_SPEC>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod pspmpupt;
#[doc = "PSPMPUSA (rw) register accessor: an alias for `Reg<PSPMPUSA_SPEC>`"]
pub type PSPMPUSA = crate::Reg<pspmpusa::PSPMPUSA_SPEC>;
#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register"]
pub mod pspmpusa;
#[doc = "PSPMPUEA (rw) register accessor: an alias for `Reg<PSPMPUEA_SPEC>`"]
pub type PSPMPUEA = crate::Reg<pspmpuea::PSPMPUEA_SPEC>;
#[doc = "Process Stack Pointer (PSP) Monitor End Address Register"]
pub mod pspmpuea;
