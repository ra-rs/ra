#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Configuration Control Register"]
    pub syscfg: SYSCFG,
    #[doc = "0x02 - CPU Bus Wait Register"]
    pub buswait: BUSWAIT,
    #[doc = "0x04 - System Configuration Status Register"]
    pub syssts0: SYSSTS0,
    #[doc = "0x06 - PLL Status Register"]
    pub pllsta: PLLSTA,
    #[doc = "0x08 - Device State Control Register 0"]
    pub dvstctr0: DVSTCTR0,
    _reserved5: [u8; 0x02],
    #[doc = "0x0c - USB Test Mode Register"]
    pub testmode: TESTMODE,
    _reserved6: [u8; 0x06],
    _reserved_6_cfifo: [u8; 0x04],
    _reserved_7_d: [u8; 0x04],
    _reserved_8_d: [u8; 0x04],
    #[doc = "0x20 - CFIFO Port Select Register"]
    pub cfifosel: CFIFOSEL,
    #[doc = "0x22 - CFIFO Port Control Register"]
    pub cfifoctr: CFIFOCTR,
    _reserved11: [u8; 0x04],
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
    _reserved17: [u8; 0x02],
    #[doc = "0x36 - BRDY Interrupt Enable Register"]
    pub brdyenb: BRDYENB,
    #[doc = "0x38 - NRDY Interrupt Enable Register"]
    pub nrdyenb: NRDYENB,
    #[doc = "0x3a - BEMP Interrupt Enable Register"]
    pub bempenb: BEMPENB,
    #[doc = "0x3c - SOF Pin Configuration Register"]
    pub sofcfg: SOFCFG,
    #[doc = "0x3e - PHY Setting Register"]
    pub physet: PHYSET,
    #[doc = "0x40 - Interrupt Status Register 0"]
    pub intsts0: INTSTS0,
    #[doc = "0x42 - Interrupt Status Register 1"]
    pub intsts1: INTSTS1,
    _reserved24: [u8; 0x02],
    #[doc = "0x46 - BRDY Interrupt Status Register"]
    pub brdysts: BRDYSTS,
    #[doc = "0x48 - NRDY Interrupt Status Register"]
    pub nrdysts: NRDYSTS,
    #[doc = "0x4a - BEMP Interrupt Status Register"]
    pub bempsts: BEMPSTS,
    #[doc = "0x4c - Frame Number Register"]
    pub frmnum: FRMNUM,
    #[doc = "0x4e - uFrame Number Register"]
    pub ufrmnum: UFRMNUM,
    #[doc = "0x50 - USB Address Register"]
    pub usbaddr: USBADDR,
    _reserved30: [u8; 0x02],
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
    _reserved37: [u8; 0x02],
    #[doc = "0x64 - Pipe Window Select Register"]
    pub pipesel: PIPESEL,
    _reserved38: [u8; 0x02],
    #[doc = "0x68 - Pipe Configuration Register"]
    pub pipecfg: PIPECFG,
    #[doc = "0x6a - Pipe Buffer Register"]
    pub pipebuf: PIPEBUF,
    #[doc = "0x6c - Pipe Maximum Packet Size Register"]
    pub pipemaxp: PIPEMAXP,
    #[doc = "0x6e - Pipe Cycle Control Register"]
    pub pipeperi: PIPEPERI,
    #[doc = "0x70..0x82 - PIPE Control Register"]
    pub pipectr: [PIPECTR; 9],
    _reserved43: [u8; 0x0e],
    #[doc = "0x90 - PIPE Transaction Counter Enable Register"]
    pub pipe1tre: PIPETRE,
    #[doc = "0x92 - PIPE Transaction Counter Register"]
    pub pipe1trn: PIPETRN,
    #[doc = "0x94 - PIPE Transaction Counter Enable Register"]
    pub pipe2tre: PIPETRE,
    #[doc = "0x96 - PIPE Transaction Counter Register"]
    pub pipe2trn: PIPETRN,
    #[doc = "0x98 - PIPE Transaction Counter Enable Register"]
    pub pipe3tre: PIPETRE,
    #[doc = "0x9a - PIPE Transaction Counter Register"]
    pub pipe3trn: PIPETRN,
    #[doc = "0x9c - PIPE Transaction Counter Enable Register"]
    pub pipe4tre: PIPETRE,
    #[doc = "0x9e - PIPE Transaction Counter Register"]
    pub pipe4trn: PIPETRN,
    #[doc = "0xa0 - PIPE Transaction Counter Enable Register"]
    pub pipe5tre: PIPETRE,
    #[doc = "0xa2 - PIPE Transaction Counter Register"]
    pub pipe5trn: PIPETRN,
    _reserved53: [u8; 0x2c],
    #[doc = "0xd0..0xe4 - Device Address Configuration Register"]
    pub devadd: [DEVADD; 10],
    #[doc = "0xe4 - Device Address Configuration Register A"]
    pub devadda: DEVADDA,
    _reserved55: [u8; 0x1a],
    #[doc = "0x100 - Low Power Control Register"]
    pub lpctrl: LPCTRL,
    #[doc = "0x102 - Low Power Status Register"]
    pub lpsts: LPSTS,
    _reserved57: [u8; 0x3c],
    #[doc = "0x140 - Battery Charging Control Register"]
    pub bcctrl: BCCTRL,
    _reserved58: [u8; 0x02],
    #[doc = "0x144 - Function L1 Control Register 1"]
    pub pl1ctrl1: PL1CTRL1,
    #[doc = "0x146 - Function L1 Control Register 2"]
    pub pl1ctrl2: PL1CTRL2,
    #[doc = "0x148 - Host L1 Control Register 1"]
    pub hl1ctrl1: HL1CTRL1,
    #[doc = "0x14a - Host L1 Control Register 2"]
    pub hl1ctrl2: HL1CTRL2,
    _reserved62: [u8; 0x14],
    #[doc = "0x160 - Deep Standby USB Transceiver Control/Pin Monitor Register"]
    pub dpusr0r: DPUSR0R,
    #[doc = "0x164 - Deep Standby USB Suspend/Resume Interrupt Register"]
    pub dpusr1r: DPUSR1R,
    #[doc = "0x168 - Deep Standby USB Suspend/Resume Interrupt Register"]
    pub dpusr2r: DPUSR2R,
    #[doc = "0x16a - Deep Standby USB Suspend/Resume Command Register"]
    pub dpusrcr: DPUSRCR,
}
impl RegisterBlock {
    #[doc = "0x14 - CFIFO Port Register LL"]
    #[inline(always)]
    pub const fn cfifoll(&self) -> &CFIFOLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
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
    #[doc = "0x16 - CFIFO Port Register H"]
    #[inline(always)]
    pub const fn cfifoh(&self) -> &CFIFOH {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x17 - CFIFO Port Register HH"]
    #[inline(always)]
    pub const fn cfifohh(&self) -> &CFIFOHH {
        unsafe { &*(self as *const Self).cast::<u8>().add(23usize).cast() }
    }
    #[doc = "0x18 - D0FIFO Port Register LL"]
    #[inline(always)]
    pub const fn d0fifoll(&self) -> &D0FIFOLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
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
    #[doc = "0x1a - D0FIFO Port Register H"]
    #[inline(always)]
    pub const fn d0fifoh(&self) -> &D0FIFOH {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1b - D0FIFO Port Register HH"]
    #[inline(always)]
    pub const fn d0fifohh(&self) -> &D0FIFOHH {
        unsafe { &*(self as *const Self).cast::<u8>().add(27usize).cast() }
    }
    #[doc = "0x1c - D1FIFO Port Register LL"]
    #[inline(always)]
    pub const fn d1fifoll(&self) -> &D1FIFOLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
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
    #[doc = "0x1e - D1FIFO Port Register H"]
    #[inline(always)]
    pub const fn d1fifoh(&self) -> &D1FIFOH {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x1f - D1FIFO Port Register HH"]
    #[inline(always)]
    pub const fn d1fifohh(&self) -> &D1FIFOHH {
        unsafe { &*(self as *const Self).cast::<u8>().add(31usize).cast() }
    }
    #[doc = "0x70 - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe1ctr(&self) -> &PIPECTR {
        &self.pipectr[0]
    }
    #[doc = "0x72 - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe2ctr(&self) -> &PIPECTR {
        &self.pipectr[1]
    }
    #[doc = "0x74 - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe3ctr(&self) -> &PIPECTR {
        &self.pipectr[2]
    }
    #[doc = "0x76 - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe4ctr(&self) -> &PIPECTR {
        &self.pipectr[3]
    }
    #[doc = "0x78 - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe5ctr(&self) -> &PIPECTR {
        &self.pipectr[4]
    }
    #[doc = "0x7a - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe6ctr(&self) -> &PIPECTR {
        &self.pipectr[5]
    }
    #[doc = "0x7c - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe7ctr(&self) -> &PIPECTR {
        &self.pipectr[6]
    }
    #[doc = "0x7e - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe8ctr(&self) -> &PIPECTR {
        &self.pipectr[7]
    }
    #[doc = "0x80 - PIPE Control Register"]
    #[inline(always)]
    pub fn pipe9ctr(&self) -> &PIPECTR {
        &self.pipectr[8]
    }
}
#[doc = "SYSCFG (rw) register accessor: an alias for `Reg<SYSCFG_SPEC>`"]
pub type SYSCFG = crate::Reg<syscfg::SYSCFG_SPEC>;
#[doc = "System Configuration Control Register"]
pub mod syscfg;
#[doc = "BUSWAIT (rw) register accessor: an alias for `Reg<BUSWAIT_SPEC>`"]
pub type BUSWAIT = crate::Reg<buswait::BUSWAIT_SPEC>;
#[doc = "CPU Bus Wait Register"]
pub mod buswait;
#[doc = "SYSSTS0 (r) register accessor: an alias for `Reg<SYSSTS0_SPEC>`"]
pub type SYSSTS0 = crate::Reg<syssts0::SYSSTS0_SPEC>;
#[doc = "System Configuration Status Register"]
pub mod syssts0;
#[doc = "PLLSTA (r) register accessor: an alias for `Reg<PLLSTA_SPEC>`"]
pub type PLLSTA = crate::Reg<pllsta::PLLSTA_SPEC>;
#[doc = "PLL Status Register"]
pub mod pllsta;
#[doc = "DVSTCTR0 (rw) register accessor: an alias for `Reg<DVSTCTR0_SPEC>`"]
pub type DVSTCTR0 = crate::Reg<dvstctr0::DVSTCTR0_SPEC>;
#[doc = "Device State Control Register 0"]
pub mod dvstctr0;
#[doc = "TESTMODE (rw) register accessor: an alias for `Reg<TESTMODE_SPEC>`"]
pub type TESTMODE = crate::Reg<testmode::TESTMODE_SPEC>;
#[doc = "USB Test Mode Register"]
pub mod testmode;
#[doc = "CFIFO (rw) register accessor: an alias for `Reg<CFIFO_SPEC>`"]
pub type CFIFO = crate::Reg<cfifo::CFIFO_SPEC>;
#[doc = "CFIFO Port Register"]
pub mod cfifo;
#[doc = "CFIFOL (rw) register accessor: an alias for `Reg<CFIFOL_SPEC>`"]
pub type CFIFOL = crate::Reg<cfifol::CFIFOL_SPEC>;
#[doc = "CFIFO Port Register L"]
pub mod cfifol;
#[doc = "CFIFOH (rw) register accessor: an alias for `Reg<CFIFOH_SPEC>`"]
pub type CFIFOH = crate::Reg<cfifoh::CFIFOH_SPEC>;
#[doc = "CFIFO Port Register H"]
pub mod cfifoh;
#[doc = "CFIFOLL (rw) register accessor: an alias for `Reg<CFIFOLL_SPEC>`"]
pub type CFIFOLL = crate::Reg<cfifoll::CFIFOLL_SPEC>;
#[doc = "CFIFO Port Register LL"]
pub mod cfifoll;
#[doc = "CFIFOHH (rw) register accessor: an alias for `Reg<CFIFOHH_SPEC>`"]
pub type CFIFOHH = crate::Reg<cfifohh::CFIFOHH_SPEC>;
#[doc = "CFIFO Port Register HH"]
pub mod cfifohh;
#[doc = "D0FIFO (rw) register accessor: an alias for `Reg<D0FIFO_SPEC>`"]
pub type D0FIFO = crate::Reg<d0fifo::D0FIFO_SPEC>;
#[doc = "D0FIFO Port Register"]
pub mod d0fifo;
#[doc = "D0FIFOL (rw) register accessor: an alias for `Reg<D0FIFOL_SPEC>`"]
pub type D0FIFOL = crate::Reg<d0fifol::D0FIFOL_SPEC>;
#[doc = "D0FIFO Port Register L"]
pub mod d0fifol;
#[doc = "D0FIFOH (rw) register accessor: an alias for `Reg<D0FIFOH_SPEC>`"]
pub type D0FIFOH = crate::Reg<d0fifoh::D0FIFOH_SPEC>;
#[doc = "D0FIFO Port Register H"]
pub mod d0fifoh;
#[doc = "D0FIFOLL (rw) register accessor: an alias for `Reg<D0FIFOLL_SPEC>`"]
pub type D0FIFOLL = crate::Reg<d0fifoll::D0FIFOLL_SPEC>;
#[doc = "D0FIFO Port Register LL"]
pub mod d0fifoll;
#[doc = "D0FIFOHH (rw) register accessor: an alias for `Reg<D0FIFOHH_SPEC>`"]
pub type D0FIFOHH = crate::Reg<d0fifohh::D0FIFOHH_SPEC>;
#[doc = "D0FIFO Port Register HH"]
pub mod d0fifohh;
#[doc = "D1FIFO (rw) register accessor: an alias for `Reg<D1FIFO_SPEC>`"]
pub type D1FIFO = crate::Reg<d1fifo::D1FIFO_SPEC>;
#[doc = "D1FIFO Port Register"]
pub mod d1fifo;
#[doc = "D1FIFOL (rw) register accessor: an alias for `Reg<D1FIFOL_SPEC>`"]
pub type D1FIFOL = crate::Reg<d1fifol::D1FIFOL_SPEC>;
#[doc = "D1FIFO Port Register L"]
pub mod d1fifol;
#[doc = "D1FIFOH (rw) register accessor: an alias for `Reg<D1FIFOH_SPEC>`"]
pub type D1FIFOH = crate::Reg<d1fifoh::D1FIFOH_SPEC>;
#[doc = "D1FIFO Port Register H"]
pub mod d1fifoh;
#[doc = "D1FIFOLL (rw) register accessor: an alias for `Reg<D1FIFOLL_SPEC>`"]
pub type D1FIFOLL = crate::Reg<d1fifoll::D1FIFOLL_SPEC>;
#[doc = "D1FIFO Port Register LL"]
pub mod d1fifoll;
#[doc = "D1FIFOHH (rw) register accessor: an alias for `Reg<D1FIFOHH_SPEC>`"]
pub type D1FIFOHH = crate::Reg<d1fifohh::D1FIFOHH_SPEC>;
#[doc = "D1FIFO Port Register HH"]
pub mod d1fifohh;
#[doc = "CFIFOSEL (rw) register accessor: an alias for `Reg<CFIFOSEL_SPEC>`"]
pub type CFIFOSEL = crate::Reg<cfifosel::CFIFOSEL_SPEC>;
#[doc = "CFIFO Port Select Register"]
pub mod cfifosel;
#[doc = "D0FIFOSEL (rw) register accessor: an alias for `Reg<D0FIFOSEL_SPEC>`"]
pub type D0FIFOSEL = crate::Reg<d0fifosel::D0FIFOSEL_SPEC>;
#[doc = "D0FIFO Port Select Register"]
pub mod d0fifosel;
#[doc = "D1FIFOSEL (rw) register accessor: an alias for `Reg<D1FIFOSEL_SPEC>`"]
pub type D1FIFOSEL = crate::Reg<d1fifosel::D1FIFOSEL_SPEC>;
#[doc = "D1FIFO Port Select Register"]
pub mod d1fifosel;
#[doc = "CFIFOCTR (rw) register accessor: an alias for `Reg<CFIFOCTR_SPEC>`"]
pub type CFIFOCTR = crate::Reg<cfifoctr::CFIFOCTR_SPEC>;
#[doc = "CFIFO Port Control Register"]
pub mod cfifoctr;
#[doc = "D0FIFOCTR (rw) register accessor: an alias for `Reg<D0FIFOCTR_SPEC>`"]
pub type D0FIFOCTR = crate::Reg<d0fifoctr::D0FIFOCTR_SPEC>;
#[doc = "D0FIFO Port Control Register"]
pub mod d0fifoctr;
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
#[doc = "SOF Pin Configuration Register"]
pub mod sofcfg;
#[doc = "PHYSET (rw) register accessor: an alias for `Reg<PHYSET_SPEC>`"]
pub type PHYSET = crate::Reg<physet::PHYSET_SPEC>;
#[doc = "PHY Setting Register"]
pub mod physet;
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
#[doc = "UFRMNUM (rw) register accessor: an alias for `Reg<UFRMNUM_SPEC>`"]
pub type UFRMNUM = crate::Reg<ufrmnum::UFRMNUM_SPEC>;
#[doc = "uFrame Number Register"]
pub mod ufrmnum;
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
#[doc = "PIPEBUF (rw) register accessor: an alias for `Reg<PIPEBUF_SPEC>`"]
pub type PIPEBUF = crate::Reg<pipebuf::PIPEBUF_SPEC>;
#[doc = "Pipe Buffer Register"]
pub mod pipebuf;
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
#[doc = "PIPE Control Register"]
pub mod pipectr;
#[doc = "PIPETRE (rw) register accessor: an alias for `Reg<PIPETRE_SPEC>`"]
pub type PIPETRE = crate::Reg<pipetre::PIPETRE_SPEC>;
#[doc = "PIPE Transaction Counter Enable Register"]
pub mod pipetre;
#[doc = "PIPETRN (rw) register accessor: an alias for `Reg<PIPETRN_SPEC>`"]
pub type PIPETRN = crate::Reg<pipetrn::PIPETRN_SPEC>;
#[doc = "PIPE Transaction Counter Register"]
pub mod pipetrn;
#[doc = "DEVADD (rw) register accessor: an alias for `Reg<DEVADD_SPEC>`"]
pub type DEVADD = crate::Reg<devadd::DEVADD_SPEC>;
#[doc = "Device Address Configuration Register"]
pub mod devadd;
#[doc = "DEVADDA (rw) register accessor: an alias for `Reg<DEVADDA_SPEC>`"]
pub type DEVADDA = crate::Reg<devadda::DEVADDA_SPEC>;
#[doc = "Device Address Configuration Register A"]
pub mod devadda;
#[doc = "LPCTRL (rw) register accessor: an alias for `Reg<LPCTRL_SPEC>`"]
pub type LPCTRL = crate::Reg<lpctrl::LPCTRL_SPEC>;
#[doc = "Low Power Control Register"]
pub mod lpctrl;
#[doc = "LPSTS (rw) register accessor: an alias for `Reg<LPSTS_SPEC>`"]
pub type LPSTS = crate::Reg<lpsts::LPSTS_SPEC>;
#[doc = "Low Power Status Register"]
pub mod lpsts;
#[doc = "BCCTRL (rw) register accessor: an alias for `Reg<BCCTRL_SPEC>`"]
pub type BCCTRL = crate::Reg<bcctrl::BCCTRL_SPEC>;
#[doc = "Battery Charging Control Register"]
pub mod bcctrl;
#[doc = "PL1CTRL1 (rw) register accessor: an alias for `Reg<PL1CTRL1_SPEC>`"]
pub type PL1CTRL1 = crate::Reg<pl1ctrl1::PL1CTRL1_SPEC>;
#[doc = "Function L1 Control Register 1"]
pub mod pl1ctrl1;
#[doc = "PL1CTRL2 (rw) register accessor: an alias for `Reg<PL1CTRL2_SPEC>`"]
pub type PL1CTRL2 = crate::Reg<pl1ctrl2::PL1CTRL2_SPEC>;
#[doc = "Function L1 Control Register 2"]
pub mod pl1ctrl2;
#[doc = "HL1CTRL1 (rw) register accessor: an alias for `Reg<HL1CTRL1_SPEC>`"]
pub type HL1CTRL1 = crate::Reg<hl1ctrl1::HL1CTRL1_SPEC>;
#[doc = "Host L1 Control Register 1"]
pub mod hl1ctrl1;
#[doc = "HL1CTRL2 (rw) register accessor: an alias for `Reg<HL1CTRL2_SPEC>`"]
pub type HL1CTRL2 = crate::Reg<hl1ctrl2::HL1CTRL2_SPEC>;
#[doc = "Host L1 Control Register 2"]
pub mod hl1ctrl2;
#[doc = "DPUSR0R (r) register accessor: an alias for `Reg<DPUSR0R_SPEC>`"]
pub type DPUSR0R = crate::Reg<dpusr0r::DPUSR0R_SPEC>;
#[doc = "Deep Standby USB Transceiver Control/Pin Monitor Register"]
pub mod dpusr0r;
#[doc = "DPUSR1R (rw) register accessor: an alias for `Reg<DPUSR1R_SPEC>`"]
pub type DPUSR1R = crate::Reg<dpusr1r::DPUSR1R_SPEC>;
#[doc = "Deep Standby USB Suspend/Resume Interrupt Register"]
pub mod dpusr1r;
#[doc = "DPUSR2R (rw) register accessor: an alias for `Reg<DPUSR2R_SPEC>`"]
pub type DPUSR2R = crate::Reg<dpusr2r::DPUSR2R_SPEC>;
#[doc = "Deep Standby USB Suspend/Resume Interrupt Register"]
pub mod dpusr2r;
#[doc = "DPUSRCR (rw) register accessor: an alias for `Reg<DPUSRCR_SPEC>`"]
pub type DPUSRCR = crate::Reg<dpusrcr::DPUSRCR_SPEC>;
#[doc = "Deep Standby USB Suspend/Resume Command Register"]
pub mod dpusrcr;
