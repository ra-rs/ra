#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_status: [u8; 0x04],
    _reserved_1_control2: [u8; 0x04],
    _reserved2: [u8; 0x08],
    #[doc = "0x10..0x28 - Limiter %s Start Value Register"]
    pub lstart: [LSTART; 6],
    #[doc = "0x28..0x40 - Limiter %s X-Axis Increment Register"]
    pub lxadd: [LXADD; 6],
    #[doc = "0x40..0x58 - Limiter %s Y-Axis Increment Register"]
    pub lyadd: [LYADD; 6],
    #[doc = "0x58..0x60 - Limiter %s Band Width Parameter Register"]
    pub lband: [LBAND; 2],
    _reserved6: [u8; 0x04],
    #[doc = "0x64 - Base Color Register"]
    pub color1: COLOR1,
    #[doc = "0x68 - Secondary Color Register"]
    pub color2: COLOR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x74 - Pattern Register"]
    pub pattern: PATTERN,
    #[doc = "0x78 - Bounding Box Dimension Register"]
    pub size: SIZE,
    #[doc = "0x7c - Framebuffer Pitch And Spanstore Delay Register"]
    pub pitch: PITCH,
    #[doc = "0x80 - Framebuffer Base Address Register"]
    pub origin: ORIGIN,
    _reserved12: [u8; 0x0c],
    #[doc = "0x90 - U Limiter Start Value Register"]
    pub lustart: LUSTART,
    #[doc = "0x94 - U Limiter X-Axis Increment Register"]
    pub luxadd: LUXADD,
    #[doc = "0x98 - U Limiter Y-Axis Increment Register"]
    pub luyadd: LUYADD,
    #[doc = "0x9c - V Limiter Start Value Integer Part Register"]
    pub lvstarti: LVSTARTI,
    #[doc = "0xa0 - V Limiter Start Value Fractional Part Register"]
    pub lvstartf: LVSTARTF,
    #[doc = "0xa4 - V Limiter X-Axis Increment Integer Part Register"]
    pub lvxaddi: LVXADDI,
    #[doc = "0xa8 - V Limiter Y-Axis Increment Integer Part Register"]
    pub lvyaddi: LVYADDI,
    #[doc = "0xac - V Limiter Increment Fractional Parts Register"]
    pub lvyxaddf: LVYXADDF,
    _reserved20: [u8; 0x04],
    #[doc = "0xb4 - Texels Per Texture Line Register"]
    pub texpitch: TEXPITCH,
    #[doc = "0xb8 - Texture Size or Texture Address Mask Register"]
    pub texmask: TEXMASK,
    #[doc = "0xbc - Texture Base Address Register"]
    pub texorigin: TEXORIGIN,
    #[doc = "0xc0 - Interrupt Control Register"]
    pub irqctl: IRQCTL,
    #[doc = "0xc4 - Cache Control Register"]
    pub cachectl: CACHECTL,
    #[doc = "0xc8 - Display List Start Address Register"]
    pub dliststart: DLISTSTART,
    #[doc = "0xcc..0xd4 - Performance Counter %s"]
    pub perfcount: [PERFCOUNT; 2],
    #[doc = "0xd4 - Performance Counters Control Register"]
    pub perftrigger: PERFTRIGGER,
    _reserved28: [u8; 0x04],
    #[doc = "0xdc - CLUT Start Address Register"]
    pub texcladdr: TEXCLADDR,
    #[doc = "0xe0 - CLUT Data Register"]
    pub texcldata: TEXCLDATA,
    #[doc = "0xe4 - CLUT Offset Register"]
    pub texcloffset: TEXCLOFFSET,
    #[doc = "0xe8 - Color Key Register"]
    pub colkey: COLKEY,
}
impl RegisterBlock {
    #[doc = "0x00 - Status Control Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Geometry Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - Hardware Version and Feature Set ID Register"]
    #[inline(always)]
    pub const fn hwrevision(&self) -> &HWREVISION {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Surface Control Register"]
    #[inline(always)]
    pub const fn control2(&self) -> &CONTROL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x10 - Limiter %s Start Value Register"]
    #[inline(always)]
    pub fn l1start(&self) -> &LSTART {
        &self.lstart[0]
    }
    #[doc = "0x14 - Limiter %s Start Value Register"]
    #[inline(always)]
    pub fn l2start(&self) -> &LSTART {
        &self.lstart[1]
    }
    #[doc = "0x18 - Limiter %s Start Value Register"]
    #[inline(always)]
    pub fn l3start(&self) -> &LSTART {
        &self.lstart[2]
    }
    #[doc = "0x1c - Limiter %s Start Value Register"]
    #[inline(always)]
    pub fn l4start(&self) -> &LSTART {
        &self.lstart[3]
    }
    #[doc = "0x20 - Limiter %s Start Value Register"]
    #[inline(always)]
    pub fn l5start(&self) -> &LSTART {
        &self.lstart[4]
    }
    #[doc = "0x24 - Limiter %s Start Value Register"]
    #[inline(always)]
    pub fn l6start(&self) -> &LSTART {
        &self.lstart[5]
    }
    #[doc = "0x28 - Limiter %s X-Axis Increment Register"]
    #[inline(always)]
    pub fn l1xadd(&self) -> &LXADD {
        &self.lxadd[0]
    }
    #[doc = "0x2c - Limiter %s X-Axis Increment Register"]
    #[inline(always)]
    pub fn l2xadd(&self) -> &LXADD {
        &self.lxadd[1]
    }
    #[doc = "0x30 - Limiter %s X-Axis Increment Register"]
    #[inline(always)]
    pub fn l3xadd(&self) -> &LXADD {
        &self.lxadd[2]
    }
    #[doc = "0x34 - Limiter %s X-Axis Increment Register"]
    #[inline(always)]
    pub fn l4xadd(&self) -> &LXADD {
        &self.lxadd[3]
    }
    #[doc = "0x38 - Limiter %s X-Axis Increment Register"]
    #[inline(always)]
    pub fn l5xadd(&self) -> &LXADD {
        &self.lxadd[4]
    }
    #[doc = "0x3c - Limiter %s X-Axis Increment Register"]
    #[inline(always)]
    pub fn l6xadd(&self) -> &LXADD {
        &self.lxadd[5]
    }
    #[doc = "0x40 - Limiter %s Y-Axis Increment Register"]
    #[inline(always)]
    pub fn l1yadd(&self) -> &LYADD {
        &self.lyadd[0]
    }
    #[doc = "0x44 - Limiter %s Y-Axis Increment Register"]
    #[inline(always)]
    pub fn l2yadd(&self) -> &LYADD {
        &self.lyadd[1]
    }
    #[doc = "0x48 - Limiter %s Y-Axis Increment Register"]
    #[inline(always)]
    pub fn l3yadd(&self) -> &LYADD {
        &self.lyadd[2]
    }
    #[doc = "0x4c - Limiter %s Y-Axis Increment Register"]
    #[inline(always)]
    pub fn l4yadd(&self) -> &LYADD {
        &self.lyadd[3]
    }
    #[doc = "0x50 - Limiter %s Y-Axis Increment Register"]
    #[inline(always)]
    pub fn l5yadd(&self) -> &LYADD {
        &self.lyadd[4]
    }
    #[doc = "0x54 - Limiter %s Y-Axis Increment Register"]
    #[inline(always)]
    pub fn l6yadd(&self) -> &LYADD {
        &self.lyadd[5]
    }
    #[doc = "0x58 - Limiter %s Band Width Parameter Register"]
    #[inline(always)]
    pub fn l1band(&self) -> &LBAND {
        &self.lband[0]
    }
    #[doc = "0x5c - Limiter %s Band Width Parameter Register"]
    #[inline(always)]
    pub fn l2band(&self) -> &LBAND {
        &self.lband[1]
    }
    #[doc = "0xcc - Performance Counter %s"]
    #[inline(always)]
    pub fn perfcount1(&self) -> &PERFCOUNT {
        &self.perfcount[0]
    }
    #[doc = "0xd0 - Performance Counter %s"]
    #[inline(always)]
    pub fn perfcount2(&self) -> &PERFCOUNT {
        &self.perfcount[1]
    }
}
#[doc = "CONTROL (w) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Geometry Control Register"]
pub mod control;
#[doc = "CONTROL2 (w) register accessor: an alias for `Reg<CONTROL2_SPEC>`"]
pub type CONTROL2 = crate::Reg<control2::CONTROL2_SPEC>;
#[doc = "Surface Control Register"]
pub mod control2;
#[doc = "IRQCTL (w) register accessor: an alias for `Reg<IRQCTL_SPEC>`"]
pub type IRQCTL = crate::Reg<irqctl::IRQCTL_SPEC>;
#[doc = "Interrupt Control Register"]
pub mod irqctl;
#[doc = "CACHECTL (w) register accessor: an alias for `Reg<CACHECTL_SPEC>`"]
pub type CACHECTL = crate::Reg<cachectl::CACHECTL_SPEC>;
#[doc = "Cache Control Register"]
pub mod cachectl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Control Register"]
pub mod status;
#[doc = "HWREVISION (r) register accessor: an alias for `Reg<HWREVISION_SPEC>`"]
pub type HWREVISION = crate::Reg<hwrevision::HWREVISION_SPEC>;
#[doc = "Hardware Version and Feature Set ID Register"]
pub mod hwrevision;
#[doc = "COLOR1 (w) register accessor: an alias for `Reg<COLOR1_SPEC>`"]
pub type COLOR1 = crate::Reg<color1::COLOR1_SPEC>;
#[doc = "Base Color Register"]
pub mod color1;
#[doc = "COLOR2 (w) register accessor: an alias for `Reg<COLOR2_SPEC>`"]
pub type COLOR2 = crate::Reg<color2::COLOR2_SPEC>;
#[doc = "Secondary Color Register"]
pub mod color2;
#[doc = "PATTERN (w) register accessor: an alias for `Reg<PATTERN_SPEC>`"]
pub type PATTERN = crate::Reg<pattern::PATTERN_SPEC>;
#[doc = "Pattern Register"]
pub mod pattern;
#[doc = "LSTART (w) register accessor: an alias for `Reg<LSTART_SPEC>`"]
pub type LSTART = crate::Reg<lstart::LSTART_SPEC>;
#[doc = "Limiter %s Start Value Register"]
pub mod lstart;
#[doc = "LXADD (w) register accessor: an alias for `Reg<LXADD_SPEC>`"]
pub type LXADD = crate::Reg<lxadd::LXADD_SPEC>;
#[doc = "Limiter %s X-Axis Increment Register"]
pub mod lxadd;
#[doc = "LYADD (w) register accessor: an alias for `Reg<LYADD_SPEC>`"]
pub type LYADD = crate::Reg<lyadd::LYADD_SPEC>;
#[doc = "Limiter %s Y-Axis Increment Register"]
pub mod lyadd;
#[doc = "LBAND (w) register accessor: an alias for `Reg<LBAND_SPEC>`"]
pub type LBAND = crate::Reg<lband::LBAND_SPEC>;
#[doc = "Limiter %s Band Width Parameter Register"]
pub mod lband;
#[doc = "TEXORIGIN (w) register accessor: an alias for `Reg<TEXORIGIN_SPEC>`"]
pub type TEXORIGIN = crate::Reg<texorigin::TEXORIGIN_SPEC>;
#[doc = "Texture Base Address Register"]
pub mod texorigin;
#[doc = "TEXPITCH (w) register accessor: an alias for `Reg<TEXPITCH_SPEC>`"]
pub type TEXPITCH = crate::Reg<texpitch::TEXPITCH_SPEC>;
#[doc = "Texels Per Texture Line Register"]
pub mod texpitch;
#[doc = "TEXMASK (w) register accessor: an alias for `Reg<TEXMASK_SPEC>`"]
pub type TEXMASK = crate::Reg<texmask::TEXMASK_SPEC>;
#[doc = "Texture Size or Texture Address Mask Register"]
pub mod texmask;
#[doc = "LUSTART (w) register accessor: an alias for `Reg<LUSTART_SPEC>`"]
pub type LUSTART = crate::Reg<lustart::LUSTART_SPEC>;
#[doc = "U Limiter Start Value Register"]
pub mod lustart;
#[doc = "LUXADD (w) register accessor: an alias for `Reg<LUXADD_SPEC>`"]
pub type LUXADD = crate::Reg<luxadd::LUXADD_SPEC>;
#[doc = "U Limiter X-Axis Increment Register"]
pub mod luxadd;
#[doc = "LUYADD (w) register accessor: an alias for `Reg<LUYADD_SPEC>`"]
pub type LUYADD = crate::Reg<luyadd::LUYADD_SPEC>;
#[doc = "U Limiter Y-Axis Increment Register"]
pub mod luyadd;
#[doc = "LVSTARTI (w) register accessor: an alias for `Reg<LVSTARTI_SPEC>`"]
pub type LVSTARTI = crate::Reg<lvstarti::LVSTARTI_SPEC>;
#[doc = "V Limiter Start Value Integer Part Register"]
pub mod lvstarti;
#[doc = "LVSTARTF (w) register accessor: an alias for `Reg<LVSTARTF_SPEC>`"]
pub type LVSTARTF = crate::Reg<lvstartf::LVSTARTF_SPEC>;
#[doc = "V Limiter Start Value Fractional Part Register"]
pub mod lvstartf;
#[doc = "LVXADDI (w) register accessor: an alias for `Reg<LVXADDI_SPEC>`"]
pub type LVXADDI = crate::Reg<lvxaddi::LVXADDI_SPEC>;
#[doc = "V Limiter X-Axis Increment Integer Part Register"]
pub mod lvxaddi;
#[doc = "LVYADDI (w) register accessor: an alias for `Reg<LVYADDI_SPEC>`"]
pub type LVYADDI = crate::Reg<lvyaddi::LVYADDI_SPEC>;
#[doc = "V Limiter Y-Axis Increment Integer Part Register"]
pub mod lvyaddi;
#[doc = "LVYXADDF (w) register accessor: an alias for `Reg<LVYXADDF_SPEC>`"]
pub type LVYXADDF = crate::Reg<lvyxaddf::LVYXADDF_SPEC>;
#[doc = "V Limiter Increment Fractional Parts Register"]
pub mod lvyxaddf;
#[doc = "TEXCLADDR (w) register accessor: an alias for `Reg<TEXCLADDR_SPEC>`"]
pub type TEXCLADDR = crate::Reg<texcladdr::TEXCLADDR_SPEC>;
#[doc = "CLUT Start Address Register"]
pub mod texcladdr;
#[doc = "TEXCLDATA (w) register accessor: an alias for `Reg<TEXCLDATA_SPEC>`"]
pub type TEXCLDATA = crate::Reg<texcldata::TEXCLDATA_SPEC>;
#[doc = "CLUT Data Register"]
pub mod texcldata;
#[doc = "TEXCLOFFSET (w) register accessor: an alias for `Reg<TEXCLOFFSET_SPEC>`"]
pub type TEXCLOFFSET = crate::Reg<texcloffset::TEXCLOFFSET_SPEC>;
#[doc = "CLUT Offset Register"]
pub mod texcloffset;
#[doc = "COLKEY (w) register accessor: an alias for `Reg<COLKEY_SPEC>`"]
pub type COLKEY = crate::Reg<colkey::COLKEY_SPEC>;
#[doc = "Color Key Register"]
pub mod colkey;
#[doc = "SIZE (w) register accessor: an alias for `Reg<SIZE_SPEC>`"]
pub type SIZE = crate::Reg<size::SIZE_SPEC>;
#[doc = "Bounding Box Dimension Register"]
pub mod size;
#[doc = "PITCH (w) register accessor: an alias for `Reg<PITCH_SPEC>`"]
pub type PITCH = crate::Reg<pitch::PITCH_SPEC>;
#[doc = "Framebuffer Pitch And Spanstore Delay Register"]
pub mod pitch;
#[doc = "ORIGIN (w) register accessor: an alias for `Reg<ORIGIN_SPEC>`"]
pub type ORIGIN = crate::Reg<origin::ORIGIN_SPEC>;
#[doc = "Framebuffer Base Address Register"]
pub mod origin;
#[doc = "DLISTSTART (w) register accessor: an alias for `Reg<DLISTSTART_SPEC>`"]
pub type DLISTSTART = crate::Reg<dliststart::DLISTSTART_SPEC>;
#[doc = "Display List Start Address Register"]
pub mod dliststart;
#[doc = "PERFTRIGGER (w) register accessor: an alias for `Reg<PERFTRIGGER_SPEC>`"]
pub type PERFTRIGGER = crate::Reg<perftrigger::PERFTRIGGER_SPEC>;
#[doc = "Performance Counters Control Register"]
pub mod perftrigger;
#[doc = "PERFCOUNT (rw) register accessor: an alias for `Reg<PERFCOUNT_SPEC>`"]
pub type PERFCOUNT = crate::Reg<perfcount::PERFCOUNT_SPEC>;
#[doc = "Performance Counter %s"]
pub mod perfcount;
