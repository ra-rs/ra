#[doc = "Register `NMICLR` reader"]
pub struct R(crate::R<NMICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMICLR` writer"]
pub struct W(crate::W<NMICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMICLR_SPEC>;
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
impl From<crate::W<NMICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IWDTCLR` reader - IWDT Clear"]
pub type IWDTCLR_R = crate::BitReader<IWDTCLR_A>;
#[doc = "IWDT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.IWDTST flag"]
    _1 = 1,
}
impl From<IWDTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDTCLR_A {
        match self.bits {
            false => IWDTCLR_A::_0,
            true => IWDTCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTCLR_A::_1
    }
}
#[doc = "Field `IWDTCLR` writer - IWDT Clear"]
pub type IWDTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, IWDTCLR_A, O>;
impl<'a, const O: u8> IWDTCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IWDTCLR_A::_0)
    }
    #[doc = "Clear the NMISR.IWDTST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IWDTCLR_A::_1)
    }
}
#[doc = "Field `WDTCLR` reader - WDT Clear"]
pub type WDTCLR_R = crate::BitReader<WDTCLR_A>;
#[doc = "WDT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.WDTST flag"]
    _1 = 1,
}
impl From<WDTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCLR_A {
        match self.bits {
            false => WDTCLR_A::_0,
            true => WDTCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTCLR_A::_1
    }
}
#[doc = "Field `WDTCLR` writer - WDT Clear"]
pub type WDTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, WDTCLR_A, O>;
impl<'a, const O: u8> WDTCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTCLR_A::_0)
    }
    #[doc = "Clear the NMISR.WDTST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTCLR_A::_1)
    }
}
#[doc = "Field `LVD1CLR` reader - LVD1 Clear"]
pub type LVD1CLR_R = crate::BitReader<LVD1CLR_A>;
#[doc = "LVD1 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1CLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD1ST flag"]
    _1 = 1,
}
impl From<LVD1CLR_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD1CLR_A {
        match self.bits {
            false => LVD1CLR_A::_0,
            true => LVD1CLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1CLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1CLR_A::_1
    }
}
#[doc = "Field `LVD1CLR` writer - LVD1 Clear"]
pub type LVD1CLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, LVD1CLR_A, O>;
impl<'a, const O: u8> LVD1CLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1CLR_A::_0)
    }
    #[doc = "Clear the NMISR.LVD1ST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1CLR_A::_1)
    }
}
#[doc = "Field `LVD2CLR` reader - LVD2 Clear"]
pub type LVD2CLR_R = crate::BitReader<LVD2CLR_A>;
#[doc = "LVD2 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2CLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD2ST flag."]
    _1 = 1,
}
impl From<LVD2CLR_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD2CLR_A {
        match self.bits {
            false => LVD2CLR_A::_0,
            true => LVD2CLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2CLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2CLR_A::_1
    }
}
#[doc = "Field `LVD2CLR` writer - LVD2 Clear"]
pub type LVD2CLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, LVD2CLR_A, O>;
impl<'a, const O: u8> LVD2CLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2CLR_A::_0)
    }
    #[doc = "Clear the NMISR.LVD2ST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2CLR_A::_1)
    }
}
#[doc = "Field `OSTCLR` reader - OST Clear"]
pub type OSTCLR_R = crate::BitReader<OSTCLR_A>;
#[doc = "OST Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.OSTST flag"]
    _1 = 1,
}
impl From<OSTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: OSTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTCLR_A {
        match self.bits {
            false => OSTCLR_A::_0,
            true => OSTCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTCLR_A::_1
    }
}
#[doc = "Field `OSTCLR` writer - OST Clear"]
pub type OSTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, OSTCLR_A, O>;
impl<'a, const O: u8> OSTCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSTCLR_A::_0)
    }
    #[doc = "Clear the NMISR.OSTST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSTCLR_A::_1)
    }
}
#[doc = "Field `NMICLR` reader - NMI Clear"]
pub type NMICLR_R = crate::BitReader<NMICLR_A>;
#[doc = "NMI Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMICLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.NMIST flag"]
    _1 = 1,
}
impl From<NMICLR_A> for bool {
    #[inline(always)]
    fn from(variant: NMICLR_A) -> Self {
        variant as u8 != 0
    }
}
impl NMICLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMICLR_A {
        match self.bits {
            false => NMICLR_A::_0,
            true => NMICLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMICLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMICLR_A::_1
    }
}
#[doc = "Field `NMICLR` writer - NMI Clear"]
pub type NMICLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, NMICLR_A, O>;
impl<'a, const O: u8> NMICLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMICLR_A::_0)
    }
    #[doc = "Clear the NMISR.NMIST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMICLR_A::_1)
    }
}
#[doc = "Field `RPECLR` reader - SRAM Parity Error Clear"]
pub type RPECLR_R = crate::BitReader<RPECLR_A>;
#[doc = "SRAM Parity Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPECLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.RPEST flag"]
    _1 = 1,
}
impl From<RPECLR_A> for bool {
    #[inline(always)]
    fn from(variant: RPECLR_A) -> Self {
        variant as u8 != 0
    }
}
impl RPECLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPECLR_A {
        match self.bits {
            false => RPECLR_A::_0,
            true => RPECLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPECLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPECLR_A::_1
    }
}
#[doc = "Field `RPECLR` writer - SRAM Parity Error Clear"]
pub type RPECLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, RPECLR_A, O>;
impl<'a, const O: u8> RPECLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPECLR_A::_0)
    }
    #[doc = "Clear the NMISR.RPEST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPECLR_A::_1)
    }
}
#[doc = "Field `BUSSCLR` reader - Bus Slave Error Clear"]
pub type BUSSCLR_R = crate::BitReader<BUSSCLR_A>;
#[doc = "Bus Slave Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.BUSSST flag"]
    _1 = 1,
}
impl From<BUSSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSSCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSSCLR_A {
        match self.bits {
            false => BUSSCLR_A::_0,
            true => BUSSCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSCLR_A::_1
    }
}
#[doc = "Field `BUSSCLR` writer - Bus Slave Error Clear"]
pub type BUSSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, BUSSCLR_A, O>;
impl<'a, const O: u8> BUSSCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSSCLR_A::_0)
    }
    #[doc = "Clear the NMISR.BUSSST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSSCLR_A::_1)
    }
}
#[doc = "Field `BUSMCLR` reader - Bus Master Error Clear"]
pub type BUSMCLR_R = crate::BitReader<BUSMCLR_A>;
#[doc = "Bus Master Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.BUSMST flag"]
    _1 = 1,
}
impl From<BUSMCLR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSMCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSMCLR_A {
        match self.bits {
            false => BUSMCLR_A::_0,
            true => BUSMCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMCLR_A::_1
    }
}
#[doc = "Field `BUSMCLR` writer - Bus Master Error Clear"]
pub type BUSMCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, BUSMCLR_A, O>;
impl<'a, const O: u8> BUSMCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSMCLR_A::_0)
    }
    #[doc = "Clear the NMISR.BUSMST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSMCLR_A::_1)
    }
}
#[doc = "Field `SPECLR` reader - CPU Stack Pointer Monitor Interrupt Clear"]
pub type SPECLR_R = crate::BitReader<SPECLR_A>;
#[doc = "CPU Stack Pointer Monitor Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPECLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the NMISR.SPEST flag"]
    _1 = 1,
}
impl From<SPECLR_A> for bool {
    #[inline(always)]
    fn from(variant: SPECLR_A) -> Self {
        variant as u8 != 0
    }
}
impl SPECLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPECLR_A {
        match self.bits {
            false => SPECLR_A::_0,
            true => SPECLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPECLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPECLR_A::_1
    }
}
#[doc = "Field `SPECLR` writer - CPU Stack Pointer Monitor Interrupt Clear"]
pub type SPECLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, SPECLR_A, O>;
impl<'a, const O: u8> SPECLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPECLR_A::_0)
    }
    #[doc = "Clear the NMISR.SPEST flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPECLR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IWDT Clear"]
    #[inline(always)]
    pub fn iwdtclr(&self) -> IWDTCLR_R {
        IWDTCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Clear"]
    #[inline(always)]
    pub fn wdtclr(&self) -> WDTCLR_R {
        WDTCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LVD1 Clear"]
    #[inline(always)]
    pub fn lvd1clr(&self) -> LVD1CLR_R {
        LVD1CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LVD2 Clear"]
    #[inline(always)]
    pub fn lvd2clr(&self) -> LVD2CLR_R {
        LVD2CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - OST Clear"]
    #[inline(always)]
    pub fn ostclr(&self) -> OSTCLR_R {
        OSTCLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NMI Clear"]
    #[inline(always)]
    pub fn nmiclr(&self) -> NMICLR_R {
        NMICLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Error Clear"]
    #[inline(always)]
    pub fn rpeclr(&self) -> RPECLR_R {
        RPECLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Slave Error Clear"]
    #[inline(always)]
    pub fn bussclr(&self) -> BUSSCLR_R {
        BUSSCLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Master Error Clear"]
    #[inline(always)]
    pub fn busmclr(&self) -> BUSMCLR_R {
        BUSMCLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Stack Pointer Monitor Interrupt Clear"]
    #[inline(always)]
    pub fn speclr(&self) -> SPECLR_R {
        SPECLR_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IWDT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn iwdtclr(&mut self) -> IWDTCLR_W<0> {
        IWDTCLR_W::new(self)
    }
    #[doc = "Bit 1 - WDT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdtclr(&mut self) -> WDTCLR_W<1> {
        WDTCLR_W::new(self)
    }
    #[doc = "Bit 2 - LVD1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1clr(&mut self) -> LVD1CLR_W<2> {
        LVD1CLR_W::new(self)
    }
    #[doc = "Bit 3 - LVD2 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2clr(&mut self) -> LVD2CLR_W<3> {
        LVD2CLR_W::new(self)
    }
    #[doc = "Bit 6 - OST Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ostclr(&mut self) -> OSTCLR_W<6> {
        OSTCLR_W::new(self)
    }
    #[doc = "Bit 7 - NMI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nmiclr(&mut self) -> NMICLR_W<7> {
        NMICLR_W::new(self)
    }
    #[doc = "Bit 8 - SRAM Parity Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpeclr(&mut self) -> RPECLR_W<8> {
        RPECLR_W::new(self)
    }
    #[doc = "Bit 10 - Bus Slave Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bussclr(&mut self) -> BUSSCLR_W<10> {
        BUSSCLR_W::new(self)
    }
    #[doc = "Bit 11 - Bus Master Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn busmclr(&mut self) -> BUSMCLR_W<11> {
        BUSMCLR_W::new(self)
    }
    #[doc = "Bit 12 - CPU Stack Pointer Monitor Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn speclr(&mut self) -> SPECLR_W<12> {
        SPECLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Maskable Interrupt Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmiclr](index.html) module"]
pub struct NMICLR_SPEC;
impl crate::RegisterSpec for NMICLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nmiclr::R](R) reader structure"]
impl crate::Readable for NMICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmiclr::W](W) writer structure"]
impl crate::Writable for NMICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMICLR to value 0"]
impl crate::Resettable for NMICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
