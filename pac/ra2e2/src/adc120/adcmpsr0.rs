#[doc = "Register `ADCMPSR0` reader"]
pub struct R(crate::R<ADCMPSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPSR0` writer"]
pub struct W(crate::W<ADCMPSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPSR0_SPEC>;
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
impl From<crate::W<ADCMPSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTCHA00` reader - Compare Window A Flag"]
pub type CMPSTCHA00_R = crate::BitReader<CMPSTCHA00_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA00_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA00_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA00_A {
        match self.bits {
            false => CMPSTCHA00_A::_0,
            true => CMPSTCHA00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA00_A::_1
    }
}
#[doc = "Field `CMPSTCHA00` writer - Compare Window A Flag"]
pub type CMPSTCHA00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA00_A, O>;
impl<'a, const O: u8> CMPSTCHA00_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA00_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA00_A::_1)
    }
}
#[doc = "Field `CMPSTCHA01` reader - Compare Window A Flag"]
pub type CMPSTCHA01_R = crate::BitReader<CMPSTCHA01_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA01_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA01_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA01_A {
        match self.bits {
            false => CMPSTCHA01_A::_0,
            true => CMPSTCHA01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA01_A::_1
    }
}
#[doc = "Field `CMPSTCHA01` writer - Compare Window A Flag"]
pub type CMPSTCHA01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA01_A, O>;
impl<'a, const O: u8> CMPSTCHA01_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA01_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA01_A::_1)
    }
}
#[doc = "Field `CMPSTCHA02` reader - Compare Window A Flag"]
pub type CMPSTCHA02_R = crate::BitReader<CMPSTCHA02_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA02_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA02_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA02_A {
        match self.bits {
            false => CMPSTCHA02_A::_0,
            true => CMPSTCHA02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA02_A::_1
    }
}
#[doc = "Field `CMPSTCHA02` writer - Compare Window A Flag"]
pub type CMPSTCHA02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA02_A, O>;
impl<'a, const O: u8> CMPSTCHA02_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA02_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA02_A::_1)
    }
}
#[doc = "Field `CMPSTCHA03` reader - Compare Window A Flag"]
pub type CMPSTCHA03_R = crate::BitReader<CMPSTCHA03_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA03_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA03_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA03_A {
        match self.bits {
            false => CMPSTCHA03_A::_0,
            true => CMPSTCHA03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA03_A::_1
    }
}
#[doc = "Field `CMPSTCHA03` writer - Compare Window A Flag"]
pub type CMPSTCHA03_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA03_A, O>;
impl<'a, const O: u8> CMPSTCHA03_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA03_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA03_A::_1)
    }
}
#[doc = "Field `CMPSTCHA04` reader - Compare Window A Flag"]
pub type CMPSTCHA04_R = crate::BitReader<CMPSTCHA04_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA04_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA04_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA04_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA04_A {
        match self.bits {
            false => CMPSTCHA04_A::_0,
            true => CMPSTCHA04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA04_A::_1
    }
}
#[doc = "Field `CMPSTCHA04` writer - Compare Window A Flag"]
pub type CMPSTCHA04_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA04_A, O>;
impl<'a, const O: u8> CMPSTCHA04_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA04_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA04_A::_1)
    }
}
#[doc = "Field `CMPSTCHA05` reader - Compare Window A Flag"]
pub type CMPSTCHA05_R = crate::BitReader<CMPSTCHA05_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA05_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA05_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA05_A {
        match self.bits {
            false => CMPSTCHA05_A::_0,
            true => CMPSTCHA05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA05_A::_1
    }
}
#[doc = "Field `CMPSTCHA05` writer - Compare Window A Flag"]
pub type CMPSTCHA05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA05_A, O>;
impl<'a, const O: u8> CMPSTCHA05_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA05_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA05_A::_1)
    }
}
#[doc = "Field `CMPSTCHA06` reader - Compare Window A Flag"]
pub type CMPSTCHA06_R = crate::BitReader<CMPSTCHA06_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA06_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA06_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA06_A {
        match self.bits {
            false => CMPSTCHA06_A::_0,
            true => CMPSTCHA06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA06_A::_1
    }
}
#[doc = "Field `CMPSTCHA06` writer - Compare Window A Flag"]
pub type CMPSTCHA06_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA06_A, O>;
impl<'a, const O: u8> CMPSTCHA06_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA06_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA06_A::_1)
    }
}
#[doc = "Field `CMPSTCHA07` reader - Compare Window A Flag"]
pub type CMPSTCHA07_R = crate::BitReader<CMPSTCHA07_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA07_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA07_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA07_A {
        match self.bits {
            false => CMPSTCHA07_A::_0,
            true => CMPSTCHA07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA07_A::_1
    }
}
#[doc = "Field `CMPSTCHA07` writer - Compare Window A Flag"]
pub type CMPSTCHA07_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA07_A, O>;
impl<'a, const O: u8> CMPSTCHA07_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA07_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA07_A::_1)
    }
}
#[doc = "Field `CMPSTCHA08` reader - Compare Window A Flag"]
pub type CMPSTCHA08_R = crate::BitReader<CMPSTCHA08_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA08_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA08_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA08_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA08_A {
        match self.bits {
            false => CMPSTCHA08_A::_0,
            true => CMPSTCHA08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA08_A::_1
    }
}
#[doc = "Field `CMPSTCHA08` writer - Compare Window A Flag"]
pub type CMPSTCHA08_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA08_A, O>;
impl<'a, const O: u8> CMPSTCHA08_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA08_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA08_A::_1)
    }
}
#[doc = "Field `CMPSTCHA09` reader - Compare Window A Flag"]
pub type CMPSTCHA09_R = crate::BitReader<CMPSTCHA09_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA09_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA09_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA09_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA09_A {
        match self.bits {
            false => CMPSTCHA09_A::_0,
            true => CMPSTCHA09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA09_A::_1
    }
}
#[doc = "Field `CMPSTCHA09` writer - Compare Window A Flag"]
pub type CMPSTCHA09_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA09_A, O>;
impl<'a, const O: u8> CMPSTCHA09_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA09_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA09_A::_1)
    }
}
#[doc = "Field `CMPSTCHA10` reader - Compare Window A Flag"]
pub type CMPSTCHA10_R = crate::BitReader<CMPSTCHA10_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA10_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA10_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA10_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA10_A {
        match self.bits {
            false => CMPSTCHA10_A::_0,
            true => CMPSTCHA10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA10_A::_1
    }
}
#[doc = "Field `CMPSTCHA10` writer - Compare Window A Flag"]
pub type CMPSTCHA10_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA10_A, O>;
impl<'a, const O: u8> CMPSTCHA10_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA10_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA10_A::_1)
    }
}
#[doc = "Field `CMPSTCHA11` reader - Compare Window A Flag"]
pub type CMPSTCHA11_R = crate::BitReader<CMPSTCHA11_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA11_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA11_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA11_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA11_A {
        match self.bits {
            false => CMPSTCHA11_A::_0,
            true => CMPSTCHA11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA11_A::_1
    }
}
#[doc = "Field `CMPSTCHA11` writer - Compare Window A Flag"]
pub type CMPSTCHA11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA11_A, O>;
impl<'a, const O: u8> CMPSTCHA11_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA11_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA11_A::_1)
    }
}
#[doc = "Field `CMPSTCHA12` reader - Compare Window A Flag"]
pub type CMPSTCHA12_R = crate::BitReader<CMPSTCHA12_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA12_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA12_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA12_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA12_A {
        match self.bits {
            false => CMPSTCHA12_A::_0,
            true => CMPSTCHA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA12_A::_1
    }
}
#[doc = "Field `CMPSTCHA12` writer - Compare Window A Flag"]
pub type CMPSTCHA12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA12_A, O>;
impl<'a, const O: u8> CMPSTCHA12_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA12_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA12_A::_1)
    }
}
#[doc = "Field `CMPSTCHA13` reader - Compare Window A Flag"]
pub type CMPSTCHA13_R = crate::BitReader<CMPSTCHA13_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA13_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA13_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA13_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA13_A {
        match self.bits {
            false => CMPSTCHA13_A::_0,
            true => CMPSTCHA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA13_A::_1
    }
}
#[doc = "Field `CMPSTCHA13` writer - Compare Window A Flag"]
pub type CMPSTCHA13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA13_A, O>;
impl<'a, const O: u8> CMPSTCHA13_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA13_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA13_A::_1)
    }
}
#[doc = "Field `CMPSTCHA14` reader - Compare Window A Flag"]
pub type CMPSTCHA14_R = crate::BitReader<CMPSTCHA14_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA14_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA14_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA14_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA14_A {
        match self.bits {
            false => CMPSTCHA14_A::_0,
            true => CMPSTCHA14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA14_A::_1
    }
}
#[doc = "Field `CMPSTCHA14` writer - Compare Window A Flag"]
pub type CMPSTCHA14_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA14_A, O>;
impl<'a, const O: u8> CMPSTCHA14_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA14_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA14_A::_1)
    }
}
#[doc = "Field `CMPSTCHA15` reader - Compare Window A Flag"]
pub type CMPSTCHA15_R = crate::BitReader<CMPSTCHA15_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA15_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA15_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA15_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA15_A {
        match self.bits {
            false => CMPSTCHA15_A::_0,
            true => CMPSTCHA15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA15_A::_1
    }
}
#[doc = "Field `CMPSTCHA15` writer - Compare Window A Flag"]
pub type CMPSTCHA15_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPSR0_SPEC, CMPSTCHA15_A, O>;
impl<'a, const O: u8> CMPSTCHA15_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA15_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha00(&self) -> CMPSTCHA00_R {
        CMPSTCHA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha01(&self) -> CMPSTCHA01_R {
        CMPSTCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha02(&self) -> CMPSTCHA02_R {
        CMPSTCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha03(&self) -> CMPSTCHA03_R {
        CMPSTCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha04(&self) -> CMPSTCHA04_R {
        CMPSTCHA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha05(&self) -> CMPSTCHA05_R {
        CMPSTCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha06(&self) -> CMPSTCHA06_R {
        CMPSTCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha07(&self) -> CMPSTCHA07_R {
        CMPSTCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha08(&self) -> CMPSTCHA08_R {
        CMPSTCHA08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha09(&self) -> CMPSTCHA09_R {
        CMPSTCHA09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha10(&self) -> CMPSTCHA10_R {
        CMPSTCHA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha11(&self) -> CMPSTCHA11_R {
        CMPSTCHA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha12(&self) -> CMPSTCHA12_R {
        CMPSTCHA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha13(&self) -> CMPSTCHA13_R {
        CMPSTCHA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha14(&self) -> CMPSTCHA14_R {
        CMPSTCHA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha15(&self) -> CMPSTCHA15_R {
        CMPSTCHA15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha00(&mut self) -> CMPSTCHA00_W<0> {
        CMPSTCHA00_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha01(&mut self) -> CMPSTCHA01_W<1> {
        CMPSTCHA01_W::new(self)
    }
    #[doc = "Bit 2 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha02(&mut self) -> CMPSTCHA02_W<2> {
        CMPSTCHA02_W::new(self)
    }
    #[doc = "Bit 3 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha03(&mut self) -> CMPSTCHA03_W<3> {
        CMPSTCHA03_W::new(self)
    }
    #[doc = "Bit 4 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha04(&mut self) -> CMPSTCHA04_W<4> {
        CMPSTCHA04_W::new(self)
    }
    #[doc = "Bit 5 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha05(&mut self) -> CMPSTCHA05_W<5> {
        CMPSTCHA05_W::new(self)
    }
    #[doc = "Bit 6 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha06(&mut self) -> CMPSTCHA06_W<6> {
        CMPSTCHA06_W::new(self)
    }
    #[doc = "Bit 7 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha07(&mut self) -> CMPSTCHA07_W<7> {
        CMPSTCHA07_W::new(self)
    }
    #[doc = "Bit 8 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha08(&mut self) -> CMPSTCHA08_W<8> {
        CMPSTCHA08_W::new(self)
    }
    #[doc = "Bit 9 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha09(&mut self) -> CMPSTCHA09_W<9> {
        CMPSTCHA09_W::new(self)
    }
    #[doc = "Bit 10 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha10(&mut self) -> CMPSTCHA10_W<10> {
        CMPSTCHA10_W::new(self)
    }
    #[doc = "Bit 11 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha11(&mut self) -> CMPSTCHA11_W<11> {
        CMPSTCHA11_W::new(self)
    }
    #[doc = "Bit 12 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha12(&mut self) -> CMPSTCHA12_W<12> {
        CMPSTCHA12_W::new(self)
    }
    #[doc = "Bit 13 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha13(&mut self) -> CMPSTCHA13_W<13> {
        CMPSTCHA13_W::new(self)
    }
    #[doc = "Bit 14 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha14(&mut self) -> CMPSTCHA14_W<14> {
        CMPSTCHA14_W::new(self)
    }
    #[doc = "Bit 15 - Compare Window A Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha15(&mut self) -> CMPSTCHA15_W<15> {
        CMPSTCHA15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpsr0](index.html) module"]
pub struct ADCMPSR0_SPEC;
impl crate::RegisterSpec for ADCMPSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpsr0::R](R) reader structure"]
impl crate::Readable for ADCMPSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpsr0::W](W) writer structure"]
impl crate::Writable for ADCMPSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPSR0 to value 0"]
impl crate::Resettable for ADCMPSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
