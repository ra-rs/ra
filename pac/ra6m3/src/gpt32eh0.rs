#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General PWM Timer Write-Protection Register"]
    pub gtwp: GTWP,
    #[doc = "0x04 - General PWM Timer Software Start Register"]
    pub gtstr: GTSTR,
    #[doc = "0x08 - General PWM Timer Software Stop Register"]
    pub gtstp: GTSTP,
    #[doc = "0x0c - General PWM Timer Software Clear Register"]
    pub gtclr: GTCLR,
    #[doc = "0x10 - General PWM Timer Start Source Select Register"]
    pub gtssr: GTSSR,
    #[doc = "0x14 - General PWM Timer Stop Source Select Register"]
    pub gtpsr: GTPSR,
    #[doc = "0x18 - General PWM Timer Clear Source Select Register"]
    pub gtcsr: GTCSR,
    #[doc = "0x1c - General PWM Timer Up Count Source Select Register"]
    pub gtupsr: GTUPSR,
    #[doc = "0x20 - General PWM Timer Down Count Source Select Register"]
    pub gtdnsr: GTDNSR,
    #[doc = "0x24 - General PWM Timer Input Capture Source Select Register A"]
    pub gticasr: GTICASR,
    #[doc = "0x28 - General PWM Timer Input Capture Source Select Register B"]
    pub gticbsr: GTICBSR,
    #[doc = "0x2c - General PWM Timer Control Register"]
    pub gtcr: GTCR,
    #[doc = "0x30 - General PWM Timer Count Direction and Duty Setting Register"]
    pub gtuddtyc: GTUDDTYC,
    #[doc = "0x34 - General PWM Timer I/O Control Register"]
    pub gtior: GTIOR,
    #[doc = "0x38 - General PWM Timer Interrupt Output Setting Register"]
    pub gtintad: GTINTAD,
    #[doc = "0x3c - General PWM Timer Status Register"]
    pub gtst: GTST,
    #[doc = "0x40 - General PWM Timer Buffer Enable Register"]
    pub gtber: GTBER,
    #[doc = "0x44 - General PWM Timer Interrupt and A/D Converter Start Request Skipping Setting Register"]
    pub gtitc: GTITC,
    #[doc = "0x48 - General PWM Timer Counter"]
    pub gtcnt: GTCNT,
    #[doc = "0x4c - General PWM Timer Compare Capture Register A"]
    pub gtccra: GTCCRA,
    #[doc = "0x50 - General PWM Timer Compare Capture Register B"]
    pub gtccrb: GTCCRB,
    #[doc = "0x54 - General PWM Timer Compare Capture Register C"]
    pub gtccrc: GTCCRC,
    #[doc = "0x58 - General PWM Timer Compare Capture Register E"]
    pub gtccre: GTCCRE,
    #[doc = "0x5c - General PWM Timer Compare Capture Register D"]
    pub gtccrd: GTCCRD,
    #[doc = "0x60 - General PWM Timer Compare Capture Register F"]
    pub gtccrf: GTCCRF,
    #[doc = "0x64 - General PWM Timer Cycle Setting Register"]
    pub gtpr: GTPR,
    #[doc = "0x68 - General PWM Timer Cycle Setting Buffer Register"]
    pub gtpbr: GTPBR,
    #[doc = "0x6c - General PWM Timer Cycle Setting Double-Buffer Register"]
    pub gtpdbr: GTPDBR,
    #[doc = "0x70 - A/D Converter Start Request Timing Register A"]
    pub gtadtra: GTADTRA,
    #[doc = "0x74 - A/D Converter Start Request Timing Buffer Register A"]
    pub gtadtbra: GTADTBRA,
    #[doc = "0x78 - A/D Converter Start Request Timing Double-Buffer Register A"]
    pub gtadtdbra: GTADTDBRA,
    #[doc = "0x7c - A/D Converter Start Request Timing Register B"]
    pub gtadtrb: GTADTRB,
    #[doc = "0x80 - A/D Converter Start Request Timing Buffer Register B"]
    pub gtadtbrb: GTADTBRB,
    #[doc = "0x84 - A/D Converter Start Request Timing Double-Buffer Register B"]
    pub gtadtdbrb: GTADTDBRB,
    #[doc = "0x88 - General PWM Timer Dead Time Control Register"]
    pub gtdtcr: GTDTCR,
    #[doc = "0x8c - General PWM Timer Dead Time Value Register U"]
    pub gtdvu: GTDVU,
    #[doc = "0x90 - General PWM Timer Dead Time Value Register D"]
    pub gtdvd: GTDVD,
    #[doc = "0x94 - General PWM Timer Dead Time Buffer Register U"]
    pub gtdbu: GTDBU,
    #[doc = "0x98 - General PWM Timer Dead Time Buffer Register D"]
    pub gtdbd: GTDBD,
    #[doc = "0x9c - General PWM Timer Output Protection Function Status Register"]
    pub gtsos: GTSOS,
    #[doc = "0xa0 - General PWM Timer Output Protection Function Temporary Release Register"]
    pub gtsotr: GTSOTR,
}
#[doc = "GTWP (rw) register accessor: an alias for `Reg<GTWP_SPEC>`"]
pub type GTWP = crate::Reg<gtwp::GTWP_SPEC>;
#[doc = "General PWM Timer Write-Protection Register"]
pub mod gtwp;
#[doc = "GTSTR (rw) register accessor: an alias for `Reg<GTSTR_SPEC>`"]
pub type GTSTR = crate::Reg<gtstr::GTSTR_SPEC>;
#[doc = "General PWM Timer Software Start Register"]
pub mod gtstr;
#[doc = "GTSTP (rw) register accessor: an alias for `Reg<GTSTP_SPEC>`"]
pub type GTSTP = crate::Reg<gtstp::GTSTP_SPEC>;
#[doc = "General PWM Timer Software Stop Register"]
pub mod gtstp;
#[doc = "GTCLR (w) register accessor: an alias for `Reg<GTCLR_SPEC>`"]
pub type GTCLR = crate::Reg<gtclr::GTCLR_SPEC>;
#[doc = "General PWM Timer Software Clear Register"]
pub mod gtclr;
#[doc = "GTSSR (rw) register accessor: an alias for `Reg<GTSSR_SPEC>`"]
pub type GTSSR = crate::Reg<gtssr::GTSSR_SPEC>;
#[doc = "General PWM Timer Start Source Select Register"]
pub mod gtssr;
#[doc = "GTPSR (rw) register accessor: an alias for `Reg<GTPSR_SPEC>`"]
pub type GTPSR = crate::Reg<gtpsr::GTPSR_SPEC>;
#[doc = "General PWM Timer Stop Source Select Register"]
pub mod gtpsr;
#[doc = "GTCSR (rw) register accessor: an alias for `Reg<GTCSR_SPEC>`"]
pub type GTCSR = crate::Reg<gtcsr::GTCSR_SPEC>;
#[doc = "General PWM Timer Clear Source Select Register"]
pub mod gtcsr;
#[doc = "GTUPSR (rw) register accessor: an alias for `Reg<GTUPSR_SPEC>`"]
pub type GTUPSR = crate::Reg<gtupsr::GTUPSR_SPEC>;
#[doc = "General PWM Timer Up Count Source Select Register"]
pub mod gtupsr;
#[doc = "GTDNSR (rw) register accessor: an alias for `Reg<GTDNSR_SPEC>`"]
pub type GTDNSR = crate::Reg<gtdnsr::GTDNSR_SPEC>;
#[doc = "General PWM Timer Down Count Source Select Register"]
pub mod gtdnsr;
#[doc = "GTICASR (rw) register accessor: an alias for `Reg<GTICASR_SPEC>`"]
pub type GTICASR = crate::Reg<gticasr::GTICASR_SPEC>;
#[doc = "General PWM Timer Input Capture Source Select Register A"]
pub mod gticasr;
#[doc = "GTICBSR (rw) register accessor: an alias for `Reg<GTICBSR_SPEC>`"]
pub type GTICBSR = crate::Reg<gticbsr::GTICBSR_SPEC>;
#[doc = "General PWM Timer Input Capture Source Select Register B"]
pub mod gticbsr;
#[doc = "GTCR (rw) register accessor: an alias for `Reg<GTCR_SPEC>`"]
pub type GTCR = crate::Reg<gtcr::GTCR_SPEC>;
#[doc = "General PWM Timer Control Register"]
pub mod gtcr;
#[doc = "GTUDDTYC (rw) register accessor: an alias for `Reg<GTUDDTYC_SPEC>`"]
pub type GTUDDTYC = crate::Reg<gtuddtyc::GTUDDTYC_SPEC>;
#[doc = "General PWM Timer Count Direction and Duty Setting Register"]
pub mod gtuddtyc;
#[doc = "GTIOR (rw) register accessor: an alias for `Reg<GTIOR_SPEC>`"]
pub type GTIOR = crate::Reg<gtior::GTIOR_SPEC>;
#[doc = "General PWM Timer I/O Control Register"]
pub mod gtior;
#[doc = "GTINTAD (rw) register accessor: an alias for `Reg<GTINTAD_SPEC>`"]
pub type GTINTAD = crate::Reg<gtintad::GTINTAD_SPEC>;
#[doc = "General PWM Timer Interrupt Output Setting Register"]
pub mod gtintad;
#[doc = "GTST (rw) register accessor: an alias for `Reg<GTST_SPEC>`"]
pub type GTST = crate::Reg<gtst::GTST_SPEC>;
#[doc = "General PWM Timer Status Register"]
pub mod gtst;
#[doc = "GTBER (rw) register accessor: an alias for `Reg<GTBER_SPEC>`"]
pub type GTBER = crate::Reg<gtber::GTBER_SPEC>;
#[doc = "General PWM Timer Buffer Enable Register"]
pub mod gtber;
#[doc = "GTITC (rw) register accessor: an alias for `Reg<GTITC_SPEC>`"]
pub type GTITC = crate::Reg<gtitc::GTITC_SPEC>;
#[doc = "General PWM Timer Interrupt and A/D Converter Start Request Skipping Setting Register"]
pub mod gtitc;
#[doc = "GTCNT (rw) register accessor: an alias for `Reg<GTCNT_SPEC>`"]
pub type GTCNT = crate::Reg<gtcnt::GTCNT_SPEC>;
#[doc = "General PWM Timer Counter"]
pub mod gtcnt;
#[doc = "GTCCRA (rw) register accessor: an alias for `Reg<GTCCRA_SPEC>`"]
pub type GTCCRA = crate::Reg<gtccra::GTCCRA_SPEC>;
#[doc = "General PWM Timer Compare Capture Register A"]
pub mod gtccra;
#[doc = "GTCCRB (rw) register accessor: an alias for `Reg<GTCCRB_SPEC>`"]
pub type GTCCRB = crate::Reg<gtccrb::GTCCRB_SPEC>;
#[doc = "General PWM Timer Compare Capture Register B"]
pub mod gtccrb;
#[doc = "GTCCRC (rw) register accessor: an alias for `Reg<GTCCRC_SPEC>`"]
pub type GTCCRC = crate::Reg<gtccrc::GTCCRC_SPEC>;
#[doc = "General PWM Timer Compare Capture Register C"]
pub mod gtccrc;
#[doc = "GTCCRE (rw) register accessor: an alias for `Reg<GTCCRE_SPEC>`"]
pub type GTCCRE = crate::Reg<gtccre::GTCCRE_SPEC>;
#[doc = "General PWM Timer Compare Capture Register E"]
pub mod gtccre;
#[doc = "GTCCRD (rw) register accessor: an alias for `Reg<GTCCRD_SPEC>`"]
pub type GTCCRD = crate::Reg<gtccrd::GTCCRD_SPEC>;
#[doc = "General PWM Timer Compare Capture Register D"]
pub mod gtccrd;
#[doc = "GTCCRF (rw) register accessor: an alias for `Reg<GTCCRF_SPEC>`"]
pub type GTCCRF = crate::Reg<gtccrf::GTCCRF_SPEC>;
#[doc = "General PWM Timer Compare Capture Register F"]
pub mod gtccrf;
#[doc = "GTPR (rw) register accessor: an alias for `Reg<GTPR_SPEC>`"]
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
#[doc = "General PWM Timer Cycle Setting Register"]
pub mod gtpr;
#[doc = "GTPBR (rw) register accessor: an alias for `Reg<GTPBR_SPEC>`"]
pub type GTPBR = crate::Reg<gtpbr::GTPBR_SPEC>;
#[doc = "General PWM Timer Cycle Setting Buffer Register"]
pub mod gtpbr;
#[doc = "GTPDBR (rw) register accessor: an alias for `Reg<GTPDBR_SPEC>`"]
pub type GTPDBR = crate::Reg<gtpdbr::GTPDBR_SPEC>;
#[doc = "General PWM Timer Cycle Setting Double-Buffer Register"]
pub mod gtpdbr;
#[doc = "GTADTRA (rw) register accessor: an alias for `Reg<GTADTRA_SPEC>`"]
pub type GTADTRA = crate::Reg<gtadtra::GTADTRA_SPEC>;
#[doc = "A/D Converter Start Request Timing Register A"]
pub mod gtadtra;
#[doc = "GTADTRB (rw) register accessor: an alias for `Reg<GTADTRB_SPEC>`"]
pub type GTADTRB = crate::Reg<gtadtrb::GTADTRB_SPEC>;
#[doc = "A/D Converter Start Request Timing Register B"]
pub mod gtadtrb;
#[doc = "GTADTBRA (rw) register accessor: an alias for `Reg<GTADTBRA_SPEC>`"]
pub type GTADTBRA = crate::Reg<gtadtbra::GTADTBRA_SPEC>;
#[doc = "A/D Converter Start Request Timing Buffer Register A"]
pub mod gtadtbra;
#[doc = "GTADTBRB (rw) register accessor: an alias for `Reg<GTADTBRB_SPEC>`"]
pub type GTADTBRB = crate::Reg<gtadtbrb::GTADTBRB_SPEC>;
#[doc = "A/D Converter Start Request Timing Buffer Register B"]
pub mod gtadtbrb;
#[doc = "GTADTDBRA (rw) register accessor: an alias for `Reg<GTADTDBRA_SPEC>`"]
pub type GTADTDBRA = crate::Reg<gtadtdbra::GTADTDBRA_SPEC>;
#[doc = "A/D Converter Start Request Timing Double-Buffer Register A"]
pub mod gtadtdbra;
#[doc = "GTADTDBRB (rw) register accessor: an alias for `Reg<GTADTDBRB_SPEC>`"]
pub type GTADTDBRB = crate::Reg<gtadtdbrb::GTADTDBRB_SPEC>;
#[doc = "A/D Converter Start Request Timing Double-Buffer Register B"]
pub mod gtadtdbrb;
#[doc = "GTDTCR (rw) register accessor: an alias for `Reg<GTDTCR_SPEC>`"]
pub type GTDTCR = crate::Reg<gtdtcr::GTDTCR_SPEC>;
#[doc = "General PWM Timer Dead Time Control Register"]
pub mod gtdtcr;
#[doc = "GTDVU (rw) register accessor: an alias for `Reg<GTDVU_SPEC>`"]
pub type GTDVU = crate::Reg<gtdvu::GTDVU_SPEC>;
#[doc = "General PWM Timer Dead Time Value Register U"]
pub mod gtdvu;
#[doc = "GTDVD (rw) register accessor: an alias for `Reg<GTDVD_SPEC>`"]
pub type GTDVD = crate::Reg<gtdvd::GTDVD_SPEC>;
#[doc = "General PWM Timer Dead Time Value Register D"]
pub mod gtdvd;
#[doc = "GTDBU (rw) register accessor: an alias for `Reg<GTDBU_SPEC>`"]
pub type GTDBU = crate::Reg<gtdbu::GTDBU_SPEC>;
#[doc = "General PWM Timer Dead Time Buffer Register U"]
pub mod gtdbu;
#[doc = "GTDBD (rw) register accessor: an alias for `Reg<GTDBD_SPEC>`"]
pub type GTDBD = crate::Reg<gtdbd::GTDBD_SPEC>;
#[doc = "General PWM Timer Dead Time Buffer Register D"]
pub mod gtdbd;
#[doc = "GTSOS (r) register accessor: an alias for `Reg<GTSOS_SPEC>`"]
pub type GTSOS = crate::Reg<gtsos::GTSOS_SPEC>;
#[doc = "General PWM Timer Output Protection Function Status Register"]
pub mod gtsos;
#[doc = "GTSOTR (rw) register accessor: an alias for `Reg<GTSOTR_SPEC>`"]
pub type GTSOTR = crate::Reg<gtsotr::GTSOTR_SPEC>;
#[doc = "General PWM Timer Output Protection Function Temporary Release Register"]
pub mod gtsotr;
