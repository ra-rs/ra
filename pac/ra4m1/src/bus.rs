#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    #[doc = "0x1000 - Master Bus Control Register %s"]
    pub busmcntm4i: BUSMCNT,
    _reserved1: [u8; 0x02],
    #[doc = "0x1004 - Master Bus Control Register %s"]
    pub busmcntm4d: BUSMCNT,
    _reserved2: [u8; 0x02],
    #[doc = "0x1008 - Master Bus Control Register %s"]
    pub busmcntsys: BUSMCNT,
    _reserved3: [u8; 0x02],
    #[doc = "0x100c - Master Bus Control Register %s"]
    pub busmcntdma: BUSMCNT,
    _reserved4: [u8; 0xf2],
    #[doc = "0x1100 - Slave Bus Control Register FLI"]
    pub busscntfli: BUSSCNTFLI,
    _reserved5: [u8; 0x06],
    #[doc = "0x1108 - Slave Bus Control Register %s"]
    pub busscntmbiu: BUSSCNT,
    _reserved6: [u8; 0x02],
    #[doc = "0x110c - Slave Bus Control Register %s"]
    pub busscntram0: BUSSCNT,
    _reserved7: [u8; 0x06],
    #[doc = "0x1114 - Slave Bus Control Register %s"]
    pub busscntp0b: BUSSCNTP0B,
    _reserved8: [u8; 0x02],
    #[doc = "0x1118 - Slave Bus Control Register %s"]
    pub busscntp2b: BUSSCNTP0B,
    _reserved9: [u8; 0x02],
    #[doc = "0x111c - Slave Bus Control Register %s"]
    pub busscntp3b: BUSSCNTP0B,
    _reserved10: [u8; 0x02],
    #[doc = "0x1120 - Slave Bus Control Register %s"]
    pub busscntp4b: BUSSCNTP0B,
    _reserved11: [u8; 0x06],
    #[doc = "0x1128 - Slave Bus Control Register P6B"]
    pub busscntp6b: BUSSCNTP6B,
    _reserved12: [u8; 0x06],
    #[doc = "0x1130 - Slave Bus Control Register FBU"]
    pub busscntfbu: BUSSCNTFBU,
    _reserved13: [u8; 0x06ce],
    #[doc = "0x1800 - Bus Error Address Register %s"]
    pub bus1erradd: BUSERRADD,
    #[doc = "0x1804 - Bus Error Status Register %s"]
    pub bus1errstat: BUSERRSTAT,
    _reserved15: [u8; 0x0b],
    #[doc = "0x1810 - Bus Error Address Register %s"]
    pub bus2erradd: BUSERRADD,
    #[doc = "0x1814 - Bus Error Status Register %s"]
    pub bus2errstat: BUSERRSTAT,
    _reserved17: [u8; 0x0b],
    #[doc = "0x1820 - Bus Error Address Register %s"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - Bus Error Status Register %s"]
    pub bus3errstat: BUSERRSTAT,
    _reserved19: [u8; 0x0b],
    #[doc = "0x1830 - Bus Error Address Register %s"]
    pub bus4erradd: BUSERRADD,
    #[doc = "0x1834 - Bus Error Status Register %s"]
    pub bus4errstat: BUSERRSTAT,
}
#[doc = "BUSMCNT (rw) register accessor: an alias for `Reg<BUSMCNT_SPEC>`"]
pub type BUSMCNT = crate::Reg<busmcnt::BUSMCNT_SPEC>;
#[doc = "Master Bus Control Register %s"]
pub mod busmcnt;
#[doc = "BUSSCNTFLI (rw) register accessor: an alias for `Reg<BUSSCNTFLI_SPEC>`"]
pub type BUSSCNTFLI = crate::Reg<busscntfli::BUSSCNTFLI_SPEC>;
#[doc = "Slave Bus Control Register FLI"]
pub mod busscntfli;
#[doc = "BUSSCNT (rw) register accessor: an alias for `Reg<BUSSCNT_SPEC>`"]
pub type BUSSCNT = crate::Reg<busscnt::BUSSCNT_SPEC>;
#[doc = "Slave Bus Control Register %s"]
pub mod busscnt;
pub use busscnt as busscntp0b;
pub use BUSSCNT as BUSSCNTP0B;
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
