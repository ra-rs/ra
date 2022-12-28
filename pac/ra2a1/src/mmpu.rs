#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bus Master MPU Control Register"]
    pub mmpuctla: MMPUCTLA,
    _reserved1: [u8; 0x0100],
    #[doc = "0x102 - Group A Protection of Register"]
    pub mmpupta: MMPUPTA,
    _reserved2: [u8; 0xfc],
    #[doc = "0x200 - Group A Region %s Access Control Register"]
    pub mmpuaca0: MMPUACA,
    _reserved3: [u8; 0x02],
    #[doc = "0x204 - Group A Region %s Start Address Register"]
    pub mmpusa0: MMPUSA,
    #[doc = "0x208 - Group A Region %s End Address Register"]
    pub mmpuea0: MMPUEA,
    _reserved5: [u8; 0x04],
    #[doc = "0x210 - Group A Region %s Access Control Register"]
    pub mmpuaca1: MMPUACA,
    _reserved6: [u8; 0x02],
    #[doc = "0x214 - Group A Region %s Start Address Register"]
    pub mmpusa1: MMPUSA,
    #[doc = "0x218 - Group A Region %s End Address Register"]
    pub mmpuea1: MMPUEA,
    _reserved8: [u8; 0x04],
    #[doc = "0x220 - Group A Region %s Access Control Register"]
    pub mmpuaca2: MMPUACA,
    _reserved9: [u8; 0x02],
    #[doc = "0x224 - Group A Region %s Start Address Register"]
    pub mmpusa2: MMPUSA,
    #[doc = "0x228 - Group A Region %s End Address Register"]
    pub mmpuea2: MMPUEA,
    _reserved11: [u8; 0x04],
    #[doc = "0x230 - Group A Region %s Access Control Register"]
    pub mmpuaca3: MMPUACA,
    _reserved12: [u8; 0x02],
    #[doc = "0x234 - Group A Region %s Start Address Register"]
    pub mmpusa3: MMPUSA,
    #[doc = "0x238 - Group A Region %s End Address Register"]
    pub mmpuea3: MMPUEA,
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
#[doc = "Group A Region %s Access Control Register"]
pub mod mmpuaca;
#[doc = "MMPUSA (rw) register accessor: an alias for `Reg<MMPUSA_SPEC>`"]
pub type MMPUSA = crate::Reg<mmpusa::MMPUSA_SPEC>;
#[doc = "Group A Region %s Start Address Register"]
pub mod mmpusa;
#[doc = "MMPUEA (rw) register accessor: an alias for `Reg<MMPUEA_SPEC>`"]
pub type MMPUEA = crate::Reg<mmpuea::MMPUEA_SPEC>;
#[doc = "Group A Region %s End Address Register"]
pub mod mmpuea;
