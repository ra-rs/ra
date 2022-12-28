#[doc = "Register `EOSR` reader"]
pub struct R(crate::R<EOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EOSR` writer"]
pub struct W(crate::W<EOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EOSR_SPEC>;
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
impl From<crate::W<EOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EOSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOSR00` reader - Pmn Event Output Set"]
pub type EOSR00_R = crate::BitReader<EOSR00_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR00_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR00_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR00_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR00_A {
        match self.bits {
            false => EOSR00_A::_0,
            true => EOSR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR00_A::_1
    }
}
#[doc = "Field `EOSR00` writer - Pmn Event Output Set"]
pub type EOSR00_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR00_A, O>;
impl<'a, const O: u8> EOSR00_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR00_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR00_A::_1)
    }
}
#[doc = "Field `EOSR01` reader - Pmn Event Output Set"]
pub type EOSR01_R = crate::BitReader<EOSR01_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR01_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR01_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR01_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR01_A {
        match self.bits {
            false => EOSR01_A::_0,
            true => EOSR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR01_A::_1
    }
}
#[doc = "Field `EOSR01` writer - Pmn Event Output Set"]
pub type EOSR01_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR01_A, O>;
impl<'a, const O: u8> EOSR01_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR01_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR01_A::_1)
    }
}
#[doc = "Field `EOSR02` reader - Pmn Event Output Set"]
pub type EOSR02_R = crate::BitReader<EOSR02_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR02_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR02_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR02_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR02_A {
        match self.bits {
            false => EOSR02_A::_0,
            true => EOSR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR02_A::_1
    }
}
#[doc = "Field `EOSR02` writer - Pmn Event Output Set"]
pub type EOSR02_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR02_A, O>;
impl<'a, const O: u8> EOSR02_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR02_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR02_A::_1)
    }
}
#[doc = "Field `EOSR03` reader - Pmn Event Output Set"]
pub type EOSR03_R = crate::BitReader<EOSR03_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR03_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR03_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR03_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR03_A {
        match self.bits {
            false => EOSR03_A::_0,
            true => EOSR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR03_A::_1
    }
}
#[doc = "Field `EOSR03` writer - Pmn Event Output Set"]
pub type EOSR03_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR03_A, O>;
impl<'a, const O: u8> EOSR03_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR03_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR03_A::_1)
    }
}
#[doc = "Field `EOSR04` reader - Pmn Event Output Set"]
pub type EOSR04_R = crate::BitReader<EOSR04_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR04_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR04_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR04_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR04_A {
        match self.bits {
            false => EOSR04_A::_0,
            true => EOSR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR04_A::_1
    }
}
#[doc = "Field `EOSR04` writer - Pmn Event Output Set"]
pub type EOSR04_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR04_A, O>;
impl<'a, const O: u8> EOSR04_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR04_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR04_A::_1)
    }
}
#[doc = "Field `EOSR05` reader - Pmn Event Output Set"]
pub type EOSR05_R = crate::BitReader<EOSR05_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR05_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR05_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR05_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR05_A {
        match self.bits {
            false => EOSR05_A::_0,
            true => EOSR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR05_A::_1
    }
}
#[doc = "Field `EOSR05` writer - Pmn Event Output Set"]
pub type EOSR05_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR05_A, O>;
impl<'a, const O: u8> EOSR05_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR05_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR05_A::_1)
    }
}
#[doc = "Field `EOSR06` reader - Pmn Event Output Set"]
pub type EOSR06_R = crate::BitReader<EOSR06_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR06_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR06_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR06_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR06_A {
        match self.bits {
            false => EOSR06_A::_0,
            true => EOSR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR06_A::_1
    }
}
#[doc = "Field `EOSR06` writer - Pmn Event Output Set"]
pub type EOSR06_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR06_A, O>;
impl<'a, const O: u8> EOSR06_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR06_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR06_A::_1)
    }
}
#[doc = "Field `EOSR07` reader - Pmn Event Output Set"]
pub type EOSR07_R = crate::BitReader<EOSR07_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR07_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR07_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR07_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR07_A {
        match self.bits {
            false => EOSR07_A::_0,
            true => EOSR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR07_A::_1
    }
}
#[doc = "Field `EOSR07` writer - Pmn Event Output Set"]
pub type EOSR07_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR07_A, O>;
impl<'a, const O: u8> EOSR07_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR07_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR07_A::_1)
    }
}
#[doc = "Field `EOSR08` reader - Pmn Event Output Set"]
pub type EOSR08_R = crate::BitReader<EOSR08_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR08_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR08_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR08_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR08_A {
        match self.bits {
            false => EOSR08_A::_0,
            true => EOSR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR08_A::_1
    }
}
#[doc = "Field `EOSR08` writer - Pmn Event Output Set"]
pub type EOSR08_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR08_A, O>;
impl<'a, const O: u8> EOSR08_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR08_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR08_A::_1)
    }
}
#[doc = "Field `EOSR09` reader - Pmn Event Output Set"]
pub type EOSR09_R = crate::BitReader<EOSR09_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR09_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR09_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR09_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR09_A {
        match self.bits {
            false => EOSR09_A::_0,
            true => EOSR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR09_A::_1
    }
}
#[doc = "Field `EOSR09` writer - Pmn Event Output Set"]
pub type EOSR09_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR09_A, O>;
impl<'a, const O: u8> EOSR09_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR09_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR09_A::_1)
    }
}
#[doc = "Field `EOSR10` reader - Pmn Event Output Set"]
pub type EOSR10_R = crate::BitReader<EOSR10_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR10_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR10_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR10_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR10_A {
        match self.bits {
            false => EOSR10_A::_0,
            true => EOSR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR10_A::_1
    }
}
#[doc = "Field `EOSR10` writer - Pmn Event Output Set"]
pub type EOSR10_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR10_A, O>;
impl<'a, const O: u8> EOSR10_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR10_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR10_A::_1)
    }
}
#[doc = "Field `EOSR11` reader - Pmn Event Output Set"]
pub type EOSR11_R = crate::BitReader<EOSR11_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR11_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR11_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR11_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR11_A {
        match self.bits {
            false => EOSR11_A::_0,
            true => EOSR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR11_A::_1
    }
}
#[doc = "Field `EOSR11` writer - Pmn Event Output Set"]
pub type EOSR11_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR11_A, O>;
impl<'a, const O: u8> EOSR11_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR11_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR11_A::_1)
    }
}
#[doc = "Field `EOSR12` reader - Pmn Event Output Set"]
pub type EOSR12_R = crate::BitReader<EOSR12_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR12_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR12_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR12_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR12_A {
        match self.bits {
            false => EOSR12_A::_0,
            true => EOSR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR12_A::_1
    }
}
#[doc = "Field `EOSR12` writer - Pmn Event Output Set"]
pub type EOSR12_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR12_A, O>;
impl<'a, const O: u8> EOSR12_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR12_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR12_A::_1)
    }
}
#[doc = "Field `EOSR13` reader - Pmn Event Output Set"]
pub type EOSR13_R = crate::BitReader<EOSR13_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR13_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR13_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR13_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR13_A {
        match self.bits {
            false => EOSR13_A::_0,
            true => EOSR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR13_A::_1
    }
}
#[doc = "Field `EOSR13` writer - Pmn Event Output Set"]
pub type EOSR13_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR13_A, O>;
impl<'a, const O: u8> EOSR13_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR13_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR13_A::_1)
    }
}
#[doc = "Field `EOSR14` reader - Pmn Event Output Set"]
pub type EOSR14_R = crate::BitReader<EOSR14_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR14_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR14_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR14_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR14_A {
        match self.bits {
            false => EOSR14_A::_0,
            true => EOSR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR14_A::_1
    }
}
#[doc = "Field `EOSR14` writer - Pmn Event Output Set"]
pub type EOSR14_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR14_A, O>;
impl<'a, const O: u8> EOSR14_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR14_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR14_A::_1)
    }
}
#[doc = "Field `EOSR15` reader - Pmn Event Output Set"]
pub type EOSR15_R = crate::BitReader<EOSR15_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR15_A {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<EOSR15_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR15_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR15_A {
        match self.bits {
            false => EOSR15_A::_0,
            true => EOSR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR15_A::_1
    }
}
#[doc = "Field `EOSR15` writer - Pmn Event Output Set"]
pub type EOSR15_W<'a, const O: u8> = crate::BitWriter<'a, u16, EOSR_SPEC, EOSR15_A, O>;
impl<'a, const O: u8> EOSR15_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR15_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr00(&self) -> EOSR00_R {
        EOSR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr01(&self) -> EOSR01_R {
        EOSR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr02(&self) -> EOSR02_R {
        EOSR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr03(&self) -> EOSR03_R {
        EOSR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr04(&self) -> EOSR04_R {
        EOSR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr05(&self) -> EOSR05_R {
        EOSR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr06(&self) -> EOSR06_R {
        EOSR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr07(&self) -> EOSR07_R {
        EOSR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr08(&self) -> EOSR08_R {
        EOSR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr09(&self) -> EOSR09_R {
        EOSR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr10(&self) -> EOSR10_R {
        EOSR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr11(&self) -> EOSR11_R {
        EOSR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr12(&self) -> EOSR12_R {
        EOSR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr13(&self) -> EOSR13_R {
        EOSR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr14(&self) -> EOSR14_R {
        EOSR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr15(&self) -> EOSR15_R {
        EOSR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr00(&mut self) -> EOSR00_W<0> {
        EOSR00_W::new(self)
    }
    #[doc = "Bit 1 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr01(&mut self) -> EOSR01_W<1> {
        EOSR01_W::new(self)
    }
    #[doc = "Bit 2 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr02(&mut self) -> EOSR02_W<2> {
        EOSR02_W::new(self)
    }
    #[doc = "Bit 3 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr03(&mut self) -> EOSR03_W<3> {
        EOSR03_W::new(self)
    }
    #[doc = "Bit 4 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr04(&mut self) -> EOSR04_W<4> {
        EOSR04_W::new(self)
    }
    #[doc = "Bit 5 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr05(&mut self) -> EOSR05_W<5> {
        EOSR05_W::new(self)
    }
    #[doc = "Bit 6 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr06(&mut self) -> EOSR06_W<6> {
        EOSR06_W::new(self)
    }
    #[doc = "Bit 7 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr07(&mut self) -> EOSR07_W<7> {
        EOSR07_W::new(self)
    }
    #[doc = "Bit 8 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr08(&mut self) -> EOSR08_W<8> {
        EOSR08_W::new(self)
    }
    #[doc = "Bit 9 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr09(&mut self) -> EOSR09_W<9> {
        EOSR09_W::new(self)
    }
    #[doc = "Bit 10 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr10(&mut self) -> EOSR10_W<10> {
        EOSR10_W::new(self)
    }
    #[doc = "Bit 11 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr11(&mut self) -> EOSR11_W<11> {
        EOSR11_W::new(self)
    }
    #[doc = "Bit 12 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr12(&mut self) -> EOSR12_W<12> {
        EOSR12_W::new(self)
    }
    #[doc = "Bit 13 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr13(&mut self) -> EOSR13_W<13> {
        EOSR13_W::new(self)
    }
    #[doc = "Bit 14 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr14(&mut self) -> EOSR14_W<14> {
        EOSR14_W::new(self)
    }
    #[doc = "Bit 15 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr15(&mut self) -> EOSR15_W<15> {
        EOSR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eosr](index.html) module"]
pub struct EOSR_SPEC;
impl crate::RegisterSpec for EOSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eosr::R](R) reader structure"]
impl crate::Readable for EOSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eosr::W](W) writer structure"]
impl crate::Writable for EOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EOSR to value 0"]
impl crate::Resettable for EOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
