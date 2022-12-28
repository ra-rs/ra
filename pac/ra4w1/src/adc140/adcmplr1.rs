#[doc = "Register `ADCMPLR1` reader"]
pub struct R(crate::R<ADCMPLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPLR1` writer"]
pub struct W(crate::W<ADCMPLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPLR1_SPEC>;
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
impl From<crate::W<ADCMPLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPLCHA16` reader - Comparison condition of AN016"]
pub type CMPLCHA16_R = crate::BitReader<CMPLCHA16_A>;
#[doc = "Comparison condition of AN016\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA16_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA16_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA16_A {
        match self.bits {
            false => CMPLCHA16_A::_0,
            true => CMPLCHA16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA16_A::_1
    }
}
#[doc = "Field `CMPLCHA16` writer - Comparison condition of AN016"]
pub type CMPLCHA16_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA16_A, O>;
impl<'a, const O: u8> CMPLCHA16_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA16_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA16_A::_1)
    }
}
#[doc = "Field `CMPLCHA17` reader - Comparison condition of AN017"]
pub type CMPLCHA17_R = crate::BitReader<CMPLCHA17_A>;
#[doc = "Comparison condition of AN017\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA17_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA17_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA17_A {
        match self.bits {
            false => CMPLCHA17_A::_0,
            true => CMPLCHA17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA17_A::_1
    }
}
#[doc = "Field `CMPLCHA17` writer - Comparison condition of AN017"]
pub type CMPLCHA17_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA17_A, O>;
impl<'a, const O: u8> CMPLCHA17_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA17_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA17_A::_1)
    }
}
#[doc = "Field `CMPLCHA18` reader - Comparison condition of AN018"]
pub type CMPLCHA18_R = crate::BitReader<CMPLCHA18_A>;
#[doc = "Comparison condition of AN018\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA18_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA18_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA18_A {
        match self.bits {
            false => CMPLCHA18_A::_0,
            true => CMPLCHA18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA18_A::_1
    }
}
#[doc = "Field `CMPLCHA18` writer - Comparison condition of AN018"]
pub type CMPLCHA18_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA18_A, O>;
impl<'a, const O: u8> CMPLCHA18_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA18_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA18_A::_1)
    }
}
#[doc = "Field `CMPLCHA19` reader - Comparison condition of AN019"]
pub type CMPLCHA19_R = crate::BitReader<CMPLCHA19_A>;
#[doc = "Comparison condition of AN019\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA19_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA19_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA19_A {
        match self.bits {
            false => CMPLCHA19_A::_0,
            true => CMPLCHA19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA19_A::_1
    }
}
#[doc = "Field `CMPLCHA19` writer - Comparison condition of AN019"]
pub type CMPLCHA19_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA19_A, O>;
impl<'a, const O: u8> CMPLCHA19_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA19_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA19_A::_1)
    }
}
#[doc = "Field `CMPLCHA20` reader - Comparison condition of AN020"]
pub type CMPLCHA20_R = crate::BitReader<CMPLCHA20_A>;
#[doc = "Comparison condition of AN020\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA20_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA20_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA20_A {
        match self.bits {
            false => CMPLCHA20_A::_0,
            true => CMPLCHA20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA20_A::_1
    }
}
#[doc = "Field `CMPLCHA20` writer - Comparison condition of AN020"]
pub type CMPLCHA20_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA20_A, O>;
impl<'a, const O: u8> CMPLCHA20_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA20_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA20_A::_1)
    }
}
#[doc = "Field `CMPLCHA21` reader - Comparison condition of AN021"]
pub type CMPLCHA21_R = crate::BitReader<CMPLCHA21_A>;
#[doc = "Comparison condition of AN021\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA21_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA21_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA21_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA21_A {
        match self.bits {
            false => CMPLCHA21_A::_0,
            true => CMPLCHA21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA21_A::_1
    }
}
#[doc = "Field `CMPLCHA21` writer - Comparison condition of AN021"]
pub type CMPLCHA21_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA21_A, O>;
impl<'a, const O: u8> CMPLCHA21_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA21_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA21_A::_1)
    }
}
#[doc = "Field `CMPLCHA22` reader - Comparison condition of AN022"]
pub type CMPLCHA22_R = crate::BitReader<CMPLCHA22_A>;
#[doc = "Comparison condition of AN022\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA22_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA22_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA22_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA22_A {
        match self.bits {
            false => CMPLCHA22_A::_0,
            true => CMPLCHA22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA22_A::_1
    }
}
#[doc = "Field `CMPLCHA22` writer - Comparison condition of AN022"]
pub type CMPLCHA22_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA22_A, O>;
impl<'a, const O: u8> CMPLCHA22_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA22_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA22_A::_1)
    }
}
#[doc = "Field `CMPLCHA23` reader - Comparison condition of AN023"]
pub type CMPLCHA23_R = crate::BitReader<CMPLCHA23_A>;
#[doc = "Comparison condition of AN023\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA23_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA23_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA23_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA23_A {
        match self.bits {
            false => CMPLCHA23_A::_0,
            true => CMPLCHA23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA23_A::_1
    }
}
#[doc = "Field `CMPLCHA23` writer - Comparison condition of AN023"]
pub type CMPLCHA23_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA23_A, O>;
impl<'a, const O: u8> CMPLCHA23_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA23_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA23_A::_1)
    }
}
#[doc = "Field `CMPLCHA24` reader - Comparison condition of AN024"]
pub type CMPLCHA24_R = crate::BitReader<CMPLCHA24_A>;
#[doc = "Comparison condition of AN024\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA24_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA24_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA24_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA24_A {
        match self.bits {
            false => CMPLCHA24_A::_0,
            true => CMPLCHA24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA24_A::_1
    }
}
#[doc = "Field `CMPLCHA24` writer - Comparison condition of AN024"]
pub type CMPLCHA24_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA24_A, O>;
impl<'a, const O: u8> CMPLCHA24_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA24_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA24_A::_1)
    }
}
#[doc = "Field `CMPLCHA25` reader - Comparison condition of AN025"]
pub type CMPLCHA25_R = crate::BitReader<CMPLCHA25_A>;
#[doc = "Comparison condition of AN025\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA25_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA25_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA25_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA25_A {
        match self.bits {
            false => CMPLCHA25_A::_0,
            true => CMPLCHA25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA25_A::_1
    }
}
#[doc = "Field `CMPLCHA25` writer - Comparison condition of AN025"]
pub type CMPLCHA25_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA25_A, O>;
impl<'a, const O: u8> CMPLCHA25_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA25_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA25_A::_1)
    }
}
#[doc = "Field `CMPLCHA26` reader - Comparison condition of AN026"]
pub type CMPLCHA26_R = crate::BitReader<CMPLCHA26_A>;
#[doc = "Comparison condition of AN026\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA26_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA26_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA26_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA26_A {
        match self.bits {
            false => CMPLCHA26_A::_0,
            true => CMPLCHA26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA26_A::_1
    }
}
#[doc = "Field `CMPLCHA26` writer - Comparison condition of AN026"]
pub type CMPLCHA26_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA26_A, O>;
impl<'a, const O: u8> CMPLCHA26_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA26_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA26_A::_1)
    }
}
#[doc = "Field `CMPLCHA27` reader - Comparison condition of AN027"]
pub type CMPLCHA27_R = crate::BitReader<CMPLCHA27_A>;
#[doc = "Comparison condition of AN027\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA27_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA27_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA27_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA27_A {
        match self.bits {
            false => CMPLCHA27_A::_0,
            true => CMPLCHA27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA27_A::_1
    }
}
#[doc = "Field `CMPLCHA27` writer - Comparison condition of AN027"]
pub type CMPLCHA27_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR1_SPEC, CMPLCHA27_A, O>;
impl<'a, const O: u8> CMPLCHA27_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA27_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA27_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparison condition of AN016"]
    #[inline(always)]
    pub fn cmplcha16(&self) -> CMPLCHA16_R {
        CMPLCHA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison condition of AN017"]
    #[inline(always)]
    pub fn cmplcha17(&self) -> CMPLCHA17_R {
        CMPLCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison condition of AN018"]
    #[inline(always)]
    pub fn cmplcha18(&self) -> CMPLCHA18_R {
        CMPLCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparison condition of AN019"]
    #[inline(always)]
    pub fn cmplcha19(&self) -> CMPLCHA19_R {
        CMPLCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison condition of AN020"]
    #[inline(always)]
    pub fn cmplcha20(&self) -> CMPLCHA20_R {
        CMPLCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison condition of AN021"]
    #[inline(always)]
    pub fn cmplcha21(&self) -> CMPLCHA21_R {
        CMPLCHA21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison condition of AN022"]
    #[inline(always)]
    pub fn cmplcha22(&self) -> CMPLCHA22_R {
        CMPLCHA22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison condition of AN023"]
    #[inline(always)]
    pub fn cmplcha23(&self) -> CMPLCHA23_R {
        CMPLCHA23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparison condition of AN024"]
    #[inline(always)]
    pub fn cmplcha24(&self) -> CMPLCHA24_R {
        CMPLCHA24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparison condition of AN025"]
    #[inline(always)]
    pub fn cmplcha25(&self) -> CMPLCHA25_R {
        CMPLCHA25_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comparison condition of AN026"]
    #[inline(always)]
    pub fn cmplcha26(&self) -> CMPLCHA26_R {
        CMPLCHA26_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparison condition of AN027"]
    #[inline(always)]
    pub fn cmplcha27(&self) -> CMPLCHA27_R {
        CMPLCHA27_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison condition of AN016"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha16(&mut self) -> CMPLCHA16_W<0> {
        CMPLCHA16_W::new(self)
    }
    #[doc = "Bit 1 - Comparison condition of AN017"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha17(&mut self) -> CMPLCHA17_W<1> {
        CMPLCHA17_W::new(self)
    }
    #[doc = "Bit 2 - Comparison condition of AN018"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha18(&mut self) -> CMPLCHA18_W<2> {
        CMPLCHA18_W::new(self)
    }
    #[doc = "Bit 3 - Comparison condition of AN019"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha19(&mut self) -> CMPLCHA19_W<3> {
        CMPLCHA19_W::new(self)
    }
    #[doc = "Bit 4 - Comparison condition of AN020"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha20(&mut self) -> CMPLCHA20_W<4> {
        CMPLCHA20_W::new(self)
    }
    #[doc = "Bit 5 - Comparison condition of AN021"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha21(&mut self) -> CMPLCHA21_W<5> {
        CMPLCHA21_W::new(self)
    }
    #[doc = "Bit 6 - Comparison condition of AN022"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha22(&mut self) -> CMPLCHA22_W<6> {
        CMPLCHA22_W::new(self)
    }
    #[doc = "Bit 7 - Comparison condition of AN023"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha23(&mut self) -> CMPLCHA23_W<7> {
        CMPLCHA23_W::new(self)
    }
    #[doc = "Bit 8 - Comparison condition of AN024"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha24(&mut self) -> CMPLCHA24_W<8> {
        CMPLCHA24_W::new(self)
    }
    #[doc = "Bit 9 - Comparison condition of AN025"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha25(&mut self) -> CMPLCHA25_W<9> {
        CMPLCHA25_W::new(self)
    }
    #[doc = "Bit 10 - Comparison condition of AN026"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha26(&mut self) -> CMPLCHA26_W<10> {
        CMPLCHA26_W::new(self)
    }
    #[doc = "Bit 11 - Comparison condition of AN027"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha27(&mut self) -> CMPLCHA27_W<11> {
        CMPLCHA27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmplr1](index.html) module"]
pub struct ADCMPLR1_SPEC;
impl crate::RegisterSpec for ADCMPLR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmplr1::R](R) reader structure"]
impl crate::Readable for ADCMPLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmplr1::W](W) writer structure"]
impl crate::Writable for ADCMPLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPLR1 to value 0"]
impl crate::Resettable for ADCMPLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
