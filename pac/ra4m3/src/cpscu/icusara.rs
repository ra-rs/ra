#[doc = "Register `ICUSARA` reader"]
pub struct R(crate::R<ICUSARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARA` writer"]
pub struct W(crate::W<ICUSARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARA_SPEC>;
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
impl From<crate::W<ICUSARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAIRQCR00` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR00_R = crate::BitReader<SAIRQCR00_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR00_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR00_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR00_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR00_A {
        match self.bits {
            false => SAIRQCR00_A::_0,
            true => SAIRQCR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR00_A::_1
    }
}
#[doc = "Field `SAIRQCR00` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR00_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR00_A, O>;
impl<'a, const O: u8> SAIRQCR00_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR00_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR00_A::_1)
    }
}
#[doc = "Field `SAIRQCR01` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR01_R = crate::BitReader<SAIRQCR01_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR01_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR01_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR01_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR01_A {
        match self.bits {
            false => SAIRQCR01_A::_0,
            true => SAIRQCR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR01_A::_1
    }
}
#[doc = "Field `SAIRQCR01` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR01_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR01_A, O>;
impl<'a, const O: u8> SAIRQCR01_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR01_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR01_A::_1)
    }
}
#[doc = "Field `SAIRQCR02` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR02_R = crate::BitReader<SAIRQCR02_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR02_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR02_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR02_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR02_A {
        match self.bits {
            false => SAIRQCR02_A::_0,
            true => SAIRQCR02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR02_A::_1
    }
}
#[doc = "Field `SAIRQCR02` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR02_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR02_A, O>;
impl<'a, const O: u8> SAIRQCR02_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR02_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR02_A::_1)
    }
}
#[doc = "Field `SAIRQCR03` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR03_R = crate::BitReader<SAIRQCR03_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR03_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR03_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR03_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR03_A {
        match self.bits {
            false => SAIRQCR03_A::_0,
            true => SAIRQCR03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR03_A::_1
    }
}
#[doc = "Field `SAIRQCR03` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR03_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR03_A, O>;
impl<'a, const O: u8> SAIRQCR03_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR03_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR03_A::_1)
    }
}
#[doc = "Field `SAIRQCR04` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR04_R = crate::BitReader<SAIRQCR04_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR04_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR04_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR04_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR04_A {
        match self.bits {
            false => SAIRQCR04_A::_0,
            true => SAIRQCR04_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR04_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR04_A::_1
    }
}
#[doc = "Field `SAIRQCR04` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR04_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR04_A, O>;
impl<'a, const O: u8> SAIRQCR04_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR04_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR04_A::_1)
    }
}
#[doc = "Field `SAIRQCR05` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR05_R = crate::BitReader<SAIRQCR05_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR05_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR05_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR05_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR05_A {
        match self.bits {
            false => SAIRQCR05_A::_0,
            true => SAIRQCR05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR05_A::_1
    }
}
#[doc = "Field `SAIRQCR05` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR05_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR05_A, O>;
impl<'a, const O: u8> SAIRQCR05_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR05_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR05_A::_1)
    }
}
#[doc = "Field `SAIRQCR06` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR06_R = crate::BitReader<SAIRQCR06_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR06_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR06_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR06_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR06_A {
        match self.bits {
            false => SAIRQCR06_A::_0,
            true => SAIRQCR06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR06_A::_1
    }
}
#[doc = "Field `SAIRQCR06` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR06_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR06_A, O>;
impl<'a, const O: u8> SAIRQCR06_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR06_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR06_A::_1)
    }
}
#[doc = "Field `SAIRQCR07` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR07_R = crate::BitReader<SAIRQCR07_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR07_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR07_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR07_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR07_A {
        match self.bits {
            false => SAIRQCR07_A::_0,
            true => SAIRQCR07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR07_A::_1
    }
}
#[doc = "Field `SAIRQCR07` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR07_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR07_A, O>;
impl<'a, const O: u8> SAIRQCR07_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR07_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR07_A::_1)
    }
}
#[doc = "Field `SAIRQCR08` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR08_R = crate::BitReader<SAIRQCR08_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR08_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR08_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR08_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR08_A {
        match self.bits {
            false => SAIRQCR08_A::_0,
            true => SAIRQCR08_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR08_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR08_A::_1
    }
}
#[doc = "Field `SAIRQCR08` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR08_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR08_A, O>;
impl<'a, const O: u8> SAIRQCR08_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR08_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR08_A::_1)
    }
}
#[doc = "Field `SAIRQCR09` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR09_R = crate::BitReader<SAIRQCR09_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR09_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR09_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR09_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR09_A {
        match self.bits {
            false => SAIRQCR09_A::_0,
            true => SAIRQCR09_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR09_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR09_A::_1
    }
}
#[doc = "Field `SAIRQCR09` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR09_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR09_A, O>;
impl<'a, const O: u8> SAIRQCR09_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR09_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR09_A::_1)
    }
}
#[doc = "Field `SAIRQCR10` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR10_R = crate::BitReader<SAIRQCR10_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR10_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR10_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR10_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR10_A {
        match self.bits {
            false => SAIRQCR10_A::_0,
            true => SAIRQCR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR10_A::_1
    }
}
#[doc = "Field `SAIRQCR10` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR10_A, O>;
impl<'a, const O: u8> SAIRQCR10_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR10_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR10_A::_1)
    }
}
#[doc = "Field `SAIRQCR11` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR11_R = crate::BitReader<SAIRQCR11_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR11_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR11_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR11_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR11_A {
        match self.bits {
            false => SAIRQCR11_A::_0,
            true => SAIRQCR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR11_A::_1
    }
}
#[doc = "Field `SAIRQCR11` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR11_A, O>;
impl<'a, const O: u8> SAIRQCR11_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR11_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR11_A::_1)
    }
}
#[doc = "Field `SAIRQCR12` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR12_R = crate::BitReader<SAIRQCR12_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR12_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR12_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR12_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR12_A {
        match self.bits {
            false => SAIRQCR12_A::_0,
            true => SAIRQCR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR12_A::_1
    }
}
#[doc = "Field `SAIRQCR12` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR12_A, O>;
impl<'a, const O: u8> SAIRQCR12_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR12_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR12_A::_1)
    }
}
#[doc = "Field `SAIRQCR13` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR13_R = crate::BitReader<SAIRQCR13_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR13_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR13_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR13_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR13_A {
        match self.bits {
            false => SAIRQCR13_A::_0,
            true => SAIRQCR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR13_A::_1
    }
}
#[doc = "Field `SAIRQCR13` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR13_A, O>;
impl<'a, const O: u8> SAIRQCR13_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR13_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR13_A::_1)
    }
}
#[doc = "Field `SAIRQCR14` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR14_R = crate::BitReader<SAIRQCR14_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR14_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR14_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR14_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR14_A {
        match self.bits {
            false => SAIRQCR14_A::_0,
            true => SAIRQCR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR14_A::_1
    }
}
#[doc = "Field `SAIRQCR14` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR14_A, O>;
impl<'a, const O: u8> SAIRQCR14_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR14_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR14_A::_1)
    }
}
#[doc = "Field `SAIRQCR15` reader - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR15_R = crate::BitReader<SAIRQCR15_A>;
#[doc = "Security attributes of registers for the IRQCRn register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR15_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR15_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR15_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR15_A {
        match self.bits {
            false => SAIRQCR15_A::_0,
            true => SAIRQCR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR15_A::_1
    }
}
#[doc = "Field `SAIRQCR15` writer - Security attributes of registers for the IRQCRn register"]
pub type SAIRQCR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR15_A, O>;
impl<'a, const O: u8> SAIRQCR15_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR15_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr00(&self) -> SAIRQCR00_R {
        SAIRQCR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr01(&self) -> SAIRQCR01_R {
        SAIRQCR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr02(&self) -> SAIRQCR02_R {
        SAIRQCR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr03(&self) -> SAIRQCR03_R {
        SAIRQCR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr04(&self) -> SAIRQCR04_R {
        SAIRQCR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr05(&self) -> SAIRQCR05_R {
        SAIRQCR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr06(&self) -> SAIRQCR06_R {
        SAIRQCR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr07(&self) -> SAIRQCR07_R {
        SAIRQCR07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr08(&self) -> SAIRQCR08_R {
        SAIRQCR08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr09(&self) -> SAIRQCR09_R {
        SAIRQCR09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr10(&self) -> SAIRQCR10_R {
        SAIRQCR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr11(&self) -> SAIRQCR11_R {
        SAIRQCR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr12(&self) -> SAIRQCR12_R {
        SAIRQCR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr13(&self) -> SAIRQCR13_R {
        SAIRQCR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr14(&self) -> SAIRQCR14_R {
        SAIRQCR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    pub fn sairqcr15(&self) -> SAIRQCR15_R {
        SAIRQCR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr00(&mut self) -> SAIRQCR00_W<0> {
        SAIRQCR00_W::new(self)
    }
    #[doc = "Bit 1 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr01(&mut self) -> SAIRQCR01_W<1> {
        SAIRQCR01_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr02(&mut self) -> SAIRQCR02_W<2> {
        SAIRQCR02_W::new(self)
    }
    #[doc = "Bit 3 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr03(&mut self) -> SAIRQCR03_W<3> {
        SAIRQCR03_W::new(self)
    }
    #[doc = "Bit 4 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr04(&mut self) -> SAIRQCR04_W<4> {
        SAIRQCR04_W::new(self)
    }
    #[doc = "Bit 5 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr05(&mut self) -> SAIRQCR05_W<5> {
        SAIRQCR05_W::new(self)
    }
    #[doc = "Bit 6 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr06(&mut self) -> SAIRQCR06_W<6> {
        SAIRQCR06_W::new(self)
    }
    #[doc = "Bit 7 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr07(&mut self) -> SAIRQCR07_W<7> {
        SAIRQCR07_W::new(self)
    }
    #[doc = "Bit 8 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr08(&mut self) -> SAIRQCR08_W<8> {
        SAIRQCR08_W::new(self)
    }
    #[doc = "Bit 9 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr09(&mut self) -> SAIRQCR09_W<9> {
        SAIRQCR09_W::new(self)
    }
    #[doc = "Bit 10 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr10(&mut self) -> SAIRQCR10_W<10> {
        SAIRQCR10_W::new(self)
    }
    #[doc = "Bit 11 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr11(&mut self) -> SAIRQCR11_W<11> {
        SAIRQCR11_W::new(self)
    }
    #[doc = "Bit 12 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr12(&mut self) -> SAIRQCR12_W<12> {
        SAIRQCR12_W::new(self)
    }
    #[doc = "Bit 13 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr13(&mut self) -> SAIRQCR13_W<13> {
        SAIRQCR13_W::new(self)
    }
    #[doc = "Bit 14 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr14(&mut self) -> SAIRQCR14_W<14> {
        SAIRQCR14_W::new(self)
    }
    #[doc = "Bit 15 - Security attributes of registers for the IRQCRn register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr15(&mut self) -> SAIRQCR15_W<15> {
        SAIRQCR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusara](index.html) module"]
pub struct ICUSARA_SPEC;
impl crate::RegisterSpec for ICUSARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusara::R](R) reader structure"]
impl crate::Readable for ICUSARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusara::W](W) writer structure"]
impl crate::Writable for ICUSARA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARA to value 0xffff_ffff"]
impl crate::Resettable for ICUSARA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
