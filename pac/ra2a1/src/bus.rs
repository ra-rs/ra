#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1008],
    #[doc = "0x1008 - Master Bus Control Register SYS"]
    pub busmcntsys: BUSMCNTSYS,
    _reserved1: [u8; 0x02],
    #[doc = "0x100c - Master Bus Control Register DMA"]
    pub busmcntdma: BUSMCNTDMA,
    _reserved2: [u8; 0xf2],
    #[doc = "0x1100 - Slave Bus Control Register FLI"]
    pub busscntfli: BUSSCNTFLI,
    _reserved3: [u8; 0x0a],
    #[doc = "0x110c - Slave Bus Control Register RAM0"]
    pub busscntram0: BUSSCNTRAM0,
    _reserved4: [u8; 0x06],
    #[doc = "0x1114 - Slave Bus Control Register %s"]
    pub busscntp0b: BUSSCNT,
    _reserved5: [u8; 0x02],
    #[doc = "0x1118 - Slave Bus Control Register %s"]
    pub busscntp2b: BUSSCNT,
    _reserved6: [u8; 0x06],
    #[doc = "0x1120 - Slave Bus Control Register P4B"]
    pub busscntp4b: BUSSCNTP4B,
    _reserved7: [u8; 0x06],
    #[doc = "0x1128 - Slave Bus Control Register P6B"]
    pub busscntp6b: BUSSCNTP6B,
    _reserved8: [u8; 0x06],
    #[doc = "0x1130 - Slave Bus Control Register FBU"]
    pub busscntfbu: BUSSCNTFBU,
    _reserved9: [u8; 0x06ee],
    #[doc = "0x1820 - Bus Error Address Register %s"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - Bus Error Status Register %s"]
    pub bus3errstat: BUSERRSTAT,
    _reserved11: [u8; 0x0b],
    #[doc = "0x1830 - Bus Error Address Register %s"]
    pub bus4erradd: BUSERRADD,
    #[doc = "0x1834 - Bus Error Status Register %s"]
    pub bus4errstat: BUSERRSTAT,
}
#[doc = "BUSMCNTSYS (rw) register accessor: an alias for `Reg<BUSMCNTSYS_SPEC>`"]
pub type BUSMCNTSYS = crate::Reg<busmcntsys::BUSMCNTSYS_SPEC>;
#[doc = "Master Bus Control Register SYS"]
pub mod busmcntsys;
#[doc = "BUSMCNTDMA (rw) register accessor: an alias for `Reg<BUSMCNTDMA_SPEC>`"]
pub type BUSMCNTDMA = crate::Reg<busmcntdma::BUSMCNTDMA_SPEC>;
#[doc = "Master Bus Control Register DMA"]
pub mod busmcntdma;
#[doc = "BUSSCNTFLI (rw) register accessor: an alias for `Reg<BUSSCNTFLI_SPEC>`"]
pub type BUSSCNTFLI = crate::Reg<busscntfli::BUSSCNTFLI_SPEC>;
#[doc = "Slave Bus Control Register FLI"]
pub mod busscntfli;
#[doc = "BUSSCNTRAM0 (rw) register accessor: an alias for `Reg<BUSSCNTRAM0_SPEC>`"]
pub type BUSSCNTRAM0 = crate::Reg<busscntram0::BUSSCNTRAM0_SPEC>;
#[doc = "Slave Bus Control Register RAM0"]
pub mod busscntram0;
#[doc = "BUSSCNT (rw) register accessor: an alias for `Reg<BUSSCNT_SPEC>`"]
pub type BUSSCNT = crate::Reg<busscnt::BUSSCNT_SPEC>;
#[doc = "Slave Bus Control Register %s"]
pub mod busscnt;
#[doc = "BUSSCNTP4B (rw) register accessor: an alias for `Reg<BUSSCNTP4B_SPEC>`"]
pub type BUSSCNTP4B = crate::Reg<busscntp4b::BUSSCNTP4B_SPEC>;
#[doc = "Slave Bus Control Register P4B"]
pub mod busscntp4b;
#[doc = "BUSSCNTP6B (rw) register accessor: an alias for `Reg<BUSSCNTP6B_SPEC>`"]
pub type BUSSCNTP6B = crate::Reg<busscntp6b::BUSSCNTP6B_SPEC>;
#[doc = "Slave Bus Control Register P6B"]
pub mod busscntp6b;
#[doc = "BUSSCNTFBU (rw) register accessor: an alias for `Reg<BUSSCNTFBU_SPEC>`"]
pub type BUSSCNTFBU = crate::Reg<busscntfbu::BUSSCNTFBU_SPEC>;
#[doc = "Slave Bus Control Register FBU"]
pub mod busscntfbu;
#[doc = "BUSERRADD (r) register accessor: an alias for `Reg<BUSERRADD_SPEC>`"]
pub type BUSERRADD = crate::Reg<buserradd::BUSERRADD_SPEC>;
#[doc = "Bus Error Address Register %s"]
pub mod buserradd;
#[doc = "BUSERRSTAT (r) register accessor: an alias for `Reg<BUSERRSTAT_SPEC>`"]
pub type BUSERRSTAT = crate::Reg<buserrstat::BUSERRSTAT_SPEC>;
#[doc = "Bus Error Status Register %s"]
pub mod buserrstat;
