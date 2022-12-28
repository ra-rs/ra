#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETHERC Mode Register"]
    pub ecmr: ECMR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Receive Frame Maximum Length Register"]
    pub rflr: RFLR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - ETHERC Status Register"]
    pub ecsr: ECSR,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - ETHERC Interrupt Enable Register"]
    pub ecsipr: ECSIPR,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - PHY Interface Register"]
    pub pir: PIR,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - PHY Status Register"]
    pub psr: PSR,
    _reserved6: [u8; 0x14],
    #[doc = "0x40 - Random Number Generation Counter Upper Limit Setting Register"]
    pub rdmlr: RDMLR,
    _reserved7: [u8; 0x0c],
    #[doc = "0x50 - Interpacket Gap Register"]
    pub ipgr: IPGR,
    #[doc = "0x54 - Automatic PAUSE Frame Register"]
    pub apr: APR,
    #[doc = "0x58 - Manual PAUSE Frame Register"]
    pub mpr: MPR,
    _reserved10: [u8; 0x04],
    #[doc = "0x60 - Received PAUSE Frame Counter"]
    pub rfcf: RFCF,
    #[doc = "0x64 - PAUSE Frame Retransmit Count Setting Register"]
    pub tpauser: TPAUSER,
    #[doc = "0x68 - PAUSE Frame Retransmit Counter"]
    pub tpausecr: TPAUSECR,
    #[doc = "0x6c - Broadcast Frame Receive Count Setting Register"]
    pub bcfrr: BCFRR,
    _reserved14: [u8; 0x50],
    #[doc = "0xc0 - MAC Address Upper Bit Register"]
    pub mahr: MAHR,
    _reserved15: [u8; 0x04],
    #[doc = "0xc8 - MAC Address Lower Bit Register"]
    pub malr: MALR,
    _reserved16: [u8; 0x04],
    #[doc = "0xd0 - Transmit Retry Over Counter Register"]
    pub trocr: TROCR,
    #[doc = "0xd4 - Late Collision Detect Counter Register"]
    pub cdcr: CDCR,
    #[doc = "0xd8 - Lost Carrier Counter Register"]
    pub lccr: LCCR,
    #[doc = "0xdc - Carrier Not Detect Counter Register"]
    pub cndcr: CNDCR,
    _reserved20: [u8; 0x04],
    #[doc = "0xe4 - CRC Error Frame Receive Counter Register"]
    pub cefcr: CEFCR,
    #[doc = "0xe8 - Frame Receive Error Counter Register"]
    pub frecr: FRECR,
    #[doc = "0xec - Too-Short Frame Receive Counter Register"]
    pub tsfrcr: TSFRCR,
    #[doc = "0xf0 - Too-Long Frame Receive Counter Register"]
    pub tlfrcr: TLFRCR,
    #[doc = "0xf4 - Received Alignment Error Frame Counter Register"]
    pub rfcr: RFCR,
    #[doc = "0xf8 - Multicast Address Frame Receive Counter Register"]
    pub mafcr: MAFCR,
}
#[doc = "ECMR (rw) register accessor: an alias for `Reg<ECMR_SPEC>`"]
pub type ECMR = crate::Reg<ecmr::ECMR_SPEC>;
#[doc = "ETHERC Mode Register"]
pub mod ecmr;
#[doc = "RFLR (rw) register accessor: an alias for `Reg<RFLR_SPEC>`"]
pub type RFLR = crate::Reg<rflr::RFLR_SPEC>;
#[doc = "Receive Frame Maximum Length Register"]
pub mod rflr;
#[doc = "ECSR (rw) register accessor: an alias for `Reg<ECSR_SPEC>`"]
pub type ECSR = crate::Reg<ecsr::ECSR_SPEC>;
#[doc = "ETHERC Status Register"]
pub mod ecsr;
#[doc = "ECSIPR (rw) register accessor: an alias for `Reg<ECSIPR_SPEC>`"]
pub type ECSIPR = crate::Reg<ecsipr::ECSIPR_SPEC>;
#[doc = "ETHERC Interrupt Enable Register"]
pub mod ecsipr;
#[doc = "PIR (rw) register accessor: an alias for `Reg<PIR_SPEC>`"]
pub type PIR = crate::Reg<pir::PIR_SPEC>;
#[doc = "PHY Interface Register"]
pub mod pir;
#[doc = "PSR (r) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "PHY Status Register"]
pub mod psr;
#[doc = "RDMLR (rw) register accessor: an alias for `Reg<RDMLR_SPEC>`"]
pub type RDMLR = crate::Reg<rdmlr::RDMLR_SPEC>;
#[doc = "Random Number Generation Counter Upper Limit Setting Register"]
pub mod rdmlr;
#[doc = "IPGR (rw) register accessor: an alias for `Reg<IPGR_SPEC>`"]
pub type IPGR = crate::Reg<ipgr::IPGR_SPEC>;
#[doc = "Interpacket Gap Register"]
pub mod ipgr;
#[doc = "APR (rw) register accessor: an alias for `Reg<APR_SPEC>`"]
pub type APR = crate::Reg<apr::APR_SPEC>;
#[doc = "Automatic PAUSE Frame Register"]
pub mod apr;
#[doc = "MPR (rw) register accessor: an alias for `Reg<MPR_SPEC>`"]
pub type MPR = crate::Reg<mpr::MPR_SPEC>;
#[doc = "Manual PAUSE Frame Register"]
pub mod mpr;
#[doc = "RFCF (r) register accessor: an alias for `Reg<RFCF_SPEC>`"]
pub type RFCF = crate::Reg<rfcf::RFCF_SPEC>;
#[doc = "Received PAUSE Frame Counter"]
pub mod rfcf;
#[doc = "TPAUSER (rw) register accessor: an alias for `Reg<TPAUSER_SPEC>`"]
pub type TPAUSER = crate::Reg<tpauser::TPAUSER_SPEC>;
#[doc = "PAUSE Frame Retransmit Count Setting Register"]
pub mod tpauser;
#[doc = "TPAUSECR (r) register accessor: an alias for `Reg<TPAUSECR_SPEC>`"]
pub type TPAUSECR = crate::Reg<tpausecr::TPAUSECR_SPEC>;
#[doc = "PAUSE Frame Retransmit Counter"]
pub mod tpausecr;
#[doc = "BCFRR (rw) register accessor: an alias for `Reg<BCFRR_SPEC>`"]
pub type BCFRR = crate::Reg<bcfrr::BCFRR_SPEC>;
#[doc = "Broadcast Frame Receive Count Setting Register"]
pub mod bcfrr;
#[doc = "MAHR (rw) register accessor: an alias for `Reg<MAHR_SPEC>`"]
pub type MAHR = crate::Reg<mahr::MAHR_SPEC>;
#[doc = "MAC Address Upper Bit Register"]
pub mod mahr;
#[doc = "MALR (rw) register accessor: an alias for `Reg<MALR_SPEC>`"]
pub type MALR = crate::Reg<malr::MALR_SPEC>;
#[doc = "MAC Address Lower Bit Register"]
pub mod malr;
#[doc = "TROCR (rw) register accessor: an alias for `Reg<TROCR_SPEC>`"]
pub type TROCR = crate::Reg<trocr::TROCR_SPEC>;
#[doc = "Transmit Retry Over Counter Register"]
pub mod trocr;
#[doc = "CDCR (rw) register accessor: an alias for `Reg<CDCR_SPEC>`"]
pub type CDCR = crate::Reg<cdcr::CDCR_SPEC>;
#[doc = "Late Collision Detect Counter Register"]
pub mod cdcr;
#[doc = "LCCR (rw) register accessor: an alias for `Reg<LCCR_SPEC>`"]
pub type LCCR = crate::Reg<lccr::LCCR_SPEC>;
#[doc = "Lost Carrier Counter Register"]
pub mod lccr;
#[doc = "CNDCR (rw) register accessor: an alias for `Reg<CNDCR_SPEC>`"]
pub type CNDCR = crate::Reg<cndcr::CNDCR_SPEC>;
#[doc = "Carrier Not Detect Counter Register"]
pub mod cndcr;
#[doc = "CEFCR (rw) register accessor: an alias for `Reg<CEFCR_SPEC>`"]
pub type CEFCR = crate::Reg<cefcr::CEFCR_SPEC>;
#[doc = "CRC Error Frame Receive Counter Register"]
pub mod cefcr;
#[doc = "FRECR (rw) register accessor: an alias for `Reg<FRECR_SPEC>`"]
pub type FRECR = crate::Reg<frecr::FRECR_SPEC>;
#[doc = "Frame Receive Error Counter Register"]
pub mod frecr;
#[doc = "TSFRCR (rw) register accessor: an alias for `Reg<TSFRCR_SPEC>`"]
pub type TSFRCR = crate::Reg<tsfrcr::TSFRCR_SPEC>;
#[doc = "Too-Short Frame Receive Counter Register"]
pub mod tsfrcr;
#[doc = "TLFRCR (rw) register accessor: an alias for `Reg<TLFRCR_SPEC>`"]
pub type TLFRCR = crate::Reg<tlfrcr::TLFRCR_SPEC>;
#[doc = "Too-Long Frame Receive Counter Register"]
pub mod tlfrcr;
#[doc = "RFCR (rw) register accessor: an alias for `Reg<RFCR_SPEC>`"]
pub type RFCR = crate::Reg<rfcr::RFCR_SPEC>;
#[doc = "Received Alignment Error Frame Counter Register"]
pub mod rfcr;
#[doc = "MAFCR (rw) register accessor: an alias for `Reg<MAFCR_SPEC>`"]
pub type MAFCR = crate::Reg<mafcr::MAFCR_SPEC>;
#[doc = "Multicast Address Frame Receive Counter Register"]
pub mod mafcr;
