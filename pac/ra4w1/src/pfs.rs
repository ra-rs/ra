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
    _reserved_9_p009: [u8; 0x04],
    _reserved_10_p010: [u8; 0x04],
    _reserved_11_p011: [u8; 0x04],
    _reserved_12_p012: [u8; 0x04],
    _reserved_13_p013: [u8; 0x04],
    _reserved_14_p014: [u8; 0x04],
    _reserved_15_p015: [u8; 0x04],
    _reserved_16_p10: [u8; 0x20],
    _reserved_17_p108: [u8; 0x04],
    _reserved_18_p109: [u8; 0x04],
    _reserved_19_p110: [u8; 0x04],
    _reserved_20_p111: [u8; 0x04],
    _reserved_21_p112: [u8; 0x04],
    _reserved_22_p113: [u8; 0x04],
    _reserved_23_p114: [u8; 0x04],
    _reserved_24_p115: [u8; 0x04],
    _reserved_25_p200: [u8; 0x04],
    _reserved_26_p201: [u8; 0x04],
    _reserved_27_p202: [u8; 0x04],
    _reserved_28_p203: [u8; 0x04],
    _reserved_29_p204: [u8; 0x04],
    _reserved_30_p205: [u8; 0x04],
    _reserved_31_p206: [u8; 0x04],
    _reserved32: [u8; 0x14],
    _reserved_32_p212: [u8; 0x04],
    _reserved_33_p213: [u8; 0x04],
    _reserved_34_p214: [u8; 0x04],
    _reserved_35_p215: [u8; 0x04],
    _reserved_36_p300: [u8; 0x04],
    _reserved_37_p301: [u8; 0x04],
    _reserved_38_p302: [u8; 0x04],
    _reserved_39_p303: [u8; 0x04],
    _reserved_40_p304: [u8; 0x04],
    _reserved_41_p305: [u8; 0x04],
    _reserved_42_p306: [u8; 0x04],
    _reserved_43_p307: [u8; 0x04],
    _reserved_44_p308: [u8; 0x04],
    _reserved_45_p309: [u8; 0x04],
    _reserved_46_p310: [u8; 0x04],
    _reserved_47_p311: [u8; 0x04],
    _reserved_48_p312: [u8; 0x04],
    _reserved_49_p313: [u8; 0x04],
    _reserved_50_p314: [u8; 0x04],
    _reserved_51_p315: [u8; 0x04],
    _reserved_52_p40: [u8; 0x20],
    _reserved_53_p408: [u8; 0x04],
    _reserved_54_p409: [u8; 0x04],
    _reserved_55_p410: [u8; 0x04],
    _reserved_56_p411: [u8; 0x04],
    _reserved_57_p412: [u8; 0x04],
    _reserved_58_p413: [u8; 0x04],
    _reserved_59_p414: [u8; 0x04],
    _reserved_60_p415: [u8; 0x04],
    _reserved_61_p50: [u8; 0x20],
    _reserved62: [u8; 0x0c],
    _reserved_62_p511: [u8; 0x04],
    _reserved_63_p512: [u8; 0x04],
    _reserved64: [u8; 0x0c],
    _reserved_64_p60: [u8; 0x1c],
    _reserved65: [u8; 0x04],
    _reserved_65_p608: [u8; 0x04],
    _reserved_66_p609: [u8; 0x04],
    _reserved_67_p610: [u8; 0x04],
    _reserved_68_p611: [u8; 0x04],
    _reserved_69_p612: [u8; 0x04],
    _reserved_70_p613: [u8; 0x04],
    _reserved_71_p614: [u8; 0x04],
    _reserved72: [u8; 0x04],
    _reserved_72_p70: [u8; 0x18],
    _reserved73: [u8; 0x08],
    _reserved_73_p708: [u8; 0x04],
    _reserved_74_p709: [u8; 0x04],
    _reserved_75_p710: [u8; 0x04],
    _reserved_76_p711: [u8; 0x04],
    _reserved_77_p712: [u8; 0x04],
    _reserved_78_p713: [u8; 0x04],
    _reserved79: [u8; 0x08],
    _reserved_79_p80: [u8; 0x28],
    _reserved80: [u8; 0x18],
    _reserved_80_p90: [u8; 0x0c],
    _reserved81: [u8; 0x2c],
    _reserved_81_p914: [u8; 0x04],
    _reserved_82_p915: [u8; 0x04],
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
    #[doc = "0x20 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x22 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(34usize).cast() }
    }
    #[doc = "0x23 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(35usize).cast() }
    }
    #[doc = "0x24 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p009pfs(&self) -> &P001PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x26 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p009pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(38usize).cast() }
    }
    #[doc = "0x27 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p009pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(39usize).cast() }
    }
    #[doc = "0x28 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs(&self) -> &P010PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x2a - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(42usize).cast() }
    }
    #[doc = "0x2b - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(43usize).cast() }
    }
    #[doc = "0x2c - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p011pfs(&self) -> &P010PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2e - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p011pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(46usize).cast() }
    }
    #[doc = "0x2f - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p011pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(47usize).cast() }
    }
    #[doc = "0x30 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs(&self) -> &P010PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x32 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(50usize).cast() }
    }
    #[doc = "0x33 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(51usize).cast() }
    }
    #[doc = "0x34 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs(&self) -> &P010PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x36 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(54usize).cast() }
    }
    #[doc = "0x37 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(55usize).cast() }
    }
    #[doc = "0x38 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P010PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x3a - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3b - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(59usize).cast() }
    }
    #[doc = "0x3c - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P010PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3e - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
    #[doc = "0x3f - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(63usize).cast() }
    }
    #[doc = "0x40..0x60 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs(&self) -> &[P100PFS; 8] {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x42 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(66usize).cast() }
    }
    #[doc = "0x43 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(67usize).cast() }
    }
    #[doc = "0x46 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(70usize).cast() }
    }
    #[doc = "0x47 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(71usize).cast() }
    }
    #[doc = "0x4a - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(74usize).cast() }
    }
    #[doc = "0x4b - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(75usize).cast() }
    }
    #[doc = "0x4e - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(78usize).cast() }
    }
    #[doc = "0x4f - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(79usize).cast() }
    }
    #[doc = "0x52 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(82usize).cast() }
    }
    #[doc = "0x53 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(83usize).cast() }
    }
    #[doc = "0x56 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(86usize).cast() }
    }
    #[doc = "0x57 - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(87usize).cast() }
    }
    #[doc = "0x5a - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(90usize).cast() }
    }
    #[doc = "0x5b - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(91usize).cast() }
    }
    #[doc = "0x5e - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(94usize).cast() }
    }
    #[doc = "0x5f - P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs_by(&self) -> &P100PFS_BY {
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
    #[doc = "0x88 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p202pfs(&self) -> &P202PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(136usize).cast() }
    }
    #[doc = "0x8a - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p202pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(138usize).cast() }
    }
    #[doc = "0x8b - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p202pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(139usize).cast() }
    }
    #[doc = "0x8c - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p203pfs(&self) -> &P202PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(140usize).cast() }
    }
    #[doc = "0x8e - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p203pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(142usize).cast() }
    }
    #[doc = "0x8f - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p203pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(143usize).cast() }
    }
    #[doc = "0x90 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs(&self) -> &P202PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(144usize).cast() }
    }
    #[doc = "0x92 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(146usize).cast() }
    }
    #[doc = "0x93 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(147usize).cast() }
    }
    #[doc = "0x94 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P202PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(148usize).cast() }
    }
    #[doc = "0x96 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(150usize).cast() }
    }
    #[doc = "0x97 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(151usize).cast() }
    }
    #[doc = "0x98 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P202PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(152usize).cast() }
    }
    #[doc = "0x9a - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(154usize).cast() }
    }
    #[doc = "0x9b - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(155usize).cast() }
    }
    #[doc = "0xb0 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(176usize).cast() }
    }
    #[doc = "0xb2 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(178usize).cast() }
    }
    #[doc = "0xb3 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(179usize).cast() }
    }
    #[doc = "0xb4 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(180usize).cast() }
    }
    #[doc = "0xb6 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(182usize).cast() }
    }
    #[doc = "0xb7 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(183usize).cast() }
    }
    #[doc = "0xb8 - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(184usize).cast() }
    }
    #[doc = "0xba - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(186usize).cast() }
    }
    #[doc = "0xbb - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(187usize).cast() }
    }
    #[doc = "0xbc - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p215pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(188usize).cast() }
    }
    #[doc = "0xbe - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p215pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(190usize).cast() }
    }
    #[doc = "0xbf - P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p215pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(191usize).cast() }
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
    #[doc = "0xe0 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p308pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(224usize).cast() }
    }
    #[doc = "0xe2 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p308pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(226usize).cast() }
    }
    #[doc = "0xe3 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p308pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(227usize).cast() }
    }
    #[doc = "0xe4 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p309pfs(&self) -> &P301PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(228usize).cast() }
    }
    #[doc = "0xe6 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p309pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(230usize).cast() }
    }
    #[doc = "0xe7 - P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p309pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(231usize).cast() }
    }
    #[doc = "0xe8 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p310pfs(&self) -> &P310PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(232usize).cast() }
    }
    #[doc = "0xea - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p310pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(234usize).cast() }
    }
    #[doc = "0xeb - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p310pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(235usize).cast() }
    }
    #[doc = "0xec - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p311pfs(&self) -> &P310PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(236usize).cast() }
    }
    #[doc = "0xee - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p311pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(238usize).cast() }
    }
    #[doc = "0xef - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p311pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(239usize).cast() }
    }
    #[doc = "0xf0 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p312pfs(&self) -> &P310PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(240usize).cast() }
    }
    #[doc = "0xf2 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p312pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(242usize).cast() }
    }
    #[doc = "0xf3 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p312pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(243usize).cast() }
    }
    #[doc = "0xf4 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p313pfs(&self) -> &P310PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(244usize).cast() }
    }
    #[doc = "0xf6 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p313pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(246usize).cast() }
    }
    #[doc = "0xf7 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p313pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(247usize).cast() }
    }
    #[doc = "0xf8 - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p314pfs(&self) -> &P310PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(248usize).cast() }
    }
    #[doc = "0xfa - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p314pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(250usize).cast() }
    }
    #[doc = "0xfb - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p314pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(251usize).cast() }
    }
    #[doc = "0xfc - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p315pfs(&self) -> &P310PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(252usize).cast() }
    }
    #[doc = "0xfe - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p315pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(254usize).cast() }
    }
    #[doc = "0xff - P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p315pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(255usize).cast() }
    }
    #[doc = "0x100..0x120 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs(&self) -> &[P400PFS; 8] {
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
    #[doc = "0x120 - P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs(&self) -> &P408PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(288usize).cast() }
    }
    #[doc = "0x122 - P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_ha(&self) -> &P408PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(290usize).cast() }
    }
    #[doc = "0x123 - P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_by(&self) -> &P408PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(291usize).cast() }
    }
    #[doc = "0x124 - P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs(&self) -> &P409PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(292usize).cast() }
    }
    #[doc = "0x126 - P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_ha(&self) -> &P409PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(294usize).cast() }
    }
    #[doc = "0x127 - P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_by(&self) -> &P409PFS_BY {
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
    #[doc = "0x140..0x160 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs(&self) -> &[P500PFS; 8] {
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
    #[doc = "0x156 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p505pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(342usize).cast() }
    }
    #[doc = "0x157 - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p505pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(343usize).cast() }
    }
    #[doc = "0x15a - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p506pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(346usize).cast() }
    }
    #[doc = "0x15b - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p506pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(347usize).cast() }
    }
    #[doc = "0x15e - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p507pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(350usize).cast() }
    }
    #[doc = "0x15f - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p507pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(351usize).cast() }
    }
    #[doc = "0x16c - P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p511pfs(&self) -> &P511PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(364usize).cast() }
    }
    #[doc = "0x16e - P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p511pfs_ha(&self) -> &P511PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(366usize).cast() }
    }
    #[doc = "0x16f - P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p511pfs_by(&self) -> &P511PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(367usize).cast() }
    }
    #[doc = "0x170 - P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p512pfs(&self) -> &P511PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(368usize).cast() }
    }
    #[doc = "0x172 - P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p512pfs_ha(&self) -> &P511PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(370usize).cast() }
    }
    #[doc = "0x173 - P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p512pfs_by(&self) -> &P511PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(371usize).cast() }
    }
    #[doc = "0x180..0x19c - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p600pfs(&self) -> &[P600PFS; 7] {
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
    #[doc = "0x18e - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p603pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(398usize).cast() }
    }
    #[doc = "0x18f - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p603pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(399usize).cast() }
    }
    #[doc = "0x192 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p604pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(402usize).cast() }
    }
    #[doc = "0x193 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p604pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(403usize).cast() }
    }
    #[doc = "0x196 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p605pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(406usize).cast() }
    }
    #[doc = "0x197 - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p605pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(407usize).cast() }
    }
    #[doc = "0x19a - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p606pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(410usize).cast() }
    }
    #[doc = "0x19b - P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p606pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(411usize).cast() }
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
    #[doc = "0x1a8 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs(&self) -> &P610PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(424usize).cast() }
    }
    #[doc = "0x1aa - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(426usize).cast() }
    }
    #[doc = "0x1ab - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(427usize).cast() }
    }
    #[doc = "0x1ac - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p611pfs(&self) -> &P610PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(428usize).cast() }
    }
    #[doc = "0x1ae - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p611pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(430usize).cast() }
    }
    #[doc = "0x1af - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p611pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(431usize).cast() }
    }
    #[doc = "0x1b0 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p612pfs(&self) -> &P610PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(432usize).cast() }
    }
    #[doc = "0x1b2 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p612pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(434usize).cast() }
    }
    #[doc = "0x1b3 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p612pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(435usize).cast() }
    }
    #[doc = "0x1b4 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p613pfs(&self) -> &P610PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(436usize).cast() }
    }
    #[doc = "0x1b6 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p613pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(438usize).cast() }
    }
    #[doc = "0x1b7 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p613pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(439usize).cast() }
    }
    #[doc = "0x1b8 - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p614pfs(&self) -> &P610PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(440usize).cast() }
    }
    #[doc = "0x1ba - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p614pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(442usize).cast() }
    }
    #[doc = "0x1bb - P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p614pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(443usize).cast() }
    }
    #[doc = "0x1c0..0x1d8 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p700pfs(&self) -> &[P700PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1c2 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p700pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(450usize).cast() }
    }
    #[doc = "0x1c3 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p700pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(451usize).cast() }
    }
    #[doc = "0x1c6 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p701pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(454usize).cast() }
    }
    #[doc = "0x1c7 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p701pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(455usize).cast() }
    }
    #[doc = "0x1ca - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p702pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(458usize).cast() }
    }
    #[doc = "0x1cb - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p702pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(459usize).cast() }
    }
    #[doc = "0x1ce - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p703pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(462usize).cast() }
    }
    #[doc = "0x1cf - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p703pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(463usize).cast() }
    }
    #[doc = "0x1d2 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p704pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(466usize).cast() }
    }
    #[doc = "0x1d3 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p704pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(467usize).cast() }
    }
    #[doc = "0x1d6 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p705pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(470usize).cast() }
    }
    #[doc = "0x1d7 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p705pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(471usize).cast() }
    }
    #[doc = "0x1e0 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs(&self) -> &P708PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(480usize).cast() }
    }
    #[doc = "0x1e2 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_ha(&self) -> &P708PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(482usize).cast() }
    }
    #[doc = "0x1e3 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_by(&self) -> &P708PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(483usize).cast() }
    }
    #[doc = "0x1e4 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p709pfs(&self) -> &P708PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(484usize).cast() }
    }
    #[doc = "0x1e6 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p709pfs_ha(&self) -> &P708PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(486usize).cast() }
    }
    #[doc = "0x1e7 - P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p709pfs_by(&self) -> &P708PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(487usize).cast() }
    }
    #[doc = "0x1e8 - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p710pfs(&self) -> &P710PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(488usize).cast() }
    }
    #[doc = "0x1ea - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p710pfs_ha(&self) -> &P710PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(490usize).cast() }
    }
    #[doc = "0x1eb - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p710pfs_by(&self) -> &P710PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(491usize).cast() }
    }
    #[doc = "0x1ec - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p711pfs(&self) -> &P710PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(492usize).cast() }
    }
    #[doc = "0x1ee - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p711pfs_ha(&self) -> &P710PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(494usize).cast() }
    }
    #[doc = "0x1ef - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p711pfs_by(&self) -> &P710PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(495usize).cast() }
    }
    #[doc = "0x1f0 - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p712pfs(&self) -> &P710PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(496usize).cast() }
    }
    #[doc = "0x1f2 - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p712pfs_ha(&self) -> &P710PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(498usize).cast() }
    }
    #[doc = "0x1f3 - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p712pfs_by(&self) -> &P710PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(499usize).cast() }
    }
    #[doc = "0x1f4 - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p713pfs(&self) -> &P710PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(500usize).cast() }
    }
    #[doc = "0x1f6 - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p713pfs_ha(&self) -> &P710PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(502usize).cast() }
    }
    #[doc = "0x1f7 - P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p713pfs_by(&self) -> &P710PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(503usize).cast() }
    }
    #[doc = "0x200..0x228 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p800pfs(&self) -> &[P800PFS; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(512usize).cast() }
    }
    #[doc = "0x202 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p800pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(514usize).cast() }
    }
    #[doc = "0x203 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p800pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(515usize).cast() }
    }
    #[doc = "0x206 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p801pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(518usize).cast() }
    }
    #[doc = "0x207 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p801pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(519usize).cast() }
    }
    #[doc = "0x20a - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p802pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(522usize).cast() }
    }
    #[doc = "0x20b - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p802pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(523usize).cast() }
    }
    #[doc = "0x20e - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p803pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(526usize).cast() }
    }
    #[doc = "0x20f - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p803pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(527usize).cast() }
    }
    #[doc = "0x212 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p804pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(530usize).cast() }
    }
    #[doc = "0x213 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p804pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(531usize).cast() }
    }
    #[doc = "0x216 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p805pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(534usize).cast() }
    }
    #[doc = "0x217 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p805pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(535usize).cast() }
    }
    #[doc = "0x21a - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p806pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(538usize).cast() }
    }
    #[doc = "0x21b - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p806pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(539usize).cast() }
    }
    #[doc = "0x21e - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p807pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(542usize).cast() }
    }
    #[doc = "0x21f - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p807pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(543usize).cast() }
    }
    #[doc = "0x222 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p808pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(546usize).cast() }
    }
    #[doc = "0x223 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p808pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(547usize).cast() }
    }
    #[doc = "0x226 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p809pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(550usize).cast() }
    }
    #[doc = "0x227 - P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p809pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(551usize).cast() }
    }
    #[doc = "0x240..0x24c - P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p900pfs(&self) -> &[P900PFS; 3] {
        unsafe { &*(self as *const Self).cast::<u8>().add(576usize).cast() }
    }
    #[doc = "0x242 - P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p900pfs_ha(&self) -> &P900PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(578usize).cast() }
    }
    #[doc = "0x243 - P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p900pfs_by(&self) -> &P900PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(579usize).cast() }
    }
    #[doc = "0x246 - P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p901pfs_ha(&self) -> &P900PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(582usize).cast() }
    }
    #[doc = "0x247 - P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p901pfs_by(&self) -> &P900PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(583usize).cast() }
    }
    #[doc = "0x24a - P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p902pfs_ha(&self) -> &P900PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(586usize).cast() }
    }
    #[doc = "0x24b - P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p902pfs_by(&self) -> &P900PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(587usize).cast() }
    }
    #[doc = "0x278 - P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs(&self) -> &P914PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(632usize).cast() }
    }
    #[doc = "0x27a - P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_ha(&self) -> &P914PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(634usize).cast() }
    }
    #[doc = "0x27b - P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_by(&self) -> &P914PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(635usize).cast() }
    }
    #[doc = "0x27c - P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs(&self) -> &P914PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(636usize).cast() }
    }
    #[doc = "0x27e - P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_ha(&self) -> &P914PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(638usize).cast() }
    }
    #[doc = "0x27f - P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_by(&self) -> &P914PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(639usize).cast() }
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
pub use p000pfs as p010pfs;
pub use p000pfs as p100pfs;
pub use p000pfs_by as p001pfs_by;
pub use p000pfs_by as p010pfs_by;
pub use p000pfs_by as p100pfs_by;
pub use p000pfs_ha as p001pfs_ha;
pub use p000pfs_ha as p010pfs_ha;
pub use p000pfs_ha as p100pfs_ha;
pub use P000PFS as P001PFS;
pub use P000PFS as P010PFS;
pub use P000PFS as P100PFS;
pub use P000PFS_BY as P001PFS_BY;
pub use P000PFS_BY as P010PFS_BY;
pub use P000PFS_BY as P100PFS_BY;
pub use P000PFS_HA as P001PFS_HA;
pub use P000PFS_HA as P010PFS_HA;
pub use P000PFS_HA as P100PFS_HA;
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
#[doc = "P109PFS (rw) register accessor: an alias for `Reg<P109PFS_SPEC>`"]
pub type P109PFS = crate::Reg<p109pfs::P109PFS_SPEC>;
#[doc = "P109 Pin Function Control Register"]
pub mod p109pfs;
pub use p000pfs as p111pfs;
pub use p000pfs as p200pfs;
pub use p000pfs_by as p109pfs_by;
pub use p000pfs_by as p111pfs_by;
pub use p000pfs_by as p200pfs_by;
pub use p000pfs_ha as p109pfs_ha;
pub use p000pfs_ha as p111pfs_ha;
pub use p000pfs_ha as p200pfs_ha;
pub use p108pfs as p110pfs;
pub use p108pfs_by as p110pfs_by;
pub use p108pfs_ha as p110pfs_ha;
pub use P000PFS as P111PFS;
pub use P000PFS as P200PFS;
pub use P000PFS_BY as P109PFS_BY;
pub use P000PFS_BY as P111PFS_BY;
pub use P000PFS_BY as P200PFS_BY;
pub use P000PFS_HA as P109PFS_HA;
pub use P000PFS_HA as P111PFS_HA;
pub use P000PFS_HA as P200PFS_HA;
pub use P108PFS as P110PFS;
pub use P108PFS_BY as P110PFS_BY;
pub use P108PFS_HA as P110PFS_HA;
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
pub use p000pfs as p202pfs;
pub use p000pfs as p212pfs;
pub use p000pfs as p301pfs;
pub use p000pfs as p310pfs;
pub use p000pfs as p400pfs;
pub use p000pfs_by as p202pfs_by;
pub use p000pfs_by as p212pfs_by;
pub use p000pfs_by as p301pfs_by;
pub use p000pfs_by as p310pfs_by;
pub use p000pfs_by as p400pfs_by;
pub use p000pfs_ha as p202pfs_ha;
pub use p000pfs_ha as p212pfs_ha;
pub use p000pfs_ha as p301pfs_ha;
pub use p000pfs_ha as p310pfs_ha;
pub use p000pfs_ha as p400pfs_ha;
pub use p108pfs as p300pfs;
pub use p108pfs_by as p300pfs_by;
pub use p108pfs_ha as p300pfs_ha;
pub use P000PFS as P202PFS;
pub use P000PFS as P212PFS;
pub use P000PFS as P301PFS;
pub use P000PFS as P310PFS;
pub use P000PFS as P400PFS;
pub use P000PFS_BY as P202PFS_BY;
pub use P000PFS_BY as P212PFS_BY;
pub use P000PFS_BY as P301PFS_BY;
pub use P000PFS_BY as P310PFS_BY;
pub use P000PFS_BY as P400PFS_BY;
pub use P000PFS_HA as P202PFS_HA;
pub use P000PFS_HA as P212PFS_HA;
pub use P000PFS_HA as P301PFS_HA;
pub use P000PFS_HA as P310PFS_HA;
pub use P000PFS_HA as P400PFS_HA;
pub use P108PFS as P300PFS;
pub use P108PFS_BY as P300PFS_BY;
pub use P108PFS_HA as P300PFS_HA;
#[doc = "P408PFS (rw) register accessor: an alias for `Reg<P408PFS_SPEC>`"]
pub type P408PFS = crate::Reg<p408pfs::P408PFS_SPEC>;
#[doc = "P408 Pin Function Control Register"]
pub mod p408pfs;
#[doc = "P408PFS_HA (rw) register accessor: an alias for `Reg<P408PFS_HA_SPEC>`"]
pub type P408PFS_HA = crate::Reg<p408pfs_ha::P408PFS_HA_SPEC>;
#[doc = "P408 Pin Function Control Register"]
pub mod p408pfs_ha;
pub use p000pfs as p409pfs;
pub use p000pfs as p410pfs;
pub use p000pfs as p500pfs;
pub use p000pfs as p511pfs;
pub use p000pfs as p600pfs;
pub use p000pfs as p608pfs;
pub use p000pfs as p610pfs;
pub use p000pfs as p700pfs;
pub use p000pfs as p708pfs;
pub use p000pfs as p710pfs;
pub use p000pfs as p800pfs;
pub use p000pfs as p900pfs;
pub use p000pfs_by as p408pfs_by;
pub use p000pfs_by as p409pfs_by;
pub use p000pfs_by as p410pfs_by;
pub use p000pfs_by as p500pfs_by;
pub use p000pfs_by as p511pfs_by;
pub use p000pfs_by as p600pfs_by;
pub use p000pfs_by as p608pfs_by;
pub use p000pfs_by as p610pfs_by;
pub use p000pfs_by as p700pfs_by;
pub use p000pfs_by as p708pfs_by;
pub use p000pfs_by as p710pfs_by;
pub use p000pfs_by as p800pfs_by;
pub use p000pfs_by as p900pfs_by;
pub use p000pfs_ha as p409pfs_ha;
pub use p000pfs_ha as p410pfs_ha;
pub use p000pfs_ha as p500pfs_ha;
pub use p000pfs_ha as p511pfs_ha;
pub use p000pfs_ha as p600pfs_ha;
pub use p000pfs_ha as p608pfs_ha;
pub use p000pfs_ha as p610pfs_ha;
pub use p000pfs_ha as p700pfs_ha;
pub use p000pfs_ha as p708pfs_ha;
pub use p000pfs_ha as p710pfs_ha;
pub use p000pfs_ha as p800pfs_ha;
pub use p000pfs_ha as p900pfs_ha;
pub use p109pfs as p914pfs;
pub use p109pfs_by as p914pfs_by;
pub use p109pfs_ha as p914pfs_ha;
pub use P000PFS as P409PFS;
pub use P000PFS as P410PFS;
pub use P000PFS as P500PFS;
pub use P000PFS as P511PFS;
pub use P000PFS as P600PFS;
pub use P000PFS as P608PFS;
pub use P000PFS as P610PFS;
pub use P000PFS as P700PFS;
pub use P000PFS as P708PFS;
pub use P000PFS as P710PFS;
pub use P000PFS as P800PFS;
pub use P000PFS as P900PFS;
pub use P000PFS_BY as P408PFS_BY;
pub use P000PFS_BY as P409PFS_BY;
pub use P000PFS_BY as P410PFS_BY;
pub use P000PFS_BY as P500PFS_BY;
pub use P000PFS_BY as P511PFS_BY;
pub use P000PFS_BY as P600PFS_BY;
pub use P000PFS_BY as P608PFS_BY;
pub use P000PFS_BY as P610PFS_BY;
pub use P000PFS_BY as P700PFS_BY;
pub use P000PFS_BY as P708PFS_BY;
pub use P000PFS_BY as P710PFS_BY;
pub use P000PFS_BY as P800PFS_BY;
pub use P000PFS_BY as P900PFS_BY;
pub use P000PFS_HA as P409PFS_HA;
pub use P000PFS_HA as P410PFS_HA;
pub use P000PFS_HA as P500PFS_HA;
pub use P000PFS_HA as P511PFS_HA;
pub use P000PFS_HA as P600PFS_HA;
pub use P000PFS_HA as P608PFS_HA;
pub use P000PFS_HA as P610PFS_HA;
pub use P000PFS_HA as P700PFS_HA;
pub use P000PFS_HA as P708PFS_HA;
pub use P000PFS_HA as P710PFS_HA;
pub use P000PFS_HA as P800PFS_HA;
pub use P000PFS_HA as P900PFS_HA;
pub use P109PFS as P914PFS;
pub use P109PFS_BY as P914PFS_BY;
pub use P109PFS_HA as P914PFS_HA;
