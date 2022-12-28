#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1008],
    #[doc = "0x1008 - Master Bus Control Register SYS"]
    pub busmcntsys: BUSMCNTSYS,
    _reserved1: [u8; 0x02],
    #[doc = "0x100c - Master Bus Control Register DMA"]
    pub busmcntdma: BUSMCNTDMA,
    _reserved2: [u8; 0x0812],
    #[doc = "0x1820 - Bus Error Address Register 3"]
    pub bus3erradd: BUS3ERRADD,
    #[doc = "0x1824 - BUS Error Status Register 3"]
    pub bus3errstat: BUS3ERRSTAT,
    _reserved4: [u8; 0x0b],
    #[doc = "0x1830 - Bus Error Address Register 4"]
    pub bus4erradd: BUS4ERRADD,
    #[doc = "0x1834 - BUS Error Status Register 4"]
    pub bus4errstat: BUS4ERRSTAT,
}
#[doc = "BUSMCNTSYS (rw) register accessor: an alias for `Reg<BUSMCNTSYS_SPEC>`"]
pub type BUSMCNTSYS = crate::Reg<busmcntsys::BUSMCNTSYS_SPEC>;
#[doc = "Master Bus Control Register SYS"]
pub mod busmcntsys;
#[doc = "BUSMCNTDMA (rw) register accessor: an alias for `Reg<BUSMCNTDMA_SPEC>`"]
pub type BUSMCNTDMA = crate::Reg<busmcntdma::BUSMCNTDMA_SPEC>;
#[doc = "Master Bus Control Register DMA"]
pub mod busmcntdma;
#[doc = "BUS3ERRADD (r) register accessor: an alias for `Reg<BUS3ERRADD_SPEC>`"]
pub type BUS3ERRADD = crate::Reg<bus3erradd::BUS3ERRADD_SPEC>;
#[doc = "Bus Error Address Register 3"]
pub mod bus3erradd;
#[doc = "BUS3ERRSTAT (r) register accessor: an alias for `Reg<BUS3ERRSTAT_SPEC>`"]
pub type BUS3ERRSTAT = crate::Reg<bus3errstat::BUS3ERRSTAT_SPEC>;
#[doc = "BUS Error Status Register 3"]
pub mod bus3errstat;
#[doc = "BUS4ERRADD (r) register accessor: an alias for `Reg<BUS4ERRADD_SPEC>`"]
pub type BUS4ERRADD = crate::Reg<bus4erradd::BUS4ERRADD_SPEC>;
#[doc = "Bus Error Address Register 4"]
pub mod bus4erradd;
#[doc = "BUS4ERRSTAT (r) register accessor: an alias for `Reg<BUS4ERRSTAT_SPEC>`"]
pub type BUS4ERRSTAT = crate::Reg<bus4errstat::BUS4ERRSTAT_SPEC>;
#[doc = "BUS Error Status Register 4"]
pub mod bus4errstat;
