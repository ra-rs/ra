#[doc = "Register `ADCMPLR0` reader"]
pub struct R(crate::R<ADCMPLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPLR0` writer"]
pub struct W(crate::W<ADCMPLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPLR0_SPEC>;
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
impl From<crate::W<ADCMPLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPLCHA0` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA0_R = crate::BitReader<CMPLCHA0_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA0_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA0_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA0_A {
        match self.bits {
            false => CMPLCHA0_A::_0,
            true => CMPLCHA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA0_A::_1
    }
}
#[doc = "Field `CMPLCHA0` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA0_A, O>;
impl<'a, const O: u8> CMPLCHA0_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA0_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA0_A::_1)
    }
}
#[doc = "Field `CMPLCHA1` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA1_R = crate::BitReader<CMPLCHA1_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA1_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA1_A {
        match self.bits {
            false => CMPLCHA1_A::_0,
            true => CMPLCHA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA1_A::_1
    }
}
#[doc = "Field `CMPLCHA1` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA1_A, O>;
impl<'a, const O: u8> CMPLCHA1_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA1_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA1_A::_1)
    }
}
#[doc = "Field `CMPLCHA2` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA2_R = crate::BitReader<CMPLCHA2_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA2_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA2_A {
        match self.bits {
            false => CMPLCHA2_A::_0,
            true => CMPLCHA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA2_A::_1
    }
}
#[doc = "Field `CMPLCHA2` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA2_A, O>;
impl<'a, const O: u8> CMPLCHA2_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA2_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA2_A::_1)
    }
}
#[doc = "Field `CMPLCHA3` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA3_R = crate::BitReader<CMPLCHA3_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA3_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA3_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA3_A {
        match self.bits {
            false => CMPLCHA3_A::_0,
            true => CMPLCHA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA3_A::_1
    }
}
#[doc = "Field `CMPLCHA3` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA3_A, O>;
impl<'a, const O: u8> CMPLCHA3_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA3_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA3_A::_1)
    }
}
#[doc = "Field `CMPLCHA4` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA4_R = crate::BitReader<CMPLCHA4_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA4_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA4_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA4_A {
        match self.bits {
            false => CMPLCHA4_A::_0,
            true => CMPLCHA4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA4_A::_1
    }
}
#[doc = "Field `CMPLCHA4` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA4_A, O>;
impl<'a, const O: u8> CMPLCHA4_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA4_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA4_A::_1)
    }
}
#[doc = "Field `CMPLCHA5` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA5_R = crate::BitReader<CMPLCHA5_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA5_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA5_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA5_A {
        match self.bits {
            false => CMPLCHA5_A::_0,
            true => CMPLCHA5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA5_A::_1
    }
}
#[doc = "Field `CMPLCHA5` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA5_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA5_A, O>;
impl<'a, const O: u8> CMPLCHA5_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA5_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA5_A::_1)
    }
}
#[doc = "Field `CMPLCHA6` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA6_R = crate::BitReader<CMPLCHA6_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA6_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA6_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA6_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA6_A {
        match self.bits {
            false => CMPLCHA6_A::_0,
            true => CMPLCHA6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA6_A::_1
    }
}
#[doc = "Field `CMPLCHA6` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA6_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA6_A, O>;
impl<'a, const O: u8> CMPLCHA6_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA6_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA6_A::_1)
    }
}
#[doc = "Field `CMPLCHA7` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA7_R = crate::BitReader<CMPLCHA7_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA7_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA7_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA7_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA7_A {
        match self.bits {
            false => CMPLCHA7_A::_0,
            true => CMPLCHA7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA7_A::_1
    }
}
#[doc = "Field `CMPLCHA7` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA7_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA7_A, O>;
impl<'a, const O: u8> CMPLCHA7_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA7_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA7_A::_1)
    }
}
#[doc = "Field `CMPLCHA8` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA8_R = crate::BitReader<CMPLCHA8_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA8_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA8_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA8_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA8_A {
        match self.bits {
            false => CMPLCHA8_A::_0,
            true => CMPLCHA8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA8_A::_1
    }
}
#[doc = "Field `CMPLCHA8` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA8_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA8_A, O>;
impl<'a, const O: u8> CMPLCHA8_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA8_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA8_A::_1)
    }
}
#[doc = "Field `CMPLCHA12` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA12_R = crate::BitReader<CMPLCHA12_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA12_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA12_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA12_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA12_A {
        match self.bits {
            false => CMPLCHA12_A::_0,
            true => CMPLCHA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA12_A::_1
    }
}
#[doc = "Field `CMPLCHA12` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA12_A, O>;
impl<'a, const O: u8> CMPLCHA12_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA12_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA12_A::_1)
    }
}
#[doc = "Field `CMPLCHA13` reader - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA13_R = crate::BitReader<CMPLCHA13_A>;
#[doc = "Compare Window A Comparison Condition Select n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA13_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHA13_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA13_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA13_A {
        match self.bits {
            false => CMPLCHA13_A::_0,
            true => CMPLCHA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA13_A::_1
    }
}
#[doc = "Field `CMPLCHA13` writer - Compare Window A Comparison Condition Select n"]
pub type CMPLCHA13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA13_A, O>;
impl<'a, const O: u8> CMPLCHA13_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA13_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA13_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha0(&self) -> CMPLCHA0_R {
        CMPLCHA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha1(&self) -> CMPLCHA1_R {
        CMPLCHA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha2(&self) -> CMPLCHA2_R {
        CMPLCHA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha3(&self) -> CMPLCHA3_R {
        CMPLCHA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha4(&self) -> CMPLCHA4_R {
        CMPLCHA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha5(&self) -> CMPLCHA5_R {
        CMPLCHA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha6(&self) -> CMPLCHA6_R {
        CMPLCHA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha7(&self) -> CMPLCHA7_R {
        CMPLCHA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha8(&self) -> CMPLCHA8_R {
        CMPLCHA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha12(&self) -> CMPLCHA12_R {
        CMPLCHA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha13(&self) -> CMPLCHA13_R {
        CMPLCHA13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha0(&mut self) -> CMPLCHA0_W<0> {
        CMPLCHA0_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha1(&mut self) -> CMPLCHA1_W<1> {
        CMPLCHA1_W::new(self)
    }
    #[doc = "Bit 2 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha2(&mut self) -> CMPLCHA2_W<2> {
        CMPLCHA2_W::new(self)
    }
    #[doc = "Bit 3 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha3(&mut self) -> CMPLCHA3_W<3> {
        CMPLCHA3_W::new(self)
    }
    #[doc = "Bit 4 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha4(&mut self) -> CMPLCHA4_W<4> {
        CMPLCHA4_W::new(self)
    }
    #[doc = "Bit 5 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha5(&mut self) -> CMPLCHA5_W<5> {
        CMPLCHA5_W::new(self)
    }
    #[doc = "Bit 6 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha6(&mut self) -> CMPLCHA6_W<6> {
        CMPLCHA6_W::new(self)
    }
    #[doc = "Bit 7 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha7(&mut self) -> CMPLCHA7_W<7> {
        CMPLCHA7_W::new(self)
    }
    #[doc = "Bit 8 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha8(&mut self) -> CMPLCHA8_W<8> {
        CMPLCHA8_W::new(self)
    }
    #[doc = "Bit 12 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha12(&mut self) -> CMPLCHA12_W<12> {
        CMPLCHA12_W::new(self)
    }
    #[doc = "Bit 13 - Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha13(&mut self) -> CMPLCHA13_W<13> {
        CMPLCHA13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmplr0](index.html) module"]
pub struct ADCMPLR0_SPEC;
impl crate::RegisterSpec for ADCMPLR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmplr0::R](R) reader structure"]
impl crate::Readable for ADCMPLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmplr0::W](W) writer structure"]
impl crate::Writable for ADCMPLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPLR0 to value 0"]
impl crate::Resettable for ADCMPLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
