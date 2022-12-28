#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x400 - Color Palette 0 Plane for Graphics 1 Plane"]
    pub gr1_clut0: [GR1_CLUT0; 256],
    #[doc = "0x400..0x800 - Color Palette 1 Plane for Graphics 1 Plane"]
    pub gr1_clut1: [GR1_CLUT1; 256],
    #[doc = "0x800..0xc00 - Color Palette 0 Plane for Graphics 2 Plane"]
    pub gr2_clut0: [GR2_CLUT0; 256],
    #[doc = "0xc00..0x1000 - Color Palette 1 Plane for Graphics 2 Plane"]
    pub gr2_clut1: [GR2_CLUT1; 256],
    #[doc = "0x1000 - Background Plane Setting Operation Control Register"]
    pub bg_en: BG_EN,
    #[doc = "0x1004 - Background Plane Setting Free-Running Period Register"]
    pub bg_peri: BG_PERI,
    #[doc = "0x1008 - Background Plane Setting Synchronization Position Register"]
    pub bg_sync: BG_SYNC,
    #[doc = "0x100c - Background Plane Setting Full Image Vertical Size Register"]
    pub bg_vsize: BG_VSIZE,
    #[doc = "0x1010 - Background Plane Setting Full Image Horizontal Size Register"]
    pub bg_hsize: BG_HSIZE,
    #[doc = "0x1014 - Background Plane Setting Background Color Register"]
    pub bg_bgc: BG_BGC,
    #[doc = "0x1018 - Background Plane Setting Status Monitor Register"]
    pub bg_mon: BG_MON,
    _reserved11: [u8; 0xe4],
    #[doc = "0x1100 - Graphics %s Register Update Control Register"]
    pub gr1_ven: GR_VEN,
    #[doc = "0x1104 - Graphics %s Frame Buffer Read Control Register"]
    pub gr1_flmrd: GR_FLMRD,
    #[doc = "0x1108 - Graphics %s Frame Buffer Control Register 1"]
    pub gr1_flm1: GR_FLM1,
    #[doc = "0x110c - Graphics %s Frame Buffer Control Register 2"]
    pub gr1_flm2: GR_FLM2,
    #[doc = "0x1110 - Graphics %s Frame Buffer Control Register 3"]
    pub gr1_flm3: GR_FLM3,
    _reserved16: [u8; 0x04],
    #[doc = "0x1118 - Graphics %s Frame Buffer Control Register 5"]
    pub gr1_flm5: GR_FLM5,
    #[doc = "0x111c - Graphics %s Frame Buffer Control Register 6"]
    pub gr1_flm6: GR_FLM6,
    #[doc = "0x1120 - Graphics %s Alpha Blending Control Register 1"]
    pub gr1_ab1: GR_AB1,
    #[doc = "0x1124 - Graphics %s Alpha Blending Control Register 2"]
    pub gr1_ab2: GR_AB2,
    #[doc = "0x1128 - Graphics %s Alpha Blending Control Register 3"]
    pub gr1_ab3: GR_AB3,
    #[doc = "0x112c - Graphics %s Alpha Blending Control Register 4"]
    pub gr1_ab4: GR_AB4,
    #[doc = "0x1130 - Graphics %s Alpha Blending Control Register 5"]
    pub gr1_ab5: GR_AB5,
    #[doc = "0x1134 - Graphics %s Alpha Blending Control Register 6"]
    pub gr1_ab6: GR_AB6,
    #[doc = "0x1138 - Graphics %s Alpha Blending Control Register 7"]
    pub gr1_ab7: GR_AB7,
    #[doc = "0x113c - Graphics %s Alpha Blending Control Register 8"]
    pub gr1_ab8: GR_AB8,
    #[doc = "0x1140 - Graphics %s Alpha Blending Control Register 9"]
    pub gr1_ab9: GR_AB9,
    _reserved27: [u8; 0x08],
    #[doc = "0x114c - Graphics %s Background Color Control Register"]
    pub gr1_base: GR_BASE,
    #[doc = "0x1150 - Graphics %s CLUT Table Interrupt Control Register"]
    pub gr1_clutint: GR_CLUTINT,
    #[doc = "0x1154 - Graphics %s Status Monitor Register"]
    pub gr1_mon: GR_MON,
    _reserved30: [u8; 0xa8],
    #[doc = "0x1200 - Graphics %s Register Update Control Register"]
    pub gr2_ven: GR_VEN,
    #[doc = "0x1204 - Graphics %s Frame Buffer Read Control Register"]
    pub gr2_flmrd: GR_FLMRD,
    #[doc = "0x1208 - Graphics %s Frame Buffer Control Register 1"]
    pub gr2_flm1: GR_FLM1,
    #[doc = "0x120c - Graphics %s Frame Buffer Control Register 2"]
    pub gr2_flm2: GR_FLM2,
    #[doc = "0x1210 - Graphics %s Frame Buffer Control Register 3"]
    pub gr2_flm3: GR_FLM3,
    _reserved35: [u8; 0x04],
    #[doc = "0x1218 - Graphics %s Frame Buffer Control Register 5"]
    pub gr2_flm5: GR_FLM5,
    #[doc = "0x121c - Graphics %s Frame Buffer Control Register 6"]
    pub gr2_flm6: GR_FLM6,
    #[doc = "0x1220 - Graphics %s Alpha Blending Control Register 1"]
    pub gr2_ab1: GR_AB1,
    #[doc = "0x1224 - Graphics %s Alpha Blending Control Register 2"]
    pub gr2_ab2: GR_AB2,
    #[doc = "0x1228 - Graphics %s Alpha Blending Control Register 3"]
    pub gr2_ab3: GR_AB3,
    #[doc = "0x122c - Graphics %s Alpha Blending Control Register 4"]
    pub gr2_ab4: GR_AB4,
    #[doc = "0x1230 - Graphics %s Alpha Blending Control Register 5"]
    pub gr2_ab5: GR_AB5,
    #[doc = "0x1234 - Graphics %s Alpha Blending Control Register 6"]
    pub gr2_ab6: GR_AB6,
    #[doc = "0x1238 - Graphics %s Alpha Blending Control Register 7"]
    pub gr2_ab7: GR_AB7,
    #[doc = "0x123c - Graphics %s Alpha Blending Control Register 8"]
    pub gr2_ab8: GR_AB8,
    #[doc = "0x1240 - Graphics %s Alpha Blending Control Register 9"]
    pub gr2_ab9: GR_AB9,
    _reserved46: [u8; 0x08],
    #[doc = "0x124c - Graphics %s Background Color Control Register"]
    pub gr2_base: GR_BASE,
    #[doc = "0x1250 - Graphics %s CLUT Table Interrupt Control Register"]
    pub gr2_clutint: GR_CLUTINT,
    #[doc = "0x1254 - Graphics %s Status Monitor Register"]
    pub gr2_mon: GR_MON,
    _reserved49: [u8; 0xa8],
    #[doc = "0x1300 - Gamma %s Register Update Control Register"]
    pub gamg_latch: GAM_LATCH,
    #[doc = "0x1304 - Gamma Correction Block Function Switch Register"]
    pub gam_sw: GAM_SW,
    #[doc = "0x1308 - Gamma %s Correction Block Table Setting Register 1"]
    pub gamg_lut1: GAM_LUT1,
    #[doc = "0x130c - Gamma %s Correction Block Table Setting Register 2"]
    pub gamg_lut2: GAM_LUT2,
    #[doc = "0x1310 - Gamma %s Correction Block Table Setting Register 3"]
    pub gamg_lut3: GAM_LUT3,
    #[doc = "0x1314 - Gamma %s Correction Block Table Setting Register 4"]
    pub gamg_lut4: GAM_LUT4,
    #[doc = "0x1318 - Gamma %s Correction Block Table Setting Register 5"]
    pub gamg_lut5: GAM_LUT5,
    #[doc = "0x131c - Gamma %s Correction Block Table Setting Register 6"]
    pub gamg_lut6: GAM_LUT6,
    #[doc = "0x1320 - Gamma %s Correction Block Table Setting Register 7"]
    pub gamg_lut7: GAM_LUT7,
    #[doc = "0x1324 - Gamma %s Correction Block Table Setting Register 8"]
    pub gamg_lut8: GAM_LUT8,
    #[doc = "0x1328 - Gamma %s Correction Block Area Setting Register 1"]
    pub gamg_area1: GAM_AREA1,
    #[doc = "0x132c - Gamma %s Correction Block Area Setting Register 2"]
    pub gamg_area2: GAM_AREA2,
    #[doc = "0x1330 - Gamma %s Correction Block Area Setting Register 3"]
    pub gamg_area3: GAM_AREA3,
    #[doc = "0x1334 - Gamma %s Correction Block Area Setting Register 4"]
    pub gamg_area4: GAM_AREA4,
    #[doc = "0x1338 - Gamma %s Correction Block Area Setting Register 5"]
    pub gamg_area5: GAM_AREA5,
    _reserved64: [u8; 0x04],
    #[doc = "0x1340 - Gamma %s Register Update Control Register"]
    pub gamb_latch: GAM_LATCH,
    _reserved65: [u8; 0x04],
    #[doc = "0x1348 - Gamma %s Correction Block Table Setting Register 1"]
    pub gamb_lut1: GAM_LUT1,
    #[doc = "0x134c - Gamma %s Correction Block Table Setting Register 2"]
    pub gamb_lut2: GAM_LUT2,
    #[doc = "0x1350 - Gamma %s Correction Block Table Setting Register 3"]
    pub gamb_lut3: GAM_LUT3,
    #[doc = "0x1354 - Gamma %s Correction Block Table Setting Register 4"]
    pub gamb_lut4: GAM_LUT4,
    #[doc = "0x1358 - Gamma %s Correction Block Table Setting Register 5"]
    pub gamb_lut5: GAM_LUT5,
    #[doc = "0x135c - Gamma %s Correction Block Table Setting Register 6"]
    pub gamb_lut6: GAM_LUT6,
    #[doc = "0x1360 - Gamma %s Correction Block Table Setting Register 7"]
    pub gamb_lut7: GAM_LUT7,
    #[doc = "0x1364 - Gamma %s Correction Block Table Setting Register 8"]
    pub gamb_lut8: GAM_LUT8,
    #[doc = "0x1368 - Gamma %s Correction Block Area Setting Register 1"]
    pub gamb_area1: GAM_AREA1,
    #[doc = "0x136c - Gamma %s Correction Block Area Setting Register 2"]
    pub gamb_area2: GAM_AREA2,
    #[doc = "0x1370 - Gamma %s Correction Block Area Setting Register 3"]
    pub gamb_area3: GAM_AREA3,
    #[doc = "0x1374 - Gamma %s Correction Block Area Setting Register 4"]
    pub gamb_area4: GAM_AREA4,
    #[doc = "0x1378 - Gamma %s Correction Block Area Setting Register 5"]
    pub gamb_area5: GAM_AREA5,
    _reserved78: [u8; 0x04],
    #[doc = "0x1380 - Gamma %s Register Update Control Register"]
    pub gamr_latch: GAM_LATCH,
    _reserved79: [u8; 0x04],
    #[doc = "0x1388 - Gamma %s Correction Block Table Setting Register 1"]
    pub gamr_lut1: GAM_LUT1,
    #[doc = "0x138c - Gamma %s Correction Block Table Setting Register 2"]
    pub gamr_lut2: GAM_LUT2,
    #[doc = "0x1390 - Gamma %s Correction Block Table Setting Register 3"]
    pub gamr_lut3: GAM_LUT3,
    #[doc = "0x1394 - Gamma %s Correction Block Table Setting Register 4"]
    pub gamr_lut4: GAM_LUT4,
    #[doc = "0x1398 - Gamma %s Correction Block Table Setting Register 5"]
    pub gamr_lut5: GAM_LUT5,
    #[doc = "0x139c - Gamma %s Correction Block Table Setting Register 6"]
    pub gamr_lut6: GAM_LUT6,
    #[doc = "0x13a0 - Gamma %s Correction Block Table Setting Register 7"]
    pub gamr_lut7: GAM_LUT7,
    #[doc = "0x13a4 - Gamma %s Correction Block Table Setting Register 8"]
    pub gamr_lut8: GAM_LUT8,
    #[doc = "0x13a8 - Gamma %s Correction Block Area Setting Register 1"]
    pub gamr_area1: GAM_AREA1,
    #[doc = "0x13ac - Gamma %s Correction Block Area Setting Register 2"]
    pub gamr_area2: GAM_AREA2,
    #[doc = "0x13b0 - Gamma %s Correction Block Area Setting Register 3"]
    pub gamr_area3: GAM_AREA3,
    #[doc = "0x13b4 - Gamma %s Correction Block Area Setting Register 4"]
    pub gamr_area4: GAM_AREA4,
    #[doc = "0x13b8 - Gamma %s Correction Block Area Setting Register 5"]
    pub gamr_area5: GAM_AREA5,
    _reserved92: [u8; 0x04],
    #[doc = "0x13c0 - Output Control Block Register Update Control Register"]
    pub out_vlatch: OUT_VLATCH,
    #[doc = "0x13c4 - Output Control Block Output Interface Register"]
    pub out_set: OUT_SET,
    #[doc = "0x13c8 - Output Control Block Brightness Correction Register 1"]
    pub out_bright1: OUT_BRIGHT1,
    #[doc = "0x13cc - Output Control Block Brightness Correction Register 2"]
    pub out_bright2: OUT_BRIGHT2,
    #[doc = "0x13d0 - Output Control Block Contrast Correction Register"]
    pub out_contrast: OUT_CONTRAST,
    #[doc = "0x13d4 - Output Control Block Panel Dither Correction Register"]
    pub out_pdtha: OUT_PDTHA,
    _reserved98: [u8; 0x0c],
    #[doc = "0x13e4 - Output Control Block Output Phase Control Register"]
    pub out_clkphase: OUT_CLKPHASE,
    _reserved99: [u8; 0x1c],
    #[doc = "0x1404 - TCON Reference Timing Setting Register"]
    pub tcon_tim: TCON_TIM,
    #[doc = "0x1408 - TCON Vertical Timing Setting Register %s1"]
    pub tcon_stva1: TCON_STV1,
    #[doc = "0x140c - TCON Vertical Timing Setting Register %s2"]
    pub tcon_stva2: TCON_STV2,
    #[doc = "0x1410 - TCON Vertical Timing Setting Register %s1"]
    pub tcon_stvb1: TCON_STV1,
    #[doc = "0x1414 - TCON Vertical Timing Setting Register %s2"]
    pub tcon_stvb2: TCON_STV2,
    #[doc = "0x1418 - TCON Horizontal Timing Setting Register STH%s1"]
    pub tcon_stha1: TCON_STH1,
    #[doc = "0x141c - TCON Horizontal Timing Setting Register STH%s2"]
    pub tcon_stha2: TCON_STH2,
    #[doc = "0x1420 - TCON Horizontal Timing Setting Register STH%s1"]
    pub tcon_sthb1: TCON_STH1,
    #[doc = "0x1424 - TCON Horizontal Timing Setting Register STH%s2"]
    pub tcon_sthb2: TCON_STH2,
    #[doc = "0x1428 - TCON Data Enable Polarity Setting Register"]
    pub tcon_de: TCON_DE,
    _reserved109: [u8; 0x14],
    #[doc = "0x1440 - System Control Block State Detection Control Register"]
    pub syscnt_dtcten: SYSCNT_DTCTEN,
    #[doc = "0x1444 - System Control Block Interrupt Request Enable Control Register"]
    pub syscnt_inten: SYSCNT_INTEN,
    #[doc = "0x1448 - System Control Block Status Clear Register"]
    pub syscnt_stclr: SYSCNT_STCLR,
    #[doc = "0x144c - System Control Block Status Monitor Register"]
    pub syscnt_stmon: SYSCNT_STMON,
    #[doc = "0x1450 - System Control Block Version and Panel Clock Control Register"]
    pub syscnt_panel_clk: SYSCNT_PANEL_CLK,
}
#[doc = "GR1_CLUT0 (rw) register accessor: an alias for `Reg<GR1_CLUT0_SPEC>`"]
pub type GR1_CLUT0 = crate::Reg<gr1_clut0::GR1_CLUT0_SPEC>;
#[doc = "Color Palette 0 Plane for Graphics 1 Plane"]
pub mod gr1_clut0;
#[doc = "GR1_CLUT1 (rw) register accessor: an alias for `Reg<GR1_CLUT1_SPEC>`"]
pub type GR1_CLUT1 = crate::Reg<gr1_clut1::GR1_CLUT1_SPEC>;
#[doc = "Color Palette 1 Plane for Graphics 1 Plane"]
pub mod gr1_clut1;
#[doc = "GR2_CLUT0 (rw) register accessor: an alias for `Reg<GR2_CLUT0_SPEC>`"]
pub type GR2_CLUT0 = crate::Reg<gr2_clut0::GR2_CLUT0_SPEC>;
#[doc = "Color Palette 0 Plane for Graphics 2 Plane"]
pub mod gr2_clut0;
#[doc = "GR2_CLUT1 (rw) register accessor: an alias for `Reg<GR2_CLUT1_SPEC>`"]
pub type GR2_CLUT1 = crate::Reg<gr2_clut1::GR2_CLUT1_SPEC>;
#[doc = "Color Palette 1 Plane for Graphics 2 Plane"]
pub mod gr2_clut1;
#[doc = "BG_EN (rw) register accessor: an alias for `Reg<BG_EN_SPEC>`"]
pub type BG_EN = crate::Reg<bg_en::BG_EN_SPEC>;
#[doc = "Background Plane Setting Operation Control Register"]
pub mod bg_en;
#[doc = "BG_PERI (rw) register accessor: an alias for `Reg<BG_PERI_SPEC>`"]
pub type BG_PERI = crate::Reg<bg_peri::BG_PERI_SPEC>;
#[doc = "Background Plane Setting Free-Running Period Register"]
pub mod bg_peri;
#[doc = "BG_SYNC (rw) register accessor: an alias for `Reg<BG_SYNC_SPEC>`"]
pub type BG_SYNC = crate::Reg<bg_sync::BG_SYNC_SPEC>;
#[doc = "Background Plane Setting Synchronization Position Register"]
pub mod bg_sync;
#[doc = "BG_VSIZE (rw) register accessor: an alias for `Reg<BG_VSIZE_SPEC>`"]
pub type BG_VSIZE = crate::Reg<bg_vsize::BG_VSIZE_SPEC>;
#[doc = "Background Plane Setting Full Image Vertical Size Register"]
pub mod bg_vsize;
#[doc = "BG_HSIZE (rw) register accessor: an alias for `Reg<BG_HSIZE_SPEC>`"]
pub type BG_HSIZE = crate::Reg<bg_hsize::BG_HSIZE_SPEC>;
#[doc = "Background Plane Setting Full Image Horizontal Size Register"]
pub mod bg_hsize;
#[doc = "BG_BGC (rw) register accessor: an alias for `Reg<BG_BGC_SPEC>`"]
pub type BG_BGC = crate::Reg<bg_bgc::BG_BGC_SPEC>;
#[doc = "Background Plane Setting Background Color Register"]
pub mod bg_bgc;
#[doc = "BG_MON (r) register accessor: an alias for `Reg<BG_MON_SPEC>`"]
pub type BG_MON = crate::Reg<bg_mon::BG_MON_SPEC>;
#[doc = "Background Plane Setting Status Monitor Register"]
pub mod bg_mon;
#[doc = "GR_VEN (rw) register accessor: an alias for `Reg<GR_VEN_SPEC>`"]
pub type GR_VEN = crate::Reg<gr_ven::GR_VEN_SPEC>;
#[doc = "Graphics %s Register Update Control Register"]
pub mod gr_ven;
#[doc = "GR_FLMRD (rw) register accessor: an alias for `Reg<GR_FLMRD_SPEC>`"]
pub type GR_FLMRD = crate::Reg<gr_flmrd::GR_FLMRD_SPEC>;
#[doc = "Graphics %s Frame Buffer Read Control Register"]
pub mod gr_flmrd;
#[doc = "GR_FLM1 (rw) register accessor: an alias for `Reg<GR_FLM1_SPEC>`"]
pub type GR_FLM1 = crate::Reg<gr_flm1::GR_FLM1_SPEC>;
#[doc = "Graphics %s Frame Buffer Control Register 1"]
pub mod gr_flm1;
#[doc = "GR_FLM2 (rw) register accessor: an alias for `Reg<GR_FLM2_SPEC>`"]
pub type GR_FLM2 = crate::Reg<gr_flm2::GR_FLM2_SPEC>;
#[doc = "Graphics %s Frame Buffer Control Register 2"]
pub mod gr_flm2;
#[doc = "GR_FLM3 (rw) register accessor: an alias for `Reg<GR_FLM3_SPEC>`"]
pub type GR_FLM3 = crate::Reg<gr_flm3::GR_FLM3_SPEC>;
#[doc = "Graphics %s Frame Buffer Control Register 3"]
pub mod gr_flm3;
#[doc = "GR_FLM5 (rw) register accessor: an alias for `Reg<GR_FLM5_SPEC>`"]
pub type GR_FLM5 = crate::Reg<gr_flm5::GR_FLM5_SPEC>;
#[doc = "Graphics %s Frame Buffer Control Register 5"]
pub mod gr_flm5;
#[doc = "GR_FLM6 (rw) register accessor: an alias for `Reg<GR_FLM6_SPEC>`"]
pub type GR_FLM6 = crate::Reg<gr_flm6::GR_FLM6_SPEC>;
#[doc = "Graphics %s Frame Buffer Control Register 6"]
pub mod gr_flm6;
#[doc = "GR_AB1 (rw) register accessor: an alias for `Reg<GR_AB1_SPEC>`"]
pub type GR_AB1 = crate::Reg<gr_ab1::GR_AB1_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 1"]
pub mod gr_ab1;
#[doc = "GR_AB2 (rw) register accessor: an alias for `Reg<GR_AB2_SPEC>`"]
pub type GR_AB2 = crate::Reg<gr_ab2::GR_AB2_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 2"]
pub mod gr_ab2;
#[doc = "GR_AB3 (rw) register accessor: an alias for `Reg<GR_AB3_SPEC>`"]
pub type GR_AB3 = crate::Reg<gr_ab3::GR_AB3_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 3"]
pub mod gr_ab3;
#[doc = "GR_AB4 (rw) register accessor: an alias for `Reg<GR_AB4_SPEC>`"]
pub type GR_AB4 = crate::Reg<gr_ab4::GR_AB4_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 4"]
pub mod gr_ab4;
#[doc = "GR_AB5 (rw) register accessor: an alias for `Reg<GR_AB5_SPEC>`"]
pub type GR_AB5 = crate::Reg<gr_ab5::GR_AB5_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 5"]
pub mod gr_ab5;
#[doc = "GR_AB6 (rw) register accessor: an alias for `Reg<GR_AB6_SPEC>`"]
pub type GR_AB6 = crate::Reg<gr_ab6::GR_AB6_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 6"]
pub mod gr_ab6;
#[doc = "GR_AB7 (rw) register accessor: an alias for `Reg<GR_AB7_SPEC>`"]
pub type GR_AB7 = crate::Reg<gr_ab7::GR_AB7_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 7"]
pub mod gr_ab7;
#[doc = "GR_AB8 (rw) register accessor: an alias for `Reg<GR_AB8_SPEC>`"]
pub type GR_AB8 = crate::Reg<gr_ab8::GR_AB8_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 8"]
pub mod gr_ab8;
#[doc = "GR_AB9 (rw) register accessor: an alias for `Reg<GR_AB9_SPEC>`"]
pub type GR_AB9 = crate::Reg<gr_ab9::GR_AB9_SPEC>;
#[doc = "Graphics %s Alpha Blending Control Register 9"]
pub mod gr_ab9;
#[doc = "GR_BASE (rw) register accessor: an alias for `Reg<GR_BASE_SPEC>`"]
pub type GR_BASE = crate::Reg<gr_base::GR_BASE_SPEC>;
#[doc = "Graphics %s Background Color Control Register"]
pub mod gr_base;
#[doc = "GR_CLUTINT (rw) register accessor: an alias for `Reg<GR_CLUTINT_SPEC>`"]
pub type GR_CLUTINT = crate::Reg<gr_clutint::GR_CLUTINT_SPEC>;
#[doc = "Graphics %s CLUT Table Interrupt Control Register"]
pub mod gr_clutint;
#[doc = "GR_MON (r) register accessor: an alias for `Reg<GR_MON_SPEC>`"]
pub type GR_MON = crate::Reg<gr_mon::GR_MON_SPEC>;
#[doc = "Graphics %s Status Monitor Register"]
pub mod gr_mon;
#[doc = "GAM_LATCH (rw) register accessor: an alias for `Reg<GAM_LATCH_SPEC>`"]
pub type GAM_LATCH = crate::Reg<gam_latch::GAM_LATCH_SPEC>;
#[doc = "Gamma %s Register Update Control Register"]
pub mod gam_latch;
#[doc = "GAM_SW (rw) register accessor: an alias for `Reg<GAM_SW_SPEC>`"]
pub type GAM_SW = crate::Reg<gam_sw::GAM_SW_SPEC>;
#[doc = "Gamma Correction Block Function Switch Register"]
pub mod gam_sw;
#[doc = "GAM_LUT1 (rw) register accessor: an alias for `Reg<GAM_LUT1_SPEC>`"]
pub type GAM_LUT1 = crate::Reg<gam_lut1::GAM_LUT1_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 1"]
pub mod gam_lut1;
#[doc = "GAM_LUT2 (rw) register accessor: an alias for `Reg<GAM_LUT2_SPEC>`"]
pub type GAM_LUT2 = crate::Reg<gam_lut2::GAM_LUT2_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 2"]
pub mod gam_lut2;
#[doc = "GAM_LUT3 (rw) register accessor: an alias for `Reg<GAM_LUT3_SPEC>`"]
pub type GAM_LUT3 = crate::Reg<gam_lut3::GAM_LUT3_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 3"]
pub mod gam_lut3;
#[doc = "GAM_LUT4 (rw) register accessor: an alias for `Reg<GAM_LUT4_SPEC>`"]
pub type GAM_LUT4 = crate::Reg<gam_lut4::GAM_LUT4_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 4"]
pub mod gam_lut4;
#[doc = "GAM_LUT5 (rw) register accessor: an alias for `Reg<GAM_LUT5_SPEC>`"]
pub type GAM_LUT5 = crate::Reg<gam_lut5::GAM_LUT5_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 5"]
pub mod gam_lut5;
#[doc = "GAM_LUT6 (rw) register accessor: an alias for `Reg<GAM_LUT6_SPEC>`"]
pub type GAM_LUT6 = crate::Reg<gam_lut6::GAM_LUT6_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 6"]
pub mod gam_lut6;
#[doc = "GAM_LUT7 (rw) register accessor: an alias for `Reg<GAM_LUT7_SPEC>`"]
pub type GAM_LUT7 = crate::Reg<gam_lut7::GAM_LUT7_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 7"]
pub mod gam_lut7;
#[doc = "GAM_LUT8 (rw) register accessor: an alias for `Reg<GAM_LUT8_SPEC>`"]
pub type GAM_LUT8 = crate::Reg<gam_lut8::GAM_LUT8_SPEC>;
#[doc = "Gamma %s Correction Block Table Setting Register 8"]
pub mod gam_lut8;
#[doc = "GAM_AREA1 (rw) register accessor: an alias for `Reg<GAM_AREA1_SPEC>`"]
pub type GAM_AREA1 = crate::Reg<gam_area1::GAM_AREA1_SPEC>;
#[doc = "Gamma %s Correction Block Area Setting Register 1"]
pub mod gam_area1;
#[doc = "GAM_AREA2 (rw) register accessor: an alias for `Reg<GAM_AREA2_SPEC>`"]
pub type GAM_AREA2 = crate::Reg<gam_area2::GAM_AREA2_SPEC>;
#[doc = "Gamma %s Correction Block Area Setting Register 2"]
pub mod gam_area2;
#[doc = "GAM_AREA3 (rw) register accessor: an alias for `Reg<GAM_AREA3_SPEC>`"]
pub type GAM_AREA3 = crate::Reg<gam_area3::GAM_AREA3_SPEC>;
#[doc = "Gamma %s Correction Block Area Setting Register 3"]
pub mod gam_area3;
#[doc = "GAM_AREA4 (rw) register accessor: an alias for `Reg<GAM_AREA4_SPEC>`"]
pub type GAM_AREA4 = crate::Reg<gam_area4::GAM_AREA4_SPEC>;
#[doc = "Gamma %s Correction Block Area Setting Register 4"]
pub mod gam_area4;
#[doc = "GAM_AREA5 (rw) register accessor: an alias for `Reg<GAM_AREA5_SPEC>`"]
pub type GAM_AREA5 = crate::Reg<gam_area5::GAM_AREA5_SPEC>;
#[doc = "Gamma %s Correction Block Area Setting Register 5"]
pub mod gam_area5;
#[doc = "OUT_VLATCH (rw) register accessor: an alias for `Reg<OUT_VLATCH_SPEC>`"]
pub type OUT_VLATCH = crate::Reg<out_vlatch::OUT_VLATCH_SPEC>;
#[doc = "Output Control Block Register Update Control Register"]
pub mod out_vlatch;
#[doc = "OUT_SET (rw) register accessor: an alias for `Reg<OUT_SET_SPEC>`"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "Output Control Block Output Interface Register"]
pub mod out_set;
#[doc = "OUT_BRIGHT1 (rw) register accessor: an alias for `Reg<OUT_BRIGHT1_SPEC>`"]
pub type OUT_BRIGHT1 = crate::Reg<out_bright1::OUT_BRIGHT1_SPEC>;
#[doc = "Output Control Block Brightness Correction Register 1"]
pub mod out_bright1;
#[doc = "OUT_BRIGHT2 (rw) register accessor: an alias for `Reg<OUT_BRIGHT2_SPEC>`"]
pub type OUT_BRIGHT2 = crate::Reg<out_bright2::OUT_BRIGHT2_SPEC>;
#[doc = "Output Control Block Brightness Correction Register 2"]
pub mod out_bright2;
#[doc = "OUT_CONTRAST (rw) register accessor: an alias for `Reg<OUT_CONTRAST_SPEC>`"]
pub type OUT_CONTRAST = crate::Reg<out_contrast::OUT_CONTRAST_SPEC>;
#[doc = "Output Control Block Contrast Correction Register"]
pub mod out_contrast;
#[doc = "OUT_PDTHA (rw) register accessor: an alias for `Reg<OUT_PDTHA_SPEC>`"]
pub type OUT_PDTHA = crate::Reg<out_pdtha::OUT_PDTHA_SPEC>;
#[doc = "Output Control Block Panel Dither Correction Register"]
pub mod out_pdtha;
#[doc = "OUT_CLKPHASE (rw) register accessor: an alias for `Reg<OUT_CLKPHASE_SPEC>`"]
pub type OUT_CLKPHASE = crate::Reg<out_clkphase::OUT_CLKPHASE_SPEC>;
#[doc = "Output Control Block Output Phase Control Register"]
pub mod out_clkphase;
#[doc = "TCON_TIM (rw) register accessor: an alias for `Reg<TCON_TIM_SPEC>`"]
pub type TCON_TIM = crate::Reg<tcon_tim::TCON_TIM_SPEC>;
#[doc = "TCON Reference Timing Setting Register"]
pub mod tcon_tim;
#[doc = "TCON_STV1 (rw) register accessor: an alias for `Reg<TCON_STV1_SPEC>`"]
pub type TCON_STV1 = crate::Reg<tcon_stv1::TCON_STV1_SPEC>;
#[doc = "TCON Vertical Timing Setting Register %s1"]
pub mod tcon_stv1;
#[doc = "TCON_STV2 (rw) register accessor: an alias for `Reg<TCON_STV2_SPEC>`"]
pub type TCON_STV2 = crate::Reg<tcon_stv2::TCON_STV2_SPEC>;
#[doc = "TCON Vertical Timing Setting Register %s2"]
pub mod tcon_stv2;
#[doc = "TCON_STH1 (rw) register accessor: an alias for `Reg<TCON_STH1_SPEC>`"]
pub type TCON_STH1 = crate::Reg<tcon_sth1::TCON_STH1_SPEC>;
#[doc = "TCON Horizontal Timing Setting Register STH%s1"]
pub mod tcon_sth1;
#[doc = "TCON_STH2 (rw) register accessor: an alias for `Reg<TCON_STH2_SPEC>`"]
pub type TCON_STH2 = crate::Reg<tcon_sth2::TCON_STH2_SPEC>;
#[doc = "TCON Horizontal Timing Setting Register STH%s2"]
pub mod tcon_sth2;
#[doc = "TCON_DE (rw) register accessor: an alias for `Reg<TCON_DE_SPEC>`"]
pub type TCON_DE = crate::Reg<tcon_de::TCON_DE_SPEC>;
#[doc = "TCON Data Enable Polarity Setting Register"]
pub mod tcon_de;
#[doc = "SYSCNT_DTCTEN (rw) register accessor: an alias for `Reg<SYSCNT_DTCTEN_SPEC>`"]
pub type SYSCNT_DTCTEN = crate::Reg<syscnt_dtcten::SYSCNT_DTCTEN_SPEC>;
#[doc = "System Control Block State Detection Control Register"]
pub mod syscnt_dtcten;
#[doc = "SYSCNT_INTEN (rw) register accessor: an alias for `Reg<SYSCNT_INTEN_SPEC>`"]
pub type SYSCNT_INTEN = crate::Reg<syscnt_inten::SYSCNT_INTEN_SPEC>;
#[doc = "System Control Block Interrupt Request Enable Control Register"]
pub mod syscnt_inten;
#[doc = "SYSCNT_STCLR (rw) register accessor: an alias for `Reg<SYSCNT_STCLR_SPEC>`"]
pub type SYSCNT_STCLR = crate::Reg<syscnt_stclr::SYSCNT_STCLR_SPEC>;
#[doc = "System Control Block Status Clear Register"]
pub mod syscnt_stclr;
#[doc = "SYSCNT_STMON (r) register accessor: an alias for `Reg<SYSCNT_STMON_SPEC>`"]
pub type SYSCNT_STMON = crate::Reg<syscnt_stmon::SYSCNT_STMON_SPEC>;
#[doc = "System Control Block Status Monitor Register"]
pub mod syscnt_stmon;
#[doc = "SYSCNT_PANEL_CLK (rw) register accessor: an alias for `Reg<SYSCNT_PANEL_CLK_SPEC>`"]
pub type SYSCNT_PANEL_CLK = crate::Reg<syscnt_panel_clk::SYSCNT_PANEL_CLK_SPEC>;
#[doc = "System Control Block Version and Panel Clock Control Register"]
pub mod syscnt_panel_clk;
