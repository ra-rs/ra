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
    _reserved14: [u8; 0x04],
    #[doc = "0x240 - Group A Region %s Access Control Register"]
    pub mmpuaca4: MMPUACA,
    _reserved15: [u8; 0x02],
    #[doc = "0x244 - Group A Region %s Start Address Register"]
    pub mmpusa4: MMPUSA,
    #[doc = "0x248 - Group A Region %s End Address Register"]
    pub mmpuea4: MMPUEA,
    _reserved17: [u8; 0x04],
    #[doc = "0x250 - Group A Region %s Access Control Register"]
    pub mmpuaca5: MMPUACA,
    _reserved18: [u8; 0x02],
    #[doc = "0x254 - Group A Region %s Start Address Register"]
    pub mmpusa5: MMPUSA,
    #[doc = "0x258 - Group A Region %s End Address Register"]
    pub mmpuea5: MMPUEA,
    _reserved20: [u8; 0x04],
    #[doc = "0x260 - Group A Region %s Access Control Register"]
    pub mmpuaca6: MMPUACA,
    _reserved21: [u8; 0x02],
    #[doc = "0x264 - Group A Region %s Start Address Register"]
    pub mmpusa6: MMPUSA,
    #[doc = "0x268 - Group A Region %s End Address Register"]
    pub mmpuea6: MMPUEA,
    _reserved23: [u8; 0x04],
    #[doc = "0x270 - Group A Region %s Access Control Register"]
    pub mmpuaca7: MMPUACA,
    _reserved24: [u8; 0x02],
    #[doc = "0x274 - Group A Region %s Start Address Register"]
    pub mmpusa7: MMPUSA,
    #[doc = "0x278 - Group A Region %s End Address Register"]
    pub mmpuea7: MMPUEA,
    _reserved26: [u8; 0x04],
    #[doc = "0x280 - Group A Region %s Access Control Register"]
    pub mmpuaca8: MMPUACA,
    _reserved27: [u8; 0x02],
    #[doc = "0x284 - Group A Region %s Start Address Register"]
    pub mmpusa8: MMPUSA,
    #[doc = "0x288 - Group A Region %s End Address Register"]
    pub mmpuea8: MMPUEA,
    _reserved29: [u8; 0x04],
    #[doc = "0x290 - Group A Region %s Access Control Register"]
    pub mmpuaca9: MMPUACA,
    _reserved30: [u8; 0x02],
    #[doc = "0x294 - Group A Region %s Start Address Register"]
    pub mmpusa9: MMPUSA,
    #[doc = "0x298 - Group A Region %s End Address Register"]
    pub mmpuea9: MMPUEA,
    _reserved32: [u8; 0x04],
    #[doc = "0x2a0 - Group A Region %s Access Control Register"]
    pub mmpuaca10: MMPUACA,
    _reserved33: [u8; 0x02],
    #[doc = "0x2a4 - Group A Region %s Start Address Register"]
    pub mmpusa10: MMPUSA,
    #[doc = "0x2a8 - Group A Region %s End Address Register"]
    pub mmpuea10: MMPUEA,
    _reserved35: [u8; 0x04],
    #[doc = "0x2b0 - Group A Region %s Access Control Register"]
    pub mmpuaca11: MMPUACA,
    _reserved36: [u8; 0x02],
    #[doc = "0x2b4 - Group A Region %s Start Address Register"]
    pub mmpusa11: MMPUSA,
    #[doc = "0x2b8 - Group A Region %s End Address Register"]
    pub mmpuea11: MMPUEA,
    _reserved38: [u8; 0x04],
    #[doc = "0x2c0 - Group A Region %s Access Control Register"]
    pub mmpuaca12: MMPUACA,
    _reserved39: [u8; 0x02],
    #[doc = "0x2c4 - Group A Region %s Start Address Register"]
    pub mmpusa12: MMPUSA,
    #[doc = "0x2c8 - Group A Region %s End Address Register"]
    pub mmpuea12: MMPUEA,
    _reserved41: [u8; 0x04],
    #[doc = "0x2d0 - Group A Region %s Access Control Register"]
    pub mmpuaca13: MMPUACA,
    _reserved42: [u8; 0x02],
    #[doc = "0x2d4 - Group A Region %s Start Address Register"]
    pub mmpusa13: MMPUSA,
    #[doc = "0x2d8 - Group A Region %s End Address Register"]
    pub mmpuea13: MMPUEA,
    _reserved44: [u8; 0x04],
    #[doc = "0x2e0 - Group A Region %s Access Control Register"]
    pub mmpuaca14: MMPUACA,
    _reserved45: [u8; 0x02],
    #[doc = "0x2e4 - Group A Region %s Start Address Register"]
    pub mmpusa14: MMPUSA,
    #[doc = "0x2e8 - Group A Region %s End Address Register"]
    pub mmpuea14: MMPUEA,
    _reserved47: [u8; 0x04],
    #[doc = "0x2f0 - Group A Region %s Access Control Register"]
    pub mmpuaca15: MMPUACA,
    _reserved48: [u8; 0x02],
    #[doc = "0x2f4 - Group A Region %s Start Address Register"]
    pub mmpusa15: MMPUSA,
    #[doc = "0x2f8 - Group A Region %s End Address Register"]
    pub mmpuea15: MMPUEA,
    _reserved50: [u8; 0x04],
    #[doc = "0x300 - Group A Region %s Access Control Register"]
    pub mmpuaca16: MMPUACA,
    _reserved51: [u8; 0x02],
    #[doc = "0x304 - Group A Region %s Start Address Register"]
    pub mmpusa16: MMPUSA,
    #[doc = "0x308 - Group A Region %s End Address Register"]
    pub mmpuea16: MMPUEA,
    _reserved53: [u8; 0x04],
    #[doc = "0x310 - Group A Region %s Access Control Register"]
    pub mmpuaca17: MMPUACA,
    _reserved54: [u8; 0x02],
    #[doc = "0x314 - Group A Region %s Start Address Register"]
    pub mmpusa17: MMPUSA,
    #[doc = "0x318 - Group A Region %s End Address Register"]
    pub mmpuea17: MMPUEA,
    _reserved56: [u8; 0x04],
    #[doc = "0x320 - Group A Region %s Access Control Register"]
    pub mmpuaca18: MMPUACA,
    _reserved57: [u8; 0x02],
    #[doc = "0x324 - Group A Region %s Start Address Register"]
    pub mmpusa18: MMPUSA,
    #[doc = "0x328 - Group A Region %s End Address Register"]
    pub mmpuea18: MMPUEA,
    _reserved59: [u8; 0x04],
    #[doc = "0x330 - Group A Region %s Access Control Register"]
    pub mmpuaca19: MMPUACA,
    _reserved60: [u8; 0x02],
    #[doc = "0x334 - Group A Region %s Start Address Register"]
    pub mmpusa19: MMPUSA,
    #[doc = "0x338 - Group A Region %s End Address Register"]
    pub mmpuea19: MMPUEA,
    _reserved62: [u8; 0x04],
    #[doc = "0x340 - Group A Region %s Access Control Register"]
    pub mmpuaca20: MMPUACA,
    _reserved63: [u8; 0x02],
    #[doc = "0x344 - Group A Region %s Start Address Register"]
    pub mmpusa20: MMPUSA,
    #[doc = "0x348 - Group A Region %s End Address Register"]
    pub mmpuea20: MMPUEA,
    _reserved65: [u8; 0x04],
    #[doc = "0x350 - Group A Region %s Access Control Register"]
    pub mmpuaca21: MMPUACA,
    _reserved66: [u8; 0x02],
    #[doc = "0x354 - Group A Region %s Start Address Register"]
    pub mmpusa21: MMPUSA,
    #[doc = "0x358 - Group A Region %s End Address Register"]
    pub mmpuea21: MMPUEA,
    _reserved68: [u8; 0x04],
    #[doc = "0x360 - Group A Region %s Access Control Register"]
    pub mmpuaca22: MMPUACA,
    _reserved69: [u8; 0x02],
    #[doc = "0x364 - Group A Region %s Start Address Register"]
    pub mmpusa22: MMPUSA,
    #[doc = "0x368 - Group A Region %s End Address Register"]
    pub mmpuea22: MMPUEA,
    _reserved71: [u8; 0x04],
    #[doc = "0x370 - Group A Region %s Access Control Register"]
    pub mmpuaca23: MMPUACA,
    _reserved72: [u8; 0x02],
    #[doc = "0x374 - Group A Region %s Start Address Register"]
    pub mmpusa23: MMPUSA,
    #[doc = "0x378 - Group A Region %s End Address Register"]
    pub mmpuea23: MMPUEA,
    _reserved74: [u8; 0x04],
    #[doc = "0x380 - Group A Region %s Access Control Register"]
    pub mmpuaca24: MMPUACA,
    _reserved75: [u8; 0x02],
    #[doc = "0x384 - Group A Region %s Start Address Register"]
    pub mmpusa24: MMPUSA,
    #[doc = "0x388 - Group A Region %s End Address Register"]
    pub mmpuea24: MMPUEA,
    _reserved77: [u8; 0x04],
    #[doc = "0x390 - Group A Region %s Access Control Register"]
    pub mmpuaca25: MMPUACA,
    _reserved78: [u8; 0x02],
    #[doc = "0x394 - Group A Region %s Start Address Register"]
    pub mmpusa25: MMPUSA,
    #[doc = "0x398 - Group A Region %s End Address Register"]
    pub mmpuea25: MMPUEA,
    _reserved80: [u8; 0x04],
    #[doc = "0x3a0 - Group A Region %s Access Control Register"]
    pub mmpuaca26: MMPUACA,
    _reserved81: [u8; 0x02],
    #[doc = "0x3a4 - Group A Region %s Start Address Register"]
    pub mmpusa26: MMPUSA,
    #[doc = "0x3a8 - Group A Region %s End Address Register"]
    pub mmpuea26: MMPUEA,
    _reserved83: [u8; 0x04],
    #[doc = "0x3b0 - Group A Region %s Access Control Register"]
    pub mmpuaca27: MMPUACA,
    _reserved84: [u8; 0x02],
    #[doc = "0x3b4 - Group A Region %s Start Address Register"]
    pub mmpusa27: MMPUSA,
    #[doc = "0x3b8 - Group A Region %s End Address Register"]
    pub mmpuea27: MMPUEA,
    _reserved86: [u8; 0x04],
    #[doc = "0x3c0 - Group A Region %s Access Control Register"]
    pub mmpuaca28: MMPUACA,
    _reserved87: [u8; 0x02],
    #[doc = "0x3c4 - Group A Region %s Start Address Register"]
    pub mmpusa28: MMPUSA,
    #[doc = "0x3c8 - Group A Region %s End Address Register"]
    pub mmpuea28: MMPUEA,
    _reserved89: [u8; 0x04],
    #[doc = "0x3d0 - Group A Region %s Access Control Register"]
    pub mmpuaca29: MMPUACA,
    _reserved90: [u8; 0x02],
    #[doc = "0x3d4 - Group A Region %s Start Address Register"]
    pub mmpusa29: MMPUSA,
    #[doc = "0x3d8 - Group A Region %s End Address Register"]
    pub mmpuea29: MMPUEA,
    _reserved92: [u8; 0x04],
    #[doc = "0x3e0 - Group A Region %s Access Control Register"]
    pub mmpuaca30: MMPUACA,
    _reserved93: [u8; 0x02],
    #[doc = "0x3e4 - Group A Region %s Start Address Register"]
    pub mmpusa30: MMPUSA,
    #[doc = "0x3e8 - Group A Region %s End Address Register"]
    pub mmpuea30: MMPUEA,
    _reserved95: [u8; 0x04],
    #[doc = "0x3f0 - Group A Region %s Access Control Register"]
    pub mmpuaca31: MMPUACA,
    _reserved96: [u8; 0x02],
    #[doc = "0x3f4 - Group A Region %s Start Address Register"]
    pub mmpusa31: MMPUSA,
    #[doc = "0x3f8 - Group A Region %s End Address Register"]
    pub mmpuea31: MMPUEA,
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
