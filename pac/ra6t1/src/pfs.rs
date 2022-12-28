#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_p000: [u8; 0x04],
    _reserved_1_p001: [u8; 0x04],
    _reserved_2_p002: [u8; 0x04],
    _reserved_3_p003: [u8; 0x04],
    _reserved_4_p004: [u8; 0x04],
    _reserved_5_p005: [u8; 0x04],
    _reserved_6_p006: [u8; 0x04],
    _reserved_7_p007: [u8; 0x04],
    _reserved_8_p008: [u8; 0x04],
    _reserved9: [u8; 0x14],
    _reserved_9_p014: [u8; 0x04],
    _reserved_10_p015: [u8; 0x04],
    _reserved_11_p100: [u8; 0x04],
    _reserved_12_p101: [u8; 0x04],
    _reserved_13_p102: [u8; 0x04],
    _reserved_14_p103: [u8; 0x04],
    _reserved_15_p104: [u8; 0x04],
    _reserved_16_p105: [u8; 0x04],
    _reserved_17_p106: [u8; 0x04],
    _reserved_18_p107: [u8; 0x04],
    _reserved_19_p108: [u8; 0x04],
    _reserved_20_p109: [u8; 0x04],
    _reserved_21_p110: [u8; 0x04],
    _reserved_22_p111: [u8; 0x04],
    _reserved_23_p112: [u8; 0x04],
    _reserved_24_p113: [u8; 0x04],
    _reserved_25_p114: [u8; 0x04],
    _reserved_26_p115: [u8; 0x04],
    _reserved_27_p200: [u8; 0x04],
    _reserved_28_p201: [u8; 0x04],
    _reserved29: [u8; 0x0c],
    _reserved_29_p205: [u8; 0x04],
    _reserved_30_p206: [u8; 0x04],
    _reserved_31_p207: [u8; 0x04],
    _reserved_32_p208: [u8; 0x04],
    _reserved_33_p209: [u8; 0x04],
    _reserved_34_p210: [u8; 0x04],
    _reserved_35_p211: [u8; 0x04],
    _reserved_36_p212: [u8; 0x04],
    _reserved_37_p213: [u8; 0x04],
    _reserved_38_p214: [u8; 0x04],
    _reserved39: [u8; 0x04],
    _reserved_39_p300: [u8; 0x04],
    _reserved_40_p301: [u8; 0x04],
    _reserved_41_p302: [u8; 0x04],
    _reserved_42_p303: [u8; 0x04],
    _reserved_43_p304: [u8; 0x04],
    _reserved_44_p305: [u8; 0x04],
    _reserved_45_p306: [u8; 0x04],
    _reserved_46_p307: [u8; 0x04],
    _reserved47: [u8; 0x20],
    _reserved_47_p40: [u8; 0x28],
    _reserved_48_p410: [u8; 0x04],
    _reserved_49_p411: [u8; 0x04],
    _reserved_50_p412: [u8; 0x04],
    _reserved_51_p413: [u8; 0x04],
    _reserved_52_p414: [u8; 0x04],
    _reserved_53_p415: [u8; 0x04],
    _reserved_54_p50: [u8; 0x14],
    _reserved55: [u8; 0x0c],
    _reserved_55_p508: [u8; 0x04],
    _reserved56: [u8; 0x1c],
    _reserved_56_p60: [u8; 0x0c],
    _reserved57: [u8; 0x14],
    _reserved_57_p608: [u8; 0x04],
    _reserved_58_p609: [u8; 0x04],
    _reserved_59_p610: [u8; 0x04],
    _reserved60: [u8; 0x34],
    _reserved_60_p708: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs(&self) -> &P000PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x02 - P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(&self) -> &P000PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x03 - P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_by(&self) -> &P000PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(3usize).cast() }
    }
    #[doc = "0x04 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p001pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x06 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p001pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x07 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p001pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(7usize).cast() }
    }
    #[doc = "0x08 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p002pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0a - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p002pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
    #[doc = "0x0b - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p002pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(11usize).cast() }
    }
    #[doc = "0x0c - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p003pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0e - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p003pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0f - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p003pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(15usize).cast() }
    }
    #[doc = "0x10 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p004pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x12 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p004pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x13 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p004pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(19usize).cast() }
    }
    #[doc = "0x14 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p005pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x16 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p005pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x17 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p005pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(23usize).cast() }
    }
    #[doc = "0x18 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p006pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1a - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p006pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1b - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p006pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(27usize).cast() }
    }
    #[doc = "0x1c - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p007pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1e - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p007pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x1f - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p007pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(31usize).cast() }
    }
    #[doc = "0x20 - P008 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P008PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x22 - P008 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P008PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(34usize).cast() }
    }
    #[doc = "0x23 - P008 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P008PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(35usize).cast() }
    }
    #[doc = "0x38 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P014PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x3a - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P014PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3b - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(59usize).cast() }
    }
    #[doc = "0x3c - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P014PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3e - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P014PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
    #[doc = "0x3f - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(63usize).cast() }
    }
    #[doc = "0x40 - P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs(&self) -> &P100PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x42 - P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(66usize).cast() }
    }
    #[doc = "0x43 - P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(67usize).cast() }
    }
    #[doc = "0x44 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs(&self) -> &P101PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x46 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(70usize).cast() }
    }
    #[doc = "0x47 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(71usize).cast() }
    }
    #[doc = "0x48 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs(&self) -> &P101PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x4a - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(74usize).cast() }
    }
    #[doc = "0x4b - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(75usize).cast() }
    }
    #[doc = "0x4c - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs(&self) -> &P101PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x4e - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(78usize).cast() }
    }
    #[doc = "0x4f - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(79usize).cast() }
    }
    #[doc = "0x50 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs(&self) -> &P101PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(80usize).cast() }
    }
    #[doc = "0x52 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(82usize).cast() }
    }
    #[doc = "0x53 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(83usize).cast() }
    }
    #[doc = "0x54 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs(&self) -> &P101PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x56 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(86usize).cast() }
    }
    #[doc = "0x57 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(87usize).cast() }
    }
    #[doc = "0x58 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs(&self) -> &P101PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(88usize).cast() }
    }
    #[doc = "0x5a - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(90usize).cast() }
    }
    #[doc = "0x5b - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(91usize).cast() }
    }
    #[doc = "0x5c - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs(&self) -> &P101PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(92usize).cast() }
    }
    #[doc = "0x5e - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(94usize).cast() }
    }
    #[doc = "0x5f - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(95usize).cast() }
    }
    #[doc = "0x60 - P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs(&self) -> &P108PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(96usize).cast() }
    }
    #[doc = "0x62 - P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(&self) -> &P108PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(98usize).cast() }
    }
    #[doc = "0x63 - P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_by(&self) -> &P108PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(99usize).cast() }
    }
    #[doc = "0x64 - P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs(&self) -> &P109PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(100usize).cast() }
    }
    #[doc = "0x66 - P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(&self) -> &P109PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(102usize).cast() }
    }
    #[doc = "0x67 - P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_by(&self) -> &P109PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(103usize).cast() }
    }
    #[doc = "0x68 - P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(104usize).cast() }
    }
    #[doc = "0x6a - P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x6b - P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(107usize).cast() }
    }
    #[doc = "0x6c - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs(&self) -> &P111PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(108usize).cast() }
    }
    #[doc = "0x6e - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(110usize).cast() }
    }
    #[doc = "0x6f - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(111usize).cast() }
    }
    #[doc = "0x70 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs(&self) -> &P111PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(112usize).cast() }
    }
    #[doc = "0x72 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x73 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(115usize).cast() }
    }
    #[doc = "0x74 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p113pfs(&self) -> &P111PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(116usize).cast() }
    }
    #[doc = "0x76 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p113pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(118usize).cast() }
    }
    #[doc = "0x77 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p113pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(119usize).cast() }
    }
    #[doc = "0x78 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p114pfs(&self) -> &P111PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(120usize).cast() }
    }
    #[doc = "0x7a - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p114pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(122usize).cast() }
    }
    #[doc = "0x7b - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p114pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(123usize).cast() }
    }
    #[doc = "0x7c - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p115pfs(&self) -> &P111PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(124usize).cast() }
    }
    #[doc = "0x7e - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p115pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(126usize).cast() }
    }
    #[doc = "0x7f - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p115pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(127usize).cast() }
    }
    #[doc = "0x80 - P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs(&self) -> &P200PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(128usize).cast() }
    }
    #[doc = "0x82 - P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(&self) -> &P200PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(130usize).cast() }
    }
    #[doc = "0x83 - P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_by(&self) -> &P200PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(131usize).cast() }
    }
    #[doc = "0x84 - P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P201PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x86 - P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(&self) -> &P201PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(134usize).cast() }
    }
    #[doc = "0x87 - P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_by(&self) -> &P201PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(135usize).cast() }
    }
    #[doc = "0x94 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P205PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(148usize).cast() }
    }
    #[doc = "0x96 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P205PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(150usize).cast() }
    }
    #[doc = "0x97 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P205PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(151usize).cast() }
    }
    #[doc = "0x98 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P205PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(152usize).cast() }
    }
    #[doc = "0x9a - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P205PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(154usize).cast() }
    }
    #[doc = "0x9b - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P205PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(155usize).cast() }
    }
    #[doc = "0x9c - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p207pfs(&self) -> &P205PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(156usize).cast() }
    }
    #[doc = "0x9e - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p207pfs_ha(&self) -> &P205PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(158usize).cast() }
    }
    #[doc = "0x9f - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p207pfs_by(&self) -> &P205PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(159usize).cast() }
    }
    #[doc = "0xa0 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p208pfs(&self) -> &P205PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(160usize).cast() }
    }
    #[doc = "0xa2 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p208pfs_ha(&self) -> &P205PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(162usize).cast() }
    }
    #[doc = "0xa3 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p208pfs_by(&self) -> &P205PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(163usize).cast() }
    }
    #[doc = "0xa4 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p209pfs(&self) -> &P205PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(164usize).cast() }
    }
    #[doc = "0xa6 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p209pfs_ha(&self) -> &P205PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(166usize).cast() }
    }
    #[doc = "0xa7 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p209pfs_by(&self) -> &P205PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(167usize).cast() }
    }
    #[doc = "0xa8 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p210pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(168usize).cast() }
    }
    #[doc = "0xaa - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p210pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(170usize).cast() }
    }
    #[doc = "0xab - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p210pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(171usize).cast() }
    }
    #[doc = "0xac - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p211pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(172usize).cast() }
    }
    #[doc = "0xae - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p211pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(174usize).cast() }
    }
    #[doc = "0xaf - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p211pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(175usize).cast() }
    }
    #[doc = "0xb0 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(176usize).cast() }
    }
    #[doc = "0xb2 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(178usize).cast() }
    }
    #[doc = "0xb3 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(179usize).cast() }
    }
    #[doc = "0xb4 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(180usize).cast() }
    }
    #[doc = "0xb6 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(182usize).cast() }
    }
    #[doc = "0xb7 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(183usize).cast() }
    }
    #[doc = "0xb8 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(184usize).cast() }
    }
    #[doc = "0xba - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(186usize).cast() }
    }
    #[doc = "0xbb - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(187usize).cast() }
    }
    #[doc = "0xc0 - P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs(&self) -> &P300PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(192usize).cast() }
    }
    #[doc = "0xc2 - P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_ha(&self) -> &P300PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(194usize).cast() }
    }
    #[doc = "0xc3 - P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_by(&self) -> &P300PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(195usize).cast() }
    }
    #[doc = "0xc4 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p301pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(196usize).cast() }
    }
    #[doc = "0xc6 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p301pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(198usize).cast() }
    }
    #[doc = "0xc7 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p301pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(199usize).cast() }
    }
    #[doc = "0xc8 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p302pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(200usize).cast() }
    }
    #[doc = "0xca - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p302pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(202usize).cast() }
    }
    #[doc = "0xcb - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p302pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(203usize).cast() }
    }
    #[doc = "0xcc - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p303pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(204usize).cast() }
    }
    #[doc = "0xce - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p303pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(206usize).cast() }
    }
    #[doc = "0xcf - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p303pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(207usize).cast() }
    }
    #[doc = "0xd0 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p304pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(208usize).cast() }
    }
    #[doc = "0xd2 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p304pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(210usize).cast() }
    }
    #[doc = "0xd3 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p304pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(211usize).cast() }
    }
    #[doc = "0xd4 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p305pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(212usize).cast() }
    }
    #[doc = "0xd6 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p305pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(214usize).cast() }
    }
    #[doc = "0xd7 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p305pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(215usize).cast() }
    }
    #[doc = "0xd8 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p306pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(216usize).cast() }
    }
    #[doc = "0xda - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p306pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(218usize).cast() }
    }
    #[doc = "0xdb - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p306pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(219usize).cast() }
    }
    #[doc = "0xdc - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p307pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(220usize).cast() }
    }
    #[doc = "0xde - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p307pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(222usize).cast() }
    }
    #[doc = "0xdf - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p307pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(223usize).cast() }
    }
    #[doc = "0x100..0x128 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs(&self) -> &[P400PFS; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
    #[doc = "0x102 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(258usize).cast() }
    }
    #[doc = "0x103 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(259usize).cast() }
    }
    #[doc = "0x106 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p401pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(262usize).cast() }
    }
    #[doc = "0x107 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p401pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(263usize).cast() }
    }
    #[doc = "0x10a - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p402pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(266usize).cast() }
    }
    #[doc = "0x10b - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p402pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(267usize).cast() }
    }
    #[doc = "0x10e - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p403pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(270usize).cast() }
    }
    #[doc = "0x10f - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p403pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(271usize).cast() }
    }
    #[doc = "0x112 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p404pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(274usize).cast() }
    }
    #[doc = "0x113 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p404pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(275usize).cast() }
    }
    #[doc = "0x116 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p405pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(278usize).cast() }
    }
    #[doc = "0x117 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p405pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(279usize).cast() }
    }
    #[doc = "0x11a - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p406pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(282usize).cast() }
    }
    #[doc = "0x11b - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p406pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(283usize).cast() }
    }
    #[doc = "0x11e - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(286usize).cast() }
    }
    #[doc = "0x11f - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(287usize).cast() }
    }
    #[doc = "0x122 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(290usize).cast() }
    }
    #[doc = "0x123 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(291usize).cast() }
    }
    #[doc = "0x126 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(294usize).cast() }
    }
    #[doc = "0x127 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(295usize).cast() }
    }
    #[doc = "0x128 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p410pfs(&self) -> &P410PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
    #[doc = "0x12a - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p410pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(298usize).cast() }
    }
    #[doc = "0x12b - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p410pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(299usize).cast() }
    }
    #[doc = "0x12c - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p411pfs(&self) -> &P410PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(300usize).cast() }
    }
    #[doc = "0x12e - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p411pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(302usize).cast() }
    }
    #[doc = "0x12f - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p411pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(303usize).cast() }
    }
    #[doc = "0x130 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p412pfs(&self) -> &P410PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(304usize).cast() }
    }
    #[doc = "0x132 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p412pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(306usize).cast() }
    }
    #[doc = "0x133 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p412pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(307usize).cast() }
    }
    #[doc = "0x134 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p413pfs(&self) -> &P410PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(308usize).cast() }
    }
    #[doc = "0x136 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p413pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(310usize).cast() }
    }
    #[doc = "0x137 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p413pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(311usize).cast() }
    }
    #[doc = "0x138 - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p414pfs(&self) -> &P410PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(312usize).cast() }
    }
    #[doc = "0x13a - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p414pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(314usize).cast() }
    }
    #[doc = "0x13b - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p414pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(315usize).cast() }
    }
    #[doc = "0x13c - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p415pfs(&self) -> &P410PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(316usize).cast() }
    }
    #[doc = "0x13e - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p415pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(318usize).cast() }
    }
    #[doc = "0x13f - P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p415pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(319usize).cast() }
    }
    #[doc = "0x140..0x154 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs(&self) -> &[P500PFS; 5] {
        unsafe { &*(self as *const Self).cast::<u8>().add(320usize).cast() }
    }
    #[doc = "0x142 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(322usize).cast() }
    }
    #[doc = "0x143 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(323usize).cast() }
    }
    #[doc = "0x146 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p501pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(326usize).cast() }
    }
    #[doc = "0x147 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p501pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(327usize).cast() }
    }
    #[doc = "0x14a - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p502pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(330usize).cast() }
    }
    #[doc = "0x14b - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p502pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(331usize).cast() }
    }
    #[doc = "0x14e - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p503pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(334usize).cast() }
    }
    #[doc = "0x14f - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p503pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(335usize).cast() }
    }
    #[doc = "0x152 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p504pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(338usize).cast() }
    }
    #[doc = "0x153 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p504pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(339usize).cast() }
    }
    #[doc = "0x160 - P508 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p508pfs(&self) -> &P508PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(352usize).cast() }
    }
    #[doc = "0x162 - P508 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p508pfs_ha(&self) -> &P508PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(354usize).cast() }
    }
    #[doc = "0x163 - P508 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p508pfs_by(&self) -> &P508PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(355usize).cast() }
    }
    #[doc = "0x180..0x18c - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p600pfs(&self) -> &[P600PFS; 3] {
        unsafe { &*(self as *const Self).cast::<u8>().add(384usize).cast() }
    }
    #[doc = "0x182 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p600pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(386usize).cast() }
    }
    #[doc = "0x183 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p600pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(387usize).cast() }
    }
    #[doc = "0x186 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p601pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(390usize).cast() }
    }
    #[doc = "0x187 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p601pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(391usize).cast() }
    }
    #[doc = "0x18a - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p602pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(394usize).cast() }
    }
    #[doc = "0x18b - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p602pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(395usize).cast() }
    }
    #[doc = "0x1a0 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p608pfs(&self) -> &P608PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(416usize).cast() }
    }
    #[doc = "0x1a2 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p608pfs_ha(&self) -> &P608PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(418usize).cast() }
    }
    #[doc = "0x1a3 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p608pfs_by(&self) -> &P608PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(419usize).cast() }
    }
    #[doc = "0x1a4 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p609pfs(&self) -> &P608PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(420usize).cast() }
    }
    #[doc = "0x1a6 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p609pfs_ha(&self) -> &P608PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(422usize).cast() }
    }
    #[doc = "0x1a7 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p609pfs_by(&self) -> &P608PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(423usize).cast() }
    }
    #[doc = "0x1a8 - P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs(&self) -> &P610PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(424usize).cast() }
    }
    #[doc = "0x1aa - P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(426usize).cast() }
    }
    #[doc = "0x1ab - P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(427usize).cast() }
    }
    #[doc = "0x1e0 - P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs(&self) -> &P708PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(480usize).cast() }
    }
    #[doc = "0x1e2 - P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_ha(&self) -> &P708PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(482usize).cast() }
    }
    #[doc = "0x1e3 - P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_by(&self) -> &P708PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(483usize).cast() }
    }
}
#[doc = "P000PFS (rw) register accessor: an alias for `Reg<P000PFS_SPEC>`"]
pub type P000PFS = crate::Reg<p000pfs::P000PFS_SPEC>;
#[doc = "P000 Pin Function Control Register"]
pub mod p000pfs;
#[doc = "P000PFS_HA (rw) register accessor: an alias for `Reg<P000PFS_HA_SPEC>`"]
pub type P000PFS_HA = crate::Reg<p000pfs_ha::P000PFS_HA_SPEC>;
#[doc = "P000 Pin Function Control Register"]
pub mod p000pfs_ha;
#[doc = "P000PFS_BY (rw) register accessor: an alias for `Reg<P000PFS_BY_SPEC>`"]
pub type P000PFS_BY = crate::Reg<p000pfs_by::P000PFS_BY_SPEC>;
#[doc = "P000 Pin Function Control Register"]
pub mod p000pfs_by;
pub use p000pfs as p001pfs;
pub use p000pfs as p008pfs;
pub use p000pfs as p014pfs;
pub use p000pfs_by as p001pfs_by;
pub use p000pfs_by as p008pfs_by;
pub use p000pfs_by as p014pfs_by;
pub use p000pfs_ha as p001pfs_ha;
pub use p000pfs_ha as p008pfs_ha;
pub use p000pfs_ha as p014pfs_ha;
pub use P000PFS as P001PFS;
pub use P000PFS as P008PFS;
pub use P000PFS as P014PFS;
pub use P000PFS_BY as P001PFS_BY;
pub use P000PFS_BY as P008PFS_BY;
pub use P000PFS_BY as P014PFS_BY;
pub use P000PFS_HA as P001PFS_HA;
pub use P000PFS_HA as P008PFS_HA;
pub use P000PFS_HA as P014PFS_HA;
#[doc = "P100PFS (rw) register accessor: an alias for `Reg<P100PFS_SPEC>`"]
pub type P100PFS = crate::Reg<p100pfs::P100PFS_SPEC>;
#[doc = "P100 Pin Function Control Register"]
pub mod p100pfs;
#[doc = "P100PFS_HA (rw) register accessor: an alias for `Reg<P100PFS_HA_SPEC>`"]
pub type P100PFS_HA = crate::Reg<p100pfs_ha::P100PFS_HA_SPEC>;
#[doc = "P100 Pin Function Control Register"]
pub mod p100pfs_ha;
#[doc = "P100PFS_BY (rw) register accessor: an alias for `Reg<P100PFS_BY_SPEC>`"]
pub type P100PFS_BY = crate::Reg<p100pfs_by::P100PFS_BY_SPEC>;
#[doc = "P100 Pin Function Control Register"]
pub mod p100pfs_by;
pub use p100pfs as p101pfs;
pub use p100pfs_by as p101pfs_by;
pub use p100pfs_ha as p101pfs_ha;
pub use P100PFS as P101PFS;
pub use P100PFS_BY as P101PFS_BY;
pub use P100PFS_HA as P101PFS_HA;
#[doc = "P108PFS (rw) register accessor: an alias for `Reg<P108PFS_SPEC>`"]
pub type P108PFS = crate::Reg<p108pfs::P108PFS_SPEC>;
#[doc = "P108 Pin Function Control Register"]
pub mod p108pfs;
#[doc = "P108PFS_HA (rw) register accessor: an alias for `Reg<P108PFS_HA_SPEC>`"]
pub type P108PFS_HA = crate::Reg<p108pfs_ha::P108PFS_HA_SPEC>;
#[doc = "P108 Pin Function Control Register"]
pub mod p108pfs_ha;
#[doc = "P108PFS_BY (rw) register accessor: an alias for `Reg<P108PFS_BY_SPEC>`"]
pub type P108PFS_BY = crate::Reg<p108pfs_by::P108PFS_BY_SPEC>;
#[doc = "P108 Pin Function Control Register"]
pub mod p108pfs_by;
pub use p108pfs as p109pfs;
pub use p108pfs_by as p109pfs_by;
pub use p108pfs_ha as p109pfs_ha;
pub use P108PFS as P109PFS;
pub use P108PFS_BY as P109PFS_BY;
pub use P108PFS_HA as P109PFS_HA;
#[doc = "P110PFS (rw) register accessor: an alias for `Reg<P110PFS_SPEC>`"]
pub type P110PFS = crate::Reg<p110pfs::P110PFS_SPEC>;
#[doc = "P110 Pin Function Control Register"]
pub mod p110pfs;
#[doc = "P110PFS_HA (rw) register accessor: an alias for `Reg<P110PFS_HA_SPEC>`"]
pub type P110PFS_HA = crate::Reg<p110pfs_ha::P110PFS_HA_SPEC>;
#[doc = "P110 Pin Function Control Register"]
pub mod p110pfs_ha;
#[doc = "P110PFS_BY (rw) register accessor: an alias for `Reg<P110PFS_BY_SPEC>`"]
pub type P110PFS_BY = crate::Reg<p110pfs_by::P110PFS_BY_SPEC>;
#[doc = "P110 Pin Function Control Register"]
pub mod p110pfs_by;
pub use p000pfs as p111pfs;
pub use p000pfs as p200pfs;
pub use p000pfs_by as p111pfs_by;
pub use p000pfs_by as p200pfs_by;
pub use p000pfs_ha as p111pfs_ha;
pub use p000pfs_ha as p200pfs_ha;
pub use P000PFS as P111PFS;
pub use P000PFS as P200PFS;
pub use P000PFS_BY as P111PFS_BY;
pub use P000PFS_BY as P200PFS_BY;
pub use P000PFS_HA as P111PFS_HA;
pub use P000PFS_HA as P200PFS_HA;
#[doc = "P201PFS (rw) register accessor: an alias for `Reg<P201PFS_SPEC>`"]
pub type P201PFS = crate::Reg<p201pfs::P201PFS_SPEC>;
#[doc = "P201 Pin Function Control Register"]
pub mod p201pfs;
#[doc = "P201PFS_HA (rw) register accessor: an alias for `Reg<P201PFS_HA_SPEC>`"]
pub type P201PFS_HA = crate::Reg<p201pfs_ha::P201PFS_HA_SPEC>;
#[doc = "P201 Pin Function Control Register"]
pub mod p201pfs_ha;
#[doc = "P201PFS_BY (rw) register accessor: an alias for `Reg<P201PFS_BY_SPEC>`"]
pub type P201PFS_BY = crate::Reg<p201pfs_by::P201PFS_BY_SPEC>;
#[doc = "P201 Pin Function Control Register"]
pub mod p201pfs_by;
pub use p000pfs as p205pfs;
pub use p000pfs as p210pfs;
pub use p000pfs as p301pfs;
pub use p000pfs as p500pfs;
pub use p000pfs as p508pfs;
pub use p000pfs as p600pfs;
pub use p000pfs as p608pfs;
pub use p000pfs as p610pfs;
pub use p000pfs as p708pfs;
pub use p000pfs_by as p205pfs_by;
pub use p000pfs_by as p210pfs_by;
pub use p000pfs_by as p301pfs_by;
pub use p000pfs_by as p500pfs_by;
pub use p000pfs_by as p508pfs_by;
pub use p000pfs_by as p600pfs_by;
pub use p000pfs_by as p608pfs_by;
pub use p000pfs_by as p610pfs_by;
pub use p000pfs_by as p708pfs_by;
pub use p000pfs_ha as p205pfs_ha;
pub use p000pfs_ha as p210pfs_ha;
pub use p000pfs_ha as p301pfs_ha;
pub use p000pfs_ha as p500pfs_ha;
pub use p000pfs_ha as p508pfs_ha;
pub use p000pfs_ha as p600pfs_ha;
pub use p000pfs_ha as p608pfs_ha;
pub use p000pfs_ha as p610pfs_ha;
pub use p000pfs_ha as p708pfs_ha;
pub use p100pfs as p400pfs;
pub use p100pfs as p410pfs;
pub use p100pfs_by as p400pfs_by;
pub use p100pfs_by as p410pfs_by;
pub use p100pfs_ha as p400pfs_ha;
pub use p100pfs_ha as p410pfs_ha;
pub use p110pfs as p300pfs;
pub use p110pfs_by as p300pfs_by;
pub use p110pfs_ha as p300pfs_ha;
pub use P000PFS as P205PFS;
pub use P000PFS as P210PFS;
pub use P000PFS as P301PFS;
pub use P000PFS as P500PFS;
pub use P000PFS as P508PFS;
pub use P000PFS as P600PFS;
pub use P000PFS as P608PFS;
pub use P000PFS as P610PFS;
pub use P000PFS as P708PFS;
pub use P000PFS_BY as P205PFS_BY;
pub use P000PFS_BY as P210PFS_BY;
pub use P000PFS_BY as P301PFS_BY;
pub use P000PFS_BY as P500PFS_BY;
pub use P000PFS_BY as P508PFS_BY;
pub use P000PFS_BY as P600PFS_BY;
pub use P000PFS_BY as P608PFS_BY;
pub use P000PFS_BY as P610PFS_BY;
pub use P000PFS_BY as P708PFS_BY;
pub use P000PFS_HA as P205PFS_HA;
pub use P000PFS_HA as P210PFS_HA;
pub use P000PFS_HA as P301PFS_HA;
pub use P000PFS_HA as P500PFS_HA;
pub use P000PFS_HA as P508PFS_HA;
pub use P000PFS_HA as P600PFS_HA;
pub use P000PFS_HA as P608PFS_HA;
pub use P000PFS_HA as P610PFS_HA;
pub use P000PFS_HA as P708PFS_HA;
pub use P100PFS as P400PFS;
pub use P100PFS as P410PFS;
pub use P100PFS_BY as P400PFS_BY;
pub use P100PFS_BY as P410PFS_BY;
pub use P100PFS_HA as P400PFS_HA;
pub use P100PFS_HA as P410PFS_HA;
pub use P110PFS as P300PFS;
pub use P110PFS_BY as P300PFS_BY;
pub use P110PFS_HA as P300PFS_HA;
