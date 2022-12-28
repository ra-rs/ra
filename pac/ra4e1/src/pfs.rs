#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_p0: [u8; 0x14],
    _reserved1: [u8; 0x2c],
    _reserved_1_p1: [u8; 0x28],
    _reserved_2_p: [u8; 0x10],
    _reserved3: [u8; 0x08],
    _reserved_3_p200: [u8; 0x04],
    _reserved_4_p201: [u8; 0x04],
    _reserved5: [u8; 0x0c],
    _reserved_5_p205: [u8; 0x04],
    _reserved_6_p206: [u8; 0x04],
    _reserved_7_p207: [u8; 0x04],
    _reserved_8_p208: [u8; 0x04],
    _reserved9: [u8; 0x0c],
    _reserved_9_p212: [u8; 0x04],
    _reserved_10_p213: [u8; 0x04],
    _reserved11: [u8; 0x08],
    _reserved_11_p3: [u8; 0x14],
    _reserved12: [u8; 0x2c],
    _reserved_12_p400: [u8; 0x04],
    _reserved_13_p401: [u8; 0x04],
    _reserved_14_p402: [u8; 0x04],
    _reserved15: [u8; 0x10],
    _reserved_15_p4: [u8; 0x0c],
    _reserved_16_p410: [u8; 0x04],
    _reserved_17_p411: [u8; 0x04],
    _reserved18: [u8; 0x10],
    _reserved_18_p500: [u8; 0x04],
    _reserved19: [u8; 0x03bf],
    #[doc = "0x503 - Write-Protect Register"]
    pub pwpr: PWPR,
    _reserved20: [u8; 0x01],
    #[doc = "0x505 - Write-Protect Register for Secure"]
    pub pwprs: PWPRS,
    _reserved21: [u8; 0x0a],
    #[doc = "0x510..0x51c - Port Security Attribution register"]
    pub psar: [PSAR; 6],
}
impl RegisterBlock {
    #[doc = "0x00..0x14 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs(&self) -> &[P00PFS; 5] {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x02 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x03 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p000pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(3usize).cast() }
    }
    #[doc = "0x06 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p001pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x07 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p001pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(7usize).cast() }
    }
    #[doc = "0x0a - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
    #[doc = "0x0b - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(11usize).cast() }
    }
    #[doc = "0x0e - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p003pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0f - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p003pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(15usize).cast() }
    }
    #[doc = "0x12 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p004pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x13 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p004pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(19usize).cast() }
    }
    #[doc = "0x40..0x68 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs(&self) -> &[P10PFS; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x42 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p100pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(66usize).cast() }
    }
    #[doc = "0x43 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p100pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(67usize).cast() }
    }
    #[doc = "0x46 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p101pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(70usize).cast() }
    }
    #[doc = "0x47 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p101pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(71usize).cast() }
    }
    #[doc = "0x4a - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p102pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(74usize).cast() }
    }
    #[doc = "0x4b - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p102pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(75usize).cast() }
    }
    #[doc = "0x4e - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p103pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(78usize).cast() }
    }
    #[doc = "0x4f - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p103pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(79usize).cast() }
    }
    #[doc = "0x52 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p104pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(82usize).cast() }
    }
    #[doc = "0x53 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p104pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(83usize).cast() }
    }
    #[doc = "0x56 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p105pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(86usize).cast() }
    }
    #[doc = "0x57 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p105pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(87usize).cast() }
    }
    #[doc = "0x5a - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p106pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(90usize).cast() }
    }
    #[doc = "0x5b - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p106pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(91usize).cast() }
    }
    #[doc = "0x5e - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p107pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(94usize).cast() }
    }
    #[doc = "0x5f - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p107pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(95usize).cast() }
    }
    #[doc = "0x62 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(98usize).cast() }
    }
    #[doc = "0x63 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(99usize).cast() }
    }
    #[doc = "0x66 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(&self) -> &P10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(102usize).cast() }
    }
    #[doc = "0x67 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs_by(&self) -> &P10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(103usize).cast() }
    }
    #[doc = "0x68..0x78 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs(&self) -> &[P1PFS; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(104usize).cast() }
    }
    #[doc = "0x68 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p110pfs(&self) -> &P1PFS {
        &self.p1pfs()[0]
    }
    #[doc = "0x6c - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p111pfs(&self) -> &P1PFS {
        &self.p1pfs()[1]
    }
    #[doc = "0x70 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p112pfs(&self) -> &P1PFS {
        &self.p1pfs()[2]
    }
    #[doc = "0x74 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p113pfs(&self) -> &P1PFS {
        &self.p1pfs()[3]
    }
    #[doc = "0x6a - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(&self) -> &P1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x6b - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs_by(&self) -> &P1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(107usize).cast() }
    }
    #[doc = "0x6e - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs_ha(&self) -> &P1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(110usize).cast() }
    }
    #[doc = "0x6f - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs_by(&self) -> &P1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(111usize).cast() }
    }
    #[doc = "0x72 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p112pfs_ha(&self) -> &P1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x73 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p112pfs_by(&self) -> &P1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(115usize).cast() }
    }
    #[doc = "0x76 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p113pfs_ha(&self) -> &P1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(118usize).cast() }
    }
    #[doc = "0x77 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p113pfs_by(&self) -> &P1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(119usize).cast() }
    }
    #[doc = "0x80 - Port 1n200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs(&self) -> &P200PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(128usize).cast() }
    }
    #[doc = "0x82 - Port 1n200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(&self) -> &P200PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(130usize).cast() }
    }
    #[doc = "0x83 - Port 1n200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_by(&self) -> &P200PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(131usize).cast() }
    }
    #[doc = "0x84 - Port 1n201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P201PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x86 - Port 1n201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(&self) -> &P201PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(134usize).cast() }
    }
    #[doc = "0x87 - Port 1n201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_by(&self) -> &P201PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(135usize).cast() }
    }
    #[doc = "0x94 - Port 1n205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P205PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(148usize).cast() }
    }
    #[doc = "0x96 - Port 1n205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P205PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(150usize).cast() }
    }
    #[doc = "0x97 - Port 1n205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P205PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(151usize).cast() }
    }
    #[doc = "0x98 - Port 1n206 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P206PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(152usize).cast() }
    }
    #[doc = "0x9a - Port 1n206 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P206PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(154usize).cast() }
    }
    #[doc = "0x9b - Port 1n206 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P206PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(155usize).cast() }
    }
    #[doc = "0x9c - Port 1n207 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs(&self) -> &P207PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(156usize).cast() }
    }
    #[doc = "0x9e - Port 1n207 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs_ha(&self) -> &P207PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(158usize).cast() }
    }
    #[doc = "0x9f - Port 1n207 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs_by(&self) -> &P207PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(159usize).cast() }
    }
    #[doc = "0xa0 - Port 1n208 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs(&self) -> &P208PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(160usize).cast() }
    }
    #[doc = "0xa2 - Port 1n208 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs_ha(&self) -> &P208PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(162usize).cast() }
    }
    #[doc = "0xa3 - Port 1n208 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs_by(&self) -> &P208PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(163usize).cast() }
    }
    #[doc = "0xb0 - Port 1n212 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(176usize).cast() }
    }
    #[doc = "0xb2 - Port 1n212 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(178usize).cast() }
    }
    #[doc = "0xb3 - Port 1n212 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(179usize).cast() }
    }
    #[doc = "0xb4 - Port 1n213 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P213PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(180usize).cast() }
    }
    #[doc = "0xb6 - Port 1n213 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P213PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(182usize).cast() }
    }
    #[doc = "0xb7 - Port 1n213 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P213PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(183usize).cast() }
    }
    #[doc = "0xc0..0xd4 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p30pfs(&self) -> &[P30PFS; 5] {
        unsafe { &*(self as *const Self).cast::<u8>().add(192usize).cast() }
    }
    #[doc = "0xc2 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(194usize).cast() }
    }
    #[doc = "0xc3 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(195usize).cast() }
    }
    #[doc = "0xc6 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p301pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(198usize).cast() }
    }
    #[doc = "0xc7 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p301pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(199usize).cast() }
    }
    #[doc = "0xca - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p302pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(202usize).cast() }
    }
    #[doc = "0xcb - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p302pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(203usize).cast() }
    }
    #[doc = "0xce - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p303pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(206usize).cast() }
    }
    #[doc = "0xcf - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p303pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(207usize).cast() }
    }
    #[doc = "0xd2 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p304pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(210usize).cast() }
    }
    #[doc = "0xd3 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p304pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(211usize).cast() }
    }
    #[doc = "0x100 - Port 400 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs(&self) -> &P400PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
    #[doc = "0x102 - Port 400 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(258usize).cast() }
    }
    #[doc = "0x103 - Port 400 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(259usize).cast() }
    }
    #[doc = "0x104 - Port 401 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs(&self) -> &P401PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(260usize).cast() }
    }
    #[doc = "0x106 - Port 401 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs_ha(&self) -> &P401PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(262usize).cast() }
    }
    #[doc = "0x107 - Port 401 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs_by(&self) -> &P401PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(263usize).cast() }
    }
    #[doc = "0x108 - Port 402 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs(&self) -> &P402PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    #[doc = "0x10a - Port 402 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs_ha(&self) -> &P402PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(266usize).cast() }
    }
    #[doc = "0x10b - Port 402 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs_by(&self) -> &P402PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(267usize).cast() }
    }
    #[doc = "0x11c..0x128 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs(&self) -> &[P40PFS; 3] {
        unsafe { &*(self as *const Self).cast::<u8>().add(284usize).cast() }
    }
    #[doc = "0x11c - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p407pfs(&self) -> &P40PFS {
        &self.p40pfs()[0]
    }
    #[doc = "0x120 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p408pfs(&self) -> &P40PFS {
        &self.p40pfs()[1]
    }
    #[doc = "0x124 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p409pfs(&self) -> &P40PFS {
        &self.p40pfs()[2]
    }
    #[doc = "0x11e - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p407pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(286usize).cast() }
    }
    #[doc = "0x11f - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p407pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(287usize).cast() }
    }
    #[doc = "0x122 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p408pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(290usize).cast() }
    }
    #[doc = "0x123 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p408pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(291usize).cast() }
    }
    #[doc = "0x126 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p409pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(294usize).cast() }
    }
    #[doc = "0x127 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p409pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(295usize).cast() }
    }
    #[doc = "0x128 - Port 410 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p410pfs(&self) -> &P410PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
    #[doc = "0x12a - Port 410 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p410pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(298usize).cast() }
    }
    #[doc = "0x12b - Port 410 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p410pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(299usize).cast() }
    }
    #[doc = "0x12c - Port 411 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p411pfs(&self) -> &P411PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(300usize).cast() }
    }
    #[doc = "0x12e - Port 411 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p411pfs_ha(&self) -> &P411PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(302usize).cast() }
    }
    #[doc = "0x12f - Port 411 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p411pfs_by(&self) -> &P411PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(303usize).cast() }
    }
    #[doc = "0x140 - Port 500 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs(&self) -> &P500PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(320usize).cast() }
    }
    #[doc = "0x142 - Port 500 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(322usize).cast() }
    }
    #[doc = "0x143 - Port 500 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(323usize).cast() }
    }
}
#[doc = "P00PFS (rw) register accessor: an alias for `Reg<P00PFS_SPEC>`"]
pub type P00PFS = crate::Reg<p00pfs::P00PFS_SPEC>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs;
#[doc = "P00PFS_HA (rw) register accessor: an alias for `Reg<P00PFS_HA_SPEC>`"]
pub type P00PFS_HA = crate::Reg<p00pfs_ha::P00PFS_HA_SPEC>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs_ha;
#[doc = "P00PFS_BY (rw) register accessor: an alias for `Reg<P00PFS_BY_SPEC>`"]
pub type P00PFS_BY = crate::Reg<p00pfs_by::P00PFS_BY_SPEC>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs_by;
#[doc = "P10PFS (rw) register accessor: an alias for `Reg<P10PFS_SPEC>`"]
pub type P10PFS = crate::Reg<p10pfs::P10PFS_SPEC>;
#[doc = "Port 10%s Pin Function Select Register"]
pub mod p10pfs;
#[doc = "P10PFS_HA (rw) register accessor: an alias for `Reg<P10PFS_HA_SPEC>`"]
pub type P10PFS_HA = crate::Reg<p10pfs_ha::P10PFS_HA_SPEC>;
#[doc = "Port 10%s Pin Function Select Register"]
pub mod p10pfs_ha;
#[doc = "P10PFS_BY (rw) register accessor: an alias for `Reg<P10PFS_BY_SPEC>`"]
pub type P10PFS_BY = crate::Reg<p10pfs_by::P10PFS_BY_SPEC>;
#[doc = "Port 10%s Pin Function Select Register"]
pub mod p10pfs_by;
#[doc = "P1PFS (rw) register accessor: an alias for `Reg<P1PFS_SPEC>`"]
pub type P1PFS = crate::Reg<p1pfs::P1PFS_SPEC>;
#[doc = "Port 1%s Pin Function Select Register"]
pub mod p1pfs;
#[doc = "P1PFS_HA (rw) register accessor: an alias for `Reg<P1PFS_HA_SPEC>`"]
pub type P1PFS_HA = crate::Reg<p1pfs_ha::P1PFS_HA_SPEC>;
#[doc = "Port 1%s Pin Function Select Register"]
pub mod p1pfs_ha;
#[doc = "P1PFS_BY (rw) register accessor: an alias for `Reg<P1PFS_BY_SPEC>`"]
pub type P1PFS_BY = crate::Reg<p1pfs_by::P1PFS_BY_SPEC>;
#[doc = "Port 1%s Pin Function Select Register"]
pub mod p1pfs_by;
#[doc = "P200PFS (rw) register accessor: an alias for `Reg<P200PFS_SPEC>`"]
pub type P200PFS = crate::Reg<p200pfs::P200PFS_SPEC>;
#[doc = "Port 1n200 Pin Function Select Register"]
pub mod p200pfs;
#[doc = "P200PFS_HA (rw) register accessor: an alias for `Reg<P200PFS_HA_SPEC>`"]
pub type P200PFS_HA = crate::Reg<p200pfs_ha::P200PFS_HA_SPEC>;
#[doc = "Port 1n200 Pin Function Select Register"]
pub mod p200pfs_ha;
#[doc = "P200PFS_BY (rw) register accessor: an alias for `Reg<P200PFS_BY_SPEC>`"]
pub type P200PFS_BY = crate::Reg<p200pfs_by::P200PFS_BY_SPEC>;
#[doc = "Port 1n200 Pin Function Select Register"]
pub mod p200pfs_by;
#[doc = "P201PFS (rw) register accessor: an alias for `Reg<P201PFS_SPEC>`"]
pub type P201PFS = crate::Reg<p201pfs::P201PFS_SPEC>;
#[doc = "Port 1n201 Pin Function Select Register"]
pub mod p201pfs;
#[doc = "P201PFS_HA (rw) register accessor: an alias for `Reg<P201PFS_HA_SPEC>`"]
pub type P201PFS_HA = crate::Reg<p201pfs_ha::P201PFS_HA_SPEC>;
#[doc = "Port 1n201 Pin Function Select Register"]
pub mod p201pfs_ha;
#[doc = "P201PFS_BY (rw) register accessor: an alias for `Reg<P201PFS_BY_SPEC>`"]
pub type P201PFS_BY = crate::Reg<p201pfs_by::P201PFS_BY_SPEC>;
#[doc = "Port 1n201 Pin Function Select Register"]
pub mod p201pfs_by;
#[doc = "P205PFS (rw) register accessor: an alias for `Reg<P205PFS_SPEC>`"]
pub type P205PFS = crate::Reg<p205pfs::P205PFS_SPEC>;
#[doc = "Port 1n205 Pin Function Select Register"]
pub mod p205pfs;
#[doc = "P205PFS_HA (rw) register accessor: an alias for `Reg<P205PFS_HA_SPEC>`"]
pub type P205PFS_HA = crate::Reg<p205pfs_ha::P205PFS_HA_SPEC>;
#[doc = "Port 1n205 Pin Function Select Register"]
pub mod p205pfs_ha;
#[doc = "P205PFS_BY (rw) register accessor: an alias for `Reg<P205PFS_BY_SPEC>`"]
pub type P205PFS_BY = crate::Reg<p205pfs_by::P205PFS_BY_SPEC>;
#[doc = "Port 1n205 Pin Function Select Register"]
pub mod p205pfs_by;
#[doc = "P206PFS (rw) register accessor: an alias for `Reg<P206PFS_SPEC>`"]
pub type P206PFS = crate::Reg<p206pfs::P206PFS_SPEC>;
#[doc = "Port 1n206 Pin Function Select Register"]
pub mod p206pfs;
#[doc = "P206PFS_HA (rw) register accessor: an alias for `Reg<P206PFS_HA_SPEC>`"]
pub type P206PFS_HA = crate::Reg<p206pfs_ha::P206PFS_HA_SPEC>;
#[doc = "Port 1n206 Pin Function Select Register"]
pub mod p206pfs_ha;
#[doc = "P206PFS_BY (rw) register accessor: an alias for `Reg<P206PFS_BY_SPEC>`"]
pub type P206PFS_BY = crate::Reg<p206pfs_by::P206PFS_BY_SPEC>;
#[doc = "Port 1n206 Pin Function Select Register"]
pub mod p206pfs_by;
#[doc = "P207PFS (rw) register accessor: an alias for `Reg<P207PFS_SPEC>`"]
pub type P207PFS = crate::Reg<p207pfs::P207PFS_SPEC>;
#[doc = "Port 1n207 Pin Function Select Register"]
pub mod p207pfs;
#[doc = "P207PFS_HA (rw) register accessor: an alias for `Reg<P207PFS_HA_SPEC>`"]
pub type P207PFS_HA = crate::Reg<p207pfs_ha::P207PFS_HA_SPEC>;
#[doc = "Port 1n207 Pin Function Select Register"]
pub mod p207pfs_ha;
#[doc = "P207PFS_BY (rw) register accessor: an alias for `Reg<P207PFS_BY_SPEC>`"]
pub type P207PFS_BY = crate::Reg<p207pfs_by::P207PFS_BY_SPEC>;
#[doc = "Port 1n207 Pin Function Select Register"]
pub mod p207pfs_by;
#[doc = "P208PFS (rw) register accessor: an alias for `Reg<P208PFS_SPEC>`"]
pub type P208PFS = crate::Reg<p208pfs::P208PFS_SPEC>;
#[doc = "Port 1n208 Pin Function Select Register"]
pub mod p208pfs;
#[doc = "P208PFS_HA (rw) register accessor: an alias for `Reg<P208PFS_HA_SPEC>`"]
pub type P208PFS_HA = crate::Reg<p208pfs_ha::P208PFS_HA_SPEC>;
#[doc = "Port 1n208 Pin Function Select Register"]
pub mod p208pfs_ha;
#[doc = "P208PFS_BY (rw) register accessor: an alias for `Reg<P208PFS_BY_SPEC>`"]
pub type P208PFS_BY = crate::Reg<p208pfs_by::P208PFS_BY_SPEC>;
#[doc = "Port 1n208 Pin Function Select Register"]
pub mod p208pfs_by;
#[doc = "P212PFS (rw) register accessor: an alias for `Reg<P212PFS_SPEC>`"]
pub type P212PFS = crate::Reg<p212pfs::P212PFS_SPEC>;
#[doc = "Port 1n212 Pin Function Select Register"]
pub mod p212pfs;
#[doc = "P212PFS_HA (rw) register accessor: an alias for `Reg<P212PFS_HA_SPEC>`"]
pub type P212PFS_HA = crate::Reg<p212pfs_ha::P212PFS_HA_SPEC>;
#[doc = "Port 1n212 Pin Function Select Register"]
pub mod p212pfs_ha;
#[doc = "P212PFS_BY (rw) register accessor: an alias for `Reg<P212PFS_BY_SPEC>`"]
pub type P212PFS_BY = crate::Reg<p212pfs_by::P212PFS_BY_SPEC>;
#[doc = "Port 1n212 Pin Function Select Register"]
pub mod p212pfs_by;
#[doc = "P213PFS (rw) register accessor: an alias for `Reg<P213PFS_SPEC>`"]
pub type P213PFS = crate::Reg<p213pfs::P213PFS_SPEC>;
#[doc = "Port 1n213 Pin Function Select Register"]
pub mod p213pfs;
#[doc = "P213PFS_HA (rw) register accessor: an alias for `Reg<P213PFS_HA_SPEC>`"]
pub type P213PFS_HA = crate::Reg<p213pfs_ha::P213PFS_HA_SPEC>;
#[doc = "Port 1n213 Pin Function Select Register"]
pub mod p213pfs_ha;
#[doc = "P213PFS_BY (rw) register accessor: an alias for `Reg<P213PFS_BY_SPEC>`"]
pub type P213PFS_BY = crate::Reg<p213pfs_by::P213PFS_BY_SPEC>;
#[doc = "Port 1n213 Pin Function Select Register"]
pub mod p213pfs_by;
#[doc = "P30PFS (rw) register accessor: an alias for `Reg<P30PFS_SPEC>`"]
pub type P30PFS = crate::Reg<p30pfs::P30PFS_SPEC>;
#[doc = "Port 30%s Pin Function Select Register"]
pub mod p30pfs;
#[doc = "P30PFS_HA (rw) register accessor: an alias for `Reg<P30PFS_HA_SPEC>`"]
pub type P30PFS_HA = crate::Reg<p30pfs_ha::P30PFS_HA_SPEC>;
#[doc = "Port 30%s Pin Function Select Register"]
pub mod p30pfs_ha;
#[doc = "P30PFS_BY (rw) register accessor: an alias for `Reg<P30PFS_BY_SPEC>`"]
pub type P30PFS_BY = crate::Reg<p30pfs_by::P30PFS_BY_SPEC>;
#[doc = "Port 30%s Pin Function Select Register"]
pub mod p30pfs_by;
pub use p40pfs as p400pfs;
pub use p40pfs as p401pfs;
pub use p40pfs as p402pfs;
pub use p40pfs_by as p400pfs_by;
pub use p40pfs_by as p401pfs_by;
pub use p40pfs_by as p402pfs_by;
pub use p40pfs_ha as p400pfs_ha;
pub use p40pfs_ha as p401pfs_ha;
pub use p40pfs_ha as p402pfs_ha;
pub use P40PFS as P400PFS;
pub use P40PFS as P401PFS;
pub use P40PFS as P402PFS;
pub use P40PFS_BY as P400PFS_BY;
pub use P40PFS_BY as P401PFS_BY;
pub use P40PFS_BY as P402PFS_BY;
pub use P40PFS_HA as P400PFS_HA;
pub use P40PFS_HA as P401PFS_HA;
pub use P40PFS_HA as P402PFS_HA;
#[doc = "P40PFS (rw) register accessor: an alias for `Reg<P40PFS_SPEC>`"]
pub type P40PFS = crate::Reg<p40pfs::P40PFS_SPEC>;
#[doc = "Port 40%s Pin Function Select Register"]
pub mod p40pfs;
#[doc = "P40PFS_HA (rw) register accessor: an alias for `Reg<P40PFS_HA_SPEC>`"]
pub type P40PFS_HA = crate::Reg<p40pfs_ha::P40PFS_HA_SPEC>;
#[doc = "Port 40%s Pin Function Select Register"]
pub mod p40pfs_ha;
#[doc = "P40PFS_BY (rw) register accessor: an alias for `Reg<P40PFS_BY_SPEC>`"]
pub type P40PFS_BY = crate::Reg<p40pfs_by::P40PFS_BY_SPEC>;
#[doc = "Port 40%s Pin Function Select Register"]
pub mod p40pfs_by;
#[doc = "P410PFS (rw) register accessor: an alias for `Reg<P410PFS_SPEC>`"]
pub type P410PFS = crate::Reg<p410pfs::P410PFS_SPEC>;
#[doc = "Port 410 Pin Function Select Register"]
pub mod p410pfs;
#[doc = "P410PFS_HA (rw) register accessor: an alias for `Reg<P410PFS_HA_SPEC>`"]
pub type P410PFS_HA = crate::Reg<p410pfs_ha::P410PFS_HA_SPEC>;
#[doc = "Port 410 Pin Function Select Register"]
pub mod p410pfs_ha;
#[doc = "P410PFS_BY (rw) register accessor: an alias for `Reg<P410PFS_BY_SPEC>`"]
pub type P410PFS_BY = crate::Reg<p410pfs_by::P410PFS_BY_SPEC>;
#[doc = "Port 410 Pin Function Select Register"]
pub mod p410pfs_by;
#[doc = "P411PFS (rw) register accessor: an alias for `Reg<P411PFS_SPEC>`"]
pub type P411PFS = crate::Reg<p411pfs::P411PFS_SPEC>;
#[doc = "Port 411 Pin Function Select Register"]
pub mod p411pfs;
#[doc = "P411PFS_HA (rw) register accessor: an alias for `Reg<P411PFS_HA_SPEC>`"]
pub type P411PFS_HA = crate::Reg<p411pfs_ha::P411PFS_HA_SPEC>;
#[doc = "Port 411 Pin Function Select Register"]
pub mod p411pfs_ha;
#[doc = "P411PFS_BY (rw) register accessor: an alias for `Reg<P411PFS_BY_SPEC>`"]
pub type P411PFS_BY = crate::Reg<p411pfs_by::P411PFS_BY_SPEC>;
#[doc = "Port 411 Pin Function Select Register"]
pub mod p411pfs_by;
#[doc = "P500PFS (rw) register accessor: an alias for `Reg<P500PFS_SPEC>`"]
pub type P500PFS = crate::Reg<p500pfs::P500PFS_SPEC>;
#[doc = "Port 500 Pin Function Select Register"]
pub mod p500pfs;
#[doc = "P500PFS_HA (rw) register accessor: an alias for `Reg<P500PFS_HA_SPEC>`"]
pub type P500PFS_HA = crate::Reg<p500pfs_ha::P500PFS_HA_SPEC>;
#[doc = "Port 500 Pin Function Select Register"]
pub mod p500pfs_ha;
#[doc = "P500PFS_BY (rw) register accessor: an alias for `Reg<P500PFS_BY_SPEC>`"]
pub type P500PFS_BY = crate::Reg<p500pfs_by::P500PFS_BY_SPEC>;
#[doc = "Port 500 Pin Function Select Register"]
pub mod p500pfs_by;
#[doc = "PWPR (rw) register accessor: an alias for `Reg<PWPR_SPEC>`"]
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
#[doc = "Write-Protect Register"]
pub mod pwpr;
#[doc = "PWPRS (rw) register accessor: an alias for `Reg<PWPRS_SPEC>`"]
pub type PWPRS = crate::Reg<pwprs::PWPRS_SPEC>;
#[doc = "Write-Protect Register for Secure"]
pub mod pwprs;
#[doc = "PSAR (rw) register accessor: an alias for `Reg<PSAR_SPEC>`"]
pub type PSAR = crate::Reg<psar::PSAR_SPEC>;
#[doc = "Port Security Attribution register"]
pub mod psar;
