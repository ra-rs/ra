#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_p00: [u8; 0x08],
    _reserved_1_p002: [u8; 0x04],
    _reserved2: [u8; 0x78],
    _reserved_2_p201: [u8; 0x04],
    _reserved3: [u8; 0x01f8],
    _reserved_3_pa0: [u8; 0x18],
    _reserved_4_pa06: [u8; 0x04],
    _reserved_5_pa07: [u8; 0x04],
    _reserved_6_pa08: [u8; 0x04],
    _reserved_7_pa09: [u8; 0x04],
    _reserved_8_pa1: [u8; 0x0c],
    _reserved_9_pa13: [u8; 0x04],
    _reserved_10_pa14: [u8; 0x04],
    _reserved_11_pa15: [u8; 0x04],
    _reserved_12_pb0: [u8; 0x08],
    _reserved_13_pb02: [u8; 0x04],
    _reserved_14_pb03: [u8; 0x04],
    _reserved_15_pb04: [u8; 0x04],
    _reserved_16_pb05: [u8; 0x04],
    _reserved_17_pb06: [u8; 0x04],
    _reserved_18_pb07: [u8; 0x04],
    _reserved_19_pb08: [u8; 0x04],
    _reserved_20_pb09: [u8; 0x04],
    _reserved_21_pb10: [u8; 0x04],
    _reserved22: [u8; 0x04],
    _reserved_22_pb1: [u8; 0x10],
    _reserved_23_pc0: [u8; 0x28],
    _reserved_24_pc1: [u8; 0x18],
    _reserved_25_pd0: [u8; 0x28],
    _reserved_26_pd1: [u8; 0x18],
    _reserved_27_pe0: [u8; 0x1c],
    _reserved28: [u8; 0x04],
    _reserved_28_pe08: [u8; 0x04],
    _reserved_29_pe09: [u8; 0x04],
    _reserved_30_pe1: [u8; 0x18],
    _reserved31: [u8; 0x014c],
    #[doc = "0x50c - Write-Protect Register"]
    pub pwpr: PWPR,
    _reserved32: [u8; 0x07],
    #[doc = "0x514 - Write-Protect Register for Secure"]
    pub pwprs: PWPRS,
    _reserved33: [u8; 0x1b],
    #[doc = "0x530 - Port 0 Security Attribution register"]
    pub p0sar: P0SAR,
    _reserved34: [u8; 0x06],
    #[doc = "0x538 - Port 2 Security Attribution register"]
    pub p2sar: P2SAR,
    _reserved35: [u8; 0x1e],
    #[doc = "0x558 - Port A Security Attribution register"]
    pub pasar: PASAR,
    _reserved36: [u8; 0x02],
    #[doc = "0x55c - Port B Security Attribution register"]
    pub pbsar: PBSAR,
    _reserved37: [u8; 0x02],
    #[doc = "0x560 - Port C Security Attribution register"]
    pub pcsar: PCSAR,
    _reserved38: [u8; 0x02],
    #[doc = "0x564 - Port D Security Attribution register"]
    pub pdsar: PDSAR,
    _reserved39: [u8; 0x02],
    #[doc = "0x568 - Port E Security Attribution register"]
    pub pesar: PESAR,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p000pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x08 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs(&self) -> &[P00PFS; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p001pfs_by(&self) -> &P00PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p001pfs_ha(&self) -> &P00PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_by(&self) -> &P002PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_ha(&self) -> &P002PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs(&self) -> &P002PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x84 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_by(&self) -> &P201PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x84 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(&self) -> &P201PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x84 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P201PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x280 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa00pfs_by(&self) -> &PA0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(640usize).cast() }
    }
    #[doc = "0x280 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa00pfs_ha(&self) -> &PA0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(640usize).cast() }
    }
    #[doc = "0x280..0x298 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa0pfs(&self) -> &[PA0PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(640usize).cast() }
    }
    #[doc = "0x284 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa01pfs_by(&self) -> &PA0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(644usize).cast() }
    }
    #[doc = "0x284 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa01pfs_ha(&self) -> &PA0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(644usize).cast() }
    }
    #[doc = "0x288 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa02pfs_by(&self) -> &PA0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(648usize).cast() }
    }
    #[doc = "0x288 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa02pfs_ha(&self) -> &PA0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(648usize).cast() }
    }
    #[doc = "0x28c - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa03pfs_by(&self) -> &PA0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(652usize).cast() }
    }
    #[doc = "0x28c - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa03pfs_ha(&self) -> &PA0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(652usize).cast() }
    }
    #[doc = "0x290 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa04pfs_by(&self) -> &PA0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(656usize).cast() }
    }
    #[doc = "0x290 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa04pfs_ha(&self) -> &PA0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(656usize).cast() }
    }
    #[doc = "0x294 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa05pfs_by(&self) -> &PA0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(660usize).cast() }
    }
    #[doc = "0x294 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa05pfs_ha(&self) -> &PA0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(660usize).cast() }
    }
    #[doc = "0x298 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa06pfs_by(&self) -> &PA06PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(664usize).cast() }
    }
    #[doc = "0x298 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa06pfs_ha(&self) -> &PA06PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(664usize).cast() }
    }
    #[doc = "0x298 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa06pfs(&self) -> &PA06PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(664usize).cast() }
    }
    #[doc = "0x29c - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa07pfs_by(&self) -> &PA06PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(668usize).cast() }
    }
    #[doc = "0x29c - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa07pfs_ha(&self) -> &PA06PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(668usize).cast() }
    }
    #[doc = "0x29c - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa07pfs(&self) -> &PA06PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(668usize).cast() }
    }
    #[doc = "0x2a0 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa08pfs_by(&self) -> &PA06PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(672usize).cast() }
    }
    #[doc = "0x2a0 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa08pfs_ha(&self) -> &PA06PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(672usize).cast() }
    }
    #[doc = "0x2a0 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa08pfs(&self) -> &PA06PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(672usize).cast() }
    }
    #[doc = "0x2a4 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa09pfs_by(&self) -> &PA06PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(676usize).cast() }
    }
    #[doc = "0x2a4 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa09pfs_ha(&self) -> &PA06PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(676usize).cast() }
    }
    #[doc = "0x2a4 - Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa09pfs(&self) -> &PA06PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(676usize).cast() }
    }
    #[doc = "0x2a8 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa10pfs_by(&self) -> &PA1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(680usize).cast() }
    }
    #[doc = "0x2a8 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa10pfs_ha(&self) -> &PA1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(680usize).cast() }
    }
    #[doc = "0x2a8..0x2b4 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa1pfs(&self) -> &[PA1PFS; 3] {
        unsafe { &*(self as *const Self).cast::<u8>().add(680usize).cast() }
    }
    #[doc = "0x2ac - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa11pfs_by(&self) -> &PA1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(684usize).cast() }
    }
    #[doc = "0x2ac - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa11pfs_ha(&self) -> &PA1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(684usize).cast() }
    }
    #[doc = "0x2b0 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa12pfs_by(&self) -> &PA1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(688usize).cast() }
    }
    #[doc = "0x2b0 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa12pfs_ha(&self) -> &PA1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(688usize).cast() }
    }
    #[doc = "0x2b4 - Port A13 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa13pfs_by(&self) -> &PA13PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(692usize).cast() }
    }
    #[doc = "0x2b4 - Port A13 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa13pfs_ha(&self) -> &PA13PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(692usize).cast() }
    }
    #[doc = "0x2b4 - Port A13 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa13pfs(&self) -> &PA13PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(692usize).cast() }
    }
    #[doc = "0x2b8 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa14pfs_by(&self) -> &PA14PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(696usize).cast() }
    }
    #[doc = "0x2b8 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa14pfs_ha(&self) -> &PA14PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(696usize).cast() }
    }
    #[doc = "0x2b8 - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa14pfs(&self) -> &PA14PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(696usize).cast() }
    }
    #[doc = "0x2bc - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa15pfs_by(&self) -> &PA14PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(700usize).cast() }
    }
    #[doc = "0x2bc - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa15pfs_ha(&self) -> &PA14PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(700usize).cast() }
    }
    #[doc = "0x2bc - Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa15pfs(&self) -> &PA14PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(700usize).cast() }
    }
    #[doc = "0x2c0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb00pfs_by(&self) -> &PB0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(704usize).cast() }
    }
    #[doc = "0x2c0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb00pfs_ha(&self) -> &PB0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(704usize).cast() }
    }
    #[doc = "0x2c0..0x2c8 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb0pfs(&self) -> &[PB0PFS; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(704usize).cast() }
    }
    #[doc = "0x2c4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb01pfs_by(&self) -> &PB0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(708usize).cast() }
    }
    #[doc = "0x2c4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb01pfs_ha(&self) -> &PB0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(708usize).cast() }
    }
    #[doc = "0x2c8 - Port B02 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb02pfs_by(&self) -> &PB02PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(712usize).cast() }
    }
    #[doc = "0x2c8 - Port B02 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb02pfs_ha(&self) -> &PB02PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(712usize).cast() }
    }
    #[doc = "0x2c8 - Port B02 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb02pfs(&self) -> &PB02PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(712usize).cast() }
    }
    #[doc = "0x2cc - Port B03 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb03pfs_by(&self) -> &PB03PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(716usize).cast() }
    }
    #[doc = "0x2cc - Port B03 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb03pfs_ha(&self) -> &PB03PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(716usize).cast() }
    }
    #[doc = "0x2cc - Port B03 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb03pfs(&self) -> &PB03PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(716usize).cast() }
    }
    #[doc = "0x2d0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb04pfs_by(&self) -> &PB04PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(720usize).cast() }
    }
    #[doc = "0x2d0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb04pfs_ha(&self) -> &PB04PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(720usize).cast() }
    }
    #[doc = "0x2d0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb04pfs(&self) -> &PB04PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(720usize).cast() }
    }
    #[doc = "0x2d4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb05pfs_by(&self) -> &PB04PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(724usize).cast() }
    }
    #[doc = "0x2d4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb05pfs_ha(&self) -> &PB04PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(724usize).cast() }
    }
    #[doc = "0x2d4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb05pfs(&self) -> &PB04PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(724usize).cast() }
    }
    #[doc = "0x2d8 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb06pfs_by(&self) -> &PB04PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(728usize).cast() }
    }
    #[doc = "0x2d8 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb06pfs_ha(&self) -> &PB04PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(728usize).cast() }
    }
    #[doc = "0x2d8 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb06pfs(&self) -> &PB04PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(728usize).cast() }
    }
    #[doc = "0x2dc - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb07pfs_by(&self) -> &PB04PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(732usize).cast() }
    }
    #[doc = "0x2dc - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb07pfs_ha(&self) -> &PB04PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(732usize).cast() }
    }
    #[doc = "0x2dc - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb07pfs(&self) -> &PB04PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(732usize).cast() }
    }
    #[doc = "0x2e0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb08pfs_by(&self) -> &PB04PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(736usize).cast() }
    }
    #[doc = "0x2e0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb08pfs_ha(&self) -> &PB04PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(736usize).cast() }
    }
    #[doc = "0x2e0 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb08pfs(&self) -> &PB04PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(736usize).cast() }
    }
    #[doc = "0x2e4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb09pfs_by(&self) -> &PB04PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(740usize).cast() }
    }
    #[doc = "0x2e4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb09pfs_ha(&self) -> &PB04PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(740usize).cast() }
    }
    #[doc = "0x2e4 - Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb09pfs(&self) -> &PB04PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(740usize).cast() }
    }
    #[doc = "0x2e8 - Port B10 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb10pfs_by(&self) -> &PB10PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(744usize).cast() }
    }
    #[doc = "0x2e8 - Port B10 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb10pfs_ha(&self) -> &PB10PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(744usize).cast() }
    }
    #[doc = "0x2e8 - Port B10 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb10pfs(&self) -> &PB10PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(744usize).cast() }
    }
    #[doc = "0x2f0 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb12pfs_by(&self) -> &PB1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(752usize).cast() }
    }
    #[doc = "0x2f0 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb12pfs_ha(&self) -> &PB1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(752usize).cast() }
    }
    #[doc = "0x2f0..0x300 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb1pfs(&self) -> &[PB1PFS; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(752usize).cast() }
    }
    #[doc = "0x2f0 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn pb12pfs(&self) -> &PB1PFS {
        &self.pb1pfs()[0]
    }
    #[doc = "0x2f4 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn pb13pfs(&self) -> &PB1PFS {
        &self.pb1pfs()[1]
    }
    #[doc = "0x2f8 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn pb14pfs(&self) -> &PB1PFS {
        &self.pb1pfs()[2]
    }
    #[doc = "0x2fc - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn pb15pfs(&self) -> &PB1PFS {
        &self.pb1pfs()[3]
    }
    #[doc = "0x2f4 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb13pfs_by(&self) -> &PB1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(756usize).cast() }
    }
    #[doc = "0x2f4 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb13pfs_ha(&self) -> &PB1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(756usize).cast() }
    }
    #[doc = "0x2f8 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb14pfs_by(&self) -> &PB1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(760usize).cast() }
    }
    #[doc = "0x2f8 - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb14pfs_ha(&self) -> &PB1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(760usize).cast() }
    }
    #[doc = "0x2fc - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb15pfs_by(&self) -> &PB1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(764usize).cast() }
    }
    #[doc = "0x2fc - Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb15pfs_ha(&self) -> &PB1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(764usize).cast() }
    }
    #[doc = "0x300 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc00pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(768usize).cast() }
    }
    #[doc = "0x300 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc00pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(768usize).cast() }
    }
    #[doc = "0x300..0x328 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc0pfs(&self) -> &[PC0PFS; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(768usize).cast() }
    }
    #[doc = "0x304 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc01pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(772usize).cast() }
    }
    #[doc = "0x304 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc01pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(772usize).cast() }
    }
    #[doc = "0x308 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc02pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(776usize).cast() }
    }
    #[doc = "0x308 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc02pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(776usize).cast() }
    }
    #[doc = "0x30c - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc03pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(780usize).cast() }
    }
    #[doc = "0x30c - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc03pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(780usize).cast() }
    }
    #[doc = "0x310 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc04pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(784usize).cast() }
    }
    #[doc = "0x310 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc04pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(784usize).cast() }
    }
    #[doc = "0x314 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc05pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(788usize).cast() }
    }
    #[doc = "0x314 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc05pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(788usize).cast() }
    }
    #[doc = "0x318 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc06pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(792usize).cast() }
    }
    #[doc = "0x318 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc06pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(792usize).cast() }
    }
    #[doc = "0x31c - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc07pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(796usize).cast() }
    }
    #[doc = "0x31c - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc07pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(796usize).cast() }
    }
    #[doc = "0x320 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc08pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(800usize).cast() }
    }
    #[doc = "0x320 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc08pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(800usize).cast() }
    }
    #[doc = "0x324 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc09pfs_by(&self) -> &PC0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(804usize).cast() }
    }
    #[doc = "0x324 - Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc09pfs_ha(&self) -> &PC0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(804usize).cast() }
    }
    #[doc = "0x328 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc10pfs_by(&self) -> &PC1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(808usize).cast() }
    }
    #[doc = "0x328 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc10pfs_ha(&self) -> &PC1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(808usize).cast() }
    }
    #[doc = "0x328..0x340 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc1pfs(&self) -> &[PC1PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(808usize).cast() }
    }
    #[doc = "0x32c - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc11pfs_by(&self) -> &PC1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(812usize).cast() }
    }
    #[doc = "0x32c - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc11pfs_ha(&self) -> &PC1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(812usize).cast() }
    }
    #[doc = "0x330 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc12pfs_by(&self) -> &PC1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(816usize).cast() }
    }
    #[doc = "0x330 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc12pfs_ha(&self) -> &PC1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(816usize).cast() }
    }
    #[doc = "0x334 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc13pfs_by(&self) -> &PC1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(820usize).cast() }
    }
    #[doc = "0x334 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc13pfs_ha(&self) -> &PC1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(820usize).cast() }
    }
    #[doc = "0x338 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc14pfs_by(&self) -> &PC1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(824usize).cast() }
    }
    #[doc = "0x338 - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc14pfs_ha(&self) -> &PC1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(824usize).cast() }
    }
    #[doc = "0x33c - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc15pfs_by(&self) -> &PC1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(828usize).cast() }
    }
    #[doc = "0x33c - Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc15pfs_ha(&self) -> &PC1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(828usize).cast() }
    }
    #[doc = "0x340 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd00pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(832usize).cast() }
    }
    #[doc = "0x340 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd00pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(832usize).cast() }
    }
    #[doc = "0x340..0x368 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd0pfs(&self) -> &[PD0PFS; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(832usize).cast() }
    }
    #[doc = "0x344 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd01pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(836usize).cast() }
    }
    #[doc = "0x344 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd01pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(836usize).cast() }
    }
    #[doc = "0x348 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd02pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(840usize).cast() }
    }
    #[doc = "0x348 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd02pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(840usize).cast() }
    }
    #[doc = "0x34c - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd03pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(844usize).cast() }
    }
    #[doc = "0x34c - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd03pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(844usize).cast() }
    }
    #[doc = "0x350 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd04pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(848usize).cast() }
    }
    #[doc = "0x350 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd04pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(848usize).cast() }
    }
    #[doc = "0x354 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd05pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(852usize).cast() }
    }
    #[doc = "0x354 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd05pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(852usize).cast() }
    }
    #[doc = "0x358 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd06pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(856usize).cast() }
    }
    #[doc = "0x358 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd06pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(856usize).cast() }
    }
    #[doc = "0x35c - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd07pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(860usize).cast() }
    }
    #[doc = "0x35c - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd07pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(860usize).cast() }
    }
    #[doc = "0x360 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd08pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(864usize).cast() }
    }
    #[doc = "0x360 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd08pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(864usize).cast() }
    }
    #[doc = "0x364 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd09pfs_by(&self) -> &PD0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(868usize).cast() }
    }
    #[doc = "0x364 - Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd09pfs_ha(&self) -> &PD0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(868usize).cast() }
    }
    #[doc = "0x368 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd10pfs_by(&self) -> &PD1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(872usize).cast() }
    }
    #[doc = "0x368 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd10pfs_ha(&self) -> &PD1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(872usize).cast() }
    }
    #[doc = "0x368..0x380 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd1pfs(&self) -> &[PD1PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(872usize).cast() }
    }
    #[doc = "0x36c - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd11pfs_by(&self) -> &PD1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(876usize).cast() }
    }
    #[doc = "0x36c - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd11pfs_ha(&self) -> &PD1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(876usize).cast() }
    }
    #[doc = "0x370 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd12pfs_by(&self) -> &PD1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(880usize).cast() }
    }
    #[doc = "0x370 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd12pfs_ha(&self) -> &PD1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(880usize).cast() }
    }
    #[doc = "0x374 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd13pfs_by(&self) -> &PD1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(884usize).cast() }
    }
    #[doc = "0x374 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd13pfs_ha(&self) -> &PD1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(884usize).cast() }
    }
    #[doc = "0x378 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd14pfs_by(&self) -> &PD1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(888usize).cast() }
    }
    #[doc = "0x378 - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd14pfs_ha(&self) -> &PD1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(888usize).cast() }
    }
    #[doc = "0x37c - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd15pfs_by(&self) -> &PD1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(892usize).cast() }
    }
    #[doc = "0x37c - Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd15pfs_ha(&self) -> &PD1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(892usize).cast() }
    }
    #[doc = "0x380 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe00pfs_by(&self) -> &PE0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(896usize).cast() }
    }
    #[doc = "0x380 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe00pfs_ha(&self) -> &PE0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(896usize).cast() }
    }
    #[doc = "0x380..0x39c - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe0pfs(&self) -> &[PE0PFS; 7] {
        unsafe { &*(self as *const Self).cast::<u8>().add(896usize).cast() }
    }
    #[doc = "0x384 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe01pfs_by(&self) -> &PE0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(900usize).cast() }
    }
    #[doc = "0x384 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe01pfs_ha(&self) -> &PE0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(900usize).cast() }
    }
    #[doc = "0x388 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe02pfs_by(&self) -> &PE0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(904usize).cast() }
    }
    #[doc = "0x388 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe02pfs_ha(&self) -> &PE0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(904usize).cast() }
    }
    #[doc = "0x38c - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe03pfs_by(&self) -> &PE0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(908usize).cast() }
    }
    #[doc = "0x38c - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe03pfs_ha(&self) -> &PE0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(908usize).cast() }
    }
    #[doc = "0x390 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe04pfs_by(&self) -> &PE0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(912usize).cast() }
    }
    #[doc = "0x390 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe04pfs_ha(&self) -> &PE0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(912usize).cast() }
    }
    #[doc = "0x394 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe05pfs_by(&self) -> &PE0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(916usize).cast() }
    }
    #[doc = "0x394 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe05pfs_ha(&self) -> &PE0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(916usize).cast() }
    }
    #[doc = "0x398 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe06pfs_by(&self) -> &PE0PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(920usize).cast() }
    }
    #[doc = "0x398 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe06pfs_ha(&self) -> &PE0PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(920usize).cast() }
    }
    #[doc = "0x3a0 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe08pfs_by(&self) -> &PE08PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(928usize).cast() }
    }
    #[doc = "0x3a0 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe08pfs_ha(&self) -> &PE08PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(928usize).cast() }
    }
    #[doc = "0x3a0 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe08pfs(&self) -> &PE08PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(928usize).cast() }
    }
    #[doc = "0x3a4 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe09pfs_by(&self) -> &PE08PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(932usize).cast() }
    }
    #[doc = "0x3a4 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe09pfs_ha(&self) -> &PE08PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(932usize).cast() }
    }
    #[doc = "0x3a4 - Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe09pfs(&self) -> &PE08PFS {
        unsafe { &*(self as *const Self).cast::<u8>().add(932usize).cast() }
    }
    #[doc = "0x3a8 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe10pfs_by(&self) -> &PE1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(936usize).cast() }
    }
    #[doc = "0x3a8 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe10pfs_ha(&self) -> &PE1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(936usize).cast() }
    }
    #[doc = "0x3a8..0x3c0 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe1pfs(&self) -> &[PE1PFS; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(936usize).cast() }
    }
    #[doc = "0x3ac - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe11pfs_by(&self) -> &PE1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(940usize).cast() }
    }
    #[doc = "0x3ac - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe11pfs_ha(&self) -> &PE1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(940usize).cast() }
    }
    #[doc = "0x3b0 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe12pfs_by(&self) -> &PE1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(944usize).cast() }
    }
    #[doc = "0x3b0 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe12pfs_ha(&self) -> &PE1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(944usize).cast() }
    }
    #[doc = "0x3b4 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe13pfs_by(&self) -> &PE1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(948usize).cast() }
    }
    #[doc = "0x3b4 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe13pfs_ha(&self) -> &PE1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(948usize).cast() }
    }
    #[doc = "0x3b8 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe14pfs_by(&self) -> &PE1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(952usize).cast() }
    }
    #[doc = "0x3b8 - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe14pfs_ha(&self) -> &PE1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(952usize).cast() }
    }
    #[doc = "0x3bc - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe15pfs_by(&self) -> &PE1PFS_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(956usize).cast() }
    }
    #[doc = "0x3bc - Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe15pfs_ha(&self) -> &PE1PFS_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(956usize).cast() }
    }
}
#[doc = "P00PFS (rw) register accessor: an alias for `Reg<P00PFS_SPEC>`"]
pub type P00PFS = crate::Reg<p00pfs::P00PFS_SPEC>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs;
#[doc = "P00PFS_HA (rw) register accessor: an alias for `Reg<P00PFS_HA_SPEC>`"]
pub type P00PFS_HA = crate::Reg<p00pfs_ha::P00PFS_HA_SPEC>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs_ha;
#[doc = "P00PFS_BY (rw) register accessor: an alias for `Reg<P00PFS_BY_SPEC>`"]
pub type P00PFS_BY = crate::Reg<p00pfs_by::P00PFS_BY_SPEC>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs_by;
#[doc = "P002PFS (rw) register accessor: an alias for `Reg<P002PFS_SPEC>`"]
pub type P002PFS = crate::Reg<p002pfs::P002PFS_SPEC>;
#[doc = "Port 002 Pin Function Select Register"]
pub mod p002pfs;
#[doc = "P002PFS_HA (rw) register accessor: an alias for `Reg<P002PFS_HA_SPEC>`"]
pub type P002PFS_HA = crate::Reg<p002pfs_ha::P002PFS_HA_SPEC>;
#[doc = "Port 002 Pin Function Select Register"]
pub mod p002pfs_ha;
pub use p00pfs_by as p002pfs_by;
pub use P00PFS_BY as P002PFS_BY;
#[doc = "P201PFS (rw) register accessor: an alias for `Reg<P201PFS_SPEC>`"]
pub type P201PFS = crate::Reg<p201pfs::P201PFS_SPEC>;
#[doc = "Port 201 Pin Function Select Register"]
pub mod p201pfs;
#[doc = "P201PFS_HA (rw) register accessor: an alias for `Reg<P201PFS_HA_SPEC>`"]
pub type P201PFS_HA = crate::Reg<p201pfs_ha::P201PFS_HA_SPEC>;
#[doc = "Port 201 Pin Function Select Register"]
pub mod p201pfs_ha;
#[doc = "P201PFS_BY (rw) register accessor: an alias for `Reg<P201PFS_BY_SPEC>`"]
pub type P201PFS_BY = crate::Reg<p201pfs_by::P201PFS_BY_SPEC>;
#[doc = "Port 201 Pin Function Select Register"]
pub mod p201pfs_by;
#[doc = "PA0PFS (rw) register accessor: an alias for `Reg<PA0PFS_SPEC>`"]
pub type PA0PFS = crate::Reg<pa0pfs::PA0PFS_SPEC>;
#[doc = "Port A0%s Pin Function Select Register"]
pub mod pa0pfs;
#[doc = "PA0PFS_HA (rw) register accessor: an alias for `Reg<PA0PFS_HA_SPEC>`"]
pub type PA0PFS_HA = crate::Reg<pa0pfs_ha::PA0PFS_HA_SPEC>;
#[doc = "Port A0%s Pin Function Select Register"]
pub mod pa0pfs_ha;
#[doc = "PA0PFS_BY (rw) register accessor: an alias for `Reg<PA0PFS_BY_SPEC>`"]
pub type PA0PFS_BY = crate::Reg<pa0pfs_by::PA0PFS_BY_SPEC>;
#[doc = "Port A0%s Pin Function Select Register"]
pub mod pa0pfs_by;
pub use pa0pfs as pa06pfs;
pub use pa0pfs_by as pa06pfs_by;
pub use pa0pfs_ha as pa06pfs_ha;
pub use PA0PFS as PA06PFS;
pub use PA0PFS_BY as PA06PFS_BY;
pub use PA0PFS_HA as PA06PFS_HA;
#[doc = "PA1PFS (rw) register accessor: an alias for `Reg<PA1PFS_SPEC>`"]
pub type PA1PFS = crate::Reg<pa1pfs::PA1PFS_SPEC>;
#[doc = "Port A1%s Pin Function Select Register"]
pub mod pa1pfs;
#[doc = "PA1PFS_HA (rw) register accessor: an alias for `Reg<PA1PFS_HA_SPEC>`"]
pub type PA1PFS_HA = crate::Reg<pa1pfs_ha::PA1PFS_HA_SPEC>;
#[doc = "Port A1%s Pin Function Select Register"]
pub mod pa1pfs_ha;
#[doc = "PA1PFS_BY (rw) register accessor: an alias for `Reg<PA1PFS_BY_SPEC>`"]
pub type PA1PFS_BY = crate::Reg<pa1pfs_by::PA1PFS_BY_SPEC>;
#[doc = "Port A1%s Pin Function Select Register"]
pub mod pa1pfs_by;
#[doc = "PA13PFS (rw) register accessor: an alias for `Reg<PA13PFS_SPEC>`"]
pub type PA13PFS = crate::Reg<pa13pfs::PA13PFS_SPEC>;
#[doc = "Port A13 Pin Function Select Register"]
pub mod pa13pfs;
#[doc = "PA13PFS_HA (rw) register accessor: an alias for `Reg<PA13PFS_HA_SPEC>`"]
pub type PA13PFS_HA = crate::Reg<pa13pfs_ha::PA13PFS_HA_SPEC>;
#[doc = "Port A13 Pin Function Select Register"]
pub mod pa13pfs_ha;
pub use pa1pfs as pa14pfs;
pub use pa1pfs_by as pa13pfs_by;
pub use pa1pfs_by as pa14pfs_by;
pub use pa1pfs_ha as pa14pfs_ha;
pub use PA1PFS as PA14PFS;
pub use PA1PFS_BY as PA13PFS_BY;
pub use PA1PFS_BY as PA14PFS_BY;
pub use PA1PFS_HA as PA14PFS_HA;
#[doc = "PB0PFS (rw) register accessor: an alias for `Reg<PB0PFS_SPEC>`"]
pub type PB0PFS = crate::Reg<pb0pfs::PB0PFS_SPEC>;
#[doc = "Port B0%s Pin Function Select Register"]
pub mod pb0pfs;
#[doc = "PB0PFS_HA (rw) register accessor: an alias for `Reg<PB0PFS_HA_SPEC>`"]
pub type PB0PFS_HA = crate::Reg<pb0pfs_ha::PB0PFS_HA_SPEC>;
#[doc = "Port B0%s Pin Function Select Register"]
pub mod pb0pfs_ha;
#[doc = "PB0PFS_BY (rw) register accessor: an alias for `Reg<PB0PFS_BY_SPEC>`"]
pub type PB0PFS_BY = crate::Reg<pb0pfs_by::PB0PFS_BY_SPEC>;
#[doc = "Port B0%s Pin Function Select Register"]
pub mod pb0pfs_by;
#[doc = "PB02PFS (rw) register accessor: an alias for `Reg<PB02PFS_SPEC>`"]
pub type PB02PFS = crate::Reg<pb02pfs::PB02PFS_SPEC>;
#[doc = "Port B02 Pin Function Select Register"]
pub mod pb02pfs;
#[doc = "PB02PFS_HA (rw) register accessor: an alias for `Reg<PB02PFS_HA_SPEC>`"]
pub type PB02PFS_HA = crate::Reg<pb02pfs_ha::PB02PFS_HA_SPEC>;
#[doc = "Port B02 Pin Function Select Register"]
pub mod pb02pfs_ha;
#[doc = "PB02PFS_BY (rw) register accessor: an alias for `Reg<PB02PFS_BY_SPEC>`"]
pub type PB02PFS_BY = crate::Reg<pb02pfs_by::PB02PFS_BY_SPEC>;
#[doc = "Port B02 Pin Function Select Register"]
pub mod pb02pfs_by;
#[doc = "PB03PFS (rw) register accessor: an alias for `Reg<PB03PFS_SPEC>`"]
pub type PB03PFS = crate::Reg<pb03pfs::PB03PFS_SPEC>;
#[doc = "Port B03 Pin Function Select Register"]
pub mod pb03pfs;
#[doc = "PB03PFS_HA (rw) register accessor: an alias for `Reg<PB03PFS_HA_SPEC>`"]
pub type PB03PFS_HA = crate::Reg<pb03pfs_ha::PB03PFS_HA_SPEC>;
#[doc = "Port B03 Pin Function Select Register"]
pub mod pb03pfs_ha;
pub use pb0pfs as pb04pfs;
pub use pb0pfs_by as pb03pfs_by;
pub use pb0pfs_by as pb04pfs_by;
pub use pb0pfs_ha as pb04pfs_ha;
pub use pb1pfs as pb10pfs;
pub use pb1pfs_by as pb10pfs_by;
pub use pb1pfs_ha as pb10pfs_ha;
pub use PB0PFS as PB04PFS;
pub use PB0PFS_BY as PB03PFS_BY;
pub use PB0PFS_BY as PB04PFS_BY;
pub use PB0PFS_HA as PB04PFS_HA;
pub use PB1PFS as PB10PFS;
pub use PB1PFS_BY as PB10PFS_BY;
pub use PB1PFS_HA as PB10PFS_HA;
#[doc = "PB1PFS (rw) register accessor: an alias for `Reg<PB1PFS_SPEC>`"]
pub type PB1PFS = crate::Reg<pb1pfs::PB1PFS_SPEC>;
#[doc = "Port B1%s Pin Function Select Register"]
pub mod pb1pfs;
#[doc = "PB1PFS_HA (rw) register accessor: an alias for `Reg<PB1PFS_HA_SPEC>`"]
pub type PB1PFS_HA = crate::Reg<pb1pfs_ha::PB1PFS_HA_SPEC>;
#[doc = "Port B1%s Pin Function Select Register"]
pub mod pb1pfs_ha;
#[doc = "PB1PFS_BY (rw) register accessor: an alias for `Reg<PB1PFS_BY_SPEC>`"]
pub type PB1PFS_BY = crate::Reg<pb1pfs_by::PB1PFS_BY_SPEC>;
#[doc = "Port B1%s Pin Function Select Register"]
pub mod pb1pfs_by;
#[doc = "PC0PFS (rw) register accessor: an alias for `Reg<PC0PFS_SPEC>`"]
pub type PC0PFS = crate::Reg<pc0pfs::PC0PFS_SPEC>;
#[doc = "Port C0%s Pin Function Select Register"]
pub mod pc0pfs;
#[doc = "PC0PFS_HA (rw) register accessor: an alias for `Reg<PC0PFS_HA_SPEC>`"]
pub type PC0PFS_HA = crate::Reg<pc0pfs_ha::PC0PFS_HA_SPEC>;
#[doc = "Port C0%s Pin Function Select Register"]
pub mod pc0pfs_ha;
#[doc = "PC0PFS_BY (rw) register accessor: an alias for `Reg<PC0PFS_BY_SPEC>`"]
pub type PC0PFS_BY = crate::Reg<pc0pfs_by::PC0PFS_BY_SPEC>;
#[doc = "Port C0%s Pin Function Select Register"]
pub mod pc0pfs_by;
#[doc = "PC1PFS (rw) register accessor: an alias for `Reg<PC1PFS_SPEC>`"]
pub type PC1PFS = crate::Reg<pc1pfs::PC1PFS_SPEC>;
#[doc = "Port C1%s Pin Function Select Register"]
pub mod pc1pfs;
#[doc = "PC1PFS_HA (rw) register accessor: an alias for `Reg<PC1PFS_HA_SPEC>`"]
pub type PC1PFS_HA = crate::Reg<pc1pfs_ha::PC1PFS_HA_SPEC>;
#[doc = "Port C1%s Pin Function Select Register"]
pub mod pc1pfs_ha;
#[doc = "PC1PFS_BY (rw) register accessor: an alias for `Reg<PC1PFS_BY_SPEC>`"]
pub type PC1PFS_BY = crate::Reg<pc1pfs_by::PC1PFS_BY_SPEC>;
#[doc = "Port C1%s Pin Function Select Register"]
pub mod pc1pfs_by;
#[doc = "PD0PFS (rw) register accessor: an alias for `Reg<PD0PFS_SPEC>`"]
pub type PD0PFS = crate::Reg<pd0pfs::PD0PFS_SPEC>;
#[doc = "Port D0%s Pin Function Select Register"]
pub mod pd0pfs;
#[doc = "PD0PFS_HA (rw) register accessor: an alias for `Reg<PD0PFS_HA_SPEC>`"]
pub type PD0PFS_HA = crate::Reg<pd0pfs_ha::PD0PFS_HA_SPEC>;
#[doc = "Port D0%s Pin Function Select Register"]
pub mod pd0pfs_ha;
#[doc = "PD0PFS_BY (rw) register accessor: an alias for `Reg<PD0PFS_BY_SPEC>`"]
pub type PD0PFS_BY = crate::Reg<pd0pfs_by::PD0PFS_BY_SPEC>;
#[doc = "Port D0%s Pin Function Select Register"]
pub mod pd0pfs_by;
#[doc = "PD1PFS (rw) register accessor: an alias for `Reg<PD1PFS_SPEC>`"]
pub type PD1PFS = crate::Reg<pd1pfs::PD1PFS_SPEC>;
#[doc = "Port D1%s Pin Function Select Register"]
pub mod pd1pfs;
#[doc = "PD1PFS_HA (rw) register accessor: an alias for `Reg<PD1PFS_HA_SPEC>`"]
pub type PD1PFS_HA = crate::Reg<pd1pfs_ha::PD1PFS_HA_SPEC>;
#[doc = "Port D1%s Pin Function Select Register"]
pub mod pd1pfs_ha;
#[doc = "PD1PFS_BY (rw) register accessor: an alias for `Reg<PD1PFS_BY_SPEC>`"]
pub type PD1PFS_BY = crate::Reg<pd1pfs_by::PD1PFS_BY_SPEC>;
#[doc = "Port D1%s Pin Function Select Register"]
pub mod pd1pfs_by;
#[doc = "PE0PFS (rw) register accessor: an alias for `Reg<PE0PFS_SPEC>`"]
pub type PE0PFS = crate::Reg<pe0pfs::PE0PFS_SPEC>;
#[doc = "Port E0%s Pin Function Select Register"]
pub mod pe0pfs;
#[doc = "PE0PFS_HA (rw) register accessor: an alias for `Reg<PE0PFS_HA_SPEC>`"]
pub type PE0PFS_HA = crate::Reg<pe0pfs_ha::PE0PFS_HA_SPEC>;
#[doc = "Port E0%s Pin Function Select Register"]
pub mod pe0pfs_ha;
#[doc = "PE0PFS_BY (rw) register accessor: an alias for `Reg<PE0PFS_BY_SPEC>`"]
pub type PE0PFS_BY = crate::Reg<pe0pfs_by::PE0PFS_BY_SPEC>;
#[doc = "Port E0%s Pin Function Select Register"]
pub mod pe0pfs_by;
pub use pe0pfs as pe08pfs;
pub use pe0pfs_by as pe08pfs_by;
pub use pe0pfs_ha as pe08pfs_ha;
pub use PE0PFS as PE08PFS;
pub use PE0PFS_BY as PE08PFS_BY;
pub use PE0PFS_HA as PE08PFS_HA;
#[doc = "PE1PFS (rw) register accessor: an alias for `Reg<PE1PFS_SPEC>`"]
pub type PE1PFS = crate::Reg<pe1pfs::PE1PFS_SPEC>;
#[doc = "Port E1%s Pin Function Select Register"]
pub mod pe1pfs;
#[doc = "PE1PFS_HA (rw) register accessor: an alias for `Reg<PE1PFS_HA_SPEC>`"]
pub type PE1PFS_HA = crate::Reg<pe1pfs_ha::PE1PFS_HA_SPEC>;
#[doc = "Port E1%s Pin Function Select Register"]
pub mod pe1pfs_ha;
#[doc = "PE1PFS_BY (rw) register accessor: an alias for `Reg<PE1PFS_BY_SPEC>`"]
pub type PE1PFS_BY = crate::Reg<pe1pfs_by::PE1PFS_BY_SPEC>;
#[doc = "Port E1%s Pin Function Select Register"]
pub mod pe1pfs_by;
#[doc = "PWPR (rw) register accessor: an alias for `Reg<PWPR_SPEC>`"]
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
#[doc = "Write-Protect Register"]
pub mod pwpr;
#[doc = "PWPRS (rw) register accessor: an alias for `Reg<PWPRS_SPEC>`"]
pub type PWPRS = crate::Reg<pwprs::PWPRS_SPEC>;
#[doc = "Write-Protect Register for Secure"]
pub mod pwprs;
#[doc = "P0SAR (rw) register accessor: an alias for `Reg<P0SAR_SPEC>`"]
pub type P0SAR = crate::Reg<p0sar::P0SAR_SPEC>;
#[doc = "Port 0 Security Attribution register"]
pub mod p0sar;
#[doc = "P2SAR (rw) register accessor: an alias for `Reg<P2SAR_SPEC>`"]
pub type P2SAR = crate::Reg<p2sar::P2SAR_SPEC>;
#[doc = "Port 2 Security Attribution register"]
pub mod p2sar;
#[doc = "PASAR (rw) register accessor: an alias for `Reg<PASAR_SPEC>`"]
pub type PASAR = crate::Reg<pasar::PASAR_SPEC>;
#[doc = "Port A Security Attribution register"]
pub mod pasar;
#[doc = "PBSAR (rw) register accessor: an alias for `Reg<PBSAR_SPEC>`"]
pub type PBSAR = crate::Reg<pbsar::PBSAR_SPEC>;
#[doc = "Port B Security Attribution register"]
pub mod pbsar;
#[doc = "PCSAR (rw) register accessor: an alias for `Reg<PCSAR_SPEC>`"]
pub type PCSAR = crate::Reg<pcsar::PCSAR_SPEC>;
#[doc = "Port C Security Attribution register"]
pub mod pcsar;
#[doc = "PDSAR (rw) register accessor: an alias for `Reg<PDSAR_SPEC>`"]
pub type PDSAR = crate::Reg<pdsar::PDSAR_SPEC>;
#[doc = "Port D Security Attribution register"]
pub mod pdsar;
#[doc = "PESAR (rw) register accessor: an alias for `Reg<PESAR_SPEC>`"]
pub type PESAR = crate::Reg<pesar::PESAR_SPEC>;
#[doc = "Port E Security Attribution register"]
pub mod pesar;
