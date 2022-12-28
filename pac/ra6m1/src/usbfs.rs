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
    _reserved4: [u8; 0x02],
    _reserved_4_d: [u8; 0x02],
    _reserved5: [u8; 0x02],
    _reserved_5_d: [u8; 0x02],
    _reserved6: [u8; 0x02],
    #[doc = "0x20 - CFIFO Port Select Register"]
    pub cfifosel: CFIFOSEL,
    #[doc = "0x22 - CFIFO Port Control Register"]
    pub cfifoctr: CFIFOCTR,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - D0FIFO Port Select Register"]
    pub d0fifosel: D0FIFOSEL,
    #[doc = "0x2a - D0FIFO Port Control Register"]
    pub d0fifoctr: D0FIFOCTR,
    #[doc = "0x2c - D1FIFO Port Select Register"]
    pub d1fifosel: D1FIFOSEL,
    #[doc = "0x2e - D1FIFO Port Control Register"]
    pub d1fifoctr: D1FIFOCTR,
    #[doc = "0x30 - Interrupt Enable Register 0"]
    pub intenb0: INTENB0,
    #[doc = "0x32 - Interrupt Enable Register 1"]
    pub intenb1: INTENB1,
    _reserved14: [u8; 0x02],
    #[doc = "0x36 - BRDY Interrupt Enable Register"]
    pub brdyenb: BRDYENB,
    #[doc = "0x38 - NRDY Interrupt Enable Register"]
    pub nrdyenb: NRDYENB,
    #[doc = "0x3a - BEMP Interrupt Enable Register"]
    pub bempenb: BEMPENB,
    #[doc = "0x3c - SOF Output Configuration Register"]
    pub sofcfg: SOFCFG,
    _reserved18: [u8; 0x02],
    #[doc = "0x40 - Interrupt Status Register 0"]
    pub intsts0: INTSTS0,
    #[doc = "0x42 - Interrupt Status Register 1"]
    pub intsts1: INTSTS1,
    _reserved20: [u8; 0x02],
    #[doc = "0x46 - BRDY Interrupt Status Register"]
    pub brdysts: BRDYSTS,
    #[doc = "0x48 - NRDY Interrupt Status Register"]
    pub nrdysts: NRDYSTS,
    #[doc = "0x4a - BEMP Interrupt Status Register"]
    pub bempsts: BEMPSTS,
    #[doc = "0x4c - Frame Number Register"]
    pub frmnum: FRMNUM,
    #[doc = "0x4e - Device State Change Register"]
    pub dvchgr: DVCHGR,
    #[doc = "0x50 - USB Address Register"]
    pub usbaddr: USBADDR,
    _reserved26: [u8; 0x02],
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
    _reserved33: [u8; 0x02],
    #[doc = "0x64 - Pipe Window Select Register"]
    pub pipesel: PIPESEL,
    _reserved34: [u8; 0x02],
    #[doc = "0x68 - Pipe Configuration Register"]
    pub pipecfg: PIPECFG,
    _reserved35: [u8; 0x02],
    #[doc = "0x6c - Pipe Maximum Packet Size Register"]
    pub pipemaxp: PIPEMAXP,
    #[doc = "0x6e - Pipe Cycle Control Register"]
    pub pipeperi: PIPEPERI,
    #[doc = "0x70..0x7a - Pipe %s Control Register"]
    pub pipectr: [PIPECTR; 5],
    #[doc = "0x7a - Pipe %s Control Register"]
    pub pipe6ctr: PIPE6CTR,
    #[doc = "0x7c - Pipe %s Control Register"]
    pub pipe7ctr: PIPE6CTR,
    #[doc = "0x7e - Pipe %s Control Register"]
    pub pipe8ctr: PIPE6CTR,
    #[doc = "0x80 - Pipe %s Control Register"]
    pub pipe9ctr: PIPE6CTR,
    _reserved42: [u8; 0x0e],
    #[doc = "0x90 - Pipe %s Transaction Counter Enable Register"]
    pub pipe1tre: PIPETRE,
    #[doc = "0x92 - Pipe %s Transaction Counter Register"]
    pub pipe1trn: PIPETRN,
    #[doc = "0x94 - Pipe %s Transaction Counter Enable Register"]
    pub pipe2tre: PIPETRE,
    #[doc = "0x96 - Pipe %s Transaction Counter Register"]
    pub pipe2trn: PIPETRN,
    #[doc = "0x98 - Pipe %s Transaction Counter Enable Register"]
    pub pipe3tre: PIPETRE,
    #[doc = "0x9a - Pipe %s Transaction Counter Register"]
    pub pipe3trn: PIPETRN,
    #[doc = "0x9c - Pipe %s Transaction Counter Enable Register"]
    pub pipe4tre: PIPETRE,
    #[doc = "0x9e - Pipe %s Transaction Counter Register"]
    pub pipe4trn: PIPETRN,
    #[doc = "0xa0 - Pipe %s Transaction Counter Enable Register"]
    pub pipe5tre: PIPETRE,
    #[doc = "0xa2 - Pipe %s Transaction Counter Register"]
    pub pipe5trn: PIPETRN,
    _reserved52: [u8; 0x2c],
    #[doc = "0xd0..0xdc - Device Address %s Configuration Register"]
    pub devadd: [DEVADD; 6],
    _reserved53: [u8; 0x14],
    #[doc = "0xf0 - PHY Cross Point Adjustment Register"]
    pub physlew: PHYSLEW,
    _reserved54: [u8; 0x030c],
    #[doc = "0x400 - Deep Software Standby USB Transceiver Control/Pin Monitor Register"]
    pub dpusr0r: DPUSR0R,
    #[doc = "0x404 - Deep Software Standby USB Suspend/Resume Interrupt Register"]
    pub dpusr1r: DPUSR1R,
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
    #[doc = "0x18 - D0FIFO Port Register L"]
    #[inline(always)]
    pub const fn d0fifol(&self) -> &D0FIFOL {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - D0FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifo(&self) -> &D0FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - D1FIFO Port Register L"]
    #[inline(always)]
    pub const fn d1fifol(&self) -> &D1FIFOL {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - D1FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifo(&self) -> &D1FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x70 - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipe1ctr(&self) -> &PIPECTR {
        &self.pipectr[0]
    }
    #[doc = "0x72 - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipe2ctr(&self) -> &PIPECTR {
        &self.pipectr[1]
    }
    #[doc = "0x74 - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipe3ctr(&self) -> &PIPECTR {
        &self.pipectr[2]
    }
    #[doc = "0x76 - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipe4ctr(&self) -> &PIPECTR {
        &self.pipectr[3]
    }
    #[doc = "0x78 - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipe5ctr(&self) -> &PIPECTR {
        &self.pipectr[4]
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
#[doc = "D0FIFO (rw) register accessor: an alias for `Reg<D0FIFO_SPEC>`"]
pub type D0FIFO = crate::Reg<d0fifo::D0FIFO_SPEC>;
#[doc = "D0FIFO Port Register"]
pub mod d0fifo;
#[doc = "D0FIFOL (rw) register accessor: an alias for `Reg<D0FIFOL_SPEC>`"]
pub type D0FIFOL = crate::Reg<d0fifol::D0FIFOL_SPEC>;
#[doc = "D0FIFO Port Register L"]
pub mod d0fifol;
#[doc = "D1FIFO (rw) register accessor: an alias for `Reg<D1FIFO_SPEC>`"]
pub type D1FIFO = crate::Reg<d1fifo::D1FIFO_SPEC>;
#[doc = "D1FIFO Port Register"]
pub mod d1fifo;
#[doc = "D1FIFOL (rw) register accessor: an alias for `Reg<D1FIFOL_SPEC>`"]
pub type D1FIFOL = crate::Reg<d1fifol::D1FIFOL_SPEC>;
#[doc = "D1FIFO Port Register L"]
pub mod d1fifol;
#[doc = "CFIFOSEL (rw) register accessor: an alias for `Reg<CFIFOSEL_SPEC>`"]
pub type CFIFOSEL = crate::Reg<cfifosel::CFIFOSEL_SPEC>;
#[doc = "CFIFO Port Select Register"]
pub mod cfifosel;
#[doc = "CFIFOCTR (rw) register accessor: an alias for `Reg<CFIFOCTR_SPEC>`"]
pub type CFIFOCTR = crate::Reg<cfifoctr::CFIFOCTR_SPEC>;
#[doc = "CFIFO Port Control Register"]
pub mod cfifoctr;
#[doc = "D0FIFOSEL (rw) register accessor: an alias for `Reg<D0FIFOSEL_SPEC>`"]
pub type D0FIFOSEL = crate::Reg<d0fifosel::D0FIFOSEL_SPEC>;
#[doc = "D0FIFO Port Select Register"]
pub mod d0fifosel;
#[doc = "D0FIFOCTR (rw) register accessor: an alias for `Reg<D0FIFOCTR_SPEC>`"]
pub type D0FIFOCTR = crate::Reg<d0fifoctr::D0FIFOCTR_SPEC>;
#[doc = "D0FIFO Port Control Register"]
pub mod d0fifoctr;
#[doc = "D1FIFOSEL (rw) register accessor: an alias for `Reg<D1FIFOSEL_SPEC>`"]
pub type D1FIFOSEL = crate::Reg<d1fifosel::D1FIFOSEL_SPEC>;
#[doc = "D1FIFO Port Select Register"]
pub mod d1fifosel;
#[doc = "D1FIFOCTR (rw) register accessor: an alias for `Reg<D1FIFOCTR_SPEC>`"]
pub type D1FIFOCTR = crate::Reg<d1fifoctr::D1FIFOCTR_SPEC>;
#[doc = "D1FIFO Port Control Register"]
pub mod d1fifoctr;
#[doc = "INTENB0 (rw) register accessor: an alias for `Reg<INTENB0_SPEC>`"]
pub type INTENB0 = crate::Reg<intenb0::INTENB0_SPEC>;
#[doc = "Interrupt Enable Register 0"]
pub mod intenb0;
#[doc = "INTENB1 (rw) register accessor: an alias for `Reg<INTENB1_SPEC>`"]
pub type INTENB1 = crate::Reg<intenb1::INTENB1_SPEC>;
#[doc = "Interrupt Enable Register 1"]
pub mod intenb1;
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
#[doc = "INTSTS1 (rw) register accessor: an alias for `Reg<INTSTS1_SPEC>`"]
pub type INTSTS1 = crate::Reg<intsts1::INTSTS1_SPEC>;
#[doc = "Interrupt Status Register 1"]
pub mod intsts1;
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
#[doc = "DVCHGR (rw) register accessor: an alias for `Reg<DVCHGR_SPEC>`"]
pub type DVCHGR = crate::Reg<dvchgr::DVCHGR_SPEC>;
#[doc = "Device State Change Register"]
pub mod dvchgr;
#[doc = "USBADDR (rw) register accessor: an alias for `Reg<USBADDR_SPEC>`"]
pub type USBADDR = crate::Reg<usbaddr::USBADDR_SPEC>;
#[doc = "USB Address Register"]
pub mod usbaddr;
#[doc = "USBREQ (rw) register accessor: an alias for `Reg<USBREQ_SPEC>`"]
pub type USBREQ = crate::Reg<usbreq::USBREQ_SPEC>;
#[doc = "USB Request Type Register"]
pub mod usbreq;
#[doc = "USBVAL (rw) register accessor: an alias for `Reg<USBVAL_SPEC>`"]
pub type USBVAL = crate::Reg<usbval::USBVAL_SPEC>;
#[doc = "USB Request Value Register"]
pub mod usbval;
#[doc = "USBINDX (rw) register accessor: an alias for `Reg<USBINDX_SPEC>`"]
pub type USBINDX = crate::Reg<usbindx::USBINDX_SPEC>;
#[doc = "USB Request Index Register"]
pub mod usbindx;
#[doc = "USBLENG (rw) register accessor: an alias for `Reg<USBLENG_SPEC>`"]
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
#[doc = "PIPEPERI (rw) register accessor: an alias for `Reg<PIPEPERI_SPEC>`"]
pub type PIPEPERI = crate::Reg<pipeperi::PIPEPERI_SPEC>;
#[doc = "Pipe Cycle Control Register"]
pub mod pipeperi;
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
#[doc = "DEVADD (rw) register accessor: an alias for `Reg<DEVADD_SPEC>`"]
pub type DEVADD = crate::Reg<devadd::DEVADD_SPEC>;
#[doc = "Device Address %s Configuration Register"]
pub mod devadd;
#[doc = "PHYSLEW (rw) register accessor: an alias for `Reg<PHYSLEW_SPEC>`"]
pub type PHYSLEW = crate::Reg<physlew::PHYSLEW_SPEC>;
#[doc = "PHY Cross Point Adjustment Register"]
pub mod physlew;
#[doc = "DPUSR0R (rw) register accessor: an alias for `Reg<DPUSR0R_SPEC>`"]
pub type DPUSR0R = crate::Reg<dpusr0r::DPUSR0R_SPEC>;
#[doc = "Deep Software Standby USB Transceiver Control/Pin Monitor Register"]
pub mod dpusr0r;
#[doc = "DPUSR1R (rw) register accessor: an alias for `Reg<DPUSR1R_SPEC>`"]
pub type DPUSR1R = crate::Reg<dpusr1r::DPUSR1R_SPEC>;
#[doc = "Deep Software Standby USB Suspend/Resume Interrupt Register"]
pub mod dpusr1r;
