#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_p0: [u8; 0x20],
    _reserved_1_p008: [u8; 0x04],
    _reserved_2_p009: [u8; 0x04],
    _reserved3: [u8; 0x10],
    _reserved_3_p: [u8; 0x08],
    _reserved_4_p1: [u8; 0x28],
    _reserved_5_p: [u8; 0x18],
    _reserved_6_p200: [u8; 0x04],
    _reserved_7_p201: [u8; 0x04],
    _reserved_8_p2: [u8; 0x20],
    _reserved_9_p210: [u8; 0x04],
    _reserved_10_p211: [u8; 0x04],
    _reserved_11_p212: [u8; 0x04],
    _reserved_12_p213: [u8; 0x04],
    _reserved_13_p214: [u8; 0x04],
    _reserved14: [u8; 0x04],
    _reserved_14_p300: [u8; 0x04],
    _reserved_15_p3: [u8; 0x24],
    _reserved_16_p: [u8; 0x10],
    _reserved17: [u8; 0x08],
    _reserved_17_p4: [u8; 0x28],
    _reserved_18_p: [u8; 0x18],
    _reserved_19_p5: [u8; 0x20],
    _reserved20: [u8; 0x0c],
    _reserved_20_p: [u8; 0x08],
    _reserved21: [u8; 0x0c],
    _reserved_21_p6: [u8; 0x18],
    _reserved22: [u8; 0x08],
    _reserved_22_p608: [u8; 0x04],
    _reserved_23_p609: [u8; 0x04],
    _reserved_24_p: [u8; 0x14],
    _reserved25: [u8; 0x04],
    _reserved_25_p7: [u8; 0x18],
    _reserved26: [u8; 0x08],
    _reserved_26_p708: [u8; 0x04],
    _reserved_27_p709: [u8; 0x04],
    _reserved_28_p: [u8; 0x10],
    _reserved29: [u8; 0x08],
    _reserved_29_p8: [u8; 0x08],
    _reserved30: [u8; 0x02fb],
    #[doc = "0x503 - Write-Protect Register"]
    pub pwpr: PWPR,
    _reserved31: [u8; 0x01],
    #[doc = "0x505 - Write-Protect Register for Secure"]
    pub pwprs: PWPRS,
    _reserved32: [u8; 0x0a],
    #[doc = "0x510..0x522 - Port Security Attribution register"]
    pub psar: [PSAR; 9],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs(&self) -> &[P00PFS; 8] {
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
    #[doc = "0x16 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p005pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x17 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p005pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(23usize).cast() }
    }
    #[doc = "0x1a - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p006pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1b - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p006pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(27usize).cast() }
    }
    #[doc = "0x1e - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p007pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x1f - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p007pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(31usize).cast() }
    }
    #[doc = "0x20 - Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P008PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x22 - Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P008PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(34usize).cast() }
    }
    #[doc = "0x23 - Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P008PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(35usize).cast() }
    }
    #[doc = "0x24 - Port 009 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p009pfs(&self) -> &P009PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x26 - Port 009 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p009pfs_ha(&self) -> &P009PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(38usize).cast() }
    }
    #[doc = "0x27 - Port 009 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p009pfs_by(&self) -> &P009PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(39usize).cast() }
    }
    #[doc = "0x38..0x40 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs(&self) -> &[P0PFS; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x38 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p014pfs(&self) -> &P0PFS {
        &self.p0pfs()[0]
    }
    #[doc = "0x3c - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p015pfs(&self) -> &P0PFS {
        &self.p0pfs()[1]
    }
    #[doc = "0x3a - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3b - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(59usize).cast() }
    }
    #[doc = "0x3e - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
    #[doc = "0x3f - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(63usize).cast() }
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
    #[doc = "0x68..0x80 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs(&self) -> &[P1PFS; 6] {
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
    #[doc = "0x78 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p114pfs(&self) -> &P1PFS {
        &self.p1pfs()[4]
    }
    #[doc = "0x7c - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p115pfs(&self) -> &P1PFS {
        &self.p1pfs()[5]
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
    #[doc = "0x7a - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p114pfs_ha(&self) -> &P1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(122usize).cast() }
    }
    #[doc = "0x7b - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p114pfs_by(&self) -> &P1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(123usize).cast() }
    }
    #[doc = "0x7e - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p115pfs_ha(&self) -> &P1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(126usize).cast() }
    }
    #[doc = "0x7f - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p115pfs_by(&self) -> &P1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(127usize).cast() }
    }
    #[doc = "0x80 - Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs(&self) -> &P200PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(128usize).cast() }
    }
    #[doc = "0x82 - Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(&self) -> &P200PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(130usize).cast() }
    }
    #[doc = "0x83 - Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_by(&self) -> &P200PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(131usize).cast() }
    }
    #[doc = "0x84 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P201PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x86 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(&self) -> &P201PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(134usize).cast() }
    }
    #[doc = "0x87 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_by(&self) -> &P201PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(135usize).cast() }
    }
    #[doc = "0x88..0xa8 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs(&self) -> &[P20PFS; 8] {
        unsafe { &*(self as *const Self).cast::<u8>().add(136usize).cast() }
    }
    #[doc = "0x88 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p202pfs(&self) -> &P20PFS {
        &self.p20pfs()[0]
    }
    #[doc = "0x8c - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p203pfs(&self) -> &P20PFS {
        &self.p20pfs()[1]
    }
    #[doc = "0x90 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p204pfs(&self) -> &P20PFS {
        &self.p20pfs()[2]
    }
    #[doc = "0x94 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p205pfs(&self) -> &P20PFS {
        &self.p20pfs()[3]
    }
    #[doc = "0x98 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p206pfs(&self) -> &P20PFS {
        &self.p20pfs()[4]
    }
    #[doc = "0x9c - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p207pfs(&self) -> &P20PFS {
        &self.p20pfs()[5]
    }
    #[doc = "0xa0 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p208pfs(&self) -> &P20PFS {
        &self.p20pfs()[6]
    }
    #[doc = "0xa4 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p209pfs(&self) -> &P20PFS {
        &self.p20pfs()[7]
    }
    #[doc = "0x8a - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p202pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(138usize).cast() }
    }
    #[doc = "0x8b - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p202pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(139usize).cast() }
    }
    #[doc = "0x8e - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p203pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(142usize).cast() }
    }
    #[doc = "0x8f - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p203pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(143usize).cast() }
    }
    #[doc = "0x92 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p204pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(146usize).cast() }
    }
    #[doc = "0x93 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p204pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(147usize).cast() }
    }
    #[doc = "0x96 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(150usize).cast() }
    }
    #[doc = "0x97 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(151usize).cast() }
    }
    #[doc = "0x9a - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(154usize).cast() }
    }
    #[doc = "0x9b - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(155usize).cast() }
    }
    #[doc = "0x9e - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(158usize).cast() }
    }
    #[doc = "0x9f - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(159usize).cast() }
    }
    #[doc = "0xa2 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(162usize).cast() }
    }
    #[doc = "0xa3 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(163usize).cast() }
    }
    #[doc = "0xa6 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p209pfs_ha(&self) -> &P20PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(166usize).cast() }
    }
    #[doc = "0xa7 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p209pfs_by(&self) -> &P20PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(167usize).cast() }
    }
    #[doc = "0xa8 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p210pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(168usize).cast() }
    }
    #[doc = "0xaa - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p210pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(170usize).cast() }
    }
    #[doc = "0xab - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p210pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(171usize).cast() }
    }
    #[doc = "0xac - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p211pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(172usize).cast() }
    }
    #[doc = "0xae - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p211pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(174usize).cast() }
    }
    #[doc = "0xaf - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p211pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(175usize).cast() }
    }
    #[doc = "0xb0 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(176usize).cast() }
    }
    #[doc = "0xb2 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(178usize).cast() }
    }
    #[doc = "0xb3 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(179usize).cast() }
    }
    #[doc = "0xb4 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(180usize).cast() }
    }
    #[doc = "0xb6 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(182usize).cast() }
    }
    #[doc = "0xb7 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(183usize).cast() }
    }
    #[doc = "0xb8 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P210PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(184usize).cast() }
    }
    #[doc = "0xba - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p214pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(186usize).cast() }
    }
    #[doc = "0xbb - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p214pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(187usize).cast() }
    }
    #[doc = "0xc0 - Port 300 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs(&self) -> &P300PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(192usize).cast() }
    }
    #[doc = "0xc2 - Port 300 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs_ha(&self) -> &P300PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(194usize).cast() }
    }
    #[doc = "0xc3 - Port 300 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs_by(&self) -> &P300PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(195usize).cast() }
    }
    #[doc = "0xc4..0xe8 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p30pfs(&self) -> &[P30PFS; 9] {
        unsafe { &*(self as *const Self).cast::<u8>().add(196usize).cast() }
    }
    #[doc = "0xc4 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p301pfs(&self) -> &P30PFS {
        &self.p30pfs()[0]
    }
    #[doc = "0xc8 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p302pfs(&self) -> &P30PFS {
        &self.p30pfs()[1]
    }
    #[doc = "0xcc - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p303pfs(&self) -> &P30PFS {
        &self.p30pfs()[2]
    }
    #[doc = "0xd0 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p304pfs(&self) -> &P30PFS {
        &self.p30pfs()[3]
    }
    #[doc = "0xd4 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p305pfs(&self) -> &P30PFS {
        &self.p30pfs()[4]
    }
    #[doc = "0xd8 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p306pfs(&self) -> &P30PFS {
        &self.p30pfs()[5]
    }
    #[doc = "0xdc - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p307pfs(&self) -> &P30PFS {
        &self.p30pfs()[6]
    }
    #[doc = "0xe0 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p308pfs(&self) -> &P30PFS {
        &self.p30pfs()[7]
    }
    #[doc = "0xe4 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p309pfs(&self) -> &P30PFS {
        &self.p30pfs()[8]
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
    #[doc = "0xd6 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p305pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(214usize).cast() }
    }
    #[doc = "0xd7 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p305pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(215usize).cast() }
    }
    #[doc = "0xda - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p306pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(218usize).cast() }
    }
    #[doc = "0xdb - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p306pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(219usize).cast() }
    }
    #[doc = "0xde - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p307pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(222usize).cast() }
    }
    #[doc = "0xdf - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p307pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(223usize).cast() }
    }
    #[doc = "0xe2 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p308pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(226usize).cast() }
    }
    #[doc = "0xe3 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p308pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(227usize).cast() }
    }
    #[doc = "0xe6 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p309pfs_ha(&self) -> &P30PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(230usize).cast() }
    }
    #[doc = "0xe7 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p309pfs_by(&self) -> &P30PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(231usize).cast() }
    }
    #[doc = "0xe8..0xf8 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p3pfs(&self) -> &[P3PFS; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(232usize).cast() }
    }
    #[doc = "0xe8 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p310pfs(&self) -> &P3PFS {
        &self.p3pfs()[0]
    }
    #[doc = "0xec - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p311pfs(&self) -> &P3PFS {
        &self.p3pfs()[1]
    }
    #[doc = "0xf0 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p312pfs(&self) -> &P3PFS {
        &self.p3pfs()[2]
    }
    #[doc = "0xf4 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p313pfs(&self) -> &P3PFS {
        &self.p3pfs()[3]
    }
    #[doc = "0xea - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p310pfs_ha(&self) -> &P3PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(234usize).cast() }
    }
    #[doc = "0xeb - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p310pfs_by(&self) -> &P3PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(235usize).cast() }
    }
    #[doc = "0xee - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p311pfs_ha(&self) -> &P3PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(238usize).cast() }
    }
    #[doc = "0xef - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p311pfs_by(&self) -> &P3PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(239usize).cast() }
    }
    #[doc = "0xf2 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p312pfs_ha(&self) -> &P3PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(242usize).cast() }
    }
    #[doc = "0xf3 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p312pfs_by(&self) -> &P3PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(243usize).cast() }
    }
    #[doc = "0xf6 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p313pfs_ha(&self) -> &P3PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(246usize).cast() }
    }
    #[doc = "0xf7 - Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p313pfs_by(&self) -> &P3PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(247usize).cast() }
    }
    #[doc = "0x100..0x128 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs(&self) -> &[P40PFS; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
    #[doc = "0x102 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(258usize).cast() }
    }
    #[doc = "0x103 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(259usize).cast() }
    }
    #[doc = "0x106 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(262usize).cast() }
    }
    #[doc = "0x107 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(263usize).cast() }
    }
    #[doc = "0x10a - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(266usize).cast() }
    }
    #[doc = "0x10b - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(267usize).cast() }
    }
    #[doc = "0x10e - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p403pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(270usize).cast() }
    }
    #[doc = "0x10f - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p403pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(271usize).cast() }
    }
    #[doc = "0x112 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p404pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(274usize).cast() }
    }
    #[doc = "0x113 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p404pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(275usize).cast() }
    }
    #[doc = "0x116 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p405pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(278usize).cast() }
    }
    #[doc = "0x117 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p405pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(279usize).cast() }
    }
    #[doc = "0x11a - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p406pfs_ha(&self) -> &P40PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(282usize).cast() }
    }
    #[doc = "0x11b - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p406pfs_by(&self) -> &P40PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(283usize).cast() }
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
    #[doc = "0x128..0x140 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p4pfs(&self) -> &[P4PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
    #[doc = "0x128 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p410pfs(&self) -> &P4PFS {
        &self.p4pfs()[0]
    }
    #[doc = "0x12c - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p411pfs(&self) -> &P4PFS {
        &self.p4pfs()[1]
    }
    #[doc = "0x130 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p412pfs(&self) -> &P4PFS {
        &self.p4pfs()[2]
    }
    #[doc = "0x134 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p413pfs(&self) -> &P4PFS {
        &self.p4pfs()[3]
    }
    #[doc = "0x138 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p414pfs(&self) -> &P4PFS {
        &self.p4pfs()[4]
    }
    #[doc = "0x13c - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p415pfs(&self) -> &P4PFS {
        &self.p4pfs()[5]
    }
    #[doc = "0x12a - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p410pfs_ha(&self) -> &P4PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(298usize).cast() }
    }
    #[doc = "0x12b - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p410pfs_by(&self) -> &P4PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(299usize).cast() }
    }
    #[doc = "0x12e - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p411pfs_ha(&self) -> &P4PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(302usize).cast() }
    }
    #[doc = "0x12f - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p411pfs_by(&self) -> &P4PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(303usize).cast() }
    }
    #[doc = "0x132 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p412pfs_ha(&self) -> &P4PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(306usize).cast() }
    }
    #[doc = "0x133 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p412pfs_by(&self) -> &P4PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(307usize).cast() }
    }
    #[doc = "0x136 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p413pfs_ha(&self) -> &P4PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(310usize).cast() }
    }
    #[doc = "0x137 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p413pfs_by(&self) -> &P4PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(311usize).cast() }
    }
    #[doc = "0x13a - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p414pfs_ha(&self) -> &P4PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(314usize).cast() }
    }
    #[doc = "0x13b - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p414pfs_by(&self) -> &P4PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(315usize).cast() }
    }
    #[doc = "0x13e - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p415pfs_ha(&self) -> &P4PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(318usize).cast() }
    }
    #[doc = "0x13f - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p415pfs_by(&self) -> &P4PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(319usize).cast() }
    }
    #[doc = "0x140..0x160 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p50pfs(&self) -> &[P50PFS; 8] {
        unsafe { &*(self as *const Self).cast::<u8>().add(320usize).cast() }
    }
    #[doc = "0x142 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(322usize).cast() }
    }
    #[doc = "0x143 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(323usize).cast() }
    }
    #[doc = "0x146 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p501pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(326usize).cast() }
    }
    #[doc = "0x147 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p501pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(327usize).cast() }
    }
    #[doc = "0x14a - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p502pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(330usize).cast() }
    }
    #[doc = "0x14b - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p502pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(331usize).cast() }
    }
    #[doc = "0x14e - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p503pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(334usize).cast() }
    }
    #[doc = "0x14f - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p503pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(335usize).cast() }
    }
    #[doc = "0x152 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p504pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(338usize).cast() }
    }
    #[doc = "0x153 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p504pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(339usize).cast() }
    }
    #[doc = "0x156 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p505pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(342usize).cast() }
    }
    #[doc = "0x157 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p505pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(343usize).cast() }
    }
    #[doc = "0x15a - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p506pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(346usize).cast() }
    }
    #[doc = "0x15b - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p506pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(347usize).cast() }
    }
    #[doc = "0x15e - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p507pfs_ha(&self) -> &P50PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(350usize).cast() }
    }
    #[doc = "0x15f - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p507pfs_by(&self) -> &P50PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(351usize).cast() }
    }
    #[doc = "0x16c..0x174 - Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p5pfs(&self) -> &[P5PFS; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(364usize).cast() }
    }
    #[doc = "0x16c - Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p511pfs(&self) -> &P5PFS {
        &self.p5pfs()[0]
    }
    #[doc = "0x170 - Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p512pfs(&self) -> &P5PFS {
        &self.p5pfs()[1]
    }
    #[doc = "0x16e - Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p511pfs_ha(&self) -> &P5PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(366usize).cast() }
    }
    #[doc = "0x16f - Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p511pfs_by(&self) -> &P5PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(367usize).cast() }
    }
    #[doc = "0x172 - Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p512pfs_ha(&self) -> &P5PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(370usize).cast() }
    }
    #[doc = "0x173 - Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p512pfs_by(&self) -> &P5PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(371usize).cast() }
    }
    #[doc = "0x180..0x198 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p60pfs(&self) -> &[P60PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(384usize).cast() }
    }
    #[doc = "0x182 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p600pfs_ha(&self) -> &P60PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(386usize).cast() }
    }
    #[doc = "0x183 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p600pfs_by(&self) -> &P60PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(387usize).cast() }
    }
    #[doc = "0x186 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p601pfs_ha(&self) -> &P60PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(390usize).cast() }
    }
    #[doc = "0x187 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p601pfs_by(&self) -> &P60PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(391usize).cast() }
    }
    #[doc = "0x18a - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p602pfs_ha(&self) -> &P60PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(394usize).cast() }
    }
    #[doc = "0x18b - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p602pfs_by(&self) -> &P60PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(395usize).cast() }
    }
    #[doc = "0x18e - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p603pfs_ha(&self) -> &P60PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(398usize).cast() }
    }
    #[doc = "0x18f - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p603pfs_by(&self) -> &P60PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(399usize).cast() }
    }
    #[doc = "0x192 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p604pfs_ha(&self) -> &P60PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(402usize).cast() }
    }
    #[doc = "0x193 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p604pfs_by(&self) -> &P60PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(403usize).cast() }
    }
    #[doc = "0x196 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p605pfs_ha(&self) -> &P60PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(406usize).cast() }
    }
    #[doc = "0x197 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p605pfs_by(&self) -> &P60PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(407usize).cast() }
    }
    #[doc = "0x1a0 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p608pfs(&self) -> &P608PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(416usize).cast() }
    }
    #[doc = "0x1a2 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p608pfs_ha(&self) -> &P608PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(418usize).cast() }
    }
    #[doc = "0x1a3 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p608pfs_by(&self) -> &P608PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(419usize).cast() }
    }
    #[doc = "0x1a4 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p609pfs(&self) -> &P608PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(420usize).cast() }
    }
    #[doc = "0x1a6 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p609pfs_ha(&self) -> &P608PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(422usize).cast() }
    }
    #[doc = "0x1a7 - Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p609pfs_by(&self) -> &P608PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(423usize).cast() }
    }
    #[doc = "0x1a8..0x1bc - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p6pfs(&self) -> &[P6PFS; 5] {
        unsafe { &*(self as *const Self).cast::<u8>().add(424usize).cast() }
    }
    #[doc = "0x1a8 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p610pfs(&self) -> &P6PFS {
        &self.p6pfs()[0]
    }
    #[doc = "0x1ac - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p611pfs(&self) -> &P6PFS {
        &self.p6pfs()[1]
    }
    #[doc = "0x1b0 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p612pfs(&self) -> &P6PFS {
        &self.p6pfs()[2]
    }
    #[doc = "0x1b4 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p613pfs(&self) -> &P6PFS {
        &self.p6pfs()[3]
    }
    #[doc = "0x1b8 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p614pfs(&self) -> &P6PFS {
        &self.p6pfs()[4]
    }
    #[doc = "0x1aa - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p610pfs_ha(&self) -> &P6PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(426usize).cast() }
    }
    #[doc = "0x1ab - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p610pfs_by(&self) -> &P6PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(427usize).cast() }
    }
    #[doc = "0x1ae - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p611pfs_ha(&self) -> &P6PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(430usize).cast() }
    }
    #[doc = "0x1af - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p611pfs_by(&self) -> &P6PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(431usize).cast() }
    }
    #[doc = "0x1b2 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p612pfs_ha(&self) -> &P6PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(434usize).cast() }
    }
    #[doc = "0x1b3 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p612pfs_by(&self) -> &P6PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(435usize).cast() }
    }
    #[doc = "0x1b6 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p613pfs_ha(&self) -> &P6PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(438usize).cast() }
    }
    #[doc = "0x1b7 - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p613pfs_by(&self) -> &P6PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(439usize).cast() }
    }
    #[doc = "0x1ba - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p614pfs_ha(&self) -> &P6PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(442usize).cast() }
    }
    #[doc = "0x1bb - Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p614pfs_by(&self) -> &P6PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(443usize).cast() }
    }
    #[doc = "0x1c0..0x1d8 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p70pfs(&self) -> &[P70PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1c2 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p700pfs_ha(&self) -> &P70PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(450usize).cast() }
    }
    #[doc = "0x1c3 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p700pfs_by(&self) -> &P70PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(451usize).cast() }
    }
    #[doc = "0x1c6 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p701pfs_ha(&self) -> &P70PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(454usize).cast() }
    }
    #[doc = "0x1c7 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p701pfs_by(&self) -> &P70PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(455usize).cast() }
    }
    #[doc = "0x1ca - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p702pfs_ha(&self) -> &P70PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(458usize).cast() }
    }
    #[doc = "0x1cb - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p702pfs_by(&self) -> &P70PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(459usize).cast() }
    }
    #[doc = "0x1ce - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p703pfs_ha(&self) -> &P70PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(462usize).cast() }
    }
    #[doc = "0x1cf - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p703pfs_by(&self) -> &P70PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(463usize).cast() }
    }
    #[doc = "0x1d2 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p704pfs_ha(&self) -> &P70PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(466usize).cast() }
    }
    #[doc = "0x1d3 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p704pfs_by(&self) -> &P70PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(467usize).cast() }
    }
    #[doc = "0x1d6 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p705pfs_ha(&self) -> &P70PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(470usize).cast() }
    }
    #[doc = "0x1d7 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p705pfs_by(&self) -> &P70PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(471usize).cast() }
    }
    #[doc = "0x1e0 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p708pfs(&self) -> &P708PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(480usize).cast() }
    }
    #[doc = "0x1e2 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p708pfs_ha(&self) -> &P708PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(482usize).cast() }
    }
    #[doc = "0x1e3 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p708pfs_by(&self) -> &P708PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(483usize).cast() }
    }
    #[doc = "0x1e4 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p709pfs(&self) -> &P708PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(484usize).cast() }
    }
    #[doc = "0x1e6 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p709pfs_ha(&self) -> &P708PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(486usize).cast() }
    }
    #[doc = "0x1e7 - Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p709pfs_by(&self) -> &P708PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(487usize).cast() }
    }
    #[doc = "0x1e8..0x1f8 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p7pfs(&self) -> &[P7PFS; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(488usize).cast() }
    }
    #[doc = "0x1e8 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p710pfs(&self) -> &P7PFS {
        &self.p7pfs()[0]
    }
    #[doc = "0x1ec - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p711pfs(&self) -> &P7PFS {
        &self.p7pfs()[1]
    }
    #[doc = "0x1f0 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p712pfs(&self) -> &P7PFS {
        &self.p7pfs()[2]
    }
    #[doc = "0x1f4 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p713pfs(&self) -> &P7PFS {
        &self.p7pfs()[3]
    }
    #[doc = "0x1ea - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p710pfs_ha(&self) -> &P7PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(490usize).cast() }
    }
    #[doc = "0x1eb - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p710pfs_by(&self) -> &P7PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(491usize).cast() }
    }
    #[doc = "0x1ee - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p711pfs_ha(&self) -> &P7PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(494usize).cast() }
    }
    #[doc = "0x1ef - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p711pfs_by(&self) -> &P7PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(495usize).cast() }
    }
    #[doc = "0x1f2 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p712pfs_ha(&self) -> &P7PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(498usize).cast() }
    }
    #[doc = "0x1f3 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p712pfs_by(&self) -> &P7PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(499usize).cast() }
    }
    #[doc = "0x1f6 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p713pfs_ha(&self) -> &P7PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(502usize).cast() }
    }
    #[doc = "0x1f7 - Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p713pfs_by(&self) -> &P7PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(503usize).cast() }
    }
    #[doc = "0x200..0x208 - Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p80pfs(&self) -> &[P80PFS; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(512usize).cast() }
    }
    #[doc = "0x202 - Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p800pfs_ha(&self) -> &P80PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(514usize).cast() }
    }
    #[doc = "0x203 - Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p800pfs_by(&self) -> &P80PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(515usize).cast() }
    }
    #[doc = "0x206 - Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p801pfs_ha(&self) -> &P80PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(518usize).cast() }
    }
    #[doc = "0x207 - Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p801pfs_by(&self) -> &P80PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(519usize).cast() }
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
#[doc = "P008PFS (rw) register accessor: an alias for `Reg<P008PFS_SPEC>`"]
pub type P008PFS = crate::Reg<p008pfs::P008PFS_SPEC>;
#[doc = "Port 008 Pin Function Select Register"]
pub mod p008pfs;
#[doc = "P008PFS_HA (rw) register accessor: an alias for `Reg<P008PFS_HA_SPEC>`"]
pub type P008PFS_HA = crate::Reg<p008pfs_ha::P008PFS_HA_SPEC>;
#[doc = "Port 008 Pin Function Select Register"]
pub mod p008pfs_ha;
#[doc = "P008PFS_BY (rw) register accessor: an alias for `Reg<P008PFS_BY_SPEC>`"]
pub type P008PFS_BY = crate::Reg<p008pfs_by::P008PFS_BY_SPEC>;
#[doc = "Port 008 Pin Function Select Register"]
pub mod p008pfs_by;
#[doc = "P009PFS (rw) register accessor: an alias for `Reg<P009PFS_SPEC>`"]
pub type P009PFS = crate::Reg<p009pfs::P009PFS_SPEC>;
#[doc = "Port 009 Pin Function Select Register"]
pub mod p009pfs;
#[doc = "P009PFS_HA (rw) register accessor: an alias for `Reg<P009PFS_HA_SPEC>`"]
pub type P009PFS_HA = crate::Reg<p009pfs_ha::P009PFS_HA_SPEC>;
#[doc = "Port 009 Pin Function Select Register"]
pub mod p009pfs_ha;
pub use p00pfs_by as p009pfs_by;
pub use P00PFS_BY as P009PFS_BY;
#[doc = "P0PFS (rw) register accessor: an alias for `Reg<P0PFS_SPEC>`"]
pub type P0PFS = crate::Reg<p0pfs::P0PFS_SPEC>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs;
#[doc = "P0PFS_HA (rw) register accessor: an alias for `Reg<P0PFS_HA_SPEC>`"]
pub type P0PFS_HA = crate::Reg<p0pfs_ha::P0PFS_HA_SPEC>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs_ha;
pub use p00pfs_by as p014pfs_by;
pub use P00PFS_BY as P014PFS_BY;
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
pub use p20pfs as p200pfs;
pub use p20pfs_by as p200pfs_by;
pub use p20pfs_ha as p200pfs_ha;
pub use P20PFS as P200PFS;
pub use P20PFS_BY as P200PFS_BY;
pub use P20PFS_HA as P200PFS_HA;
#[doc = "P201PFS (rw) register accessor: an alias for `Reg<P201PFS_SPEC>`"]
pub type P201PFS = crate::Reg<p201pfs::P201PFS_SPEC>;
#[doc = "Port 201 Pin Function Select Register"]
pub mod p201pfs;
#[doc = "P201PFS_HA (rw) register accessor: an alias for `Reg<P201PFS_HA_SPEC>`"]
pub type P201PFS_HA = crate::Reg<p201pfs_ha::P201PFS_HA_SPEC>;
#[doc = "Port 201 Pin Function Select Register"]
pub mod p201pfs_ha;
#[doc = "P201PFS_BY (rw) register accessor: an alias for `Reg<P201PFS_BY_SPEC>`"]
pub type P201PFS_BY = crate::Reg<p201pfs_by::P201PFS_BY_SPEC>;
#[doc = "Port 201 Pin Function Select Register"]
pub mod p201pfs_by;
#[doc = "P20PFS (rw) register accessor: an alias for `Reg<P20PFS_SPEC>`"]
pub type P20PFS = crate::Reg<p20pfs::P20PFS_SPEC>;
#[doc = "Port 20%s Pin Function Select Register"]
pub mod p20pfs;
#[doc = "P20PFS_HA (rw) register accessor: an alias for `Reg<P20PFS_HA_SPEC>`"]
pub type P20PFS_HA = crate::Reg<p20pfs_ha::P20PFS_HA_SPEC>;
#[doc = "Port 20%s Pin Function Select Register"]
pub mod p20pfs_ha;
#[doc = "P20PFS_BY (rw) register accessor: an alias for `Reg<P20PFS_BY_SPEC>`"]
pub type P20PFS_BY = crate::Reg<p20pfs_by::P20PFS_BY_SPEC>;
#[doc = "Port 20%s Pin Function Select Register"]
pub mod p20pfs_by;
pub use p20pfs as p210pfs;
pub use p20pfs_by as p210pfs_by;
pub use p20pfs_ha as p210pfs_ha;
pub use P20PFS as P210PFS;
pub use P20PFS_BY as P210PFS_BY;
pub use P20PFS_HA as P210PFS_HA;
#[doc = "P300PFS (rw) register accessor: an alias for `Reg<P300PFS_SPEC>`"]
pub type P300PFS = crate::Reg<p300pfs::P300PFS_SPEC>;
#[doc = "Port 300 Pin Function Select Register"]
pub mod p300pfs;
#[doc = "P300PFS_HA (rw) register accessor: an alias for `Reg<P300PFS_HA_SPEC>`"]
pub type P300PFS_HA = crate::Reg<p300pfs_ha::P300PFS_HA_SPEC>;
#[doc = "Port 300 Pin Function Select Register"]
pub mod p300pfs_ha;
#[doc = "P300PFS_BY (rw) register accessor: an alias for `Reg<P300PFS_BY_SPEC>`"]
pub type P300PFS_BY = crate::Reg<p300pfs_by::P300PFS_BY_SPEC>;
#[doc = "Port 300 Pin Function Select Register"]
pub mod p300pfs_by;
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
#[doc = "P3PFS (rw) register accessor: an alias for `Reg<P3PFS_SPEC>`"]
pub type P3PFS = crate::Reg<p3pfs::P3PFS_SPEC>;
#[doc = "Port 3%s Pin Function Select Register"]
pub mod p3pfs;
#[doc = "P3PFS_HA (rw) register accessor: an alias for `Reg<P3PFS_HA_SPEC>`"]
pub type P3PFS_HA = crate::Reg<p3pfs_ha::P3PFS_HA_SPEC>;
#[doc = "Port 3%s Pin Function Select Register"]
pub mod p3pfs_ha;
#[doc = "P3PFS_BY (rw) register accessor: an alias for `Reg<P3PFS_BY_SPEC>`"]
pub type P3PFS_BY = crate::Reg<p3pfs_by::P3PFS_BY_SPEC>;
#[doc = "Port 3%s Pin Function Select Register"]
pub mod p3pfs_by;
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
#[doc = "P4PFS (rw) register accessor: an alias for `Reg<P4PFS_SPEC>`"]
pub type P4PFS = crate::Reg<p4pfs::P4PFS_SPEC>;
#[doc = "Port 4%s Pin Function Select Register"]
pub mod p4pfs;
#[doc = "P4PFS_HA (rw) register accessor: an alias for `Reg<P4PFS_HA_SPEC>`"]
pub type P4PFS_HA = crate::Reg<p4pfs_ha::P4PFS_HA_SPEC>;
#[doc = "Port 4%s Pin Function Select Register"]
pub mod p4pfs_ha;
#[doc = "P4PFS_BY (rw) register accessor: an alias for `Reg<P4PFS_BY_SPEC>`"]
pub type P4PFS_BY = crate::Reg<p4pfs_by::P4PFS_BY_SPEC>;
#[doc = "Port 4%s Pin Function Select Register"]
pub mod p4pfs_by;
#[doc = "P50PFS (rw) register accessor: an alias for `Reg<P50PFS_SPEC>`"]
pub type P50PFS = crate::Reg<p50pfs::P50PFS_SPEC>;
#[doc = "Port 50%s Pin Function Select Register"]
pub mod p50pfs;
#[doc = "P50PFS_HA (rw) register accessor: an alias for `Reg<P50PFS_HA_SPEC>`"]
pub type P50PFS_HA = crate::Reg<p50pfs_ha::P50PFS_HA_SPEC>;
#[doc = "Port 50%s Pin Function Select Register"]
pub mod p50pfs_ha;
#[doc = "P50PFS_BY (rw) register accessor: an alias for `Reg<P50PFS_BY_SPEC>`"]
pub type P50PFS_BY = crate::Reg<p50pfs_by::P50PFS_BY_SPEC>;
#[doc = "Port 50%s Pin Function Select Register"]
pub mod p50pfs_by;
#[doc = "P5PFS (rw) register accessor: an alias for `Reg<P5PFS_SPEC>`"]
pub type P5PFS = crate::Reg<p5pfs::P5PFS_SPEC>;
#[doc = "Port 5%s Pin Function Select Register"]
pub mod p5pfs;
#[doc = "P5PFS_HA (rw) register accessor: an alias for `Reg<P5PFS_HA_SPEC>`"]
pub type P5PFS_HA = crate::Reg<p5pfs_ha::P5PFS_HA_SPEC>;
#[doc = "Port 5%s Pin Function Select Register"]
pub mod p5pfs_ha;
#[doc = "P5PFS_BY (rw) register accessor: an alias for `Reg<P5PFS_BY_SPEC>`"]
pub type P5PFS_BY = crate::Reg<p5pfs_by::P5PFS_BY_SPEC>;
#[doc = "Port 5%s Pin Function Select Register"]
pub mod p5pfs_by;
#[doc = "P60PFS (rw) register accessor: an alias for `Reg<P60PFS_SPEC>`"]
pub type P60PFS = crate::Reg<p60pfs::P60PFS_SPEC>;
#[doc = "Port 60%s Pin Function Select Register"]
pub mod p60pfs;
#[doc = "P60PFS_HA (rw) register accessor: an alias for `Reg<P60PFS_HA_SPEC>`"]
pub type P60PFS_HA = crate::Reg<p60pfs_ha::P60PFS_HA_SPEC>;
#[doc = "Port 60%s Pin Function Select Register"]
pub mod p60pfs_ha;
#[doc = "P60PFS_BY (rw) register accessor: an alias for `Reg<P60PFS_BY_SPEC>`"]
pub type P60PFS_BY = crate::Reg<p60pfs_by::P60PFS_BY_SPEC>;
#[doc = "Port 60%s Pin Function Select Register"]
pub mod p60pfs_by;
pub use p60pfs as p608pfs;
pub use p60pfs_by as p608pfs_by;
pub use p60pfs_ha as p608pfs_ha;
pub use P60PFS as P608PFS;
pub use P60PFS_BY as P608PFS_BY;
pub use P60PFS_HA as P608PFS_HA;
#[doc = "P6PFS (rw) register accessor: an alias for `Reg<P6PFS_SPEC>`"]
pub type P6PFS = crate::Reg<p6pfs::P6PFS_SPEC>;
#[doc = "Port 6%s Pin Function Select Register"]
pub mod p6pfs;
#[doc = "P6PFS_HA (rw) register accessor: an alias for `Reg<P6PFS_HA_SPEC>`"]
pub type P6PFS_HA = crate::Reg<p6pfs_ha::P6PFS_HA_SPEC>;
#[doc = "Port 6%s Pin Function Select Register"]
pub mod p6pfs_ha;
#[doc = "P6PFS_BY (rw) register accessor: an alias for `Reg<P6PFS_BY_SPEC>`"]
pub type P6PFS_BY = crate::Reg<p6pfs_by::P6PFS_BY_SPEC>;
#[doc = "Port 6%s Pin Function Select Register"]
pub mod p6pfs_by;
#[doc = "P70PFS (rw) register accessor: an alias for `Reg<P70PFS_SPEC>`"]
pub type P70PFS = crate::Reg<p70pfs::P70PFS_SPEC>;
#[doc = "Port 70%s Pin Function Select Register"]
pub mod p70pfs;
#[doc = "P70PFS_HA (rw) register accessor: an alias for `Reg<P70PFS_HA_SPEC>`"]
pub type P70PFS_HA = crate::Reg<p70pfs_ha::P70PFS_HA_SPEC>;
#[doc = "Port 70%s Pin Function Select Register"]
pub mod p70pfs_ha;
#[doc = "P70PFS_BY (rw) register accessor: an alias for `Reg<P70PFS_BY_SPEC>`"]
pub type P70PFS_BY = crate::Reg<p70pfs_by::P70PFS_BY_SPEC>;
#[doc = "Port 70%s Pin Function Select Register"]
pub mod p70pfs_by;
pub use p70pfs as p708pfs;
pub use p70pfs_by as p708pfs_by;
pub use p70pfs_ha as p708pfs_ha;
pub use P70PFS as P708PFS;
pub use P70PFS_BY as P708PFS_BY;
pub use P70PFS_HA as P708PFS_HA;
#[doc = "P7PFS (rw) register accessor: an alias for `Reg<P7PFS_SPEC>`"]
pub type P7PFS = crate::Reg<p7pfs::P7PFS_SPEC>;
#[doc = "Port 7%s Pin Function Select Register"]
pub mod p7pfs;
#[doc = "P7PFS_HA (rw) register accessor: an alias for `Reg<P7PFS_HA_SPEC>`"]
pub type P7PFS_HA = crate::Reg<p7pfs_ha::P7PFS_HA_SPEC>;
#[doc = "Port 7%s Pin Function Select Register"]
pub mod p7pfs_ha;
#[doc = "P7PFS_BY (rw) register accessor: an alias for `Reg<P7PFS_BY_SPEC>`"]
pub type P7PFS_BY = crate::Reg<p7pfs_by::P7PFS_BY_SPEC>;
#[doc = "Port 7%s Pin Function Select Register"]
pub mod p7pfs_by;
#[doc = "P80PFS (rw) register accessor: an alias for `Reg<P80PFS_SPEC>`"]
pub type P80PFS = crate::Reg<p80pfs::P80PFS_SPEC>;
#[doc = "Port 80%s Pin Function Select Register"]
pub mod p80pfs;
#[doc = "P80PFS_HA (rw) register accessor: an alias for `Reg<P80PFS_HA_SPEC>`"]
pub type P80PFS_HA = crate::Reg<p80pfs_ha::P80PFS_HA_SPEC>;
#[doc = "Port 80%s Pin Function Select Register"]
pub mod p80pfs_ha;
#[doc = "P80PFS_BY (rw) register accessor: an alias for `Reg<P80PFS_BY_SPEC>`"]
pub type P80PFS_BY = crate::Reg<p80pfs_by::P80PFS_BY_SPEC>;
#[doc = "Port 80%s Pin Function Select Register"]
pub mod p80pfs_by;
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
