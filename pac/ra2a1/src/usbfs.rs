#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Configuration Control Register"]
    pub syscfg: SYSCFG,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - System Configuration Status Register 0"]
    pub syssts0: SYSSTS0,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - Device State Control Register 0"]
    pub dvstctr0: DVSTCTR0,
    _reserved3: [u8; 0x0a],
    _reserved_3_cfifo: [u8; 0x02],
    _reserved4: [u8; 0x0a],
    #[doc = "0x20 - CFIFO Port Select Register"]
    pub cfifosel: CFIFOSEL,
    #[doc = "0x22 - CFIFO Port Control Register"]
    pub cfifoctr: CFIFOCTR,
    _reserved6: [u8; 0x0c],
    #[doc = "0x30 - Interrupt Enable Register 0"]
    pub intenb0: INTENB0,
    _reserved7: [u8; 0x04],
    #[doc = "0x36 - BRDY Interrupt Enable Register"]
    pub brdyenb: BRDYENB,
    #[doc = "0x38 - NRDY Interrupt Enable Register"]
    pub nrdyenb: NRDYENB,
    #[doc = "0x3a - BEMP Interrupt Enable Register"]
    pub bempenb: BEMPENB,
    #[doc = "0x3c - SOF Output Configuration Register"]
    pub sofcfg: SOFCFG,
    _reserved11: [u8; 0x02],
    #[doc = "0x40 - Interrupt Status Register 0"]
    pub intsts0: INTSTS0,
    _reserved12: [u8; 0x04],
    #[doc = "0x46 - BRDY Interrupt Status Register"]
    pub brdysts: BRDYSTS,
    #[doc = "0x48 - NRDY Interrupt Status Register"]
    pub nrdysts: NRDYSTS,
    #[doc = "0x4a - BEMP Interrupt Status Register"]
    pub bempsts: BEMPSTS,
    #[doc = "0x4c - Frame Number Register"]
    pub frmnum: FRMNUM,
    _reserved16: [u8; 0x06],
    #[doc = "0x54 - USB Request Type Register"]
    pub usbreq: USBREQ,
    #[doc = "0x56 - USB Request Value Register"]
    pub usbval: USBVAL,
    #[doc = "0x58 - USB Request Index Register"]
    pub usbindx: USBINDX,
    #[doc = "0x5a - USB Request Length Register"]
    pub usbleng: USBLENG,
    #[doc = "0x5c - DCP Configuration Register"]
    pub dcpcfg: DCPCFG,
    #[doc = "0x5e - DCP Maximum Packet Size Register"]
    pub dcpmaxp: DCPMAXP,
    #[doc = "0x60 - DCP Control Register"]
    pub dcpctr: DCPCTR,
    _reserved23: [u8; 0x02],
    #[doc = "0x64 - Pipe Window Select Register"]
    pub pipesel: PIPESEL,
    _reserved24: [u8; 0x02],
    #[doc = "0x68 - Pipe Configuration Register"]
    pub pipecfg: PIPECFG,
    _reserved25: [u8; 0x02],
    #[doc = "0x6c - Pipe Maximum Packet Size Register"]
    pub pipemaxp: PIPEMAXP,
    _reserved26: [u8; 0x08],
    #[doc = "0x76 - Pipe %s Control Register"]
    pub pipectr: [PIPECTR; 2],
    #[doc = "0x7a - Pipe %s Control Register"]
    pub pipe6ctr: PIPE6CTR,
    #[doc = "0x7c - Pipe %s Control Register"]
    pub pipe7ctr: PIPE6CTR,
    _reserved29: [u8; 0x1e],
    #[doc = "0x9c - Pipe %s Transaction Counter Enable Register"]
    pub pipe4tre: PIPETRE,
    #[doc = "0x9e - Pipe %s Transaction Counter Register"]
    pub pipe4trn: PIPETRN,
    #[doc = "0xa0 - Pipe %s Transaction Counter Enable Register"]
    pub pipe5tre: PIPETRE,
    #[doc = "0xa2 - Pipe %s Transaction Counter Register"]
    pub pipe5trn: PIPETRN,
    _reserved33: [u8; 0x0c],
    #[doc = "0xb0 - BC Control Register 0"]
    pub usbbcctrl0: USBBCCTRL0,
    _reserved34: [u8; 0x12],
    #[doc = "0xc4 - USB Clock Selection Register"]
    pub ucksel: UCKSEL,
    _reserved35: [u8; 0x06],
    #[doc = "0xcc - USB Module Control Register"]
    pub usbmc: USBMC,
}
impl RegisterBlock {
    #[doc = "0x14 - CFIFO Port Register L"]
    #[inline(always)]
    pub const fn cfifol(&self) -> &CFIFOL {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - CFIFO Port Register"]
    #[inline(always)]
    pub const fn cfifo(&self) -> &CFIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x76 - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipe4ctr(&self) -> &PIPECTR {
        &self.pipectr[0]
    }
    #[doc = "0x78 - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipe5ctr(&self) -> &PIPECTR {
        &self.pipectr[1]
    }
}
#[doc = "SYSCFG (rw) register accessor: an alias for `Reg<SYSCFG_SPEC>`"]
pub type SYSCFG = crate::Reg<syscfg::SYSCFG_SPEC>;
#[doc = "System Configuration Control Register"]
pub mod syscfg;
#[doc = "SYSSTS0 (r) register accessor: an alias for `Reg<SYSSTS0_SPEC>`"]
pub type SYSSTS0 = crate::Reg<syssts0::SYSSTS0_SPEC>;
#[doc = "System Configuration Status Register 0"]
pub mod syssts0;
#[doc = "DVSTCTR0 (rw) register accessor: an alias for `Reg<DVSTCTR0_SPEC>`"]
pub type DVSTCTR0 = crate::Reg<dvstctr0::DVSTCTR0_SPEC>;
#[doc = "Device State Control Register 0"]
pub mod dvstctr0;
#[doc = "CFIFO (rw) register accessor: an alias for `Reg<CFIFO_SPEC>`"]
pub type CFIFO = crate::Reg<cfifo::CFIFO_SPEC>;
#[doc = "CFIFO Port Register"]
pub mod cfifo;
#[doc = "CFIFOL (rw) register accessor: an alias for `Reg<CFIFOL_SPEC>`"]
pub type CFIFOL = crate::Reg<cfifol::CFIFOL_SPEC>;
#[doc = "CFIFO Port Register L"]
pub mod cfifol;
#[doc = "CFIFOSEL (rw) register accessor: an alias for `Reg<CFIFOSEL_SPEC>`"]
pub type CFIFOSEL = crate::Reg<cfifosel::CFIFOSEL_SPEC>;
#[doc = "CFIFO Port Select Register"]
pub mod cfifosel;
#[doc = "CFIFOCTR (rw) register accessor: an alias for `Reg<CFIFOCTR_SPEC>`"]
pub type CFIFOCTR = crate::Reg<cfifoctr::CFIFOCTR_SPEC>;
#[doc = "CFIFO Port Control Register"]
pub mod cfifoctr;
#[doc = "INTENB0 (rw) register accessor: an alias for `Reg<INTENB0_SPEC>`"]
pub type INTENB0 = crate::Reg<intenb0::INTENB0_SPEC>;
#[doc = "Interrupt Enable Register 0"]
pub mod intenb0;
#[doc = "BRDYENB (rw) register accessor: an alias for `Reg<BRDYENB_SPEC>`"]
pub type BRDYENB = crate::Reg<brdyenb::BRDYENB_SPEC>;
#[doc = "BRDY Interrupt Enable Register"]
pub mod brdyenb;
#[doc = "NRDYENB (rw) register accessor: an alias for `Reg<NRDYENB_SPEC>`"]
pub type NRDYENB = crate::Reg<nrdyenb::NRDYENB_SPEC>;
#[doc = "NRDY Interrupt Enable Register"]
pub mod nrdyenb;
#[doc = "BEMPENB (rw) register accessor: an alias for `Reg<BEMPENB_SPEC>`"]
pub type BEMPENB = crate::Reg<bempenb::BEMPENB_SPEC>;
#[doc = "BEMP Interrupt Enable Register"]
pub mod bempenb;
#[doc = "SOFCFG (rw) register accessor: an alias for `Reg<SOFCFG_SPEC>`"]
pub type SOFCFG = crate::Reg<sofcfg::SOFCFG_SPEC>;
#[doc = "SOF Output Configuration Register"]
pub mod sofcfg;
#[doc = "INTSTS0 (rw) register accessor: an alias for `Reg<INTSTS0_SPEC>`"]
pub type INTSTS0 = crate::Reg<intsts0::INTSTS0_SPEC>;
#[doc = "Interrupt Status Register 0"]
pub mod intsts0;
#[doc = "BRDYSTS (rw) register accessor: an alias for `Reg<BRDYSTS_SPEC>`"]
pub type BRDYSTS = crate::Reg<brdysts::BRDYSTS_SPEC>;
#[doc = "BRDY Interrupt Status Register"]
pub mod brdysts;
#[doc = "NRDYSTS (rw) register accessor: an alias for `Reg<NRDYSTS_SPEC>`"]
pub type NRDYSTS = crate::Reg<nrdysts::NRDYSTS_SPEC>;
#[doc = "NRDY Interrupt Status Register"]
pub mod nrdysts;
#[doc = "BEMPSTS (rw) register accessor: an alias for `Reg<BEMPSTS_SPEC>`"]
pub type BEMPSTS = crate::Reg<bempsts::BEMPSTS_SPEC>;
#[doc = "BEMP Interrupt Status Register"]
pub mod bempsts;
#[doc = "FRMNUM (rw) register accessor: an alias for `Reg<FRMNUM_SPEC>`"]
pub type FRMNUM = crate::Reg<frmnum::FRMNUM_SPEC>;
#[doc = "Frame Number Register"]
pub mod frmnum;
#[doc = "USBREQ (r) register accessor: an alias for `Reg<USBREQ_SPEC>`"]
pub type USBREQ = crate::Reg<usbreq::USBREQ_SPEC>;
#[doc = "USB Request Type Register"]
pub mod usbreq;
#[doc = "USBVAL (r) register accessor: an alias for `Reg<USBVAL_SPEC>`"]
pub type USBVAL = crate::Reg<usbval::USBVAL_SPEC>;
#[doc = "USB Request Value Register"]
pub mod usbval;
#[doc = "USBINDX (r) register accessor: an alias for `Reg<USBINDX_SPEC>`"]
pub type USBINDX = crate::Reg<usbindx::USBINDX_SPEC>;
#[doc = "USB Request Index Register"]
pub mod usbindx;
#[doc = "USBLENG (r) register accessor: an alias for `Reg<USBLENG_SPEC>`"]
pub type USBLENG = crate::Reg<usbleng::USBLENG_SPEC>;
#[doc = "USB Request Length Register"]
pub mod usbleng;
#[doc = "DCPCFG (rw) register accessor: an alias for `Reg<DCPCFG_SPEC>`"]
pub type DCPCFG = crate::Reg<dcpcfg::DCPCFG_SPEC>;
#[doc = "DCP Configuration Register"]
pub mod dcpcfg;
#[doc = "DCPMAXP (rw) register accessor: an alias for `Reg<DCPMAXP_SPEC>`"]
pub type DCPMAXP = crate::Reg<dcpmaxp::DCPMAXP_SPEC>;
#[doc = "DCP Maximum Packet Size Register"]
pub mod dcpmaxp;
#[doc = "DCPCTR (rw) register accessor: an alias for `Reg<DCPCTR_SPEC>`"]
pub type DCPCTR = crate::Reg<dcpctr::DCPCTR_SPEC>;
#[doc = "DCP Control Register"]
pub mod dcpctr;
#[doc = "PIPESEL (rw) register accessor: an alias for `Reg<PIPESEL_SPEC>`"]
pub type PIPESEL = crate::Reg<pipesel::PIPESEL_SPEC>;
#[doc = "Pipe Window Select Register"]
pub mod pipesel;
#[doc = "PIPECFG (rw) register accessor: an alias for `Reg<PIPECFG_SPEC>`"]
pub type PIPECFG = crate::Reg<pipecfg::PIPECFG_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod pipecfg;
#[doc = "PIPEMAXP (rw) register accessor: an alias for `Reg<PIPEMAXP_SPEC>`"]
pub type PIPEMAXP = crate::Reg<pipemaxp::PIPEMAXP_SPEC>;
#[doc = "Pipe Maximum Packet Size Register"]
pub mod pipemaxp;
#[doc = "PIPECTR (rw) register accessor: an alias for `Reg<PIPECTR_SPEC>`"]
pub type PIPECTR = crate::Reg<pipectr::PIPECTR_SPEC>;
#[doc = "Pipe %s Control Register"]
pub mod pipectr;
pub use pipectr as pipe6ctr;
pub use PIPECTR as PIPE6CTR;
#[doc = "PIPETRE (rw) register accessor: an alias for `Reg<PIPETRE_SPEC>`"]
pub type PIPETRE = crate::Reg<pipetre::PIPETRE_SPEC>;
#[doc = "Pipe %s Transaction Counter Enable Register"]
pub mod pipetre;
#[doc = "PIPETRN (rw) register accessor: an alias for `Reg<PIPETRN_SPEC>`"]
pub type PIPETRN = crate::Reg<pipetrn::PIPETRN_SPEC>;
#[doc = "Pipe %s Transaction Counter Register"]
pub mod pipetrn;
#[doc = "USBMC (rw) register accessor: an alias for `Reg<USBMC_SPEC>`"]
pub type USBMC = crate::Reg<usbmc::USBMC_SPEC>;
#[doc = "USB Module Control Register"]
pub mod usbmc;
#[doc = "USBBCCTRL0 (rw) register accessor: an alias for `Reg<USBBCCTRL0_SPEC>`"]
pub type USBBCCTRL0 = crate::Reg<usbbcctrl0::USBBCCTRL0_SPEC>;
#[doc = "BC Control Register 0"]
pub mod usbbcctrl0;
#[doc = "UCKSEL (rw) register accessor: an alias for `Reg<UCKSEL_SPEC>`"]
pub type UCKSEL = crate::Reg<ucksel::UCKSEL_SPEC>;
#[doc = "USB Clock Selection Register"]
pub mod ucksel;
