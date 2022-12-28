#[doc = "Register `MSTPCRE` reader"]
pub struct R(crate::R<MSTPCRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRE` writer"]
pub struct W(crate::W<MSTPCRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRE_SPEC>;
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
impl From<crate::W<MSTPCRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPE14` reader - Low Power Asynchronous General Purpose Timer 5 Module Stop"]
pub type MSTPE14_R = crate::BitReader<MSTPE14_A>;
#[doc = "Low Power Asynchronous General Purpose Timer 5 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE14_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE14_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE14_A {
        match self.bits {
            false => MSTPE14_A::_0,
            true => MSTPE14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE14_A::_1
    }
}
#[doc = "Field `MSTPE14` writer - Low Power Asynchronous General Purpose Timer 5 Module Stop"]
pub type MSTPE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE14_A, O>;
impl<'a, const O: u8> MSTPE14_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE14_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE14_A::_1)
    }
}
#[doc = "Field `MSTPE15` reader - Low Power Asynchronous General Purpose Timer 4 Module Stop"]
pub type MSTPE15_R = crate::BitReader<MSTPE15_A>;
#[doc = "Low Power Asynchronous General Purpose Timer 4 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE15_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE15_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE15_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE15_A {
        match self.bits {
            false => MSTPE15_A::_0,
            true => MSTPE15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE15_A::_1
    }
}
#[doc = "Field `MSTPE15` writer - Low Power Asynchronous General Purpose Timer 4 Module Stop"]
pub type MSTPE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE15_A, O>;
impl<'a, const O: u8> MSTPE15_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE15_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE15_A::_1)
    }
}
#[doc = "Field `MSTPE22` reader - GPT9 Module Stop"]
pub type MSTPE22_R = crate::BitReader<MSTPE22_A>;
#[doc = "GPT9 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE22_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE22_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE22_A {
        match self.bits {
            false => MSTPE22_A::_0,
            true => MSTPE22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE22_A::_1
    }
}
#[doc = "Field `MSTPE22` writer - GPT9 Module Stop"]
pub type MSTPE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE22_A, O>;
impl<'a, const O: u8> MSTPE22_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE22_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE22_A::_1)
    }
}
#[doc = "Field `MSTPE23` reader - GPT8 Module Stop"]
pub type MSTPE23_R = crate::BitReader<MSTPE23_A>;
#[doc = "GPT8 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE23_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE23_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE23_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE23_A {
        match self.bits {
            false => MSTPE23_A::_0,
            true => MSTPE23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE23_A::_1
    }
}
#[doc = "Field `MSTPE23` writer - GPT8 Module Stop"]
pub type MSTPE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE23_A, O>;
impl<'a, const O: u8> MSTPE23_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE23_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE23_A::_1)
    }
}
#[doc = "Field `MSTPE24` reader - GPT7 Module Stop"]
pub type MSTPE24_R = crate::BitReader<MSTPE24_A>;
#[doc = "GPT7 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE24_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE24_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE24_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE24_A {
        match self.bits {
            false => MSTPE24_A::_0,
            true => MSTPE24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE24_A::_1
    }
}
#[doc = "Field `MSTPE24` writer - GPT7 Module Stop"]
pub type MSTPE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE24_A, O>;
impl<'a, const O: u8> MSTPE24_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE24_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE24_A::_1)
    }
}
#[doc = "Field `MSTPE25` reader - GPT6 Module Stop"]
pub type MSTPE25_R = crate::BitReader<MSTPE25_A>;
#[doc = "GPT6 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE25_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE25_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE25_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE25_A {
        match self.bits {
            false => MSTPE25_A::_0,
            true => MSTPE25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE25_A::_1
    }
}
#[doc = "Field `MSTPE25` writer - GPT6 Module Stop"]
pub type MSTPE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE25_A, O>;
impl<'a, const O: u8> MSTPE25_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE25_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE25_A::_1)
    }
}
#[doc = "Field `MSTPE26` reader - GPT5 Module Stop"]
pub type MSTPE26_R = crate::BitReader<MSTPE26_A>;
#[doc = "GPT5 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE26_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE26_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE26_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE26_A {
        match self.bits {
            false => MSTPE26_A::_0,
            true => MSTPE26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE26_A::_1
    }
}
#[doc = "Field `MSTPE26` writer - GPT5 Module Stop"]
pub type MSTPE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE26_A, O>;
impl<'a, const O: u8> MSTPE26_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE26_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE26_A::_1)
    }
}
#[doc = "Field `MSTPE27` reader - GPT4 Module Stop"]
pub type MSTPE27_R = crate::BitReader<MSTPE27_A>;
#[doc = "GPT4 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE27_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE27_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE27_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE27_A {
        match self.bits {
            false => MSTPE27_A::_0,
            true => MSTPE27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE27_A::_1
    }
}
#[doc = "Field `MSTPE27` writer - GPT4 Module Stop"]
pub type MSTPE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE27_A, O>;
impl<'a, const O: u8> MSTPE27_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE27_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE27_A::_1)
    }
}
#[doc = "Field `MSTPE28` reader - GPT3 Module Stop"]
pub type MSTPE28_R = crate::BitReader<MSTPE28_A>;
#[doc = "GPT3 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE28_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE28_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE28_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE28_A {
        match self.bits {
            false => MSTPE28_A::_0,
            true => MSTPE28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE28_A::_1
    }
}
#[doc = "Field `MSTPE28` writer - GPT3 Module Stop"]
pub type MSTPE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE28_A, O>;
impl<'a, const O: u8> MSTPE28_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE28_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE28_A::_1)
    }
}
#[doc = "Field `MSTPE29` reader - GPT2 Module Stop"]
pub type MSTPE29_R = crate::BitReader<MSTPE29_A>;
#[doc = "GPT2 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE29_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE29_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE29_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE29_A {
        match self.bits {
            false => MSTPE29_A::_0,
            true => MSTPE29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE29_A::_1
    }
}
#[doc = "Field `MSTPE29` writer - GPT2 Module Stop"]
pub type MSTPE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE29_A, O>;
impl<'a, const O: u8> MSTPE29_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE29_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE29_A::_1)
    }
}
#[doc = "Field `MSTPE30` reader - GPT1 Module Stop"]
pub type MSTPE30_R = crate::BitReader<MSTPE30_A>;
#[doc = "GPT1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE30_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE30_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE30_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE30_A {
        match self.bits {
            false => MSTPE30_A::_0,
            true => MSTPE30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE30_A::_1
    }
}
#[doc = "Field `MSTPE30` writer - GPT1 Module Stop"]
pub type MSTPE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE30_A, O>;
impl<'a, const O: u8> MSTPE30_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE30_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE30_A::_1)
    }
}
#[doc = "Field `MSTPE31` reader - GPT0 Module Stop"]
pub type MSTPE31_R = crate::BitReader<MSTPE31_A>;
#[doc = "GPT0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE31_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE31_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE31_A {
        match self.bits {
            false => MSTPE31_A::_0,
            true => MSTPE31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE31_A::_1
    }
}
#[doc = "Field `MSTPE31` writer - GPT0 Module Stop"]
pub type MSTPE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE31_A, O>;
impl<'a, const O: u8> MSTPE31_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE31_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE31_A::_1)
    }
}
impl R {
    #[doc = "Bit 14 - Low Power Asynchronous General Purpose Timer 5 Module Stop"]
    #[inline(always)]
    pub fn mstpe14(&self) -> MSTPE14_R {
        MSTPE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Low Power Asynchronous General Purpose Timer 4 Module Stop"]
    #[inline(always)]
    pub fn mstpe15(&self) -> MSTPE15_R {
        MSTPE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - GPT9 Module Stop"]
    #[inline(always)]
    pub fn mstpe22(&self) -> MSTPE22_R {
        MSTPE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPT8 Module Stop"]
    #[inline(always)]
    pub fn mstpe23(&self) -> MSTPE23_R {
        MSTPE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPT7 Module Stop"]
    #[inline(always)]
    pub fn mstpe24(&self) -> MSTPE24_R {
        MSTPE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPT6 Module Stop"]
    #[inline(always)]
    pub fn mstpe25(&self) -> MSTPE25_R {
        MSTPE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPT5 Module Stop"]
    #[inline(always)]
    pub fn mstpe26(&self) -> MSTPE26_R {
        MSTPE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPT4 Module Stop"]
    #[inline(always)]
    pub fn mstpe27(&self) -> MSTPE27_R {
        MSTPE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPT3 Module Stop"]
    #[inline(always)]
    pub fn mstpe28(&self) -> MSTPE28_R {
        MSTPE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPT2 Module Stop"]
    #[inline(always)]
    pub fn mstpe29(&self) -> MSTPE29_R {
        MSTPE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPT1 Module Stop"]
    #[inline(always)]
    pub fn mstpe30(&self) -> MSTPE30_R {
        MSTPE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPT0 Module Stop"]
    #[inline(always)]
    pub fn mstpe31(&self) -> MSTPE31_R {
        MSTPE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Low Power Asynchronous General Purpose Timer 5 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe14(&mut self) -> MSTPE14_W<14> {
        MSTPE14_W::new(self)
    }
    #[doc = "Bit 15 - Low Power Asynchronous General Purpose Timer 4 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe15(&mut self) -> MSTPE15_W<15> {
        MSTPE15_W::new(self)
    }
    #[doc = "Bit 22 - GPT9 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe22(&mut self) -> MSTPE22_W<22> {
        MSTPE22_W::new(self)
    }
    #[doc = "Bit 23 - GPT8 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe23(&mut self) -> MSTPE23_W<23> {
        MSTPE23_W::new(self)
    }
    #[doc = "Bit 24 - GPT7 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe24(&mut self) -> MSTPE24_W<24> {
        MSTPE24_W::new(self)
    }
    #[doc = "Bit 25 - GPT6 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe25(&mut self) -> MSTPE25_W<25> {
        MSTPE25_W::new(self)
    }
    #[doc = "Bit 26 - GPT5 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe26(&mut self) -> MSTPE26_W<26> {
        MSTPE26_W::new(self)
    }
    #[doc = "Bit 27 - GPT4 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe27(&mut self) -> MSTPE27_W<27> {
        MSTPE27_W::new(self)
    }
    #[doc = "Bit 28 - GPT3 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe28(&mut self) -> MSTPE28_W<28> {
        MSTPE28_W::new(self)
    }
    #[doc = "Bit 29 - GPT2 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe29(&mut self) -> MSTPE29_W<29> {
        MSTPE29_W::new(self)
    }
    #[doc = "Bit 30 - GPT1 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe30(&mut self) -> MSTPE30_W<30> {
        MSTPE30_W::new(self)
    }
    #[doc = "Bit 31 - GPT0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe31(&mut self) -> MSTPE31_W<31> {
        MSTPE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcre](index.html) module"]
pub struct MSTPCRE_SPEC;
impl crate::RegisterSpec for MSTPCRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcre::R](R) reader structure"]
impl crate::Readable for MSTPCRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcre::W](W) writer structure"]
impl crate::Writable for MSTPCRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRE to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
