#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1100],
    #[doc = "0x1100 - Slave Bus Control Register"]
    pub busscntfhbiu: BUSSCNTFHBIU,
    _reserved1: [u8; 0x02],
    #[doc = "0x1104 - Slave Bus Control Register"]
    pub busscntflbiu: BUSSCNTFLBIU,
    _reserved2: [u8; 0x0a],
    #[doc = "0x1110 - Slave Bus Control Register"]
    pub busscnts0biu: BUSSCNTS0BIU,
    _reserved3: [u8; 0x0e],
    #[doc = "0x1120 - Slave Bus Control Register"]
    pub busscntpsbiu: BUSSCNTPSBIU,
    _reserved4: [u8; 0x0e],
    #[doc = "0x1130 - Slave Bus Control Register"]
    pub busscntplbiu: BUSSCNTPLBIU,
    _reserved5: [u8; 0x02],
    #[doc = "0x1134 - Slave Bus Control Register"]
    pub busscntphbiu: BUSSCNTPHBIU,
    _reserved6: [u8; 0x06ca],
    #[doc = "0x1800 - BUS Error Address Register"]
    pub bus1erradd: BUSERRADD,
    #[doc = "0x1804 - BUS Error Read Write Register"]
    pub bus1errrw: BUSERRRW,
    _reserved8: [u8; 0x0b],
    #[doc = "0x1810 - BUS Error Address Register"]
    pub bus2erradd: BUSERRADD,
    #[doc = "0x1814 - BUS Error Read Write Register"]
    pub bus2errrw: BUSERRRW,
    _reserved10: [u8; 0x0b],
    #[doc = "0x1820 - BUS Error Address Register"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - BUS Error Read Write Register"]
    pub bus3errrw: BUSERRRW,
    _reserved12: [u8; 0xdb],
    #[doc = "0x1900 - BUS TZF Error Address Register"]
    pub btzf1erradd: BTZFERRADD,
    #[doc = "0x1904 - BUS TZF Error Read Write Register"]
    pub btzf1errrw: BTZFERRRW,
    _reserved14: [u8; 0x0b],
    #[doc = "0x1910 - BUS TZF Error Address Register"]
    pub btzf2erradd: BTZFERRADD,
    #[doc = "0x1914 - BUS TZF Error Read Write Register"]
    pub btzf2errrw: BTZFERRRW,
    _reserved16: [u8; 0x0b],
    #[doc = "0x1920 - BUS TZF Error Address Register"]
    pub btzf3erradd: BTZFERRADD,
    #[doc = "0x1924 - BUS TZF Error Read Write Register"]
    pub btzf3errrw: BTZFERRRW,
    _reserved18: [u8; 0xdb],
    #[doc = "0x1a00 - BUS Error Status Register %s"]
    pub bus1errstat: BUSERRSTAT,
    _reserved19: [u8; 0x07],
    #[doc = "0x1a08 - BUS Error Clear Register %s"]
    pub bus1errclr: BUSERRCLR,
    _reserved20: [u8; 0x07],
    #[doc = "0x1a10 - BUS Error Status Register %s"]
    pub bus2errstat: BUSERRSTAT,
    _reserved21: [u8; 0x07],
    #[doc = "0x1a18 - BUS Error Clear Register %s"]
    pub bus2errclr: BUSERRCLR,
    _reserved22: [u8; 0x07],
    #[doc = "0x1a20 - BUS Error Status Register %s"]
    pub bus3errstat: BUSERRSTAT,
    _reserved23: [u8; 0x03],
    #[doc = "0x1a24 - DMAC/DTC Error Status Register"]
    pub dmacdtcerrstat: DMACDTCERRSTAT,
    _reserved24: [u8; 0x03],
    #[doc = "0x1a28 - BUS Error Clear Register %s"]
    pub bus3errclr: BUSERRCLR,
    _reserved25: [u8; 0x03],
    #[doc = "0x1a2c - DMAC/DTC Error Clear Register"]
    pub dmacdtcerrclr: DMACDTCERRCLR,
}
#[doc = "BUSSCNTFHBIU (rw) register accessor: an alias for `Reg<BUSSCNTFHBIU_SPEC>`"]
pub type BUSSCNTFHBIU = crate::Reg<busscntfhbiu::BUSSCNTFHBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntfhbiu;
#[doc = "BUSSCNTFLBIU (rw) register accessor: an alias for `Reg<BUSSCNTFLBIU_SPEC>`"]
pub type BUSSCNTFLBIU = crate::Reg<busscntflbiu::BUSSCNTFLBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntflbiu;
#[doc = "BUSSCNTS0BIU (rw) register accessor: an alias for `Reg<BUSSCNTS0BIU_SPEC>`"]
pub type BUSSCNTS0BIU = crate::Reg<busscnts0biu::BUSSCNTS0BIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscnts0biu;
#[doc = "BUSSCNTPSBIU (rw) register accessor: an alias for `Reg<BUSSCNTPSBIU_SPEC>`"]
pub type BUSSCNTPSBIU = crate::Reg<busscntpsbiu::BUSSCNTPSBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntpsbiu;
#[doc = "BUSSCNTPLBIU (rw) register accessor: an alias for `Reg<BUSSCNTPLBIU_SPEC>`"]
pub type BUSSCNTPLBIU = crate::Reg<busscntplbiu::BUSSCNTPLBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntplbiu;
#[doc = "BUSSCNTPHBIU (rw) register accessor: an alias for `Reg<BUSSCNTPHBIU_SPEC>`"]
pub type BUSSCNTPHBIU = crate::Reg<busscntphbiu::BUSSCNTPHBIU_SPEC>;
#[doc = "Slave Bus Control Register"]
pub mod busscntphbiu;
#[doc = "BUSERRADD (r) register accessor: an alias for `Reg<BUSERRADD_SPEC>`"]
pub type BUSERRADD = crate::Reg<buserradd::BUSERRADD_SPEC>;
#[doc = "BUS Error Address Register"]
pub mod buserradd;
#[doc = "BUSERRRW (rw) register accessor: an alias for `Reg<BUSERRRW_SPEC>`"]
pub type BUSERRRW = crate::Reg<buserrrw::BUSERRRW_SPEC>;
#[doc = "BUS Error Read Write Register"]
pub mod buserrrw;
#[doc = "BTZFERRADD (r) register accessor: an alias for `Reg<BTZFERRADD_SPEC>`"]
pub type BTZFERRADD = crate::Reg<btzferradd::BTZFERRADD_SPEC>;
#[doc = "BUS TZF Error Address Register"]
pub mod btzferradd;
#[doc = "BTZFERRRW (rw) register accessor: an alias for `Reg<BTZFERRRW_SPEC>`"]
pub type BTZFERRRW = crate::Reg<btzferrrw::BTZFERRRW_SPEC>;
#[doc = "BUS TZF Error Read Write Register"]
pub mod btzferrrw;
#[doc = "BUSERRSTAT (r) register accessor: an alias for `Reg<BUSERRSTAT_SPEC>`"]
pub type BUSERRSTAT = crate::Reg<buserrstat::BUSERRSTAT_SPEC>;
#[doc = "BUS Error Status Register %s"]
pub mod buserrstat;
#[doc = "BUSERRCLR (rw) register accessor: an alias for `Reg<BUSERRCLR_SPEC>`"]
pub type BUSERRCLR = crate::Reg<buserrclr::BUSERRCLR_SPEC>;
#[doc = "BUS Error Clear Register %s"]
pub mod buserrclr;
#[doc = "DMACDTCERRSTAT (r) register accessor: an alias for `Reg<DMACDTCERRSTAT_SPEC>`"]
pub type DMACDTCERRSTAT = crate::Reg<dmacdtcerrstat::DMACDTCERRSTAT_SPEC>;
#[doc = "DMAC/DTC Error Status Register"]
pub mod dmacdtcerrstat;
#[doc = "DMACDTCERRCLR (rw) register accessor: an alias for `Reg<DMACDTCERRCLR_SPEC>`"]
pub type DMACDTCERRCLR = crate::Reg<dmacdtcerrclr::DMACDTCERRCLR_SPEC>;
#[doc = "DMAC/DTC Error Clear Register"]
pub mod dmacdtcerrclr;
