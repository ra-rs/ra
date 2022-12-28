#[doc = "Register `ICUSARH` reader"]
pub struct R(crate::R<ICUSARH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARH` writer"]
pub struct W(crate::W<ICUSARH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARH_SPEC>;
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
impl From<crate::W<ICUSARH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAIELSR32` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR32_R = crate::BitReader<SAIELSR32_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR32_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR32_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR32_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR32_A {
        match self.bits {
            false => SAIELSR32_A::_0,
            true => SAIELSR32_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR32_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR32_A::_1
    }
}
#[doc = "Field `SAIELSR32` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR32_A, O>;
impl<'a, const O: u8> SAIELSR32_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR32_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR32_A::_1)
    }
}
#[doc = "Field `SAIELSR33` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR33_R = crate::BitReader<SAIELSR33_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR33_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR33_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR33_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR33_A {
        match self.bits {
            false => SAIELSR33_A::_0,
            true => SAIELSR33_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR33_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR33_A::_1
    }
}
#[doc = "Field `SAIELSR33` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR33_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR33_A, O>;
impl<'a, const O: u8> SAIELSR33_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR33_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR33_A::_1)
    }
}
#[doc = "Field `SAIELSR34` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR34_R = crate::BitReader<SAIELSR34_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR34_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR34_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR34_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR34_A {
        match self.bits {
            false => SAIELSR34_A::_0,
            true => SAIELSR34_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR34_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR34_A::_1
    }
}
#[doc = "Field `SAIELSR34` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR34_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR34_A, O>;
impl<'a, const O: u8> SAIELSR34_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR34_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR34_A::_1)
    }
}
#[doc = "Field `SAIELSR35` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR35_R = crate::BitReader<SAIELSR35_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR35_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR35_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR35_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR35_A {
        match self.bits {
            false => SAIELSR35_A::_0,
            true => SAIELSR35_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR35_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR35_A::_1
    }
}
#[doc = "Field `SAIELSR35` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR35_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR35_A, O>;
impl<'a, const O: u8> SAIELSR35_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR35_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR35_A::_1)
    }
}
#[doc = "Field `SAIELSR36` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR36_R = crate::BitReader<SAIELSR36_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR36_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR36_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR36_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR36_A {
        match self.bits {
            false => SAIELSR36_A::_0,
            true => SAIELSR36_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR36_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR36_A::_1
    }
}
#[doc = "Field `SAIELSR36` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR36_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR36_A, O>;
impl<'a, const O: u8> SAIELSR36_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR36_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR36_A::_1)
    }
}
#[doc = "Field `SAIELSR37` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR37_R = crate::BitReader<SAIELSR37_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR37_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR37_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR37_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR37_A {
        match self.bits {
            false => SAIELSR37_A::_0,
            true => SAIELSR37_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR37_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR37_A::_1
    }
}
#[doc = "Field `SAIELSR37` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR37_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR37_A, O>;
impl<'a, const O: u8> SAIELSR37_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR37_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR37_A::_1)
    }
}
#[doc = "Field `SAIELSR38` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR38_R = crate::BitReader<SAIELSR38_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR38_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR38_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR38_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR38_A {
        match self.bits {
            false => SAIELSR38_A::_0,
            true => SAIELSR38_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR38_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR38_A::_1
    }
}
#[doc = "Field `SAIELSR38` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR38_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR38_A, O>;
impl<'a, const O: u8> SAIELSR38_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR38_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR38_A::_1)
    }
}
#[doc = "Field `SAIELSR39` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR39_R = crate::BitReader<SAIELSR39_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR39_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR39_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR39_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR39_A {
        match self.bits {
            false => SAIELSR39_A::_0,
            true => SAIELSR39_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR39_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR39_A::_1
    }
}
#[doc = "Field `SAIELSR39` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR39_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR39_A, O>;
impl<'a, const O: u8> SAIELSR39_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR39_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR39_A::_1)
    }
}
#[doc = "Field `SAIELSR40` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR40_R = crate::BitReader<SAIELSR40_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR40_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR40_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR40_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR40_A {
        match self.bits {
            false => SAIELSR40_A::_0,
            true => SAIELSR40_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR40_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR40_A::_1
    }
}
#[doc = "Field `SAIELSR40` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR40_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR40_A, O>;
impl<'a, const O: u8> SAIELSR40_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR40_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR40_A::_1)
    }
}
#[doc = "Field `SAIELSR41` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR41_R = crate::BitReader<SAIELSR41_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR41_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR41_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR41_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR41_A {
        match self.bits {
            false => SAIELSR41_A::_0,
            true => SAIELSR41_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR41_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR41_A::_1
    }
}
#[doc = "Field `SAIELSR41` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR41_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR41_A, O>;
impl<'a, const O: u8> SAIELSR41_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR41_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR41_A::_1)
    }
}
#[doc = "Field `SAIELSR42` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR42_R = crate::BitReader<SAIELSR42_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR42_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR42_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR42_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR42_A {
        match self.bits {
            false => SAIELSR42_A::_0,
            true => SAIELSR42_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR42_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR42_A::_1
    }
}
#[doc = "Field `SAIELSR42` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR42_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR42_A, O>;
impl<'a, const O: u8> SAIELSR42_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR42_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR42_A::_1)
    }
}
#[doc = "Field `SAIELSR43` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR43_R = crate::BitReader<SAIELSR43_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR43_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR43_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR43_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR43_A {
        match self.bits {
            false => SAIELSR43_A::_0,
            true => SAIELSR43_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR43_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR43_A::_1
    }
}
#[doc = "Field `SAIELSR43` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR43_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR43_A, O>;
impl<'a, const O: u8> SAIELSR43_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR43_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR43_A::_1)
    }
}
#[doc = "Field `SAIELSR44` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR44_R = crate::BitReader<SAIELSR44_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR44_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR44_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR44_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR44_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR44_A {
        match self.bits {
            false => SAIELSR44_A::_0,
            true => SAIELSR44_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR44_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR44_A::_1
    }
}
#[doc = "Field `SAIELSR44` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR44_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR44_A, O>;
impl<'a, const O: u8> SAIELSR44_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR44_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR44_A::_1)
    }
}
#[doc = "Field `SAIELSR45` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR45_R = crate::BitReader<SAIELSR45_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR45_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR45_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR45_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR45_A {
        match self.bits {
            false => SAIELSR45_A::_0,
            true => SAIELSR45_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR45_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR45_A::_1
    }
}
#[doc = "Field `SAIELSR45` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR45_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR45_A, O>;
impl<'a, const O: u8> SAIELSR45_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR45_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR45_A::_1)
    }
}
#[doc = "Field `SAIELSR46` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR46_R = crate::BitReader<SAIELSR46_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR46_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR46_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR46_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR46_A {
        match self.bits {
            false => SAIELSR46_A::_0,
            true => SAIELSR46_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR46_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR46_A::_1
    }
}
#[doc = "Field `SAIELSR46` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR46_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR46_A, O>;
impl<'a, const O: u8> SAIELSR46_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR46_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR46_A::_1)
    }
}
#[doc = "Field `SAIELSR47` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR47_R = crate::BitReader<SAIELSR47_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR47_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR47_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR47_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR47_A {
        match self.bits {
            false => SAIELSR47_A::_0,
            true => SAIELSR47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR47_A::_1
    }
}
#[doc = "Field `SAIELSR47` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR47_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR47_A, O>;
impl<'a, const O: u8> SAIELSR47_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR47_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR47_A::_1)
    }
}
#[doc = "Field `SAIELSR48` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR48_R = crate::BitReader<SAIELSR48_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR48_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR48_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR48_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR48_A {
        match self.bits {
            false => SAIELSR48_A::_0,
            true => SAIELSR48_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR48_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR48_A::_1
    }
}
#[doc = "Field `SAIELSR48` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR48_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR48_A, O>;
impl<'a, const O: u8> SAIELSR48_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR48_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR48_A::_1)
    }
}
#[doc = "Field `SAIELSR49` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR49_R = crate::BitReader<SAIELSR49_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR49_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR49_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR49_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR49_A {
        match self.bits {
            false => SAIELSR49_A::_0,
            true => SAIELSR49_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR49_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR49_A::_1
    }
}
#[doc = "Field `SAIELSR49` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR49_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR49_A, O>;
impl<'a, const O: u8> SAIELSR49_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR49_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR49_A::_1)
    }
}
#[doc = "Field `SAIELSR50` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR50_R = crate::BitReader<SAIELSR50_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR50_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR50_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR50_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR50_A {
        match self.bits {
            false => SAIELSR50_A::_0,
            true => SAIELSR50_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR50_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR50_A::_1
    }
}
#[doc = "Field `SAIELSR50` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR50_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR50_A, O>;
impl<'a, const O: u8> SAIELSR50_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR50_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR50_A::_1)
    }
}
#[doc = "Field `SAIELSR51` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR51_R = crate::BitReader<SAIELSR51_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR51_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR51_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR51_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR51_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR51_A {
        match self.bits {
            false => SAIELSR51_A::_0,
            true => SAIELSR51_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR51_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR51_A::_1
    }
}
#[doc = "Field `SAIELSR51` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR51_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR51_A, O>;
impl<'a, const O: u8> SAIELSR51_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR51_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR51_A::_1)
    }
}
#[doc = "Field `SAIELSR52` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR52_R = crate::BitReader<SAIELSR52_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR52_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR52_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR52_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR52_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR52_A {
        match self.bits {
            false => SAIELSR52_A::_0,
            true => SAIELSR52_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR52_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR52_A::_1
    }
}
#[doc = "Field `SAIELSR52` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR52_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR52_A, O>;
impl<'a, const O: u8> SAIELSR52_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR52_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR52_A::_1)
    }
}
#[doc = "Field `SAIELSR53` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR53_R = crate::BitReader<SAIELSR53_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR53_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR53_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR53_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR53_A {
        match self.bits {
            false => SAIELSR53_A::_0,
            true => SAIELSR53_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR53_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR53_A::_1
    }
}
#[doc = "Field `SAIELSR53` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR53_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR53_A, O>;
impl<'a, const O: u8> SAIELSR53_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR53_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR53_A::_1)
    }
}
#[doc = "Field `SAIELSR54` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR54_R = crate::BitReader<SAIELSR54_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR54_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR54_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR54_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR54_A {
        match self.bits {
            false => SAIELSR54_A::_0,
            true => SAIELSR54_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR54_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR54_A::_1
    }
}
#[doc = "Field `SAIELSR54` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR54_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR54_A, O>;
impl<'a, const O: u8> SAIELSR54_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR54_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR54_A::_1)
    }
}
#[doc = "Field `SAIELSR55` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR55_R = crate::BitReader<SAIELSR55_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR55_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR55_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR55_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR55_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR55_A {
        match self.bits {
            false => SAIELSR55_A::_0,
            true => SAIELSR55_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR55_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR55_A::_1
    }
}
#[doc = "Field `SAIELSR55` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR55_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR55_A, O>;
impl<'a, const O: u8> SAIELSR55_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR55_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR55_A::_1)
    }
}
#[doc = "Field `SAIELSR56` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR56_R = crate::BitReader<SAIELSR56_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR56_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR56_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR56_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR56_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR56_A {
        match self.bits {
            false => SAIELSR56_A::_0,
            true => SAIELSR56_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR56_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR56_A::_1
    }
}
#[doc = "Field `SAIELSR56` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR56_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR56_A, O>;
impl<'a, const O: u8> SAIELSR56_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR56_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR56_A::_1)
    }
}
#[doc = "Field `SAIELSR57` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR57_R = crate::BitReader<SAIELSR57_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR57_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR57_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR57_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR57_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR57_A {
        match self.bits {
            false => SAIELSR57_A::_0,
            true => SAIELSR57_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR57_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR57_A::_1
    }
}
#[doc = "Field `SAIELSR57` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR57_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR57_A, O>;
impl<'a, const O: u8> SAIELSR57_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR57_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR57_A::_1)
    }
}
#[doc = "Field `SAIELSR58` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR58_R = crate::BitReader<SAIELSR58_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR58_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR58_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR58_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR58_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR58_A {
        match self.bits {
            false => SAIELSR58_A::_0,
            true => SAIELSR58_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR58_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR58_A::_1
    }
}
#[doc = "Field `SAIELSR58` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR58_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR58_A, O>;
impl<'a, const O: u8> SAIELSR58_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR58_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR58_A::_1)
    }
}
#[doc = "Field `SAIELSR59` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR59_R = crate::BitReader<SAIELSR59_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR59_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR59_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR59_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR59_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR59_A {
        match self.bits {
            false => SAIELSR59_A::_0,
            true => SAIELSR59_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR59_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR59_A::_1
    }
}
#[doc = "Field `SAIELSR59` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR59_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR59_A, O>;
impl<'a, const O: u8> SAIELSR59_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR59_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR59_A::_1)
    }
}
#[doc = "Field `SAIELSR60` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR60_R = crate::BitReader<SAIELSR60_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR60_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR60_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR60_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR60_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR60_A {
        match self.bits {
            false => SAIELSR60_A::_0,
            true => SAIELSR60_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR60_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR60_A::_1
    }
}
#[doc = "Field `SAIELSR60` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR60_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR60_A, O>;
impl<'a, const O: u8> SAIELSR60_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR60_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR60_A::_1)
    }
}
#[doc = "Field `SAIELSR61` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR61_R = crate::BitReader<SAIELSR61_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR61_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR61_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR61_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR61_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR61_A {
        match self.bits {
            false => SAIELSR61_A::_0,
            true => SAIELSR61_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR61_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR61_A::_1
    }
}
#[doc = "Field `SAIELSR61` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR61_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR61_A, O>;
impl<'a, const O: u8> SAIELSR61_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR61_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR61_A::_1)
    }
}
#[doc = "Field `SAIELSR62` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR62_R = crate::BitReader<SAIELSR62_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR62_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR62_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR62_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR62_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR62_A {
        match self.bits {
            false => SAIELSR62_A::_0,
            true => SAIELSR62_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR62_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR62_A::_1
    }
}
#[doc = "Field `SAIELSR62` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR62_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR62_A, O>;
impl<'a, const O: u8> SAIELSR62_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR62_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR62_A::_1)
    }
}
#[doc = "Field `SAIELSR63` reader - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR63_R = crate::BitReader<SAIELSR63_A>;
#[doc = "Security attributes of registers for IELSR63 to IELSR32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR63_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR63_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR63_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR63_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR63_A {
        match self.bits {
            false => SAIELSR63_A::_0,
            true => SAIELSR63_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR63_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR63_A::_1
    }
}
#[doc = "Field `SAIELSR63` writer - Security attributes of registers for IELSR63 to IELSR32"]
pub type SAIELSR63_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARH_SPEC, SAIELSR63_A, O>;
impl<'a, const O: u8> SAIELSR63_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR63_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR63_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr32(&self) -> SAIELSR32_R {
        SAIELSR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr33(&self) -> SAIELSR33_R {
        SAIELSR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr34(&self) -> SAIELSR34_R {
        SAIELSR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr35(&self) -> SAIELSR35_R {
        SAIELSR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr36(&self) -> SAIELSR36_R {
        SAIELSR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr37(&self) -> SAIELSR37_R {
        SAIELSR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr38(&self) -> SAIELSR38_R {
        SAIELSR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr39(&self) -> SAIELSR39_R {
        SAIELSR39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr40(&self) -> SAIELSR40_R {
        SAIELSR40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr41(&self) -> SAIELSR41_R {
        SAIELSR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr42(&self) -> SAIELSR42_R {
        SAIELSR42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr43(&self) -> SAIELSR43_R {
        SAIELSR43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr44(&self) -> SAIELSR44_R {
        SAIELSR44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr45(&self) -> SAIELSR45_R {
        SAIELSR45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr46(&self) -> SAIELSR46_R {
        SAIELSR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr47(&self) -> SAIELSR47_R {
        SAIELSR47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr48(&self) -> SAIELSR48_R {
        SAIELSR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr49(&self) -> SAIELSR49_R {
        SAIELSR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr50(&self) -> SAIELSR50_R {
        SAIELSR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr51(&self) -> SAIELSR51_R {
        SAIELSR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr52(&self) -> SAIELSR52_R {
        SAIELSR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr53(&self) -> SAIELSR53_R {
        SAIELSR53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr54(&self) -> SAIELSR54_R {
        SAIELSR54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr55(&self) -> SAIELSR55_R {
        SAIELSR55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr56(&self) -> SAIELSR56_R {
        SAIELSR56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr57(&self) -> SAIELSR57_R {
        SAIELSR57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr58(&self) -> SAIELSR58_R {
        SAIELSR58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr59(&self) -> SAIELSR59_R {
        SAIELSR59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr60(&self) -> SAIELSR60_R {
        SAIELSR60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr61(&self) -> SAIELSR61_R {
        SAIELSR61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr62(&self) -> SAIELSR62_R {
        SAIELSR62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    pub fn saielsr63(&self) -> SAIELSR63_R {
        SAIELSR63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr32(&mut self) -> SAIELSR32_W<0> {
        SAIELSR32_W::new(self)
    }
    #[doc = "Bit 1 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr33(&mut self) -> SAIELSR33_W<1> {
        SAIELSR33_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr34(&mut self) -> SAIELSR34_W<2> {
        SAIELSR34_W::new(self)
    }
    #[doc = "Bit 3 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr35(&mut self) -> SAIELSR35_W<3> {
        SAIELSR35_W::new(self)
    }
    #[doc = "Bit 4 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr36(&mut self) -> SAIELSR36_W<4> {
        SAIELSR36_W::new(self)
    }
    #[doc = "Bit 5 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr37(&mut self) -> SAIELSR37_W<5> {
        SAIELSR37_W::new(self)
    }
    #[doc = "Bit 6 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr38(&mut self) -> SAIELSR38_W<6> {
        SAIELSR38_W::new(self)
    }
    #[doc = "Bit 7 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr39(&mut self) -> SAIELSR39_W<7> {
        SAIELSR39_W::new(self)
    }
    #[doc = "Bit 8 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr40(&mut self) -> SAIELSR40_W<8> {
        SAIELSR40_W::new(self)
    }
    #[doc = "Bit 9 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr41(&mut self) -> SAIELSR41_W<9> {
        SAIELSR41_W::new(self)
    }
    #[doc = "Bit 10 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr42(&mut self) -> SAIELSR42_W<10> {
        SAIELSR42_W::new(self)
    }
    #[doc = "Bit 11 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr43(&mut self) -> SAIELSR43_W<11> {
        SAIELSR43_W::new(self)
    }
    #[doc = "Bit 12 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr44(&mut self) -> SAIELSR44_W<12> {
        SAIELSR44_W::new(self)
    }
    #[doc = "Bit 13 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr45(&mut self) -> SAIELSR45_W<13> {
        SAIELSR45_W::new(self)
    }
    #[doc = "Bit 14 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr46(&mut self) -> SAIELSR46_W<14> {
        SAIELSR46_W::new(self)
    }
    #[doc = "Bit 15 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr47(&mut self) -> SAIELSR47_W<15> {
        SAIELSR47_W::new(self)
    }
    #[doc = "Bit 16 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr48(&mut self) -> SAIELSR48_W<16> {
        SAIELSR48_W::new(self)
    }
    #[doc = "Bit 17 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr49(&mut self) -> SAIELSR49_W<17> {
        SAIELSR49_W::new(self)
    }
    #[doc = "Bit 18 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr50(&mut self) -> SAIELSR50_W<18> {
        SAIELSR50_W::new(self)
    }
    #[doc = "Bit 19 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr51(&mut self) -> SAIELSR51_W<19> {
        SAIELSR51_W::new(self)
    }
    #[doc = "Bit 20 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr52(&mut self) -> SAIELSR52_W<20> {
        SAIELSR52_W::new(self)
    }
    #[doc = "Bit 21 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr53(&mut self) -> SAIELSR53_W<21> {
        SAIELSR53_W::new(self)
    }
    #[doc = "Bit 22 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr54(&mut self) -> SAIELSR54_W<22> {
        SAIELSR54_W::new(self)
    }
    #[doc = "Bit 23 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr55(&mut self) -> SAIELSR55_W<23> {
        SAIELSR55_W::new(self)
    }
    #[doc = "Bit 24 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr56(&mut self) -> SAIELSR56_W<24> {
        SAIELSR56_W::new(self)
    }
    #[doc = "Bit 25 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr57(&mut self) -> SAIELSR57_W<25> {
        SAIELSR57_W::new(self)
    }
    #[doc = "Bit 26 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr58(&mut self) -> SAIELSR58_W<26> {
        SAIELSR58_W::new(self)
    }
    #[doc = "Bit 27 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr59(&mut self) -> SAIELSR59_W<27> {
        SAIELSR59_W::new(self)
    }
    #[doc = "Bit 28 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr60(&mut self) -> SAIELSR60_W<28> {
        SAIELSR60_W::new(self)
    }
    #[doc = "Bit 29 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr61(&mut self) -> SAIELSR61_W<29> {
        SAIELSR61_W::new(self)
    }
    #[doc = "Bit 30 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr62(&mut self) -> SAIELSR62_W<30> {
        SAIELSR62_W::new(self)
    }
    #[doc = "Bit 31 - Security attributes of registers for IELSR63 to IELSR32"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr63(&mut self) -> SAIELSR63_W<31> {
        SAIELSR63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusarh](index.html) module"]
pub struct ICUSARH_SPEC;
impl crate::RegisterSpec for ICUSARH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusarh::R](R) reader structure"]
impl crate::Readable for ICUSARH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusarh::W](W) writer structure"]
impl crate::Writable for ICUSARH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARH to value 0xffff_ffff"]
impl crate::Resettable for ICUSARH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
