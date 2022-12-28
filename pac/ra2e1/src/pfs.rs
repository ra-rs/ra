#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_p0: [u8; 0x24],
    _reserved1: [u8; 0x04],
    _reserved_1_p: [u8; 0x18],
    _reserved_2_p1: [u8; 0x20],
    _reserved_3_p108: [u8; 0x04],
    _reserved_4_p109: [u8; 0x04],
    _reserved_5_p110: [u8; 0x04],
    _reserved_6_p111: [u8; 0x04],
    _reserved_7_p112: [u8; 0x04],
    _reserved_8_p113: [u8; 0x04],
    _reserved_9_p114: [u8; 0x04],
    _reserved_10_p115: [u8; 0x04],
    _reserved_11_p200: [u8; 0x04],
    _reserved_12_p201: [u8; 0x04],
    _reserved_13_p2: [u8; 0x1c],
    _reserved14: [u8; 0x0c],
    _reserved_14_p212: [u8; 0x04],
    _reserved_15_p213: [u8; 0x04],
    _reserved_16_p214: [u8; 0x04],
    _reserved_17_p215: [u8; 0x04],
    _reserved_18_p300: [u8; 0x04],
    _reserved_19_p3: [u8; 0x1c],
    _reserved20: [u8; 0x20],
    _reserved_20_p4: [u8; 0x28],
    _reserved_21_p: [u8; 0x08],
    _reserved22: [u8; 0x10],
    _reserved_22_p5: [u8; 0x18],
    _reserved23: [u8; 0x011c],
    _reserved_23_p: [u8; 0x0c],
    _reserved24: [u8; 0x0283],
    #[doc = "0x503 - Write-Protect Register"]
    pub pwpr: PWPR,
    _reserved25: [u8; 0x0b],
    #[doc = "0x50f - Port Read Wait Control Register"]
    pub prwcntr: PRWCNTR,
}
impl RegisterBlock {
    #[doc = "0x00..0x24 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs(&self) -> &[P00PFS; 9] {
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
    #[doc = "0x22 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(34usize).cast() }
    }
    #[doc = "0x23 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(35usize).cast() }
    }
    #[doc = "0x28..0x40 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs(&self) -> &[P0PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p010pfs(&self) -> &P0PFS {
        &self.p0pfs()[0]
    }
    #[doc = "0x2c - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p011pfs(&self) -> &P0PFS {
        &self.p0pfs()[1]
    }
    #[doc = "0x30 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p012pfs(&self) -> &P0PFS {
        &self.p0pfs()[2]
    }
    #[doc = "0x34 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p013pfs(&self) -> &P0PFS {
        &self.p0pfs()[3]
    }
    #[doc = "0x38 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p014pfs(&self) -> &P0PFS {
        &self.p0pfs()[4]
    }
    #[doc = "0x3c - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p015pfs(&self) -> &P0PFS {
        &self.p0pfs()[5]
    }
    #[doc = "0x2a - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p010pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(42usize).cast() }
    }
    #[doc = "0x2b - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p010pfs_by(&self) -> &P0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(43usize).cast() }
    }
    #[doc = "0x2e - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p011pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(46usize).cast() }
    }
    #[doc = "0x2f - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p011pfs_by(&self) -> &P0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(47usize).cast() }
    }
    #[doc = "0x32 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p012pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(50usize).cast() }
    }
    #[doc = "0x33 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p012pfs_by(&self) -> &P0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(51usize).cast() }
    }
    #[doc = "0x36 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p013pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(54usize).cast() }
    }
    #[doc = "0x37 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p013pfs_by(&self) -> &P0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(55usize).cast() }
    }
    #[doc = "0x3a - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3b - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(59usize).cast() }
    }
    #[doc = "0x3e - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
    #[doc = "0x3f - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(63usize).cast() }
    }
    #[doc = "0x40..0x60 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs(&self) -> &[P10PFS; 8] {
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
    #[doc = "0x60 - Port 108 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs(&self) -> &P108PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(96usize).cast() }
    }
    #[doc = "0x62 - Port 108 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(&self) -> &P108PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(98usize).cast() }
    }
    #[doc = "0x63 - Port 108 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs_by(&self) -> &P108PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(99usize).cast() }
    }
    #[doc = "0x64 - Port 109 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs(&self) -> &P109PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(100usize).cast() }
    }
    #[doc = "0x66 - Port 109 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(&self) -> &P109PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(102usize).cast() }
    }
    #[doc = "0x67 - Port 109 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs_by(&self) -> &P109PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(103usize).cast() }
    }
    #[doc = "0x68 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(104usize).cast() }
    }
    #[doc = "0x6a - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x6b - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(107usize).cast() }
    }
    #[doc = "0x6c - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(108usize).cast() }
    }
    #[doc = "0x6e - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(110usize).cast() }
    }
    #[doc = "0x6f - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(111usize).cast() }
    }
    #[doc = "0x70 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p112pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(112usize).cast() }
    }
    #[doc = "0x72 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p112pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x73 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p112pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(115usize).cast() }
    }
    #[doc = "0x74 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p113pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(116usize).cast() }
    }
    #[doc = "0x76 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p113pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(118usize).cast() }
    }
    #[doc = "0x77 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p113pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(119usize).cast() }
    }
    #[doc = "0x78 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p114pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(120usize).cast() }
    }
    #[doc = "0x7a - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p114pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(122usize).cast() }
    }
    #[doc = "0x7b - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p114pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(123usize).cast() }
    }
    #[doc = "0x7c - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p115pfs(&self) -> &P110PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(124usize).cast() }
    }
    #[doc = "0x7e - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p115pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(126usize).cast() }
    }
    #[doc = "0x7f - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p115pfs_by(&self) -> &P110PFS_BY {
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
    #[doc = "0x88..0xa4 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs(&self) -> &[P20PFS; 7] {
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
    #[doc = "0xb0 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(176usize).cast() }
    }
    #[doc = "0xb2 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(178usize).cast() }
    }
    #[doc = "0xb3 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(179usize).cast() }
    }
    #[doc = "0xb4 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(180usize).cast() }
    }
    #[doc = "0xb6 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(182usize).cast() }
    }
    #[doc = "0xb7 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(183usize).cast() }
    }
    #[doc = "0xb8 - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(184usize).cast() }
    }
    #[doc = "0xba - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p214pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(186usize).cast() }
    }
    #[doc = "0xbb - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p214pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(187usize).cast() }
    }
    #[doc = "0xbc - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p215pfs(&self) -> &P212PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(188usize).cast() }
    }
    #[doc = "0xbe - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p215pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(190usize).cast() }
    }
    #[doc = "0xbf - Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p215pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(191usize).cast() }
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
    #[doc = "0xc4..0xe0 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p30pfs(&self) -> &[P30PFS; 7] {
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
    #[doc = "0x128..0x130 - Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p4pfs(&self) -> &[P4PFS; 2] {
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
    #[doc = "0x140..0x158 - Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p50pfs(&self) -> &[P50PFS; 6] {
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
    #[doc = "0x274..0x280 - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p9pfs(&self) -> &[P9PFS; 3] {
        unsafe { &*(self as *const Self).cast::<u8>().add(628usize).cast() }
    }
    #[doc = "0x274 - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p913pfs(&self) -> &P9PFS {
        &self.p9pfs()[0]
    }
    #[doc = "0x278 - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p914pfs(&self) -> &P9PFS {
        &self.p9pfs()[1]
    }
    #[doc = "0x27c - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p915pfs(&self) -> &P9PFS {
        &self.p9pfs()[2]
    }
    #[doc = "0x276 - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p913pfs_ha(&self) -> &P9PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(630usize).cast() }
    }
    #[doc = "0x277 - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p913pfs_by(&self) -> &P9PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(631usize).cast() }
    }
    #[doc = "0x27a - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p914pfs_ha(&self) -> &P9PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(634usize).cast() }
    }
    #[doc = "0x27b - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p914pfs_by(&self) -> &P9PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(635usize).cast() }
    }
    #[doc = "0x27e - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p915pfs_ha(&self) -> &P9PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(638usize).cast() }
    }
    #[doc = "0x27f - Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p915pfs_by(&self) -> &P9PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(639usize).cast() }
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
#[doc = "P0PFS (rw) register accessor: an alias for `Reg<P0PFS_SPEC>`"]
pub type P0PFS = crate::Reg<p0pfs::P0PFS_SPEC>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs;
#[doc = "P0PFS_HA (rw) register accessor: an alias for `Reg<P0PFS_HA_SPEC>`"]
pub type P0PFS_HA = crate::Reg<p0pfs_ha::P0PFS_HA_SPEC>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs_ha;
#[doc = "P0PFS_BY (rw) register accessor: an alias for `Reg<P0PFS_BY_SPEC>`"]
pub type P0PFS_BY = crate::Reg<p0pfs_by::P0PFS_BY_SPEC>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs_by;
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
#[doc = "P108PFS (rw) register accessor: an alias for `Reg<P108PFS_SPEC>`"]
pub type P108PFS = crate::Reg<p108pfs::P108PFS_SPEC>;
#[doc = "Port 108 Pin Function Select Register"]
pub mod p108pfs;
#[doc = "P108PFS_HA (rw) register accessor: an alias for `Reg<P108PFS_HA_SPEC>`"]
pub type P108PFS_HA = crate::Reg<p108pfs_ha::P108PFS_HA_SPEC>;
#[doc = "Port 108 Pin Function Select Register"]
pub mod p108pfs_ha;
#[doc = "P108PFS_BY (rw) register accessor: an alias for `Reg<P108PFS_BY_SPEC>`"]
pub type P108PFS_BY = crate::Reg<p108pfs_by::P108PFS_BY_SPEC>;
#[doc = "Port 108 Pin Function Select Register"]
pub mod p108pfs_by;
pub use p10pfs as p109pfs;
pub use p10pfs as p110pfs;
pub use p10pfs_by as p109pfs_by;
pub use p10pfs_by as p110pfs_by;
pub use p10pfs_ha as p109pfs_ha;
pub use p10pfs_ha as p110pfs_ha;
pub use p20pfs as p200pfs;
pub use p20pfs_by as p200pfs_by;
pub use p20pfs_ha as p200pfs_ha;
pub use P10PFS as P109PFS;
pub use P10PFS as P110PFS;
pub use P10PFS_BY as P109PFS_BY;
pub use P10PFS_BY as P110PFS_BY;
pub use P10PFS_HA as P109PFS_HA;
pub use P10PFS_HA as P110PFS_HA;
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
pub use p20pfs as p212pfs;
pub use p20pfs_by as p212pfs_by;
pub use p20pfs_ha as p212pfs_ha;
pub use P20PFS as P212PFS;
pub use P20PFS_BY as P212PFS_BY;
pub use P20PFS_HA as P212PFS_HA;
#[doc = "P300PFS (rw) register accessor: an alias for `Reg<P300PFS_SPEC>`"]
pub type P300PFS = crate::Reg<p300pfs::P300PFS_SPEC>;
#[doc = "Port 300 Pin Function Select Register"]
pub mod p300pfs;
pub use p30pfs_by as p300pfs_by;
pub use p30pfs_ha as p300pfs_ha;
pub use P30PFS_BY as P300PFS_BY;
pub use P30PFS_HA as P300PFS_HA;
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
#[doc = "P9PFS (rw) register accessor: an alias for `Reg<P9PFS_SPEC>`"]
pub type P9PFS = crate::Reg<p9pfs::P9PFS_SPEC>;
#[doc = "Port 9%s Pin Function Select Register"]
pub mod p9pfs;
#[doc = "P9PFS_HA (rw) register accessor: an alias for `Reg<P9PFS_HA_SPEC>`"]
pub type P9PFS_HA = crate::Reg<p9pfs_ha::P9PFS_HA_SPEC>;
#[doc = "Port 9%s Pin Function Select Register"]
pub mod p9pfs_ha;
#[doc = "P9PFS_BY (rw) register accessor: an alias for `Reg<P9PFS_BY_SPEC>`"]
pub type P9PFS_BY = crate::Reg<p9pfs_by::P9PFS_BY_SPEC>;
#[doc = "Port 9%s Pin Function Select Register"]
pub mod p9pfs_by;
#[doc = "PWPR (rw) register accessor: an alias for `Reg<PWPR_SPEC>`"]
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
#[doc = "Write-Protect Register"]
pub mod pwpr;
#[doc = "PRWCNTR (rw) register accessor: an alias for `Reg<PRWCNTR_SPEC>`"]
pub type PRWCNTR = crate::Reg<prwcntr::PRWCNTR_SPEC>;
#[doc = "Port Read Wait Control Register"]
pub mod prwcntr;
