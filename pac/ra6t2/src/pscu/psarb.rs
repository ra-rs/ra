#[doc = "Register `PSARB` reader"]
pub struct R(crate::R<PSARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSARB` writer"]
pub struct W(crate::W<PSARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSARB_SPEC>;
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
impl From<crate::W<PSARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSARB8` reader - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
pub type PSARB8_R = crate::BitReader<PSARB8_A>;
#[doc = "IIC1 and the MSTPCRB.MSTPB8 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB8_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB8_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB8_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB8_A {
        match self.bits {
            false => PSARB8_A::_0,
            true => PSARB8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB8_A::_1
    }
}
#[doc = "Field `PSARB8` writer - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
pub type PSARB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB8_A, O>;
impl<'a, const O: u8> PSARB8_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB8_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB8_A::_1)
    }
}
#[doc = "Field `PSARB9` reader - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
pub type PSARB9_R = crate::BitReader<PSARB9_A>;
#[doc = "IIC0 and the MSTPCRB.MSTPB9 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB9_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB9_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB9_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB9_A {
        match self.bits {
            false => PSARB9_A::_0,
            true => PSARB9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB9_A::_1
    }
}
#[doc = "Field `PSARB9` writer - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
pub type PSARB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB9_A, O>;
impl<'a, const O: u8> PSARB9_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB9_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB9_A::_1)
    }
}
#[doc = "Field `PSARB18` reader - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
pub type PSARB18_R = crate::BitReader<PSARB18_A>;
#[doc = "SPI1 and the MSTPCRB.MSTPB18 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB18_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB18_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB18_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB18_A {
        match self.bits {
            false => PSARB18_A::_0,
            true => PSARB18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB18_A::_1
    }
}
#[doc = "Field `PSARB18` writer - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
pub type PSARB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB18_A, O>;
impl<'a, const O: u8> PSARB18_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB18_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB18_A::_1)
    }
}
#[doc = "Field `PSARB19` reader - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
pub type PSARB19_R = crate::BitReader<PSARB19_A>;
#[doc = "SPI0 and the MSTPCRB.MSTPB19 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB19_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB19_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB19_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB19_A {
        match self.bits {
            false => PSARB19_A::_0,
            true => PSARB19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB19_A::_1
    }
}
#[doc = "Field `PSARB19` writer - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
pub type PSARB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB19_A, O>;
impl<'a, const O: u8> PSARB19_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB19_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB19_A::_1)
    }
}
#[doc = "Field `PSARB22` reader - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
pub type PSARB22_R = crate::BitReader<PSARB22_A>;
#[doc = "SCI9 and the MSTPCRB.MSTPB22 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB22_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB22_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB22_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB22_A {
        match self.bits {
            false => PSARB22_A::_0,
            true => PSARB22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB22_A::_1
    }
}
#[doc = "Field `PSARB22` writer - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
pub type PSARB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB22_A, O>;
impl<'a, const O: u8> PSARB22_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB22_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB22_A::_1)
    }
}
#[doc = "Field `PSARB27` reader - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
pub type PSARB27_R = crate::BitReader<PSARB27_A>;
#[doc = "SCI4 and the MSTPCRB.MSTPB27 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB27_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB27_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB27_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB27_A {
        match self.bits {
            false => PSARB27_A::_0,
            true => PSARB27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB27_A::_1
    }
}
#[doc = "Field `PSARB27` writer - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
pub type PSARB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB27_A, O>;
impl<'a, const O: u8> PSARB27_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB27_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB27_A::_1)
    }
}
#[doc = "Field `PSARB28` reader - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
pub type PSARB28_R = crate::BitReader<PSARB28_A>;
#[doc = "SCI3 and the MSTPCRB.MSTPB28 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB28_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB28_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB28_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB28_A {
        match self.bits {
            false => PSARB28_A::_0,
            true => PSARB28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB28_A::_1
    }
}
#[doc = "Field `PSARB28` writer - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
pub type PSARB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB28_A, O>;
impl<'a, const O: u8> PSARB28_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB28_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB28_A::_1)
    }
}
#[doc = "Field `PSARB29` reader - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
pub type PSARB29_R = crate::BitReader<PSARB29_A>;
#[doc = "SCI2 and the MSTPCRB.MSTPB29 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB29_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB29_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB29_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB29_A {
        match self.bits {
            false => PSARB29_A::_0,
            true => PSARB29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB29_A::_1
    }
}
#[doc = "Field `PSARB29` writer - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
pub type PSARB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB29_A, O>;
impl<'a, const O: u8> PSARB29_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB29_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB29_A::_1)
    }
}
#[doc = "Field `PSARB30` reader - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
pub type PSARB30_R = crate::BitReader<PSARB30_A>;
#[doc = "SCI1 and the MSTPCRB.MSTPB30 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB30_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB30_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB30_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB30_A {
        match self.bits {
            false => PSARB30_A::_0,
            true => PSARB30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB30_A::_1
    }
}
#[doc = "Field `PSARB30` writer - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
pub type PSARB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB30_A, O>;
impl<'a, const O: u8> PSARB30_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB30_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB30_A::_1)
    }
}
#[doc = "Field `PSARB31` reader - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
pub type PSARB31_R = crate::BitReader<PSARB31_A>;
#[doc = "SCI0 and the MSTPCRB.MSTPB31 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB31_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB31_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB31_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB31_A {
        match self.bits {
            false => PSARB31_A::_0,
            true => PSARB31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB31_A::_1
    }
}
#[doc = "Field `PSARB31` writer - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
pub type PSARB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB31_A, O>;
impl<'a, const O: u8> PSARB31_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB31_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB31_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
    #[inline(always)]
    pub fn psarb8(&self) -> PSARB8_R {
        PSARB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
    #[inline(always)]
    pub fn psarb9(&self) -> PSARB9_R {
        PSARB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 18 - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
    #[inline(always)]
    pub fn psarb18(&self) -> PSARB18_R {
        PSARB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
    #[inline(always)]
    pub fn psarb19(&self) -> PSARB19_R {
        PSARB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
    #[inline(always)]
    pub fn psarb22(&self) -> PSARB22_R {
        PSARB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
    #[inline(always)]
    pub fn psarb27(&self) -> PSARB27_R {
        PSARB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
    #[inline(always)]
    pub fn psarb28(&self) -> PSARB28_R {
        PSARB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
    #[inline(always)]
    pub fn psarb29(&self) -> PSARB29_R {
        PSARB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
    #[inline(always)]
    pub fn psarb30(&self) -> PSARB30_R {
        PSARB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
    #[inline(always)]
    pub fn psarb31(&self) -> PSARB31_R {
        PSARB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb8(&mut self) -> PSARB8_W<8> {
        PSARB8_W::new(self)
    }
    #[doc = "Bit 9 - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb9(&mut self) -> PSARB9_W<9> {
        PSARB9_W::new(self)
    }
    #[doc = "Bit 18 - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb18(&mut self) -> PSARB18_W<18> {
        PSARB18_W::new(self)
    }
    #[doc = "Bit 19 - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb19(&mut self) -> PSARB19_W<19> {
        PSARB19_W::new(self)
    }
    #[doc = "Bit 22 - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb22(&mut self) -> PSARB22_W<22> {
        PSARB22_W::new(self)
    }
    #[doc = "Bit 27 - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb27(&mut self) -> PSARB27_W<27> {
        PSARB27_W::new(self)
    }
    #[doc = "Bit 28 - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb28(&mut self) -> PSARB28_W<28> {
        PSARB28_W::new(self)
    }
    #[doc = "Bit 29 - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb29(&mut self) -> PSARB29_W<29> {
        PSARB29_W::new(self)
    }
    #[doc = "Bit 30 - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb30(&mut self) -> PSARB30_W<30> {
        PSARB30_W::new(self)
    }
    #[doc = "Bit 31 - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb31(&mut self) -> PSARB31_W<31> {
        PSARB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Security Attribution Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psarb](index.html) module"]
pub struct PSARB_SPEC;
impl crate::RegisterSpec for PSARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psarb::R](R) reader structure"]
impl crate::Readable for PSARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psarb::W](W) writer structure"]
impl crate::Writable for PSARB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSARB to value 0xffff_ffff"]
impl crate::Resettable for PSARB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
