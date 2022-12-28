#[doc = "Register `SNZREQCR` reader"]
pub struct R(crate::R<SNZREQCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNZREQCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNZREQCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNZREQCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNZREQCR` writer"]
pub struct W(crate::W<SNZREQCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNZREQCR_SPEC>;
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
impl From<crate::W<SNZREQCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNZREQCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNZREQEN0` reader - Enable IRQ0 pin snooze request"]
pub type SNZREQEN0_R = crate::BitReader<SNZREQEN0_A>;
#[doc = "Enable IRQ0 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN0_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN0_A {
        match self.bits {
            false => SNZREQEN0_A::_0,
            true => SNZREQEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN0_A::_1
    }
}
#[doc = "Field `SNZREQEN0` writer - Enable IRQ0 pin snooze request"]
pub type SNZREQEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN0_A, O>;
impl<'a, const O: u8> SNZREQEN0_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN0_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN0_A::_1)
    }
}
#[doc = "Field `SNZREQEN1` reader - Enable IRQ1 pin snooze request"]
pub type SNZREQEN1_R = crate::BitReader<SNZREQEN1_A>;
#[doc = "Enable IRQ1 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN1_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN1_A {
        match self.bits {
            false => SNZREQEN1_A::_0,
            true => SNZREQEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN1_A::_1
    }
}
#[doc = "Field `SNZREQEN1` writer - Enable IRQ1 pin snooze request"]
pub type SNZREQEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN1_A, O>;
impl<'a, const O: u8> SNZREQEN1_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN1_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN1_A::_1)
    }
}
#[doc = "Field `SNZREQEN2` reader - Enable IRQ2 pin snooze request"]
pub type SNZREQEN2_R = crate::BitReader<SNZREQEN2_A>;
#[doc = "Enable IRQ2 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN2_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN2_A {
        match self.bits {
            false => SNZREQEN2_A::_0,
            true => SNZREQEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN2_A::_1
    }
}
#[doc = "Field `SNZREQEN2` writer - Enable IRQ2 pin snooze request"]
pub type SNZREQEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN2_A, O>;
impl<'a, const O: u8> SNZREQEN2_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN2_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN2_A::_1)
    }
}
#[doc = "Field `SNZREQEN3` reader - Enable IRQ3 pin snooze request"]
pub type SNZREQEN3_R = crate::BitReader<SNZREQEN3_A>;
#[doc = "Enable IRQ3 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN3_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN3_A {
        match self.bits {
            false => SNZREQEN3_A::_0,
            true => SNZREQEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN3_A::_1
    }
}
#[doc = "Field `SNZREQEN3` writer - Enable IRQ3 pin snooze request"]
pub type SNZREQEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN3_A, O>;
impl<'a, const O: u8> SNZREQEN3_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN3_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN3_A::_1)
    }
}
#[doc = "Field `SNZREQEN4` reader - Enable IRQ4 pin snooze request"]
pub type SNZREQEN4_R = crate::BitReader<SNZREQEN4_A>;
#[doc = "Enable IRQ4 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN4_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN4_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN4_A {
        match self.bits {
            false => SNZREQEN4_A::_0,
            true => SNZREQEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN4_A::_1
    }
}
#[doc = "Field `SNZREQEN4` writer - Enable IRQ4 pin snooze request"]
pub type SNZREQEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN4_A, O>;
impl<'a, const O: u8> SNZREQEN4_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN4_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN4_A::_1)
    }
}
#[doc = "Field `SNZREQEN5` reader - Enable IRQ5 pin snooze request"]
pub type SNZREQEN5_R = crate::BitReader<SNZREQEN5_A>;
#[doc = "Enable IRQ5 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN5_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN5_A {
        match self.bits {
            false => SNZREQEN5_A::_0,
            true => SNZREQEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN5_A::_1
    }
}
#[doc = "Field `SNZREQEN5` writer - Enable IRQ5 pin snooze request"]
pub type SNZREQEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN5_A, O>;
impl<'a, const O: u8> SNZREQEN5_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN5_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN5_A::_1)
    }
}
#[doc = "Field `SNZREQEN6` reader - Enable IRQ6 pin snooze request"]
pub type SNZREQEN6_R = crate::BitReader<SNZREQEN6_A>;
#[doc = "Enable IRQ6 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN6_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN6_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN6_A {
        match self.bits {
            false => SNZREQEN6_A::_0,
            true => SNZREQEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN6_A::_1
    }
}
#[doc = "Field `SNZREQEN6` writer - Enable IRQ6 pin snooze request"]
pub type SNZREQEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN6_A, O>;
impl<'a, const O: u8> SNZREQEN6_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN6_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN6_A::_1)
    }
}
#[doc = "Field `SNZREQEN7` reader - Enable IRQ7 pin snooze request"]
pub type SNZREQEN7_R = crate::BitReader<SNZREQEN7_A>;
#[doc = "Enable IRQ7 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN7_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN7_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN7_A {
        match self.bits {
            false => SNZREQEN7_A::_0,
            true => SNZREQEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN7_A::_1
    }
}
#[doc = "Field `SNZREQEN7` writer - Enable IRQ7 pin snooze request"]
pub type SNZREQEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN7_A, O>;
impl<'a, const O: u8> SNZREQEN7_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN7_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN7_A::_1)
    }
}
#[doc = "Field `SNZREQEN8` reader - Enable IRQ8 pin snooze request"]
pub type SNZREQEN8_R = crate::BitReader<SNZREQEN8_A>;
#[doc = "Enable IRQ8 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN8_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN8_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN8_A {
        match self.bits {
            false => SNZREQEN8_A::_0,
            true => SNZREQEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN8_A::_1
    }
}
#[doc = "Field `SNZREQEN8` writer - Enable IRQ8 pin snooze request"]
pub type SNZREQEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN8_A, O>;
impl<'a, const O: u8> SNZREQEN8_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN8_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN8_A::_1)
    }
}
#[doc = "Field `SNZREQEN9` reader - Enable IRQ9 pin snooze request"]
pub type SNZREQEN9_R = crate::BitReader<SNZREQEN9_A>;
#[doc = "Enable IRQ9 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN9_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN9_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN9_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN9_A {
        match self.bits {
            false => SNZREQEN9_A::_0,
            true => SNZREQEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN9_A::_1
    }
}
#[doc = "Field `SNZREQEN9` writer - Enable IRQ9 pin snooze request"]
pub type SNZREQEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN9_A, O>;
impl<'a, const O: u8> SNZREQEN9_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN9_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN9_A::_1)
    }
}
#[doc = "Field `SNZREQEN10` reader - Enable IRQ10 pin snooze request"]
pub type SNZREQEN10_R = crate::BitReader<SNZREQEN10_A>;
#[doc = "Enable IRQ10 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN10_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN10_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN10_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN10_A {
        match self.bits {
            false => SNZREQEN10_A::_0,
            true => SNZREQEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN10_A::_1
    }
}
#[doc = "Field `SNZREQEN10` writer - Enable IRQ10 pin snooze request"]
pub type SNZREQEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN10_A, O>;
impl<'a, const O: u8> SNZREQEN10_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN10_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN10_A::_1)
    }
}
#[doc = "Field `SNZREQEN11` reader - Enable IRQ11 pin snooze request"]
pub type SNZREQEN11_R = crate::BitReader<SNZREQEN11_A>;
#[doc = "Enable IRQ11 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN11_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN11_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN11_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN11_A {
        match self.bits {
            false => SNZREQEN11_A::_0,
            true => SNZREQEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN11_A::_1
    }
}
#[doc = "Field `SNZREQEN11` writer - Enable IRQ11 pin snooze request"]
pub type SNZREQEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN11_A, O>;
impl<'a, const O: u8> SNZREQEN11_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN11_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN11_A::_1)
    }
}
#[doc = "Field `SNZREQEN12` reader - Enable IRQ12 pin snooze request"]
pub type SNZREQEN12_R = crate::BitReader<SNZREQEN12_A>;
#[doc = "Enable IRQ12 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN12_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN12_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN12_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN12_A {
        match self.bits {
            false => SNZREQEN12_A::_0,
            true => SNZREQEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN12_A::_1
    }
}
#[doc = "Field `SNZREQEN12` writer - Enable IRQ12 pin snooze request"]
pub type SNZREQEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN12_A, O>;
impl<'a, const O: u8> SNZREQEN12_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN12_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN12_A::_1)
    }
}
#[doc = "Field `SNZREQEN13` reader - Enable IRQ13 pin snooze request"]
pub type SNZREQEN13_R = crate::BitReader<SNZREQEN13_A>;
#[doc = "Enable IRQ13 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN13_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN13_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN13_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN13_A {
        match self.bits {
            false => SNZREQEN13_A::_0,
            true => SNZREQEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN13_A::_1
    }
}
#[doc = "Field `SNZREQEN13` writer - Enable IRQ13 pin snooze request"]
pub type SNZREQEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN13_A, O>;
impl<'a, const O: u8> SNZREQEN13_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN13_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN13_A::_1)
    }
}
#[doc = "Field `SNZREQEN17` reader - Enable KR snooze request"]
pub type SNZREQEN17_R = crate::BitReader<SNZREQEN17_A>;
#[doc = "Enable KR snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN17_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN17_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN17_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN17_A {
        match self.bits {
            false => SNZREQEN17_A::_0,
            true => SNZREQEN17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN17_A::_1
    }
}
#[doc = "Field `SNZREQEN17` writer - Enable KR snooze request"]
pub type SNZREQEN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN17_A, O>;
impl<'a, const O: u8> SNZREQEN17_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN17_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN17_A::_1)
    }
}
#[doc = "Field `SNZREQEN22` reader - Enable ACMPHS0 snooze request"]
pub type SNZREQEN22_R = crate::BitReader<SNZREQEN22_A>;
#[doc = "Enable ACMPHS0 snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN22_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN22_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN22_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN22_A {
        match self.bits {
            false => SNZREQEN22_A::_0,
            true => SNZREQEN22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN22_A::_1
    }
}
#[doc = "Field `SNZREQEN22` writer - Enable ACMPHS0 snooze request"]
pub type SNZREQEN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN22_A, O>;
impl<'a, const O: u8> SNZREQEN22_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN22_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN22_A::_1)
    }
}
#[doc = "Field `SNZREQEN24` reader - Enable RTC alarm snooze request"]
pub type SNZREQEN24_R = crate::BitReader<SNZREQEN24_A>;
#[doc = "Enable RTC alarm snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN24_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN24_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN24_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN24_A {
        match self.bits {
            false => SNZREQEN24_A::_0,
            true => SNZREQEN24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN24_A::_1
    }
}
#[doc = "Field `SNZREQEN24` writer - Enable RTC alarm snooze request"]
pub type SNZREQEN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN24_A, O>;
impl<'a, const O: u8> SNZREQEN24_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN24_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN24_A::_1)
    }
}
#[doc = "Field `SNZREQEN25` reader - Enable RTC period snooze request"]
pub type SNZREQEN25_R = crate::BitReader<SNZREQEN25_A>;
#[doc = "Enable RTC period snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN25_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN25_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN25_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN25_A {
        match self.bits {
            false => SNZREQEN25_A::_0,
            true => SNZREQEN25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN25_A::_1
    }
}
#[doc = "Field `SNZREQEN25` writer - Enable RTC period snooze request"]
pub type SNZREQEN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN25_A, O>;
impl<'a, const O: u8> SNZREQEN25_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN25_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN25_A::_1)
    }
}
#[doc = "Field `SNZREQEN28` reader - Enable AGT1 underflow snooze request"]
pub type SNZREQEN28_R = crate::BitReader<SNZREQEN28_A>;
#[doc = "Enable AGT1 underflow snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN28_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN28_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN28_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN28_A {
        match self.bits {
            false => SNZREQEN28_A::_0,
            true => SNZREQEN28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN28_A::_1
    }
}
#[doc = "Field `SNZREQEN28` writer - Enable AGT1 underflow snooze request"]
pub type SNZREQEN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN28_A, O>;
impl<'a, const O: u8> SNZREQEN28_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN28_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN28_A::_1)
    }
}
#[doc = "Field `SNZREQEN29` reader - Enable AGT1 compare match A snooze request"]
pub type SNZREQEN29_R = crate::BitReader<SNZREQEN29_A>;
#[doc = "Enable AGT1 compare match A snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN29_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN29_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN29_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN29_A {
        match self.bits {
            false => SNZREQEN29_A::_0,
            true => SNZREQEN29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN29_A::_1
    }
}
#[doc = "Field `SNZREQEN29` writer - Enable AGT1 compare match A snooze request"]
pub type SNZREQEN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN29_A, O>;
impl<'a, const O: u8> SNZREQEN29_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN29_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN29_A::_1)
    }
}
#[doc = "Field `SNZREQEN30` reader - Enable AGT1 compare match B snooze request"]
pub type SNZREQEN30_R = crate::BitReader<SNZREQEN30_A>;
#[doc = "Enable AGT1 compare match B snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN30_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN30_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN30_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZREQEN30_A {
        match self.bits {
            false => SNZREQEN30_A::_0,
            true => SNZREQEN30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN30_A::_1
    }
}
#[doc = "Field `SNZREQEN30` writer - Enable AGT1 compare match B snooze request"]
pub type SNZREQEN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNZREQCR_SPEC, SNZREQEN30_A, O>;
impl<'a, const O: u8> SNZREQEN30_W<'a, O> {
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZREQEN30_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZREQEN30_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(&self) -> SNZREQEN0_R {
        SNZREQEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(&self) -> SNZREQEN1_R {
        SNZREQEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(&self) -> SNZREQEN2_R {
        SNZREQEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(&self) -> SNZREQEN3_R {
        SNZREQEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(&self) -> SNZREQEN4_R {
        SNZREQEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(&self) -> SNZREQEN5_R {
        SNZREQEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(&self) -> SNZREQEN6_R {
        SNZREQEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(&self) -> SNZREQEN7_R {
        SNZREQEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable IRQ8 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen8(&self) -> SNZREQEN8_R {
        SNZREQEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable IRQ9 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen9(&self) -> SNZREQEN9_R {
        SNZREQEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable IRQ10 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen10(&self) -> SNZREQEN10_R {
        SNZREQEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable IRQ11 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen11(&self) -> SNZREQEN11_R {
        SNZREQEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable IRQ12 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen12(&self) -> SNZREQEN12_R {
        SNZREQEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable IRQ13 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen13(&self) -> SNZREQEN13_R {
        SNZREQEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable KR snooze request"]
    #[inline(always)]
    pub fn snzreqen17(&self) -> SNZREQEN17_R {
        SNZREQEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable ACMPHS0 snooze request"]
    #[inline(always)]
    pub fn snzreqen22(&self) -> SNZREQEN22_R {
        SNZREQEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(&self) -> SNZREQEN24_R {
        SNZREQEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable RTC period snooze request"]
    #[inline(always)]
    pub fn snzreqen25(&self) -> SNZREQEN25_R {
        SNZREQEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable AGT1 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen28(&self) -> SNZREQEN28_R {
        SNZREQEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen29(&self) -> SNZREQEN29_R {
        SNZREQEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen30(&self) -> SNZREQEN30_R {
        SNZREQEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IRQ0 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen0(&mut self) -> SNZREQEN0_W<0> {
        SNZREQEN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable IRQ1 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen1(&mut self) -> SNZREQEN1_W<1> {
        SNZREQEN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable IRQ2 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen2(&mut self) -> SNZREQEN2_W<2> {
        SNZREQEN2_W::new(self)
    }
    #[doc = "Bit 3 - Enable IRQ3 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen3(&mut self) -> SNZREQEN3_W<3> {
        SNZREQEN3_W::new(self)
    }
    #[doc = "Bit 4 - Enable IRQ4 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen4(&mut self) -> SNZREQEN4_W<4> {
        SNZREQEN4_W::new(self)
    }
    #[doc = "Bit 5 - Enable IRQ5 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen5(&mut self) -> SNZREQEN5_W<5> {
        SNZREQEN5_W::new(self)
    }
    #[doc = "Bit 6 - Enable IRQ6 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen6(&mut self) -> SNZREQEN6_W<6> {
        SNZREQEN6_W::new(self)
    }
    #[doc = "Bit 7 - Enable IRQ7 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen7(&mut self) -> SNZREQEN7_W<7> {
        SNZREQEN7_W::new(self)
    }
    #[doc = "Bit 8 - Enable IRQ8 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen8(&mut self) -> SNZREQEN8_W<8> {
        SNZREQEN8_W::new(self)
    }
    #[doc = "Bit 9 - Enable IRQ9 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen9(&mut self) -> SNZREQEN9_W<9> {
        SNZREQEN9_W::new(self)
    }
    #[doc = "Bit 10 - Enable IRQ10 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen10(&mut self) -> SNZREQEN10_W<10> {
        SNZREQEN10_W::new(self)
    }
    #[doc = "Bit 11 - Enable IRQ11 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen11(&mut self) -> SNZREQEN11_W<11> {
        SNZREQEN11_W::new(self)
    }
    #[doc = "Bit 12 - Enable IRQ12 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen12(&mut self) -> SNZREQEN12_W<12> {
        SNZREQEN12_W::new(self)
    }
    #[doc = "Bit 13 - Enable IRQ13 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen13(&mut self) -> SNZREQEN13_W<13> {
        SNZREQEN13_W::new(self)
    }
    #[doc = "Bit 17 - Enable KR snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen17(&mut self) -> SNZREQEN17_W<17> {
        SNZREQEN17_W::new(self)
    }
    #[doc = "Bit 22 - Enable ACMPHS0 snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen22(&mut self) -> SNZREQEN22_W<22> {
        SNZREQEN22_W::new(self)
    }
    #[doc = "Bit 24 - Enable RTC alarm snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen24(&mut self) -> SNZREQEN24_W<24> {
        SNZREQEN24_W::new(self)
    }
    #[doc = "Bit 25 - Enable RTC period snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen25(&mut self) -> SNZREQEN25_W<25> {
        SNZREQEN25_W::new(self)
    }
    #[doc = "Bit 28 - Enable AGT1 underflow snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen28(&mut self) -> SNZREQEN28_W<28> {
        SNZREQEN28_W::new(self)
    }
    #[doc = "Bit 29 - Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen29(&mut self) -> SNZREQEN29_W<29> {
        SNZREQEN29_W::new(self)
    }
    #[doc = "Bit 30 - Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen30(&mut self) -> SNZREQEN30_W<30> {
        SNZREQEN30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snooze Request Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snzreqcr](index.html) module"]
pub struct SNZREQCR_SPEC;
impl crate::RegisterSpec for SNZREQCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snzreqcr::R](R) reader structure"]
impl crate::Readable for SNZREQCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snzreqcr::W](W) writer structure"]
impl crate::Writable for SNZREQCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZREQCR to value 0"]
impl crate::Resettable for SNZREQCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
