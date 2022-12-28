#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Source Address Register"]
    pub dmsar: DMSAR,
    #[doc = "0x04 - DMA Destination Address Register"]
    pub dmdar: DMDAR,
    #[doc = "0x08 - DMA Transfer Count Register"]
    pub dmcra: DMCRA,
    #[doc = "0x0c - DMA Block Transfer Count Register"]
    pub dmcrb: DMCRB,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - DMA Transfer Mode Register"]
    pub dmtmd: DMTMD,
    _reserved5: [u8; 0x01],
    #[doc = "0x13 - DMA Interrupt Setting Register"]
    pub dmint: DMINT,
    #[doc = "0x14 - DMA Address Mode Register"]
    pub dmamd: DMAMD,
    _reserved7: [u8; 0x02],
    #[doc = "0x18 - DMA Offset Register"]
    pub dmofr: DMOFR,
    #[doc = "0x1c - DMA Transfer Enable Register"]
    pub dmcnt: DMCNT,
    #[doc = "0x1d - DMA Software Start Register"]
    pub dmreq: DMREQ,
    #[doc = "0x1e - DMAC Module Activation Register"]
    pub dmsts: DMSTS,
}
#[doc = "DMSAR (rw) register accessor: an alias for `Reg<DMSAR_SPEC>`"]
pub type DMSAR = crate::Reg<dmsar::DMSAR_SPEC>;
#[doc = "DMA Source Address Register"]
pub mod dmsar;
#[doc = "DMDAR (rw) register accessor: an alias for `Reg<DMDAR_SPEC>`"]
pub type DMDAR = crate::Reg<dmdar::DMDAR_SPEC>;
#[doc = "DMA Destination Address Register"]
pub mod dmdar;
#[doc = "DMCRA (rw) register accessor: an alias for `Reg<DMCRA_SPEC>`"]
pub type DMCRA = crate::Reg<dmcra::DMCRA_SPEC>;
#[doc = "DMA Transfer Count Register"]
pub mod dmcra;
#[doc = "DMCRB (rw) register accessor: an alias for `Reg<DMCRB_SPEC>`"]
pub type DMCRB = crate::Reg<dmcrb::DMCRB_SPEC>;
#[doc = "DMA Block Transfer Count Register"]
pub mod dmcrb;
#[doc = "DMTMD (rw) register accessor: an alias for `Reg<DMTMD_SPEC>`"]
pub type DMTMD = crate::Reg<dmtmd::DMTMD_SPEC>;
#[doc = "DMA Transfer Mode Register"]
pub mod dmtmd;
#[doc = "DMINT (rw) register accessor: an alias for `Reg<DMINT_SPEC>`"]
pub type DMINT = crate::Reg<dmint::DMINT_SPEC>;
#[doc = "DMA Interrupt Setting Register"]
pub mod dmint;
#[doc = "DMAMD (rw) register accessor: an alias for `Reg<DMAMD_SPEC>`"]
pub type DMAMD = crate::Reg<dmamd::DMAMD_SPEC>;
#[doc = "DMA Address Mode Register"]
pub mod dmamd;
#[doc = "DMOFR (rw) register accessor: an alias for `Reg<DMOFR_SPEC>`"]
pub type DMOFR = crate::Reg<dmofr::DMOFR_SPEC>;
#[doc = "DMA Offset Register"]
pub mod dmofr;
#[doc = "DMCNT (rw) register accessor: an alias for `Reg<DMCNT_SPEC>`"]
pub type DMCNT = crate::Reg<dmcnt::DMCNT_SPEC>;
#[doc = "DMA Transfer Enable Register"]
pub mod dmcnt;
#[doc = "DMREQ (rw) register accessor: an alias for `Reg<DMREQ_SPEC>`"]
pub type DMREQ = crate::Reg<dmreq::DMREQ_SPEC>;
#[doc = "DMA Software Start Register"]
pub mod dmreq;
#[doc = "DMSTS (rw) register accessor: an alias for `Reg<DMSTS_SPEC>`"]
pub type DMSTS = crate::Reg<dmsts::DMSTS_SPEC>;
#[doc = "DMAC Module Activation Register"]
pub mod dmsts;
