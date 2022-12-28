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
#[doc = "Field `SAIRQCR0` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR0_R = crate::BitReader<SAIRQCR0_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR0_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR0_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR0_A {
        match self.bits {
            false => SAIRQCR0_A::_0,
            true => SAIRQCR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR0_A::_1
    }
}
#[doc = "Field `SAIRQCR0` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR0_A, O>;
impl<'a, const O: u8> SAIRQCR0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR0_A::_1)
    }
}
#[doc = "Field `SAIRQCR1` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR1_R = crate::BitReader<SAIRQCR1_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR1_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR1_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR1_A {
        match self.bits {
            false => SAIRQCR1_A::_0,
            true => SAIRQCR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR1_A::_1
    }
}
#[doc = "Field `SAIRQCR1` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR1_A, O>;
impl<'a, const O: u8> SAIRQCR1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR1_A::_1)
    }
}
#[doc = "Field `SAIRQCR2` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR2_R = crate::BitReader<SAIRQCR2_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR2_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR2_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR2_A {
        match self.bits {
            false => SAIRQCR2_A::_0,
            true => SAIRQCR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR2_A::_1
    }
}
#[doc = "Field `SAIRQCR2` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR2_A, O>;
impl<'a, const O: u8> SAIRQCR2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR2_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR2_A::_1)
    }
}
#[doc = "Field `SAIRQCR3` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR3_R = crate::BitReader<SAIRQCR3_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR3_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR3_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR3_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR3_A {
        match self.bits {
            false => SAIRQCR3_A::_0,
            true => SAIRQCR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR3_A::_1
    }
}
#[doc = "Field `SAIRQCR3` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR3_A, O>;
impl<'a, const O: u8> SAIRQCR3_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR3_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR3_A::_1)
    }
}
#[doc = "Field `SAIRQCR4` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR4_R = crate::BitReader<SAIRQCR4_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR4_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR4_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR4_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR4_A {
        match self.bits {
            false => SAIRQCR4_A::_0,
            true => SAIRQCR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR4_A::_1
    }
}
#[doc = "Field `SAIRQCR4` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR4_A, O>;
impl<'a, const O: u8> SAIRQCR4_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR4_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR4_A::_1)
    }
}
#[doc = "Field `SAIRQCR5` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR5_R = crate::BitReader<SAIRQCR5_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR5_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR5_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR5_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR5_A {
        match self.bits {
            false => SAIRQCR5_A::_0,
            true => SAIRQCR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR5_A::_1
    }
}
#[doc = "Field `SAIRQCR5` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR5_A, O>;
impl<'a, const O: u8> SAIRQCR5_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR5_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR5_A::_1)
    }
}
#[doc = "Field `SAIRQCR6` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR6_R = crate::BitReader<SAIRQCR6_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR6_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR6_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR6_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR6_A {
        match self.bits {
            false => SAIRQCR6_A::_0,
            true => SAIRQCR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR6_A::_1
    }
}
#[doc = "Field `SAIRQCR6` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR6_A, O>;
impl<'a, const O: u8> SAIRQCR6_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR6_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR6_A::_1)
    }
}
#[doc = "Field `SAIRQCR7` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR7_R = crate::BitReader<SAIRQCR7_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR7_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR7_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR7_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR7_A {
        match self.bits {
            false => SAIRQCR7_A::_0,
            true => SAIRQCR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR7_A::_1
    }
}
#[doc = "Field `SAIRQCR7` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR7_A, O>;
impl<'a, const O: u8> SAIRQCR7_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR7_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR7_A::_1)
    }
}
#[doc = "Field `SAIRQCR8` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR8_R = crate::BitReader<SAIRQCR8_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR8_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR8_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR8_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR8_A {
        match self.bits {
            false => SAIRQCR8_A::_0,
            true => SAIRQCR8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR8_A::_1
    }
}
#[doc = "Field `SAIRQCR8` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR8_A, O>;
impl<'a, const O: u8> SAIRQCR8_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR8_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR8_A::_1)
    }
}
#[doc = "Field `SAIRQCR9` reader - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR9_R = crate::BitReader<SAIRQCR9_A>;
#[doc = "Security attributes of registers for the IRQCRn register (n = 0 to 9)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIRQCR9_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIRQCR9_A> for bool {
    #[inline(always)]
    fn from(variant: SAIRQCR9_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIRQCR9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIRQCR9_A {
        match self.bits {
            false => SAIRQCR9_A::_0,
            true => SAIRQCR9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIRQCR9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIRQCR9_A::_1
    }
}
#[doc = "Field `SAIRQCR9` writer - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
pub type SAIRQCR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARA_SPEC, SAIRQCR9_A, O>;
impl<'a, const O: u8> SAIRQCR9_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIRQCR9_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIRQCR9_A::_1)
    }
}
#[doc = "Field `SAIRQCR13` reader - Security attributes of registers for the IRQCR13 register"]
pub type SAIRQCR13_R = crate::BitReader<SAIRQCR13_A>;
#[doc = "Security attributes of registers for the IRQCR13 register\n\nValue on reset: 1"]
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
#[doc = "Field `SAIRQCR13` writer - Security attributes of registers for the IRQCR13 register"]
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
impl R {
    #[doc = "Bit 0 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr0(&self) -> SAIRQCR0_R {
        SAIRQCR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr1(&self) -> SAIRQCR1_R {
        SAIRQCR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr2(&self) -> SAIRQCR2_R {
        SAIRQCR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr3(&self) -> SAIRQCR3_R {
        SAIRQCR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr4(&self) -> SAIRQCR4_R {
        SAIRQCR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr5(&self) -> SAIRQCR5_R {
        SAIRQCR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr6(&self) -> SAIRQCR6_R {
        SAIRQCR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr7(&self) -> SAIRQCR7_R {
        SAIRQCR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr8(&self) -> SAIRQCR8_R {
        SAIRQCR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    pub fn sairqcr9(&self) -> SAIRQCR9_R {
        SAIRQCR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Security attributes of registers for the IRQCR13 register"]
    #[inline(always)]
    pub fn sairqcr13(&self) -> SAIRQCR13_R {
        SAIRQCR13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr0(&mut self) -> SAIRQCR0_W<0> {
        SAIRQCR0_W::new(self)
    }
    #[doc = "Bit 1 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr1(&mut self) -> SAIRQCR1_W<1> {
        SAIRQCR1_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr2(&mut self) -> SAIRQCR2_W<2> {
        SAIRQCR2_W::new(self)
    }
    #[doc = "Bit 3 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr3(&mut self) -> SAIRQCR3_W<3> {
        SAIRQCR3_W::new(self)
    }
    #[doc = "Bit 4 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr4(&mut self) -> SAIRQCR4_W<4> {
        SAIRQCR4_W::new(self)
    }
    #[doc = "Bit 5 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr5(&mut self) -> SAIRQCR5_W<5> {
        SAIRQCR5_W::new(self)
    }
    #[doc = "Bit 6 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr6(&mut self) -> SAIRQCR6_W<6> {
        SAIRQCR6_W::new(self)
    }
    #[doc = "Bit 7 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr7(&mut self) -> SAIRQCR7_W<7> {
        SAIRQCR7_W::new(self)
    }
    #[doc = "Bit 8 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr8(&mut self) -> SAIRQCR8_W<8> {
        SAIRQCR8_W::new(self)
    }
    #[doc = "Bit 9 - Security attributes of registers for the IRQCRn register (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr9(&mut self) -> SAIRQCR9_W<9> {
        SAIRQCR9_W::new(self)
    }
    #[doc = "Bit 13 - Security attributes of registers for the IRQCR13 register"]
    #[inline(always)]
    #[must_use]
    pub fn sairqcr13(&mut self) -> SAIRQCR13_W<13> {
        SAIRQCR13_W::new(self)
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
