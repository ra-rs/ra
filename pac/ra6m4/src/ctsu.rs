#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTSU Control Register 0"]
    pub ctsucr0: CTSUCR0,
    #[doc = "0x01 - CTSU Control Register 1"]
    pub ctsucr1: CTSUCR1,
    #[doc = "0x02 - CTSU Synchronous Noise Reduction Setting Register"]
    pub ctsusdprs: CTSUSDPRS,
    #[doc = "0x03 - CTSU Sensor Stabilization Wait Control Register"]
    pub ctsusst: CTSUSST,
    #[doc = "0x04 - CTSU Measurement Channel Register 0"]
    pub ctsumch0: CTSUMCH0,
    #[doc = "0x05 - CTSU Measurement Channel Register 1"]
    pub ctsumch1: CTSUMCH1,
    #[doc = "0x06 - CTSU Channel Enable Control Register 0"]
    pub ctsuchac0: CTSUCHAC0,
    #[doc = "0x07 - CTSU Channel Enable Control Register 1"]
    pub ctsuchac1: CTSUCHAC1,
    #[doc = "0x08 - CTSU Channel Enable Control Register 2"]
    pub ctsuchac2: CTSUCHAC2,
    _reserved9: [u8; 0x02],
    #[doc = "0x0b - CTSU Channel Transmit/Receive Control Register 0"]
    pub ctsuchtrc0: CTSUCHTRC0,
    #[doc = "0x0c - CTSU Channel Transmit/Receive Control Register 1"]
    pub ctsuchtrc1: CTSUCHTRC1,
    #[doc = "0x0d - CTSU Channel Transmit/Receive Control Register 2"]
    pub ctsuchtrc2: CTSUCHTRC2,
    _reserved12: [u8; 0x02],
    #[doc = "0x10 - CTSU High-Pass Noise Reduction Control Register"]
    pub ctsudclkc: CTSUDCLKC,
    #[doc = "0x11 - CTSU Status Register"]
    pub ctsust: CTSUST,
    #[doc = "0x12 - CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
    pub ctsussc: CTSUSSC,
    #[doc = "0x14 - CTSU Sensor Offset Register 0"]
    pub ctsuso0: CTSUSO0,
    #[doc = "0x16 - CTSU Sensor Offset Register 1"]
    pub ctsuso1: CTSUSO1,
    #[doc = "0x18 - CTSU Sensor Counter"]
    pub ctsusc: CTSUSC,
    #[doc = "0x1a - CTSU Reference Counter"]
    pub ctsurc: CTSURC,
    #[doc = "0x1c - CTSU Error Status Register"]
    pub ctsuerrs: CTSUERRS,
    _reserved20: [u8; 0x02],
    #[doc = "0x20 - CTSU Reference Current Calibration Register"]
    pub ctsutrmr: CTSUTRMR,
}
#[doc = "CTSUCR0 (rw) register accessor: an alias for `Reg<CTSUCR0_SPEC>`"]
pub type CTSUCR0 = crate::Reg<ctsucr0::CTSUCR0_SPEC>;
#[doc = "CTSU Control Register 0"]
pub mod ctsucr0;
#[doc = "CTSUCR1 (rw) register accessor: an alias for `Reg<CTSUCR1_SPEC>`"]
pub type CTSUCR1 = crate::Reg<ctsucr1::CTSUCR1_SPEC>;
#[doc = "CTSU Control Register 1"]
pub mod ctsucr1;
#[doc = "CTSUSDPRS (rw) register accessor: an alias for `Reg<CTSUSDPRS_SPEC>`"]
pub type CTSUSDPRS = crate::Reg<ctsusdprs::CTSUSDPRS_SPEC>;
#[doc = "CTSU Synchronous Noise Reduction Setting Register"]
pub mod ctsusdprs;
#[doc = "CTSUSST (rw) register accessor: an alias for `Reg<CTSUSST_SPEC>`"]
pub type CTSUSST = crate::Reg<ctsusst::CTSUSST_SPEC>;
#[doc = "CTSU Sensor Stabilization Wait Control Register"]
pub mod ctsusst;
#[doc = "CTSUMCH0 (rw) register accessor: an alias for `Reg<CTSUMCH0_SPEC>`"]
pub type CTSUMCH0 = crate::Reg<ctsumch0::CTSUMCH0_SPEC>;
#[doc = "CTSU Measurement Channel Register 0"]
pub mod ctsumch0;
#[doc = "CTSUMCH1 (r) register accessor: an alias for `Reg<CTSUMCH1_SPEC>`"]
pub type CTSUMCH1 = crate::Reg<ctsumch1::CTSUMCH1_SPEC>;
#[doc = "CTSU Measurement Channel Register 1"]
pub mod ctsumch1;
#[doc = "CTSUCHAC0 (rw) register accessor: an alias for `Reg<CTSUCHAC0_SPEC>`"]
pub type CTSUCHAC0 = crate::Reg<ctsuchac0::CTSUCHAC0_SPEC>;
#[doc = "CTSU Channel Enable Control Register 0"]
pub mod ctsuchac0;
#[doc = "CTSUCHAC1 (rw) register accessor: an alias for `Reg<CTSUCHAC1_SPEC>`"]
pub type CTSUCHAC1 = crate::Reg<ctsuchac1::CTSUCHAC1_SPEC>;
#[doc = "CTSU Channel Enable Control Register 1"]
pub mod ctsuchac1;
#[doc = "CTSUCHAC2 (rw) register accessor: an alias for `Reg<CTSUCHAC2_SPEC>`"]
pub type CTSUCHAC2 = crate::Reg<ctsuchac2::CTSUCHAC2_SPEC>;
#[doc = "CTSU Channel Enable Control Register 2"]
pub mod ctsuchac2;
#[doc = "CTSUCHTRC0 (rw) register accessor: an alias for `Reg<CTSUCHTRC0_SPEC>`"]
pub type CTSUCHTRC0 = crate::Reg<ctsuchtrc0::CTSUCHTRC0_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 0"]
pub mod ctsuchtrc0;
#[doc = "CTSUCHTRC1 (rw) register accessor: an alias for `Reg<CTSUCHTRC1_SPEC>`"]
pub type CTSUCHTRC1 = crate::Reg<ctsuchtrc1::CTSUCHTRC1_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 1"]
pub mod ctsuchtrc1;
#[doc = "CTSUCHTRC2 (rw) register accessor: an alias for `Reg<CTSUCHTRC2_SPEC>`"]
pub type CTSUCHTRC2 = crate::Reg<ctsuchtrc2::CTSUCHTRC2_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 2"]
pub mod ctsuchtrc2;
#[doc = "CTSUDCLKC (rw) register accessor: an alias for `Reg<CTSUDCLKC_SPEC>`"]
pub type CTSUDCLKC = crate::Reg<ctsudclkc::CTSUDCLKC_SPEC>;
#[doc = "CTSU High-Pass Noise Reduction Control Register"]
pub mod ctsudclkc;
#[doc = "CTSUST (rw) register accessor: an alias for `Reg<CTSUST_SPEC>`"]
pub type CTSUST = crate::Reg<ctsust::CTSUST_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsust;
#[doc = "CTSUSSC (rw) register accessor: an alias for `Reg<CTSUSSC_SPEC>`"]
pub type CTSUSSC = crate::Reg<ctsussc::CTSUSSC_SPEC>;
#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
pub mod ctsussc;
#[doc = "CTSUSO0 (rw) register accessor: an alias for `Reg<CTSUSO0_SPEC>`"]
pub type CTSUSO0 = crate::Reg<ctsuso0::CTSUSO0_SPEC>;
#[doc = "CTSU Sensor Offset Register 0"]
pub mod ctsuso0;
#[doc = "CTSUSO1 (rw) register accessor: an alias for `Reg<CTSUSO1_SPEC>`"]
pub type CTSUSO1 = crate::Reg<ctsuso1::CTSUSO1_SPEC>;
#[doc = "CTSU Sensor Offset Register 1"]
pub mod ctsuso1;
#[doc = "CTSUSC (r) register accessor: an alias for `Reg<CTSUSC_SPEC>`"]
pub type CTSUSC = crate::Reg<ctsusc::CTSUSC_SPEC>;
#[doc = "CTSU Sensor Counter"]
pub mod ctsusc;
#[doc = "CTSURC (r) register accessor: an alias for `Reg<CTSURC_SPEC>`"]
pub type CTSURC = crate::Reg<ctsurc::CTSURC_SPEC>;
#[doc = "CTSU Reference Counter"]
pub mod ctsurc;
#[doc = "CTSUERRS (rw) register accessor: an alias for `Reg<CTSUERRS_SPEC>`"]
pub type CTSUERRS = crate::Reg<ctsuerrs::CTSUERRS_SPEC>;
#[doc = "CTSU Error Status Register"]
pub mod ctsuerrs;
#[doc = "CTSUTRMR (rw) register accessor: an alias for `Reg<CTSUTRMR_SPEC>`"]
pub type CTSUTRMR = crate::Reg<ctsutrmr::CTSUTRMR_SPEC>;
#[doc = "CTSU Reference Current Calibration Register"]
pub mod ctsutrmr;
