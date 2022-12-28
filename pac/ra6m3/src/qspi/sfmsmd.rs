#[doc = "Register `SFMSMD` reader"]
pub struct R(crate::R<SFMSMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSMD` writer"]
pub struct W(crate::W<SFMSMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SFMSMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMRM` reader - Serial interface read mode selection"]
pub type SFMRM_R = crate::FieldReader<u8, SFMRM_A>;
#[doc = "Serial interface read mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMRM_A {
    #[doc = "0: Standard Read"]
    _000 = 0,
    #[doc = "1: Fast Read"]
    _001 = 1,
    #[doc = "2: Fast Read Dual Output"]
    _010 = 2,
    #[doc = "3: Fast Read Dual I/O"]
    _011 = 3,
    #[doc = "4: Fast Read Quad Output"]
    _100 = 4,
    #[doc = "5: Fast Read Quad I/O"]
    _101 = 5,
    #[doc = "6: Setting prohibited"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<SFMRM_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMRM_A) -> Self {
        variant as _
    }
}
impl SFMRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMRM_A {
        match self.bits {
            0 => SFMRM_A::_000,
            1 => SFMRM_A::_001,
            2 => SFMRM_A::_010,
            3 => SFMRM_A::_011,
            4 => SFMRM_A::_100,
            5 => SFMRM_A::_101,
            6 => SFMRM_A::_110,
            7 => SFMRM_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SFMRM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SFMRM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SFMRM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SFMRM_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SFMRM_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SFMRM_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SFMRM_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SFMRM_A::_111
    }
}
#[doc = "Field `SFMRM` writer - Serial interface read mode selection"]
pub type SFMRM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFMSMD_SPEC, u8, SFMRM_A, 3, O>;
impl<'a, const O: u8> SFMRM_W<'a, O> {
    #[doc = "Standard Read"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SFMRM_A::_000)
    }
    #[doc = "Fast Read"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SFMRM_A::_001)
    }
    #[doc = "Fast Read Dual Output"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SFMRM_A::_010)
    }
    #[doc = "Fast Read Dual I/O"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SFMRM_A::_011)
    }
    #[doc = "Fast Read Quad Output"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SFMRM_A::_100)
    }
    #[doc = "Fast Read Quad I/O"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SFMRM_A::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SFMRM_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SFMRM_A::_111)
    }
}
#[doc = "Field `SFMSE` reader - Selection of the prefetch function"]
pub type SFMSE_R = crate::FieldReader<u8, SFMSE_A>;
#[doc = "Selection of the prefetch function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMSE_A {
    #[doc = "0: Does not extend QSSL"]
    _00 = 0,
    #[doc = "1: Extends QSSL by 33*QSPCLK"]
    _01 = 1,
    #[doc = "2: Extends QSSL by 129*QSPCLK"]
    _10 = 2,
    #[doc = "3: Extends QSSL infinitely"]
    _11 = 3,
}
impl From<SFMSE_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMSE_A) -> Self {
        variant as _
    }
}
impl SFMSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSE_A {
        match self.bits {
            0 => SFMSE_A::_00,
            1 => SFMSE_A::_01,
            2 => SFMSE_A::_10,
            3 => SFMSE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SFMSE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SFMSE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SFMSE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SFMSE_A::_11
    }
}
#[doc = "Field `SFMSE` writer - Selection of the prefetch function"]
pub type SFMSE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFMSMD_SPEC, u8, SFMSE_A, 2, O>;
impl<'a, const O: u8> SFMSE_W<'a, O> {
    #[doc = "Does not extend QSSL"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SFMSE_A::_00)
    }
    #[doc = "Extends QSSL by 33*QSPCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SFMSE_A::_01)
    }
    #[doc = "Extends QSSL by 129*QSPCLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SFMSE_A::_10)
    }
    #[doc = "Extends QSSL infinitely"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SFMSE_A::_11)
    }
}
#[doc = "Field `SFMPFE` reader - Selection of the prefetch function"]
pub type SFMPFE_R = crate::BitReader<SFMPFE_A>;
#[doc = "Selection of the prefetch function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMPFE_A {
    #[doc = "0: Disables prefetch"]
    _0 = 0,
    #[doc = "1: Enables prefetch"]
    _1 = 1,
}
impl From<SFMPFE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMPFE_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMPFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMPFE_A {
        match self.bits {
            false => SFMPFE_A::_0,
            true => SFMPFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMPFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMPFE_A::_1
    }
}
#[doc = "Field `SFMPFE` writer - Selection of the prefetch function"]
pub type SFMPFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSMD_SPEC, SFMPFE_A, O>;
impl<'a, const O: u8> SFMPFE_W<'a, O> {
    #[doc = "Disables prefetch"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMPFE_A::_0)
    }
    #[doc = "Enables prefetch"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMPFE_A::_1)
    }
}
#[doc = "Field `SFMPAE` reader - Selection of the function for stopping prefetch at locations other than on byte boundaries"]
pub type SFMPAE_R = crate::BitReader<SFMPAE_A>;
#[doc = "Selection of the function for stopping prefetch at locations other than on byte boundaries\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMPAE_A {
    #[doc = "0: Disables prefetch stopping at locations other than on byte boundaries"]
    _0 = 0,
    #[doc = "1: Enables prefetch stopping at locations other than on byte boundaries"]
    _1 = 1,
}
impl From<SFMPAE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMPAE_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMPAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMPAE_A {
        match self.bits {
            false => SFMPAE_A::_0,
            true => SFMPAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMPAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMPAE_A::_1
    }
}
#[doc = "Field `SFMPAE` writer - Selection of the function for stopping prefetch at locations other than on byte boundaries"]
pub type SFMPAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSMD_SPEC, SFMPAE_A, O>;
impl<'a, const O: u8> SFMPAE_W<'a, O> {
    #[doc = "Disables prefetch stopping at locations other than on byte boundaries"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMPAE_A::_0)
    }
    #[doc = "Enables prefetch stopping at locations other than on byte boundaries"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMPAE_A::_1)
    }
}
#[doc = "Field `SFMMD3` reader - SPI mode selection. An initial value is determined by input to CFGMD3."]
pub type SFMMD3_R = crate::BitReader<SFMMD3_A>;
#[doc = "SPI mode selection. An initial value is determined by input to CFGMD3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMMD3_A {
    #[doc = "0: SPI mode 0"]
    _0 = 0,
    #[doc = "1: SPI mode 3"]
    _1 = 1,
}
impl From<SFMMD3_A> for bool {
    #[inline(always)]
    fn from(variant: SFMMD3_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMMD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMMD3_A {
        match self.bits {
            false => SFMMD3_A::_0,
            true => SFMMD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMMD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMMD3_A::_1
    }
}
#[doc = "Field `SFMMD3` writer - SPI mode selection. An initial value is determined by input to CFGMD3."]
pub type SFMMD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSMD_SPEC, SFMMD3_A, O>;
impl<'a, const O: u8> SFMMD3_W<'a, O> {
    #[doc = "SPI mode 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMMD3_A::_0)
    }
    #[doc = "SPI mode 3"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMMD3_A::_1)
    }
}
#[doc = "Field `SFMOEX` reader - Extension of the I/O buffer output enable signal for the serial interface"]
pub type SFMOEX_R = crate::BitReader<SFMOEX_A>;
#[doc = "Extension of the I/O buffer output enable signal for the serial interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMOEX_A {
    #[doc = "0: Does not extend the output enable signal"]
    _0 = 0,
    #[doc = "1: Extends the output enable signal by 1*QSPCLK"]
    _1 = 1,
}
impl From<SFMOEX_A> for bool {
    #[inline(always)]
    fn from(variant: SFMOEX_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMOEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMOEX_A {
        match self.bits {
            false => SFMOEX_A::_0,
            true => SFMOEX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMOEX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMOEX_A::_1
    }
}
#[doc = "Field `SFMOEX` writer - Extension of the I/O buffer output enable signal for the serial interface"]
pub type SFMOEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSMD_SPEC, SFMOEX_A, O>;
impl<'a, const O: u8> SFMOEX_W<'a, O> {
    #[doc = "Does not extend the output enable signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMOEX_A::_0)
    }
    #[doc = "Extends the output enable signal by 1*QSPCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMOEX_A::_1)
    }
}
#[doc = "Field `SFMOHW` reader - Hold time adjustment for serial transmission"]
pub type SFMOHW_R = crate::BitReader<SFMOHW_A>;
#[doc = "Hold time adjustment for serial transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMOHW_A {
    #[doc = "0: Does not extend the high-level width of SCK at transmission time"]
    _0 = 0,
    #[doc = "1: Extends the high-level width of SCK by 1*PCLKA at transmission time"]
    _1 = 1,
}
impl From<SFMOHW_A> for bool {
    #[inline(always)]
    fn from(variant: SFMOHW_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMOHW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMOHW_A {
        match self.bits {
            false => SFMOHW_A::_0,
            true => SFMOHW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMOHW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMOHW_A::_1
    }
}
#[doc = "Field `SFMOHW` writer - Hold time adjustment for serial transmission"]
pub type SFMOHW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSMD_SPEC, SFMOHW_A, O>;
impl<'a, const O: u8> SFMOHW_W<'a, O> {
    #[doc = "Does not extend the high-level width of SCK at transmission time"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMOHW_A::_0)
    }
    #[doc = "Extends the high-level width of SCK by 1*PCLKA at transmission time"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMOHW_A::_1)
    }
}
#[doc = "Field `SFMOSW` reader - Setup time adjustment for serial transmission"]
pub type SFMOSW_R = crate::BitReader<SFMOSW_A>;
#[doc = "Setup time adjustment for serial transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMOSW_A {
    #[doc = "0: Does not extend the low-level width of SCK at transmission time"]
    _0 = 0,
    #[doc = "1: Extends the low-level width of SCK by 1*PCLKA at transmission time"]
    _1 = 1,
}
impl From<SFMOSW_A> for bool {
    #[inline(always)]
    fn from(variant: SFMOSW_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMOSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMOSW_A {
        match self.bits {
            false => SFMOSW_A::_0,
            true => SFMOSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMOSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMOSW_A::_1
    }
}
#[doc = "Field `SFMOSW` writer - Setup time adjustment for serial transmission"]
pub type SFMOSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSMD_SPEC, SFMOSW_A, O>;
impl<'a, const O: u8> SFMOSW_W<'a, O> {
    #[doc = "Does not extend the low-level width of SCK at transmission time"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMOSW_A::_0)
    }
    #[doc = "Extends the low-level width of SCK by 1*PCLKA at transmission time"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMOSW_A::_1)
    }
}
#[doc = "Field `SFMCCE` reader - Read instruction code selection."]
pub type SFMCCE_R = crate::BitReader<SFMCCE_A>;
#[doc = "Read instruction code selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMCCE_A {
    #[doc = "0: Default instruction code set for each instruction"]
    _0 = 0,
    #[doc = "1: Instruction code written in the SFMSIC register"]
    _1 = 1,
}
impl From<SFMCCE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMCCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMCCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMCCE_A {
        match self.bits {
            false => SFMCCE_A::_0,
            true => SFMCCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMCCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMCCE_A::_1
    }
}
#[doc = "Field `SFMCCE` writer - Read instruction code selection."]
pub type SFMCCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSMD_SPEC, SFMCCE_A, O>;
impl<'a, const O: u8> SFMCCE_W<'a, O> {
    #[doc = "Default instruction code set for each instruction"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMCCE_A::_0)
    }
    #[doc = "Instruction code written in the SFMSIC register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMCCE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Serial interface read mode selection"]
    #[inline(always)]
    pub fn sfmrm(&self) -> SFMRM_R {
        SFMRM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Selection of the prefetch function"]
    #[inline(always)]
    pub fn sfmse(&self) -> SFMSE_R {
        SFMSE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Selection of the prefetch function"]
    #[inline(always)]
    pub fn sfmpfe(&self) -> SFMPFE_R {
        SFMPFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selection of the function for stopping prefetch at locations other than on byte boundaries"]
    #[inline(always)]
    pub fn sfmpae(&self) -> SFMPAE_R {
        SFMPAE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI mode selection. An initial value is determined by input to CFGMD3."]
    #[inline(always)]
    pub fn sfmmd3(&self) -> SFMMD3_R {
        SFMMD3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Extension of the I/O buffer output enable signal for the serial interface"]
    #[inline(always)]
    pub fn sfmoex(&self) -> SFMOEX_R {
        SFMOEX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hold time adjustment for serial transmission"]
    #[inline(always)]
    pub fn sfmohw(&self) -> SFMOHW_R {
        SFMOHW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Setup time adjustment for serial transmission"]
    #[inline(always)]
    pub fn sfmosw(&self) -> SFMOSW_R {
        SFMOSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Read instruction code selection."]
    #[inline(always)]
    pub fn sfmcce(&self) -> SFMCCE_R {
        SFMCCE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Serial interface read mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sfmrm(&mut self) -> SFMRM_W<0> {
        SFMRM_W::new(self)
    }
    #[doc = "Bits 4:5 - Selection of the prefetch function"]
    #[inline(always)]
    #[must_use]
    pub fn sfmse(&mut self) -> SFMSE_W<4> {
        SFMSE_W::new(self)
    }
    #[doc = "Bit 6 - Selection of the prefetch function"]
    #[inline(always)]
    #[must_use]
    pub fn sfmpfe(&mut self) -> SFMPFE_W<6> {
        SFMPFE_W::new(self)
    }
    #[doc = "Bit 7 - Selection of the function for stopping prefetch at locations other than on byte boundaries"]
    #[inline(always)]
    #[must_use]
    pub fn sfmpae(&mut self) -> SFMPAE_W<7> {
        SFMPAE_W::new(self)
    }
    #[doc = "Bit 8 - SPI mode selection. An initial value is determined by input to CFGMD3."]
    #[inline(always)]
    #[must_use]
    pub fn sfmmd3(&mut self) -> SFMMD3_W<8> {
        SFMMD3_W::new(self)
    }
    #[doc = "Bit 9 - Extension of the I/O buffer output enable signal for the serial interface"]
    #[inline(always)]
    #[must_use]
    pub fn sfmoex(&mut self) -> SFMOEX_W<9> {
        SFMOEX_W::new(self)
    }
    #[doc = "Bit 10 - Hold time adjustment for serial transmission"]
    #[inline(always)]
    #[must_use]
    pub fn sfmohw(&mut self) -> SFMOHW_W<10> {
        SFMOHW_W::new(self)
    }
    #[doc = "Bit 11 - Setup time adjustment for serial transmission"]
    #[inline(always)]
    #[must_use]
    pub fn sfmosw(&mut self) -> SFMOSW_W<11> {
        SFMOSW_W::new(self)
    }
    #[doc = "Bit 15 - Read instruction code selection."]
    #[inline(always)]
    #[must_use]
    pub fn sfmcce(&mut self) -> SFMCCE_W<15> {
        SFMCCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmsmd](index.html) module"]
pub struct SFMSMD_SPEC;
impl crate::RegisterSpec for SFMSMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmsmd::R](R) reader structure"]
impl crate::Readable for SFMSMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmsmd::W](W) writer structure"]
impl crate::Writable for SFMSMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSMD to value 0"]
impl crate::Resettable for SFMSMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
