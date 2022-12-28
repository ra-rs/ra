#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_p000: [u8; 0x04],
    _reserved_1_p001: [u8; 0x04],
    _reserved_2_p002: [u8; 0x04],
    _reserved_3_p003: [u8; 0x04],
    _reserved4: [u8; 0x20],
    _reserved_4_p012: [u8; 0x04],
    _reserved_5_p013: [u8; 0x04],
    _reserved_6_p014: [u8; 0x04],
    _reserved_7_p015: [u8; 0x04],
    _reserved_8_p100: [u8; 0x04],
    _reserved_9_p101: [u8; 0x04],
    _reserved_10_p102: [u8; 0x04],
    _reserved_11_p103: [u8; 0x04],
    _reserved_12_p104: [u8; 0x04],
    _reserved_13_p105: [u8; 0x04],
    _reserved_14_p106: [u8; 0x04],
    _reserved_15_p107: [u8; 0x04],
    _reserved_16_p108: [u8; 0x04],
    _reserved_17_p109: [u8; 0x04],
    _reserved_18_p110: [u8; 0x04],
    _reserved_19_p111: [u8; 0x04],
    _reserved_20_p112: [u8; 0x04],
    _reserved21: [u8; 0x0c],
    _reserved_21_p200: [u8; 0x04],
    _reserved_22_p201: [u8; 0x04],
    _reserved23: [u8; 0x08],
    _reserved_23_p204: [u8; 0x04],
    _reserved_24_p205: [u8; 0x04],
    _reserved_25_p206: [u8; 0x04],
    _reserved26: [u8; 0x14],
    _reserved_26_p212: [u8; 0x04],
    _reserved_27_p213: [u8; 0x04],
    _reserved_28_p214: [u8; 0x04],
    _reserved_29_p215: [u8; 0x04],
    _reserved_30_p300: [u8; 0x04],
    _reserved_31_p301: [u8; 0x04],
    _reserved_32_p302: [u8; 0x04],
    _reserved_33_p303: [u8; 0x04],
    _reserved_34_p304: [u8; 0x04],
    _reserved35: [u8; 0x2c],
    _reserved_35_p40: [u8; 0x10],
    _reserved36: [u8; 0x0c],
    _reserved_36_p407: [u8; 0x04],
    _reserved_37_p408: [u8; 0x04],
    _reserved_38_p409: [u8; 0x04],
    _reserved_39_p410: [u8; 0x04],
    _reserved_40_p411: [u8; 0x04],
    _reserved41: [u8; 0x10],
    _reserved_41_p50: [u8; 0x0c],
    _reserved42: [u8; 0x012c],
    _reserved_42_p914: [u8; 0x04],
    _reserved_43_p915: [u8; 0x04],
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
    #[doc = "0x30 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs(&self) -> &P012PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x32 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs_ha(&self) -> &P012PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(50usize).cast() }
    }
    #[doc = "0x33 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs_by(&self) -> &P012PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(51usize).cast() }
    }
    #[doc = "0x34 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs(&self) -> &P012PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x36 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs_ha(&self) -> &P012PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(54usize).cast() }
    }
    #[doc = "0x37 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs_by(&self) -> &P012PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(55usize).cast() }
    }
    #[doc = "0x38 - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P012PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x3a - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P012PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3b - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P012PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(59usize).cast() }
    }
    #[doc = "0x3c - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P012PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3e - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P012PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
    #[doc = "0x3f - P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P012PFS_BY {
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
    #[doc = "0x68 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(104usize).cast() }
    }
    #[doc = "0x6a - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x6b - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(107usize).cast() }
    }
    #[doc = "0x6c - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(108usize).cast() }
    }
    #[doc = "0x6e - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(110usize).cast() }
    }
    #[doc = "0x6f - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(111usize).cast() }
    }
    #[doc = "0x70 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(112usize).cast() }
    }
    #[doc = "0x72 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x73 - P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(115usize).cast() }
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
    #[doc = "0x90 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs(&self) -> &P204PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(144usize).cast() }
    }
    #[doc = "0x92 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs_ha(&self) -> &P204PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(146usize).cast() }
    }
    #[doc = "0x93 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs_by(&self) -> &P204PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(147usize).cast() }
    }
    #[doc = "0x94 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P204PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(148usize).cast() }
    }
    #[doc = "0x96 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P204PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(150usize).cast() }
    }
    #[doc = "0x97 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P204PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(151usize).cast() }
    }
    #[doc = "0x98 - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P204PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(152usize).cast() }
    }
    #[doc = "0x9a - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P204PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(154usize).cast() }
    }
    #[doc = "0x9b - P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P204PFS_BY {
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
    #[doc = "0x100..0x110 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs(&self) -> &[P400PFS; 4] {
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
    #[doc = "0x11c - P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs(&self) -> &P407PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(284usize).cast() }
    }
    #[doc = "0x11e - P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_ha(&self) -> &P407PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(286usize).cast() }
    }
    #[doc = "0x11f - P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_by(&self) -> &P407PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(287usize).cast() }
    }
    #[doc = "0x120 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs(&self) -> &P408PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(288usize).cast() }
    }
    #[doc = "0x122 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_ha(&self) -> &P408PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(290usize).cast() }
    }
    #[doc = "0x123 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_by(&self) -> &P408PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(291usize).cast() }
    }
    #[doc = "0x124 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs(&self) -> &P408PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(292usize).cast() }
    }
    #[doc = "0x126 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_ha(&self) -> &P408PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(294usize).cast() }
    }
    #[doc = "0x127 - P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_by(&self) -> &P408PFS_BY {
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
    #[doc = "0x140..0x14c - P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs(&self) -> &[P500PFS; 3] {
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
    #[doc = "0x278 - P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs(&self) -> &P914PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(632usize).cast() }
    }
    #[doc = "0x27a - P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_ha(&self) -> &P914PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(634usize).cast() }
    }
    #[doc = "0x27b - P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_by(&self) -> &P914PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(635usize).cast() }
    }
    #[doc = "0x27c - P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs(&self) -> &P915PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(636usize).cast() }
    }
    #[doc = "0x27e - P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_ha(&self) -> &P915PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(638usize).cast() }
    }
    #[doc = "0x27f - P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_by(&self) -> &P915PFS_BY {
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
pub use p000pfs as p012pfs;
pub use p000pfs_by as p001pfs_by;
pub use p000pfs_by as p012pfs_by;
pub use p000pfs_ha as p001pfs_ha;
pub use p000pfs_ha as p012pfs_ha;
pub use P000PFS as P001PFS;
pub use P000PFS as P012PFS;
pub use P000PFS_BY as P001PFS_BY;
pub use P000PFS_BY as P012PFS_BY;
pub use P000PFS_HA as P001PFS_HA;
pub use P000PFS_HA as P012PFS_HA;
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
pub use p000pfs as p300pfs;
pub use p000pfs as p301pfs;
pub use p000pfs as p400pfs;
pub use p000pfs_by as p300pfs_by;
pub use p000pfs_by as p301pfs_by;
pub use p000pfs_by as p400pfs_by;
pub use p000pfs_ha as p300pfs_ha;
pub use p000pfs_ha as p301pfs_ha;
pub use p000pfs_ha as p400pfs_ha;
pub use p100pfs as p101pfs;
pub use p100pfs as p108pfs;
pub use p100pfs as p109pfs;
pub use p100pfs as p110pfs;
pub use p100pfs as p200pfs;
pub use p100pfs as p201pfs;
pub use p100pfs as p204pfs;
pub use p100pfs as p212pfs;
pub use p100pfs_by as p101pfs_by;
pub use p100pfs_by as p108pfs_by;
pub use p100pfs_by as p109pfs_by;
pub use p100pfs_by as p110pfs_by;
pub use p100pfs_by as p200pfs_by;
pub use p100pfs_by as p201pfs_by;
pub use p100pfs_by as p204pfs_by;
pub use p100pfs_by as p212pfs_by;
pub use p100pfs_ha as p101pfs_ha;
pub use p100pfs_ha as p108pfs_ha;
pub use p100pfs_ha as p109pfs_ha;
pub use p100pfs_ha as p110pfs_ha;
pub use p100pfs_ha as p200pfs_ha;
pub use p100pfs_ha as p201pfs_ha;
pub use p100pfs_ha as p204pfs_ha;
pub use p100pfs_ha as p212pfs_ha;
pub use P000PFS as P300PFS;
pub use P000PFS as P301PFS;
pub use P000PFS as P400PFS;
pub use P000PFS_BY as P300PFS_BY;
pub use P000PFS_BY as P301PFS_BY;
pub use P000PFS_BY as P400PFS_BY;
pub use P000PFS_HA as P300PFS_HA;
pub use P000PFS_HA as P301PFS_HA;
pub use P000PFS_HA as P400PFS_HA;
pub use P100PFS as P101PFS;
pub use P100PFS as P108PFS;
pub use P100PFS as P109PFS;
pub use P100PFS as P110PFS;
pub use P100PFS as P200PFS;
pub use P100PFS as P201PFS;
pub use P100PFS as P204PFS;
pub use P100PFS as P212PFS;
pub use P100PFS_BY as P101PFS_BY;
pub use P100PFS_BY as P108PFS_BY;
pub use P100PFS_BY as P109PFS_BY;
pub use P100PFS_BY as P110PFS_BY;
pub use P100PFS_BY as P200PFS_BY;
pub use P100PFS_BY as P201PFS_BY;
pub use P100PFS_BY as P204PFS_BY;
pub use P100PFS_BY as P212PFS_BY;
pub use P100PFS_HA as P101PFS_HA;
pub use P100PFS_HA as P108PFS_HA;
pub use P100PFS_HA as P109PFS_HA;
pub use P100PFS_HA as P110PFS_HA;
pub use P100PFS_HA as P200PFS_HA;
pub use P100PFS_HA as P201PFS_HA;
pub use P100PFS_HA as P204PFS_HA;
pub use P100PFS_HA as P212PFS_HA;
#[doc = "P407PFS (rw) register accessor: an alias for `Reg<P407PFS_SPEC>`"]
pub type P407PFS = crate::Reg<p407pfs::P407PFS_SPEC>;
#[doc = "P407 Pin Function Control Register"]
pub mod p407pfs;
#[doc = "P407PFS_HA (rw) register accessor: an alias for `Reg<P407PFS_HA_SPEC>`"]
pub type P407PFS_HA = crate::Reg<p407pfs_ha::P407PFS_HA_SPEC>;
#[doc = "P407 Pin Function Control Register"]
pub mod p407pfs_ha;
#[doc = "P407PFS_BY (rw) register accessor: an alias for `Reg<P407PFS_BY_SPEC>`"]
pub type P407PFS_BY = crate::Reg<p407pfs_by::P407PFS_BY_SPEC>;
#[doc = "P407 Pin Function Control Register"]
pub mod p407pfs_by;
pub use p000pfs as p410pfs;
pub use p000pfs as p500pfs;
pub use p000pfs as p914pfs;
pub use p000pfs as p915pfs;
pub use p000pfs_by as p410pfs_by;
pub use p000pfs_by as p500pfs_by;
pub use p000pfs_by as p914pfs_by;
pub use p000pfs_by as p915pfs_by;
pub use p000pfs_ha as p410pfs_ha;
pub use p000pfs_ha as p500pfs_ha;
pub use p000pfs_ha as p914pfs_ha;
pub use p000pfs_ha as p915pfs_ha;
pub use p407pfs as p408pfs;
pub use p407pfs_by as p408pfs_by;
pub use p407pfs_ha as p408pfs_ha;
pub use P000PFS as P410PFS;
pub use P000PFS as P500PFS;
pub use P000PFS as P914PFS;
pub use P000PFS as P915PFS;
pub use P000PFS_BY as P410PFS_BY;
pub use P000PFS_BY as P500PFS_BY;
pub use P000PFS_BY as P914PFS_BY;
pub use P000PFS_BY as P915PFS_BY;
pub use P000PFS_HA as P410PFS_HA;
pub use P000PFS_HA as P500PFS_HA;
pub use P000PFS_HA as P914PFS_HA;
pub use P000PFS_HA as P915PFS_HA;
pub use P407PFS as P408PFS;
pub use P407PFS_BY as P408PFS_BY;
pub use P407PFS_HA as P408PFS_HA;
