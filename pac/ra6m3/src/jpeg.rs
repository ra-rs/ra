#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG Code Mode Register"]
    pub jcmod: JCMOD,
    #[doc = "0x01 - JPEG Code Command Register"]
    pub jccmd: JCCMD,
    _reserved2: [u8; 0x01],
    #[doc = "0x03 - JPEG Code Quantization Table Number Register"]
    pub jcqtn: JCQTN,
    #[doc = "0x04 - JPEG Code Huffman Table Number Register"]
    pub jchtn: JCHTN,
    #[doc = "0x05 - JPEG Code DRI Upper Register"]
    pub jcdriu: JCDRIU,
    #[doc = "0x06 - JPEG Code DRI Lower Register"]
    pub jcdrid: JCDRID,
    #[doc = "0x07 - JPEG Code Vertical Size Upper Register"]
    pub jcvszu: JCVSZU,
    #[doc = "0x08 - JPEG Code Vertical Size Lower Register"]
    pub jcvszd: JCVSZD,
    #[doc = "0x09 - JPEG Code Horizontal Size Upper Register"]
    pub jchszu: JCHSZU,
    #[doc = "0x0a - JPEG Coded Horizontal Size Lower Register"]
    pub jchszd: JCHSZD,
    #[doc = "0x0b - JPEG Code Data Count Upper Register"]
    pub jcdtcu: JCDTCU,
    #[doc = "0x0c - JPEG Code Data Count Middle Register"]
    pub jcdtcm: JCDTCM,
    #[doc = "0x0d - JPEG Code Data Count Lower Register"]
    pub jcdtcd: JCDTCD,
    #[doc = "0x0e - JPEG Interrupt Enable Register 0"]
    pub jinte0: JINTE0,
    #[doc = "0x0f - JPEG Interrupt Status Register 0"]
    pub jints0: JINTS0,
    #[doc = "0x10 - JPEG Code Decode Error Register"]
    pub jcderr: JCDERR,
    #[doc = "0x11 - JPEG Code Reset Register"]
    pub jcrst: JCRST,
    _reserved17: [u8; 0x2e],
    #[doc = "0x40 - JPEG Interface Compression Control Register"]
    pub jifecnt: JIFECNT,
    #[doc = "0x44 - JPEG Interface Compression Source Address Register"]
    pub jifesa: JIFESA,
    #[doc = "0x48 - JPEG Interface Compression Line Offset Register"]
    pub jifesofst: JIFESOFST,
    #[doc = "0x4c - JPEG Interface Compression Destination Address Register"]
    pub jifeda: JIFEDA,
    #[doc = "0x50 - JPEG Interface Compression Source Line Count Register"]
    pub jifeslc: JIFESLC,
    _reserved22: [u8; 0x04],
    #[doc = "0x58 - JPEG Interface Decompression Control Register"]
    pub jifdcnt: JIFDCNT,
    #[doc = "0x5c - JPEG Interface Decompression Source Address Register"]
    pub jifdsa: JIFDSA,
    #[doc = "0x60 - JPEG Interface Decompression Line Offset Register"]
    pub jifddofst: JIFDDOFST,
    #[doc = "0x64 - JPEG Interface Decompression Destination Address Register"]
    pub jifdda: JIFDDA,
    #[doc = "0x68 - JPEG Interface Decompression Source Data Count Register"]
    pub jifdsdc: JIFDSDC,
    #[doc = "0x6c - JPEG Interface Decompression Destination Line Count Register"]
    pub jifddlc: JIFDDLC,
    #[doc = "0x70 - JPEG Interface Decompression alpha Set Register"]
    pub jifdadt: JIFDADT,
    _reserved29: [u8; 0x18],
    #[doc = "0x8c - JPEG Interrupt Enable Register 1"]
    pub jinte1: JINTE1,
    #[doc = "0x90 - JPEG Interrupt Status Register 1"]
    pub jints1: JINTS1,
}
#[doc = "JCMOD (rw) register accessor: an alias for `Reg<JCMOD_SPEC>`"]
pub type JCMOD = crate::Reg<jcmod::JCMOD_SPEC>;
#[doc = "JPEG Code Mode Register"]
pub mod jcmod;
#[doc = "JCCMD (w) register accessor: an alias for `Reg<JCCMD_SPEC>`"]
pub type JCCMD = crate::Reg<jccmd::JCCMD_SPEC>;
#[doc = "JPEG Code Command Register"]
pub mod jccmd;
#[doc = "JCQTN (rw) register accessor: an alias for `Reg<JCQTN_SPEC>`"]
pub type JCQTN = crate::Reg<jcqtn::JCQTN_SPEC>;
#[doc = "JPEG Code Quantization Table Number Register"]
pub mod jcqtn;
#[doc = "JCHTN (rw) register accessor: an alias for `Reg<JCHTN_SPEC>`"]
pub type JCHTN = crate::Reg<jchtn::JCHTN_SPEC>;
#[doc = "JPEG Code Huffman Table Number Register"]
pub mod jchtn;
#[doc = "JCDRIU (rw) register accessor: an alias for `Reg<JCDRIU_SPEC>`"]
pub type JCDRIU = crate::Reg<jcdriu::JCDRIU_SPEC>;
#[doc = "JPEG Code DRI Upper Register"]
pub mod jcdriu;
#[doc = "JCDRID (rw) register accessor: an alias for `Reg<JCDRID_SPEC>`"]
pub type JCDRID = crate::Reg<jcdrid::JCDRID_SPEC>;
#[doc = "JPEG Code DRI Lower Register"]
pub mod jcdrid;
#[doc = "JCVSZU (rw) register accessor: an alias for `Reg<JCVSZU_SPEC>`"]
pub type JCVSZU = crate::Reg<jcvszu::JCVSZU_SPEC>;
#[doc = "JPEG Code Vertical Size Upper Register"]
pub mod jcvszu;
#[doc = "JCVSZD (rw) register accessor: an alias for `Reg<JCVSZD_SPEC>`"]
pub type JCVSZD = crate::Reg<jcvszd::JCVSZD_SPEC>;
#[doc = "JPEG Code Vertical Size Lower Register"]
pub mod jcvszd;
#[doc = "JCHSZU (rw) register accessor: an alias for `Reg<JCHSZU_SPEC>`"]
pub type JCHSZU = crate::Reg<jchszu::JCHSZU_SPEC>;
#[doc = "JPEG Code Horizontal Size Upper Register"]
pub mod jchszu;
#[doc = "JCHSZD (rw) register accessor: an alias for `Reg<JCHSZD_SPEC>`"]
pub type JCHSZD = crate::Reg<jchszd::JCHSZD_SPEC>;
#[doc = "JPEG Coded Horizontal Size Lower Register"]
pub mod jchszd;
#[doc = "JCDTCU (r) register accessor: an alias for `Reg<JCDTCU_SPEC>`"]
pub type JCDTCU = crate::Reg<jcdtcu::JCDTCU_SPEC>;
#[doc = "JPEG Code Data Count Upper Register"]
pub mod jcdtcu;
#[doc = "JCDTCM (r) register accessor: an alias for `Reg<JCDTCM_SPEC>`"]
pub type JCDTCM = crate::Reg<jcdtcm::JCDTCM_SPEC>;
#[doc = "JPEG Code Data Count Middle Register"]
pub mod jcdtcm;
#[doc = "JCDTCD (r) register accessor: an alias for `Reg<JCDTCD_SPEC>`"]
pub type JCDTCD = crate::Reg<jcdtcd::JCDTCD_SPEC>;
#[doc = "JPEG Code Data Count Lower Register"]
pub mod jcdtcd;
#[doc = "JINTE0 (rw) register accessor: an alias for `Reg<JINTE0_SPEC>`"]
pub type JINTE0 = crate::Reg<jinte0::JINTE0_SPEC>;
#[doc = "JPEG Interrupt Enable Register 0"]
pub mod jinte0;
#[doc = "JINTS0 (rw) register accessor: an alias for `Reg<JINTS0_SPEC>`"]
pub type JINTS0 = crate::Reg<jints0::JINTS0_SPEC>;
#[doc = "JPEG Interrupt Status Register 0"]
pub mod jints0;
#[doc = "JCDERR (rw) register accessor: an alias for `Reg<JCDERR_SPEC>`"]
pub type JCDERR = crate::Reg<jcderr::JCDERR_SPEC>;
#[doc = "JPEG Code Decode Error Register"]
pub mod jcderr;
#[doc = "JCRST (r) register accessor: an alias for `Reg<JCRST_SPEC>`"]
pub type JCRST = crate::Reg<jcrst::JCRST_SPEC>;
#[doc = "JPEG Code Reset Register"]
pub mod jcrst;
#[doc = "JIFECNT (rw) register accessor: an alias for `Reg<JIFECNT_SPEC>`"]
pub type JIFECNT = crate::Reg<jifecnt::JIFECNT_SPEC>;
#[doc = "JPEG Interface Compression Control Register"]
pub mod jifecnt;
#[doc = "JIFESA (rw) register accessor: an alias for `Reg<JIFESA_SPEC>`"]
pub type JIFESA = crate::Reg<jifesa::JIFESA_SPEC>;
#[doc = "JPEG Interface Compression Source Address Register"]
pub mod jifesa;
#[doc = "JIFESOFST (rw) register accessor: an alias for `Reg<JIFESOFST_SPEC>`"]
pub type JIFESOFST = crate::Reg<jifesofst::JIFESOFST_SPEC>;
#[doc = "JPEG Interface Compression Line Offset Register"]
pub mod jifesofst;
#[doc = "JIFEDA (rw) register accessor: an alias for `Reg<JIFEDA_SPEC>`"]
pub type JIFEDA = crate::Reg<jifeda::JIFEDA_SPEC>;
#[doc = "JPEG Interface Compression Destination Address Register"]
pub mod jifeda;
#[doc = "JIFESLC (rw) register accessor: an alias for `Reg<JIFESLC_SPEC>`"]
pub type JIFESLC = crate::Reg<jifeslc::JIFESLC_SPEC>;
#[doc = "JPEG Interface Compression Source Line Count Register"]
pub mod jifeslc;
#[doc = "JIFDCNT (rw) register accessor: an alias for `Reg<JIFDCNT_SPEC>`"]
pub type JIFDCNT = crate::Reg<jifdcnt::JIFDCNT_SPEC>;
#[doc = "JPEG Interface Decompression Control Register"]
pub mod jifdcnt;
#[doc = "JIFDSA (rw) register accessor: an alias for `Reg<JIFDSA_SPEC>`"]
pub type JIFDSA = crate::Reg<jifdsa::JIFDSA_SPEC>;
#[doc = "JPEG Interface Decompression Source Address Register"]
pub mod jifdsa;
#[doc = "JIFDDOFST (rw) register accessor: an alias for `Reg<JIFDDOFST_SPEC>`"]
pub type JIFDDOFST = crate::Reg<jifddofst::JIFDDOFST_SPEC>;
#[doc = "JPEG Interface Decompression Line Offset Register"]
pub mod jifddofst;
#[doc = "JIFDDA (rw) register accessor: an alias for `Reg<JIFDDA_SPEC>`"]
pub type JIFDDA = crate::Reg<jifdda::JIFDDA_SPEC>;
#[doc = "JPEG Interface Decompression Destination Address Register"]
pub mod jifdda;
#[doc = "JIFDSDC (rw) register accessor: an alias for `Reg<JIFDSDC_SPEC>`"]
pub type JIFDSDC = crate::Reg<jifdsdc::JIFDSDC_SPEC>;
#[doc = "JPEG Interface Decompression Source Data Count Register"]
pub mod jifdsdc;
#[doc = "JIFDDLC (rw) register accessor: an alias for `Reg<JIFDDLC_SPEC>`"]
pub type JIFDDLC = crate::Reg<jifddlc::JIFDDLC_SPEC>;
#[doc = "JPEG Interface Decompression Destination Line Count Register"]
pub mod jifddlc;
#[doc = "JIFDADT (rw) register accessor: an alias for `Reg<JIFDADT_SPEC>`"]
pub type JIFDADT = crate::Reg<jifdadt::JIFDADT_SPEC>;
#[doc = "JPEG Interface Decompression alpha Set Register"]
pub mod jifdadt;
#[doc = "JINTE1 (rw) register accessor: an alias for `Reg<JINTE1_SPEC>`"]
pub type JINTE1 = crate::Reg<jinte1::JINTE1_SPEC>;
#[doc = "JPEG Interrupt Enable Register 1"]
pub mod jinte1;
#[doc = "JINTS1 (rw) register accessor: an alias for `Reg<JINTS1_SPEC>`"]
pub type JINTS1 = crate::Reg<jints1::JINTS1_SPEC>;
#[doc = "JPEG Interrupt Status Register 1"]
pub mod jints1;
