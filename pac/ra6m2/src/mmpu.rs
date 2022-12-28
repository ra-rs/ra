#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bus Master MPU Control Register"]
    pub mmpuctla: MMPUCTL,
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
    _reserved98: [u8; 0x04],
    #[doc = "0x400 - Bus Master MPU Control Register"]
    pub mmpuctlb: MMPUCTL,
    _reserved99: [u8; 0x0100],
    #[doc = "0x502 - Group B Protection of Register"]
    pub mmpuptb: MMPUPTB,
    _reserved100: [u8; 0xfc],
    #[doc = "0x600 - Group B Region %s Access Control Register"]
    pub mmpuacb0: MMPUACB,
    _reserved101: [u8; 0x02],
    #[doc = "0x604 - Group B Region %s Start Address Register"]
    pub mmpusb0: MMPUSB,
    #[doc = "0x608 - Group B Region %s End Address Register"]
    pub mmpueb0: MMPUEB,
    _reserved103: [u8; 0x04],
    #[doc = "0x610 - Group B Region %s Access Control Register"]
    pub mmpuacb1: MMPUACB,
    _reserved104: [u8; 0x02],
    #[doc = "0x614 - Group B Region %s Start Address Register"]
    pub mmpusb1: MMPUSB,
    #[doc = "0x618 - Group B Region %s End Address Register"]
    pub mmpueb1: MMPUEB,
    _reserved106: [u8; 0x04],
    #[doc = "0x620 - Group B Region %s Access Control Register"]
    pub mmpuacb2: MMPUACB,
    _reserved107: [u8; 0x02],
    #[doc = "0x624 - Group B Region %s Start Address Register"]
    pub mmpusb2: MMPUSB,
    #[doc = "0x628 - Group B Region %s End Address Register"]
    pub mmpueb2: MMPUEB,
    _reserved109: [u8; 0x04],
    #[doc = "0x630 - Group B Region %s Access Control Register"]
    pub mmpuacb3: MMPUACB,
    _reserved110: [u8; 0x02],
    #[doc = "0x634 - Group B Region %s Start Address Register"]
    pub mmpusb3: MMPUSB,
    #[doc = "0x638 - Group B Region %s End Address Register"]
    pub mmpueb3: MMPUEB,
    _reserved112: [u8; 0x04],
    #[doc = "0x640 - Group B Region %s Access Control Register"]
    pub mmpuacb4: MMPUACB,
    _reserved113: [u8; 0x02],
    #[doc = "0x644 - Group B Region %s Start Address Register"]
    pub mmpusb4: MMPUSB,
    #[doc = "0x648 - Group B Region %s End Address Register"]
    pub mmpueb4: MMPUEB,
    _reserved115: [u8; 0x04],
    #[doc = "0x650 - Group B Region %s Access Control Register"]
    pub mmpuacb5: MMPUACB,
    _reserved116: [u8; 0x02],
    #[doc = "0x654 - Group B Region %s Start Address Register"]
    pub mmpusb5: MMPUSB,
    #[doc = "0x658 - Group B Region %s End Address Register"]
    pub mmpueb5: MMPUEB,
    _reserved118: [u8; 0x04],
    #[doc = "0x660 - Group B Region %s Access Control Register"]
    pub mmpuacb6: MMPUACB,
    _reserved119: [u8; 0x02],
    #[doc = "0x664 - Group B Region %s Start Address Register"]
    pub mmpusb6: MMPUSB,
    #[doc = "0x668 - Group B Region %s End Address Register"]
    pub mmpueb6: MMPUEB,
    _reserved121: [u8; 0x04],
    #[doc = "0x670 - Group B Region %s Access Control Register"]
    pub mmpuacb7: MMPUACB,
    _reserved122: [u8; 0x02],
    #[doc = "0x674 - Group B Region %s Start Address Register"]
    pub mmpusb7: MMPUSB,
    #[doc = "0x678 - Group B Region %s End Address Register"]
    pub mmpueb7: MMPUEB,
    _reserved124: [u8; 0x0184],
    #[doc = "0x800 - Bus Master MPU Control Register"]
    pub mmpuctlc: MMPUCTL,
    _reserved125: [u8; 0x0100],
    #[doc = "0x902 - Group C protection of register"]
    pub mmpuptc: MMPUPTC,
    _reserved126: [u8; 0xfc],
    #[doc = "0xa00 - Group C Region %s Access Control Register"]
    pub mmpuacc0: MMPUACC,
    _reserved127: [u8; 0x02],
    #[doc = "0xa04 - Group C Region %s Start Address Register"]
    pub mmpusc0: MMPUSC,
    #[doc = "0xa08 - Group C Region %s End Address Register"]
    pub mmpuec0: MMPUEC,
    _reserved129: [u8; 0x04],
    #[doc = "0xa10 - Group C Region %s Access Control Register"]
    pub mmpuacc1: MMPUACC,
    _reserved130: [u8; 0x02],
    #[doc = "0xa14 - Group C Region %s Start Address Register"]
    pub mmpusc1: MMPUSC,
    #[doc = "0xa18 - Group C Region %s End Address Register"]
    pub mmpuec1: MMPUEC,
    _reserved132: [u8; 0x04],
    #[doc = "0xa20 - Group C Region %s Access Control Register"]
    pub mmpuacc2: MMPUACC,
    _reserved133: [u8; 0x02],
    #[doc = "0xa24 - Group C Region %s Start Address Register"]
    pub mmpusc2: MMPUSC,
    #[doc = "0xa28 - Group C Region %s End Address Register"]
    pub mmpuec2: MMPUEC,
    _reserved135: [u8; 0x04],
    #[doc = "0xa30 - Group C Region %s Access Control Register"]
    pub mmpuacc3: MMPUACC,
    _reserved136: [u8; 0x02],
    #[doc = "0xa34 - Group C Region %s Start Address Register"]
    pub mmpusc3: MMPUSC,
    #[doc = "0xa38 - Group C Region %s End Address Register"]
    pub mmpuec3: MMPUEC,
    _reserved138: [u8; 0x04],
    #[doc = "0xa40 - Group C Region %s Access Control Register"]
    pub mmpuacc4: MMPUACC,
    _reserved139: [u8; 0x02],
    #[doc = "0xa44 - Group C Region %s Start Address Register"]
    pub mmpusc4: MMPUSC,
    #[doc = "0xa48 - Group C Region %s End Address Register"]
    pub mmpuec4: MMPUEC,
    _reserved141: [u8; 0x04],
    #[doc = "0xa50 - Group C Region %s Access Control Register"]
    pub mmpuacc5: MMPUACC,
    _reserved142: [u8; 0x02],
    #[doc = "0xa54 - Group C Region %s Start Address Register"]
    pub mmpusc5: MMPUSC,
    #[doc = "0xa58 - Group C Region %s End Address Register"]
    pub mmpuec5: MMPUEC,
    _reserved144: [u8; 0x04],
    #[doc = "0xa60 - Group C Region %s Access Control Register"]
    pub mmpuacc6: MMPUACC,
    _reserved145: [u8; 0x02],
    #[doc = "0xa64 - Group C Region %s Start Address Register"]
    pub mmpusc6: MMPUSC,
    #[doc = "0xa68 - Group C Region %s End Address Register"]
    pub mmpuec6: MMPUEC,
    _reserved147: [u8; 0x04],
    #[doc = "0xa70 - Group C Region %s Access Control Register"]
    pub mmpuacc7: MMPUACC,
    _reserved148: [u8; 0x02],
    #[doc = "0xa74 - Group C Region %s Start Address Register"]
    pub mmpusc7: MMPUSC,
    #[doc = "0xa78 - Group C Region %s End Address Register"]
    pub mmpuec7: MMPUEC,
}
#[doc = "MMPUCTL (rw) register accessor: an alias for `Reg<MMPUCTL_SPEC>`"]
pub type MMPUCTL = crate::Reg<mmpuctl::MMPUCTL_SPEC>;
#[doc = "Bus Master MPU Control Register"]
pub mod mmpuctl;
#[doc = "MMPUACA (rw) register accessor: an alias for `Reg<MMPUACA_SPEC>`"]
pub type MMPUACA = crate::Reg<mmpuaca::MMPUACA_SPEC>;
#[doc = "Group A Region %s Access Control Register"]
pub mod mmpuaca;
#[doc = "MMPUACB (rw) register accessor: an alias for `Reg<MMPUACB_SPEC>`"]
pub type MMPUACB = crate::Reg<mmpuacb::MMPUACB_SPEC>;
#[doc = "Group B Region %s Access Control Register"]
pub mod mmpuacb;
#[doc = "MMPUACC (rw) register accessor: an alias for `Reg<MMPUACC_SPEC>`"]
pub type MMPUACC = crate::Reg<mmpuacc::MMPUACC_SPEC>;
#[doc = "Group C Region %s Access Control Register"]
pub mod mmpuacc;
#[doc = "MMPUSA (rw) register accessor: an alias for `Reg<MMPUSA_SPEC>`"]
pub type MMPUSA = crate::Reg<mmpusa::MMPUSA_SPEC>;
#[doc = "Group A Region %s Start Address Register"]
pub mod mmpusa;
#[doc = "MMPUSB (rw) register accessor: an alias for `Reg<MMPUSB_SPEC>`"]
pub type MMPUSB = crate::Reg<mmpusb::MMPUSB_SPEC>;
#[doc = "Group B Region %s Start Address Register"]
pub mod mmpusb;
#[doc = "MMPUSC (rw) register accessor: an alias for `Reg<MMPUSC_SPEC>`"]
pub type MMPUSC = crate::Reg<mmpusc::MMPUSC_SPEC>;
#[doc = "Group C Region %s Start Address Register"]
pub mod mmpusc;
#[doc = "MMPUEA (rw) register accessor: an alias for `Reg<MMPUEA_SPEC>`"]
pub type MMPUEA = crate::Reg<mmpuea::MMPUEA_SPEC>;
#[doc = "Group A Region %s End Address Register"]
pub mod mmpuea;
#[doc = "MMPUEB (rw) register accessor: an alias for `Reg<MMPUEB_SPEC>`"]
pub type MMPUEB = crate::Reg<mmpueb::MMPUEB_SPEC>;
#[doc = "Group B Region %s End Address Register"]
pub mod mmpueb;
#[doc = "MMPUEC (rw) register accessor: an alias for `Reg<MMPUEC_SPEC>`"]
pub type MMPUEC = crate::Reg<mmpuec::MMPUEC_SPEC>;
#[doc = "Group C Region %s End Address Register"]
pub mod mmpuec;
#[doc = "MMPUPTA (rw) register accessor: an alias for `Reg<MMPUPTA_SPEC>`"]
pub type MMPUPTA = crate::Reg<mmpupta::MMPUPTA_SPEC>;
#[doc = "Group A Protection of Register"]
pub mod mmpupta;
#[doc = "MMPUPTB (rw) register accessor: an alias for `Reg<MMPUPTB_SPEC>`"]
pub type MMPUPTB = crate::Reg<mmpuptb::MMPUPTB_SPEC>;
#[doc = "Group B Protection of Register"]
pub mod mmpuptb;
#[doc = "MMPUPTC (rw) register accessor: an alias for `Reg<MMPUPTC_SPEC>`"]
pub type MMPUPTC = crate::Reg<mmpuptc::MMPUPTC_SPEC>;
#[doc = "Group C protection of register"]
pub mod mmpuptc;
