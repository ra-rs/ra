#[doc = "Register `ICUSARG` reader"]
pub struct R(crate::R<ICUSARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARG` writer"]
pub struct W(crate::W<ICUSARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARG_SPEC>;
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
impl From<crate::W<ICUSARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAIELSR00` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR00_R = crate::BitReader<SAIELSR00_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR00_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR00_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR00_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR00_A {
        match self.bits {
            false => SAIELSR00_A::_0,
            true => SAIELSR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR00_A::_1
    }
}
#[doc = "Field `SAIELSR00` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR00_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR00_A, O>;
impl<'a, const O: u8> SAIELSR00_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR00_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR00_A::_1)
    }
}
#[doc = "Field `SAIELSR01` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR01_R = crate::BitReader<SAIELSR01_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR01_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR01_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR01_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR01_A {
        match self.bits {
            false => SAIELSR01_A::_0,
            true => SAIELSR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR01_A::_1
    }
}
#[doc = "Field `SAIELSR01` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR01_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR01_A, O>;
impl<'a, const O: u8> SAIELSR01_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR01_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR01_A::_1)
    }
}
#[doc = "Field `SAIELSR02` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR02_R = crate::BitReader<SAIELSR02_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR02_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR02_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR02_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR02_A {
        match self.bits {
            false => SAIELSR02_A::_0,
            true => SAIELSR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR02_A::_1
    }
}
#[doc = "Field `SAIELSR02` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR02_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR02_A, O>;
impl<'a, const O: u8> SAIELSR02_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR02_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR02_A::_1)
    }
}
#[doc = "Field `SAIELSR03` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR03_R = crate::BitReader<SAIELSR03_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR03_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR03_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR03_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR03_A {
        match self.bits {
            false => SAIELSR03_A::_0,
            true => SAIELSR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR03_A::_1
    }
}
#[doc = "Field `SAIELSR03` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR03_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR03_A, O>;
impl<'a, const O: u8> SAIELSR03_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR03_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR03_A::_1)
    }
}
#[doc = "Field `SAIELSR04` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR04_R = crate::BitReader<SAIELSR04_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR04_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR04_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR04_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR04_A {
        match self.bits {
            false => SAIELSR04_A::_0,
            true => SAIELSR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR04_A::_1
    }
}
#[doc = "Field `SAIELSR04` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR04_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR04_A, O>;
impl<'a, const O: u8> SAIELSR04_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR04_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR04_A::_1)
    }
}
#[doc = "Field `SAIELSR05` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR05_R = crate::BitReader<SAIELSR05_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR05_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR05_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR05_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR05_A {
        match self.bits {
            false => SAIELSR05_A::_0,
            true => SAIELSR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR05_A::_1
    }
}
#[doc = "Field `SAIELSR05` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR05_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR05_A, O>;
impl<'a, const O: u8> SAIELSR05_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR05_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR05_A::_1)
    }
}
#[doc = "Field `SAIELSR06` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR06_R = crate::BitReader<SAIELSR06_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR06_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR06_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR06_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR06_A {
        match self.bits {
            false => SAIELSR06_A::_0,
            true => SAIELSR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR06_A::_1
    }
}
#[doc = "Field `SAIELSR06` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR06_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR06_A, O>;
impl<'a, const O: u8> SAIELSR06_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR06_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR06_A::_1)
    }
}
#[doc = "Field `SAIELSR07` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR07_R = crate::BitReader<SAIELSR07_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR07_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR07_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR07_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR07_A {
        match self.bits {
            false => SAIELSR07_A::_0,
            true => SAIELSR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR07_A::_1
    }
}
#[doc = "Field `SAIELSR07` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR07_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR07_A, O>;
impl<'a, const O: u8> SAIELSR07_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR07_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR07_A::_1)
    }
}
#[doc = "Field `SAIELSR08` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR08_R = crate::BitReader<SAIELSR08_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR08_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR08_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR08_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR08_A {
        match self.bits {
            false => SAIELSR08_A::_0,
            true => SAIELSR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR08_A::_1
    }
}
#[doc = "Field `SAIELSR08` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR08_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR08_A, O>;
impl<'a, const O: u8> SAIELSR08_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR08_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR08_A::_1)
    }
}
#[doc = "Field `SAIELSR09` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR09_R = crate::BitReader<SAIELSR09_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR09_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR09_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR09_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR09_A {
        match self.bits {
            false => SAIELSR09_A::_0,
            true => SAIELSR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR09_A::_1
    }
}
#[doc = "Field `SAIELSR09` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR09_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR09_A, O>;
impl<'a, const O: u8> SAIELSR09_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR09_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR09_A::_1)
    }
}
#[doc = "Field `SAIELSR10` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR10_R = crate::BitReader<SAIELSR10_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR10_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR10_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR10_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR10_A {
        match self.bits {
            false => SAIELSR10_A::_0,
            true => SAIELSR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR10_A::_1
    }
}
#[doc = "Field `SAIELSR10` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR10_A, O>;
impl<'a, const O: u8> SAIELSR10_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR10_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR10_A::_1)
    }
}
#[doc = "Field `SAIELSR11` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR11_R = crate::BitReader<SAIELSR11_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR11_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR11_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR11_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR11_A {
        match self.bits {
            false => SAIELSR11_A::_0,
            true => SAIELSR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR11_A::_1
    }
}
#[doc = "Field `SAIELSR11` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR11_A, O>;
impl<'a, const O: u8> SAIELSR11_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR11_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR11_A::_1)
    }
}
#[doc = "Field `SAIELSR12` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR12_R = crate::BitReader<SAIELSR12_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR12_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR12_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR12_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR12_A {
        match self.bits {
            false => SAIELSR12_A::_0,
            true => SAIELSR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR12_A::_1
    }
}
#[doc = "Field `SAIELSR12` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR12_A, O>;
impl<'a, const O: u8> SAIELSR12_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR12_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR12_A::_1)
    }
}
#[doc = "Field `SAIELSR13` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR13_R = crate::BitReader<SAIELSR13_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR13_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR13_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR13_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR13_A {
        match self.bits {
            false => SAIELSR13_A::_0,
            true => SAIELSR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR13_A::_1
    }
}
#[doc = "Field `SAIELSR13` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR13_A, O>;
impl<'a, const O: u8> SAIELSR13_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR13_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR13_A::_1)
    }
}
#[doc = "Field `SAIELSR14` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR14_R = crate::BitReader<SAIELSR14_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR14_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR14_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR14_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR14_A {
        match self.bits {
            false => SAIELSR14_A::_0,
            true => SAIELSR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR14_A::_1
    }
}
#[doc = "Field `SAIELSR14` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR14_A, O>;
impl<'a, const O: u8> SAIELSR14_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR14_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR14_A::_1)
    }
}
#[doc = "Field `SAIELSR15` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR15_R = crate::BitReader<SAIELSR15_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR15_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR15_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR15_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR15_A {
        match self.bits {
            false => SAIELSR15_A::_0,
            true => SAIELSR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR15_A::_1
    }
}
#[doc = "Field `SAIELSR15` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR15_A, O>;
impl<'a, const O: u8> SAIELSR15_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR15_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR15_A::_1)
    }
}
#[doc = "Field `SAIELSR16` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR16_R = crate::BitReader<SAIELSR16_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR16_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR16_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR16_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR16_A {
        match self.bits {
            false => SAIELSR16_A::_0,
            true => SAIELSR16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR16_A::_1
    }
}
#[doc = "Field `SAIELSR16` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR16_A, O>;
impl<'a, const O: u8> SAIELSR16_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR16_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR16_A::_1)
    }
}
#[doc = "Field `SAIELSR17` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR17_R = crate::BitReader<SAIELSR17_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR17_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR17_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR17_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR17_A {
        match self.bits {
            false => SAIELSR17_A::_0,
            true => SAIELSR17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR17_A::_1
    }
}
#[doc = "Field `SAIELSR17` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR17_A, O>;
impl<'a, const O: u8> SAIELSR17_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR17_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR17_A::_1)
    }
}
#[doc = "Field `SAIELSR18` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR18_R = crate::BitReader<SAIELSR18_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR18_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR18_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR18_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR18_A {
        match self.bits {
            false => SAIELSR18_A::_0,
            true => SAIELSR18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR18_A::_1
    }
}
#[doc = "Field `SAIELSR18` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR18_A, O>;
impl<'a, const O: u8> SAIELSR18_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR18_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR18_A::_1)
    }
}
#[doc = "Field `SAIELSR19` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR19_R = crate::BitReader<SAIELSR19_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR19_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR19_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR19_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR19_A {
        match self.bits {
            false => SAIELSR19_A::_0,
            true => SAIELSR19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR19_A::_1
    }
}
#[doc = "Field `SAIELSR19` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR19_A, O>;
impl<'a, const O: u8> SAIELSR19_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR19_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR19_A::_1)
    }
}
#[doc = "Field `SAIELSR20` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR20_R = crate::BitReader<SAIELSR20_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR20_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR20_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR20_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR20_A {
        match self.bits {
            false => SAIELSR20_A::_0,
            true => SAIELSR20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR20_A::_1
    }
}
#[doc = "Field `SAIELSR20` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR20_A, O>;
impl<'a, const O: u8> SAIELSR20_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR20_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR20_A::_1)
    }
}
#[doc = "Field `SAIELSR21` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR21_R = crate::BitReader<SAIELSR21_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR21_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR21_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR21_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR21_A {
        match self.bits {
            false => SAIELSR21_A::_0,
            true => SAIELSR21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR21_A::_1
    }
}
#[doc = "Field `SAIELSR21` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR21_A, O>;
impl<'a, const O: u8> SAIELSR21_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR21_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR21_A::_1)
    }
}
#[doc = "Field `SAIELSR22` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR22_R = crate::BitReader<SAIELSR22_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR22_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR22_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR22_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR22_A {
        match self.bits {
            false => SAIELSR22_A::_0,
            true => SAIELSR22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR22_A::_1
    }
}
#[doc = "Field `SAIELSR22` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR22_A, O>;
impl<'a, const O: u8> SAIELSR22_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR22_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR22_A::_1)
    }
}
#[doc = "Field `SAIELSR23` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR23_R = crate::BitReader<SAIELSR23_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR23_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR23_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR23_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR23_A {
        match self.bits {
            false => SAIELSR23_A::_0,
            true => SAIELSR23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR23_A::_1
    }
}
#[doc = "Field `SAIELSR23` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR23_A, O>;
impl<'a, const O: u8> SAIELSR23_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR23_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR23_A::_1)
    }
}
#[doc = "Field `SAIELSR24` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR24_R = crate::BitReader<SAIELSR24_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR24_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR24_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR24_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR24_A {
        match self.bits {
            false => SAIELSR24_A::_0,
            true => SAIELSR24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR24_A::_1
    }
}
#[doc = "Field `SAIELSR24` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR24_A, O>;
impl<'a, const O: u8> SAIELSR24_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR24_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR24_A::_1)
    }
}
#[doc = "Field `SAIELSR25` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR25_R = crate::BitReader<SAIELSR25_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR25_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR25_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR25_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR25_A {
        match self.bits {
            false => SAIELSR25_A::_0,
            true => SAIELSR25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR25_A::_1
    }
}
#[doc = "Field `SAIELSR25` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR25_A, O>;
impl<'a, const O: u8> SAIELSR25_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR25_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR25_A::_1)
    }
}
#[doc = "Field `SAIELSR26` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR26_R = crate::BitReader<SAIELSR26_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR26_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR26_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR26_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR26_A {
        match self.bits {
            false => SAIELSR26_A::_0,
            true => SAIELSR26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR26_A::_1
    }
}
#[doc = "Field `SAIELSR26` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR26_A, O>;
impl<'a, const O: u8> SAIELSR26_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR26_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR26_A::_1)
    }
}
#[doc = "Field `SAIELSR27` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR27_R = crate::BitReader<SAIELSR27_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR27_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR27_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR27_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR27_A {
        match self.bits {
            false => SAIELSR27_A::_0,
            true => SAIELSR27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR27_A::_1
    }
}
#[doc = "Field `SAIELSR27` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR27_A, O>;
impl<'a, const O: u8> SAIELSR27_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR27_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR27_A::_1)
    }
}
#[doc = "Field `SAIELSR28` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR28_R = crate::BitReader<SAIELSR28_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR28_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR28_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR28_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR28_A {
        match self.bits {
            false => SAIELSR28_A::_0,
            true => SAIELSR28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR28_A::_1
    }
}
#[doc = "Field `SAIELSR28` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR28_A, O>;
impl<'a, const O: u8> SAIELSR28_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR28_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR28_A::_1)
    }
}
#[doc = "Field `SAIELSR29` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR29_R = crate::BitReader<SAIELSR29_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR29_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR29_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR29_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR29_A {
        match self.bits {
            false => SAIELSR29_A::_0,
            true => SAIELSR29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR29_A::_1
    }
}
#[doc = "Field `SAIELSR29` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR29_A, O>;
impl<'a, const O: u8> SAIELSR29_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR29_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR29_A::_1)
    }
}
#[doc = "Field `SAIELSR30` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR30_R = crate::BitReader<SAIELSR30_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR30_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR30_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR30_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR30_A {
        match self.bits {
            false => SAIELSR30_A::_0,
            true => SAIELSR30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR30_A::_1
    }
}
#[doc = "Field `SAIELSR30` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR30_A, O>;
impl<'a, const O: u8> SAIELSR30_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR30_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR30_A::_1)
    }
}
#[doc = "Field `SAIELSR31` reader - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR31_R = crate::BitReader<SAIELSR31_A>;
#[doc = "Security attributes of registers for IELSR31 to IELSR0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR31_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR31_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR31_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR31_A {
        match self.bits {
            false => SAIELSR31_A::_0,
            true => SAIELSR31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR31_A::_1
    }
}
#[doc = "Field `SAIELSR31` writer - Security attributes of registers for IELSR31 to IELSR0"]
pub type SAIELSR31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARG_SPEC, SAIELSR31_A, O>;
impl<'a, const O: u8> SAIELSR31_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR31_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr00(&self) -> SAIELSR00_R {
        SAIELSR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr01(&self) -> SAIELSR01_R {
        SAIELSR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr02(&self) -> SAIELSR02_R {
        SAIELSR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr03(&self) -> SAIELSR03_R {
        SAIELSR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr04(&self) -> SAIELSR04_R {
        SAIELSR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr05(&self) -> SAIELSR05_R {
        SAIELSR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr06(&self) -> SAIELSR06_R {
        SAIELSR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr07(&self) -> SAIELSR07_R {
        SAIELSR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr08(&self) -> SAIELSR08_R {
        SAIELSR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr09(&self) -> SAIELSR09_R {
        SAIELSR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr10(&self) -> SAIELSR10_R {
        SAIELSR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr11(&self) -> SAIELSR11_R {
        SAIELSR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr12(&self) -> SAIELSR12_R {
        SAIELSR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr13(&self) -> SAIELSR13_R {
        SAIELSR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr14(&self) -> SAIELSR14_R {
        SAIELSR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr15(&self) -> SAIELSR15_R {
        SAIELSR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr16(&self) -> SAIELSR16_R {
        SAIELSR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr17(&self) -> SAIELSR17_R {
        SAIELSR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr18(&self) -> SAIELSR18_R {
        SAIELSR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr19(&self) -> SAIELSR19_R {
        SAIELSR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr20(&self) -> SAIELSR20_R {
        SAIELSR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr21(&self) -> SAIELSR21_R {
        SAIELSR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr22(&self) -> SAIELSR22_R {
        SAIELSR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr23(&self) -> SAIELSR23_R {
        SAIELSR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr24(&self) -> SAIELSR24_R {
        SAIELSR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr25(&self) -> SAIELSR25_R {
        SAIELSR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr26(&self) -> SAIELSR26_R {
        SAIELSR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr27(&self) -> SAIELSR27_R {
        SAIELSR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr28(&self) -> SAIELSR28_R {
        SAIELSR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr29(&self) -> SAIELSR29_R {
        SAIELSR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr30(&self) -> SAIELSR30_R {
        SAIELSR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    pub fn saielsr31(&self) -> SAIELSR31_R {
        SAIELSR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr00(&mut self) -> SAIELSR00_W<0> {
        SAIELSR00_W::new(self)
    }
    #[doc = "Bit 1 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr01(&mut self) -> SAIELSR01_W<1> {
        SAIELSR01_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr02(&mut self) -> SAIELSR02_W<2> {
        SAIELSR02_W::new(self)
    }
    #[doc = "Bit 3 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr03(&mut self) -> SAIELSR03_W<3> {
        SAIELSR03_W::new(self)
    }
    #[doc = "Bit 4 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr04(&mut self) -> SAIELSR04_W<4> {
        SAIELSR04_W::new(self)
    }
    #[doc = "Bit 5 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr05(&mut self) -> SAIELSR05_W<5> {
        SAIELSR05_W::new(self)
    }
    #[doc = "Bit 6 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr06(&mut self) -> SAIELSR06_W<6> {
        SAIELSR06_W::new(self)
    }
    #[doc = "Bit 7 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr07(&mut self) -> SAIELSR07_W<7> {
        SAIELSR07_W::new(self)
    }
    #[doc = "Bit 8 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr08(&mut self) -> SAIELSR08_W<8> {
        SAIELSR08_W::new(self)
    }
    #[doc = "Bit 9 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr09(&mut self) -> SAIELSR09_W<9> {
        SAIELSR09_W::new(self)
    }
    #[doc = "Bit 10 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr10(&mut self) -> SAIELSR10_W<10> {
        SAIELSR10_W::new(self)
    }
    #[doc = "Bit 11 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr11(&mut self) -> SAIELSR11_W<11> {
        SAIELSR11_W::new(self)
    }
    #[doc = "Bit 12 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr12(&mut self) -> SAIELSR12_W<12> {
        SAIELSR12_W::new(self)
    }
    #[doc = "Bit 13 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr13(&mut self) -> SAIELSR13_W<13> {
        SAIELSR13_W::new(self)
    }
    #[doc = "Bit 14 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr14(&mut self) -> SAIELSR14_W<14> {
        SAIELSR14_W::new(self)
    }
    #[doc = "Bit 15 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr15(&mut self) -> SAIELSR15_W<15> {
        SAIELSR15_W::new(self)
    }
    #[doc = "Bit 16 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr16(&mut self) -> SAIELSR16_W<16> {
        SAIELSR16_W::new(self)
    }
    #[doc = "Bit 17 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr17(&mut self) -> SAIELSR17_W<17> {
        SAIELSR17_W::new(self)
    }
    #[doc = "Bit 18 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr18(&mut self) -> SAIELSR18_W<18> {
        SAIELSR18_W::new(self)
    }
    #[doc = "Bit 19 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr19(&mut self) -> SAIELSR19_W<19> {
        SAIELSR19_W::new(self)
    }
    #[doc = "Bit 20 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr20(&mut self) -> SAIELSR20_W<20> {
        SAIELSR20_W::new(self)
    }
    #[doc = "Bit 21 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr21(&mut self) -> SAIELSR21_W<21> {
        SAIELSR21_W::new(self)
    }
    #[doc = "Bit 22 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr22(&mut self) -> SAIELSR22_W<22> {
        SAIELSR22_W::new(self)
    }
    #[doc = "Bit 23 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr23(&mut self) -> SAIELSR23_W<23> {
        SAIELSR23_W::new(self)
    }
    #[doc = "Bit 24 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr24(&mut self) -> SAIELSR24_W<24> {
        SAIELSR24_W::new(self)
    }
    #[doc = "Bit 25 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr25(&mut self) -> SAIELSR25_W<25> {
        SAIELSR25_W::new(self)
    }
    #[doc = "Bit 26 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr26(&mut self) -> SAIELSR26_W<26> {
        SAIELSR26_W::new(self)
    }
    #[doc = "Bit 27 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr27(&mut self) -> SAIELSR27_W<27> {
        SAIELSR27_W::new(self)
    }
    #[doc = "Bit 28 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr28(&mut self) -> SAIELSR28_W<28> {
        SAIELSR28_W::new(self)
    }
    #[doc = "Bit 29 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr29(&mut self) -> SAIELSR29_W<29> {
        SAIELSR29_W::new(self)
    }
    #[doc = "Bit 30 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr30(&mut self) -> SAIELSR30_W<30> {
        SAIELSR30_W::new(self)
    }
    #[doc = "Bit 31 - Security attributes of registers for IELSR31 to IELSR0"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr31(&mut self) -> SAIELSR31_W<31> {
        SAIELSR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register G\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusarg](index.html) module"]
pub struct ICUSARG_SPEC;
impl crate::RegisterSpec for ICUSARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusarg::R](R) reader structure"]
impl crate::Readable for ICUSARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusarg::W](W) writer structure"]
impl crate::Writable for ICUSARG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARG to value 0xffff_ffff"]
impl crate::Resettable for ICUSARG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
