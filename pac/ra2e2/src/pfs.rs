#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x28],
    _reserved_0_p: [u8; 0x08],
    _reserved1: [u8; 0x08],
    _reserved_1_p014: [u8; 0x04],
    _reserved_2_p015: [u8; 0x04],
    _reserved_3_p1: [u8; 0x10],
    _reserved4: [u8; 0x10],
    _reserved_4_p108: [u8; 0x04],
    _reserved_5_p109: [u8; 0x04],
    _reserved_6_p110: [u8; 0x04],
    _reserved_7_p111: [u8; 0x04],
    _reserved_8_p112: [u8; 0x04],
    _reserved9: [u8; 0x0c],
    _reserved_9_p200: [u8; 0x04],
    _reserved_10_p201: [u8; 0x04],
    _reserved11: [u8; 0x0c],
    _reserved_11_p205: [u8; 0x04],
    _reserved12: [u8; 0x28],
    _reserved_12_p300: [u8; 0x04],
    _reserved13: [u8; 0x3c],
    _reserved_13_p4: [u8; 0x08],
    _reserved14: [u8; 0x03fb],
    #[doc = "0x503 - Write-Protect Register"]
    pub pwpr: PWPR,
    _reserved15: [u8; 0x0b],
    #[doc = "0x50f - Port Read Wait Control Register"]
    pub prwcntr: PRWCNTR,
    _reserved16: [u8; 0x0568],
    _reserved_16_p914: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x28..0x30 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs(&self) -> &[P0PFS; 2] {
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
    #[doc = "0x38 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P014PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x3a - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P014PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3b - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(59usize).cast() }
    }
    #[doc = "0x3c - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P014PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3e - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P014PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
    #[doc = "0x3f - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(63usize).cast() }
    }
    #[doc = "0x40..0x50 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs(&self) -> &[P10PFS; 4] {
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
    #[doc = "0x94 - Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P205PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(148usize).cast() }
    }
    #[doc = "0x96 - Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P205PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(150usize).cast() }
    }
    #[doc = "0x97 - Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P205PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(151usize).cast() }
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
    #[doc = "0x100..0x108 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs(&self) -> &[P40PFS; 2] {
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
    #[doc = "0xa78 - Port 914 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p914pfs(&self) -> &P914PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(2680usize).cast() }
    }
    #[doc = "0xa7a - Port 914 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p914pfs_ha(&self) -> &P914PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(2682usize).cast() }
    }
    #[doc = "0xa7b - Port 914 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p914pfs_by(&self) -> &P914PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(2683usize).cast() }
    }
}
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
pub use p0pfs as p014pfs;
pub use p0pfs_by as p014pfs_by;
pub use p0pfs_ha as p014pfs_ha;
pub use P0PFS as P014PFS;
pub use P0PFS_BY as P014PFS_BY;
pub use P0PFS_HA as P014PFS_HA;
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
pub use P10PFS as P109PFS;
pub use P10PFS as P110PFS;
pub use P10PFS_BY as P109PFS_BY;
pub use P10PFS_BY as P110PFS_BY;
pub use P10PFS_HA as P109PFS_HA;
pub use P10PFS_HA as P110PFS_HA;
#[doc = "P200PFS (rw) register accessor: an alias for `Reg<P200PFS_SPEC>`"]
pub type P200PFS = crate::Reg<p200pfs::P200PFS_SPEC>;
#[doc = "Port 200 Pin Function Select Register"]
pub mod p200pfs;
#[doc = "P200PFS_HA (rw) register accessor: an alias for `Reg<P200PFS_HA_SPEC>`"]
pub type P200PFS_HA = crate::Reg<p200pfs_ha::P200PFS_HA_SPEC>;
#[doc = "Port 200 Pin Function Select Register"]
pub mod p200pfs_ha;
#[doc = "P200PFS_BY (rw) register accessor: an alias for `Reg<P200PFS_BY_SPEC>`"]
pub type P200PFS_BY = crate::Reg<p200pfs_by::P200PFS_BY_SPEC>;
#[doc = "Port 200 Pin Function Select Register"]
pub mod p200pfs_by;
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
#[doc = "P205PFS (rw) register accessor: an alias for `Reg<P205PFS_SPEC>`"]
pub type P205PFS = crate::Reg<p205pfs::P205PFS_SPEC>;
#[doc = "Port 205 Pin Function Select Register"]
pub mod p205pfs;
#[doc = "P205PFS_HA (rw) register accessor: an alias for `Reg<P205PFS_HA_SPEC>`"]
pub type P205PFS_HA = crate::Reg<p205pfs_ha::P205PFS_HA_SPEC>;
#[doc = "Port 205 Pin Function Select Register"]
pub mod p205pfs_ha;
#[doc = "P205PFS_BY (rw) register accessor: an alias for `Reg<P205PFS_BY_SPEC>`"]
pub type P205PFS_BY = crate::Reg<p205pfs_by::P205PFS_BY_SPEC>;
#[doc = "Port 205 Pin Function Select Register"]
pub mod p205pfs_by;
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
#[doc = "PWPR (rw) register accessor: an alias for `Reg<PWPR_SPEC>`"]
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
#[doc = "Write-Protect Register"]
pub mod pwpr;
#[doc = "PRWCNTR (rw) register accessor: an alias for `Reg<PRWCNTR_SPEC>`"]
pub type PRWCNTR = crate::Reg<prwcntr::PRWCNTR_SPEC>;
#[doc = "Port Read Wait Control Register"]
pub mod prwcntr;
#[doc = "P914PFS (rw) register accessor: an alias for `Reg<P914PFS_SPEC>`"]
pub type P914PFS = crate::Reg<p914pfs::P914PFS_SPEC>;
#[doc = "Port 914 Pin Function Select Register"]
pub mod p914pfs;
#[doc = "P914PFS_HA (rw) register accessor: an alias for `Reg<P914PFS_HA_SPEC>`"]
pub type P914PFS_HA = crate::Reg<p914pfs_ha::P914PFS_HA_SPEC>;
#[doc = "Port 914 Pin Function Select Register"]
pub mod p914pfs_ha;
#[doc = "P914PFS_BY (rw) register accessor: an alias for `Reg<P914PFS_BY_SPEC>`"]
pub type P914PFS_BY = crate::Reg<p914pfs_by::P914PFS_BY_SPEC>;
#[doc = "Port 914 Pin Function Select Register"]
pub mod p914pfs_by;
