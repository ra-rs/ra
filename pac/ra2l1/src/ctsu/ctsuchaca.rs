#[doc = "Register `CTSUCHACA` reader"]
pub struct R(crate::R<CTSUCHACA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHACA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHACA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHACA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHACA` writer"]
pub struct W(crate::W<CTSUCHACA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHACA_SPEC>;
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
impl From<crate::W<CTSUCHACA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHACA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHAC00` reader - CTSU Channel Enable Control A"]
pub type CHAC00_R = crate::BitReader<CHAC00_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC00_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC00_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC00_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC00_A {
        match self.bits {
            false => CHAC00_A::_0,
            true => CHAC00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC00_A::_1
    }
}
#[doc = "Field `CHAC00` writer - CTSU Channel Enable Control A"]
pub type CHAC00_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC00_A, O>;
impl<'a, const O: u8> CHAC00_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC00_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC00_A::_1)
    }
}
#[doc = "Field `CHAC02` reader - CTSU Channel Enable Control A"]
pub type CHAC02_R = crate::BitReader<CHAC02_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC02_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC02_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC02_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC02_A {
        match self.bits {
            false => CHAC02_A::_0,
            true => CHAC02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC02_A::_1
    }
}
#[doc = "Field `CHAC02` writer - CTSU Channel Enable Control A"]
pub type CHAC02_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC02_A, O>;
impl<'a, const O: u8> CHAC02_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC02_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC02_A::_1)
    }
}
#[doc = "Field `CHAC04` reader - CTSU Channel Enable Control A"]
pub type CHAC04_R = crate::BitReader<CHAC04_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC04_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC04_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC04_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC04_A {
        match self.bits {
            false => CHAC04_A::_0,
            true => CHAC04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC04_A::_1
    }
}
#[doc = "Field `CHAC04` writer - CTSU Channel Enable Control A"]
pub type CHAC04_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC04_A, O>;
impl<'a, const O: u8> CHAC04_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC04_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC04_A::_1)
    }
}
#[doc = "Field `CHAC05` reader - CTSU Channel Enable Control A"]
pub type CHAC05_R = crate::BitReader<CHAC05_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC05_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC05_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC05_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC05_A {
        match self.bits {
            false => CHAC05_A::_0,
            true => CHAC05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC05_A::_1
    }
}
#[doc = "Field `CHAC05` writer - CTSU Channel Enable Control A"]
pub type CHAC05_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC05_A, O>;
impl<'a, const O: u8> CHAC05_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC05_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC05_A::_1)
    }
}
#[doc = "Field `CHAC06` reader - CTSU Channel Enable Control A"]
pub type CHAC06_R = crate::BitReader<CHAC06_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC06_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC06_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC06_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC06_A {
        match self.bits {
            false => CHAC06_A::_0,
            true => CHAC06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC06_A::_1
    }
}
#[doc = "Field `CHAC06` writer - CTSU Channel Enable Control A"]
pub type CHAC06_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC06_A, O>;
impl<'a, const O: u8> CHAC06_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC06_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC06_A::_1)
    }
}
#[doc = "Field `CHAC07` reader - CTSU Channel Enable Control A"]
pub type CHAC07_R = crate::BitReader<CHAC07_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC07_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC07_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC07_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC07_A {
        match self.bits {
            false => CHAC07_A::_0,
            true => CHAC07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC07_A::_1
    }
}
#[doc = "Field `CHAC07` writer - CTSU Channel Enable Control A"]
pub type CHAC07_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC07_A, O>;
impl<'a, const O: u8> CHAC07_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC07_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC07_A::_1)
    }
}
#[doc = "Field `CHAC08` reader - CTSU Channel Enable Control A"]
pub type CHAC08_R = crate::BitReader<CHAC08_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC08_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC08_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC08_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC08_A {
        match self.bits {
            false => CHAC08_A::_0,
            true => CHAC08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC08_A::_1
    }
}
#[doc = "Field `CHAC08` writer - CTSU Channel Enable Control A"]
pub type CHAC08_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC08_A, O>;
impl<'a, const O: u8> CHAC08_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC08_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC08_A::_1)
    }
}
#[doc = "Field `CHAC09` reader - CTSU Channel Enable Control A"]
pub type CHAC09_R = crate::BitReader<CHAC09_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC09_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC09_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC09_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC09_A {
        match self.bits {
            false => CHAC09_A::_0,
            true => CHAC09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC09_A::_1
    }
}
#[doc = "Field `CHAC09` writer - CTSU Channel Enable Control A"]
pub type CHAC09_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC09_A, O>;
impl<'a, const O: u8> CHAC09_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC09_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC09_A::_1)
    }
}
#[doc = "Field `CHAC10` reader - CTSU Channel Enable Control A"]
pub type CHAC10_R = crate::BitReader<CHAC10_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC10_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC10_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC10_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC10_A {
        match self.bits {
            false => CHAC10_A::_0,
            true => CHAC10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC10_A::_1
    }
}
#[doc = "Field `CHAC10` writer - CTSU Channel Enable Control A"]
pub type CHAC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC10_A, O>;
impl<'a, const O: u8> CHAC10_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC10_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC10_A::_1)
    }
}
#[doc = "Field `CHAC11` reader - CTSU Channel Enable Control A"]
pub type CHAC11_R = crate::BitReader<CHAC11_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC11_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC11_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC11_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC11_A {
        match self.bits {
            false => CHAC11_A::_0,
            true => CHAC11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC11_A::_1
    }
}
#[doc = "Field `CHAC11` writer - CTSU Channel Enable Control A"]
pub type CHAC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC11_A, O>;
impl<'a, const O: u8> CHAC11_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC11_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC11_A::_1)
    }
}
#[doc = "Field `CHAC12` reader - CTSU Channel Enable Control A"]
pub type CHAC12_R = crate::BitReader<CHAC12_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC12_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC12_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC12_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC12_A {
        match self.bits {
            false => CHAC12_A::_0,
            true => CHAC12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC12_A::_1
    }
}
#[doc = "Field `CHAC12` writer - CTSU Channel Enable Control A"]
pub type CHAC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC12_A, O>;
impl<'a, const O: u8> CHAC12_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC12_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC12_A::_1)
    }
}
#[doc = "Field `CHAC13` reader - CTSU Channel Enable Control A"]
pub type CHAC13_R = crate::BitReader<CHAC13_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC13_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC13_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC13_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC13_A {
        match self.bits {
            false => CHAC13_A::_0,
            true => CHAC13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC13_A::_1
    }
}
#[doc = "Field `CHAC13` writer - CTSU Channel Enable Control A"]
pub type CHAC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC13_A, O>;
impl<'a, const O: u8> CHAC13_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC13_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC13_A::_1)
    }
}
#[doc = "Field `CHAC14` reader - CTSU Channel Enable Control A"]
pub type CHAC14_R = crate::BitReader<CHAC14_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC14_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC14_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC14_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC14_A {
        match self.bits {
            false => CHAC14_A::_0,
            true => CHAC14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC14_A::_1
    }
}
#[doc = "Field `CHAC14` writer - CTSU Channel Enable Control A"]
pub type CHAC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC14_A, O>;
impl<'a, const O: u8> CHAC14_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC14_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC14_A::_1)
    }
}
#[doc = "Field `CHAC15` reader - CTSU Channel Enable Control A"]
pub type CHAC15_R = crate::BitReader<CHAC15_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC15_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC15_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC15_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC15_A {
        match self.bits {
            false => CHAC15_A::_0,
            true => CHAC15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC15_A::_1
    }
}
#[doc = "Field `CHAC15` writer - CTSU Channel Enable Control A"]
pub type CHAC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC15_A, O>;
impl<'a, const O: u8> CHAC15_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC15_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC15_A::_1)
    }
}
#[doc = "Field `CHAC16` reader - CTSU Channel Enable Control A"]
pub type CHAC16_R = crate::BitReader<CHAC16_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC16_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC16_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC16_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC16_A {
        match self.bits {
            false => CHAC16_A::_0,
            true => CHAC16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC16_A::_1
    }
}
#[doc = "Field `CHAC16` writer - CTSU Channel Enable Control A"]
pub type CHAC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC16_A, O>;
impl<'a, const O: u8> CHAC16_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC16_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC16_A::_1)
    }
}
#[doc = "Field `CHAC17` reader - CTSU Channel Enable Control A"]
pub type CHAC17_R = crate::BitReader<CHAC17_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC17_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC17_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC17_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC17_A {
        match self.bits {
            false => CHAC17_A::_0,
            true => CHAC17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC17_A::_1
    }
}
#[doc = "Field `CHAC17` writer - CTSU Channel Enable Control A"]
pub type CHAC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC17_A, O>;
impl<'a, const O: u8> CHAC17_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC17_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC17_A::_1)
    }
}
#[doc = "Field `CHAC18` reader - CTSU Channel Enable Control A"]
pub type CHAC18_R = crate::BitReader<CHAC18_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC18_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC18_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC18_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC18_A {
        match self.bits {
            false => CHAC18_A::_0,
            true => CHAC18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC18_A::_1
    }
}
#[doc = "Field `CHAC18` writer - CTSU Channel Enable Control A"]
pub type CHAC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC18_A, O>;
impl<'a, const O: u8> CHAC18_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC18_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC18_A::_1)
    }
}
#[doc = "Field `CHAC21` reader - CTSU Channel Enable Control A"]
pub type CHAC21_R = crate::BitReader<CHAC21_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC21_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC21_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC21_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC21_A {
        match self.bits {
            false => CHAC21_A::_0,
            true => CHAC21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC21_A::_1
    }
}
#[doc = "Field `CHAC21` writer - CTSU Channel Enable Control A"]
pub type CHAC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC21_A, O>;
impl<'a, const O: u8> CHAC21_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC21_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC21_A::_1)
    }
}
#[doc = "Field `CHAC22` reader - CTSU Channel Enable Control A"]
pub type CHAC22_R = crate::BitReader<CHAC22_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC22_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC22_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC22_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC22_A {
        match self.bits {
            false => CHAC22_A::_0,
            true => CHAC22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC22_A::_1
    }
}
#[doc = "Field `CHAC22` writer - CTSU Channel Enable Control A"]
pub type CHAC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC22_A, O>;
impl<'a, const O: u8> CHAC22_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC22_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC22_A::_1)
    }
}
#[doc = "Field `CHAC23` reader - CTSU Channel Enable Control A"]
pub type CHAC23_R = crate::BitReader<CHAC23_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC23_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC23_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC23_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC23_A {
        match self.bits {
            false => CHAC23_A::_0,
            true => CHAC23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC23_A::_1
    }
}
#[doc = "Field `CHAC23` writer - CTSU Channel Enable Control A"]
pub type CHAC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC23_A, O>;
impl<'a, const O: u8> CHAC23_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC23_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC23_A::_1)
    }
}
#[doc = "Field `CHAC24` reader - CTSU Channel Enable Control A"]
pub type CHAC24_R = crate::BitReader<CHAC24_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC24_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC24_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC24_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC24_A {
        match self.bits {
            false => CHAC24_A::_0,
            true => CHAC24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC24_A::_1
    }
}
#[doc = "Field `CHAC24` writer - CTSU Channel Enable Control A"]
pub type CHAC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC24_A, O>;
impl<'a, const O: u8> CHAC24_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC24_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC24_A::_1)
    }
}
#[doc = "Field `CHAC25` reader - CTSU Channel Enable Control A"]
pub type CHAC25_R = crate::BitReader<CHAC25_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC25_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC25_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC25_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC25_A {
        match self.bits {
            false => CHAC25_A::_0,
            true => CHAC25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC25_A::_1
    }
}
#[doc = "Field `CHAC25` writer - CTSU Channel Enable Control A"]
pub type CHAC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC25_A, O>;
impl<'a, const O: u8> CHAC25_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC25_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC25_A::_1)
    }
}
#[doc = "Field `CHAC26` reader - CTSU Channel Enable Control A"]
pub type CHAC26_R = crate::BitReader<CHAC26_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC26_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC26_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC26_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC26_A {
        match self.bits {
            false => CHAC26_A::_0,
            true => CHAC26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC26_A::_1
    }
}
#[doc = "Field `CHAC26` writer - CTSU Channel Enable Control A"]
pub type CHAC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC26_A, O>;
impl<'a, const O: u8> CHAC26_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC26_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC26_A::_1)
    }
}
#[doc = "Field `CHAC27` reader - CTSU Channel Enable Control A"]
pub type CHAC27_R = crate::BitReader<CHAC27_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC27_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC27_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC27_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC27_A {
        match self.bits {
            false => CHAC27_A::_0,
            true => CHAC27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC27_A::_1
    }
}
#[doc = "Field `CHAC27` writer - CTSU Channel Enable Control A"]
pub type CHAC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC27_A, O>;
impl<'a, const O: u8> CHAC27_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC27_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC27_A::_1)
    }
}
#[doc = "Field `CHAC28` reader - CTSU Channel Enable Control A"]
pub type CHAC28_R = crate::BitReader<CHAC28_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC28_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC28_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC28_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC28_A {
        match self.bits {
            false => CHAC28_A::_0,
            true => CHAC28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC28_A::_1
    }
}
#[doc = "Field `CHAC28` writer - CTSU Channel Enable Control A"]
pub type CHAC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC28_A, O>;
impl<'a, const O: u8> CHAC28_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC28_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC28_A::_1)
    }
}
#[doc = "Field `CHAC29` reader - CTSU Channel Enable Control A"]
pub type CHAC29_R = crate::BitReader<CHAC29_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC29_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC29_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC29_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC29_A {
        match self.bits {
            false => CHAC29_A::_0,
            true => CHAC29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC29_A::_1
    }
}
#[doc = "Field `CHAC29` writer - CTSU Channel Enable Control A"]
pub type CHAC29_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC29_A, O>;
impl<'a, const O: u8> CHAC29_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC29_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC29_A::_1)
    }
}
#[doc = "Field `CHAC30` reader - CTSU Channel Enable Control A"]
pub type CHAC30_R = crate::BitReader<CHAC30_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC30_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC30_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC30_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC30_A {
        match self.bits {
            false => CHAC30_A::_0,
            true => CHAC30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC30_A::_1
    }
}
#[doc = "Field `CHAC30` writer - CTSU Channel Enable Control A"]
pub type CHAC30_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC30_A, O>;
impl<'a, const O: u8> CHAC30_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC30_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC30_A::_1)
    }
}
#[doc = "Field `CHAC31` reader - CTSU Channel Enable Control A"]
pub type CHAC31_R = crate::BitReader<CHAC31_A>;
#[doc = "CTSU Channel Enable Control A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC31_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC31_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC31_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC31_A {
        match self.bits {
            false => CHAC31_A::_0,
            true => CHAC31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC31_A::_1
    }
}
#[doc = "Field `CHAC31` writer - CTSU Channel Enable Control A"]
pub type CHAC31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACA_SPEC, CHAC31_A, O>;
impl<'a, const O: u8> CHAC31_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC31_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac00(&self) -> CHAC00_R {
        CHAC00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac02(&self) -> CHAC02_R {
        CHAC02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac04(&self) -> CHAC04_R {
        CHAC04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac05(&self) -> CHAC05_R {
        CHAC05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac06(&self) -> CHAC06_R {
        CHAC06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac07(&self) -> CHAC07_R {
        CHAC07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac08(&self) -> CHAC08_R {
        CHAC08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac09(&self) -> CHAC09_R {
        CHAC09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac10(&self) -> CHAC10_R {
        CHAC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac11(&self) -> CHAC11_R {
        CHAC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac12(&self) -> CHAC12_R {
        CHAC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac13(&self) -> CHAC13_R {
        CHAC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac14(&self) -> CHAC14_R {
        CHAC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac15(&self) -> CHAC15_R {
        CHAC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac16(&self) -> CHAC16_R {
        CHAC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac17(&self) -> CHAC17_R {
        CHAC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac18(&self) -> CHAC18_R {
        CHAC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac21(&self) -> CHAC21_R {
        CHAC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac22(&self) -> CHAC22_R {
        CHAC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac23(&self) -> CHAC23_R {
        CHAC23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac24(&self) -> CHAC24_R {
        CHAC24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac25(&self) -> CHAC25_R {
        CHAC25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac26(&self) -> CHAC26_R {
        CHAC26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac27(&self) -> CHAC27_R {
        CHAC27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac28(&self) -> CHAC28_R {
        CHAC28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac29(&self) -> CHAC29_R {
        CHAC29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac30(&self) -> CHAC30_R {
        CHAC30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac31(&self) -> CHAC31_R {
        CHAC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac00(&mut self) -> CHAC00_W<0> {
        CHAC00_W::new(self)
    }
    #[doc = "Bit 2 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac02(&mut self) -> CHAC02_W<2> {
        CHAC02_W::new(self)
    }
    #[doc = "Bit 4 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac04(&mut self) -> CHAC04_W<4> {
        CHAC04_W::new(self)
    }
    #[doc = "Bit 5 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac05(&mut self) -> CHAC05_W<5> {
        CHAC05_W::new(self)
    }
    #[doc = "Bit 6 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac06(&mut self) -> CHAC06_W<6> {
        CHAC06_W::new(self)
    }
    #[doc = "Bit 7 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac07(&mut self) -> CHAC07_W<7> {
        CHAC07_W::new(self)
    }
    #[doc = "Bit 8 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac08(&mut self) -> CHAC08_W<8> {
        CHAC08_W::new(self)
    }
    #[doc = "Bit 9 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac09(&mut self) -> CHAC09_W<9> {
        CHAC09_W::new(self)
    }
    #[doc = "Bit 10 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac10(&mut self) -> CHAC10_W<10> {
        CHAC10_W::new(self)
    }
    #[doc = "Bit 11 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac11(&mut self) -> CHAC11_W<11> {
        CHAC11_W::new(self)
    }
    #[doc = "Bit 12 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac12(&mut self) -> CHAC12_W<12> {
        CHAC12_W::new(self)
    }
    #[doc = "Bit 13 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac13(&mut self) -> CHAC13_W<13> {
        CHAC13_W::new(self)
    }
    #[doc = "Bit 14 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac14(&mut self) -> CHAC14_W<14> {
        CHAC14_W::new(self)
    }
    #[doc = "Bit 15 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac15(&mut self) -> CHAC15_W<15> {
        CHAC15_W::new(self)
    }
    #[doc = "Bit 16 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac16(&mut self) -> CHAC16_W<16> {
        CHAC16_W::new(self)
    }
    #[doc = "Bit 17 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac17(&mut self) -> CHAC17_W<17> {
        CHAC17_W::new(self)
    }
    #[doc = "Bit 18 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac18(&mut self) -> CHAC18_W<18> {
        CHAC18_W::new(self)
    }
    #[doc = "Bit 21 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac21(&mut self) -> CHAC21_W<21> {
        CHAC21_W::new(self)
    }
    #[doc = "Bit 22 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac22(&mut self) -> CHAC22_W<22> {
        CHAC22_W::new(self)
    }
    #[doc = "Bit 23 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac23(&mut self) -> CHAC23_W<23> {
        CHAC23_W::new(self)
    }
    #[doc = "Bit 24 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac24(&mut self) -> CHAC24_W<24> {
        CHAC24_W::new(self)
    }
    #[doc = "Bit 25 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac25(&mut self) -> CHAC25_W<25> {
        CHAC25_W::new(self)
    }
    #[doc = "Bit 26 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac26(&mut self) -> CHAC26_W<26> {
        CHAC26_W::new(self)
    }
    #[doc = "Bit 27 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac27(&mut self) -> CHAC27_W<27> {
        CHAC27_W::new(self)
    }
    #[doc = "Bit 28 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac28(&mut self) -> CHAC28_W<28> {
        CHAC28_W::new(self)
    }
    #[doc = "Bit 29 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac29(&mut self) -> CHAC29_W<29> {
        CHAC29_W::new(self)
    }
    #[doc = "Bit 30 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac30(&mut self) -> CHAC30_W<30> {
        CHAC30_W::new(self)
    }
    #[doc = "Bit 31 - CTSU Channel Enable Control A"]
    #[inline(always)]
    #[must_use]
    pub fn chac31(&mut self) -> CHAC31_W<31> {
        CHAC31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Enable Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchaca](index.html) module"]
pub struct CTSUCHACA_SPEC;
impl crate::RegisterSpec for CTSUCHACA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsuchaca::R](R) reader structure"]
impl crate::Readable for CTSUCHACA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchaca::W](W) writer structure"]
impl crate::Writable for CTSUCHACA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHACA to value 0"]
impl crate::Resettable for CTSUCHACA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
