#[doc = "Register `ICUSARI` reader"]
pub struct R(crate::R<ICUSARI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARI` writer"]
pub struct W(crate::W<ICUSARI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARI_SPEC>;
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
impl From<crate::W<ICUSARI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAIELSR64` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR64_R = crate::BitReader<SAIELSR64_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR64_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR64_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR64_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR64_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR64_A {
        match self.bits {
            false => SAIELSR64_A::_0,
            true => SAIELSR64_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR64_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR64_A::_1
    }
}
#[doc = "Field `SAIELSR64` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR64_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR64_A, O>;
impl<'a, const O: u8> SAIELSR64_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR64_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR64_A::_1)
    }
}
#[doc = "Field `SAIELSR65` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR65_R = crate::BitReader<SAIELSR65_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR65_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR65_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR65_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR65_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR65_A {
        match self.bits {
            false => SAIELSR65_A::_0,
            true => SAIELSR65_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR65_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR65_A::_1
    }
}
#[doc = "Field `SAIELSR65` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR65_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR65_A, O>;
impl<'a, const O: u8> SAIELSR65_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR65_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR65_A::_1)
    }
}
#[doc = "Field `SAIELSR66` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR66_R = crate::BitReader<SAIELSR66_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR66_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR66_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR66_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR66_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR66_A {
        match self.bits {
            false => SAIELSR66_A::_0,
            true => SAIELSR66_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR66_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR66_A::_1
    }
}
#[doc = "Field `SAIELSR66` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR66_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR66_A, O>;
impl<'a, const O: u8> SAIELSR66_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR66_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR66_A::_1)
    }
}
#[doc = "Field `SAIELSR67` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR67_R = crate::BitReader<SAIELSR67_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR67_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR67_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR67_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR67_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR67_A {
        match self.bits {
            false => SAIELSR67_A::_0,
            true => SAIELSR67_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR67_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR67_A::_1
    }
}
#[doc = "Field `SAIELSR67` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR67_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR67_A, O>;
impl<'a, const O: u8> SAIELSR67_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR67_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR67_A::_1)
    }
}
#[doc = "Field `SAIELSR68` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR68_R = crate::BitReader<SAIELSR68_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR68_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR68_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR68_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR68_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR68_A {
        match self.bits {
            false => SAIELSR68_A::_0,
            true => SAIELSR68_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR68_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR68_A::_1
    }
}
#[doc = "Field `SAIELSR68` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR68_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR68_A, O>;
impl<'a, const O: u8> SAIELSR68_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR68_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR68_A::_1)
    }
}
#[doc = "Field `SAIELSR69` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR69_R = crate::BitReader<SAIELSR69_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR69_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR69_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR69_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR69_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR69_A {
        match self.bits {
            false => SAIELSR69_A::_0,
            true => SAIELSR69_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR69_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR69_A::_1
    }
}
#[doc = "Field `SAIELSR69` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR69_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR69_A, O>;
impl<'a, const O: u8> SAIELSR69_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR69_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR69_A::_1)
    }
}
#[doc = "Field `SAIELSR70` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR70_R = crate::BitReader<SAIELSR70_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR70_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR70_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR70_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR70_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR70_A {
        match self.bits {
            false => SAIELSR70_A::_0,
            true => SAIELSR70_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR70_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR70_A::_1
    }
}
#[doc = "Field `SAIELSR70` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR70_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR70_A, O>;
impl<'a, const O: u8> SAIELSR70_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR70_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR70_A::_1)
    }
}
#[doc = "Field `SAIELSR71` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR71_R = crate::BitReader<SAIELSR71_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR71_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR71_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR71_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR71_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR71_A {
        match self.bits {
            false => SAIELSR71_A::_0,
            true => SAIELSR71_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR71_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR71_A::_1
    }
}
#[doc = "Field `SAIELSR71` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR71_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR71_A, O>;
impl<'a, const O: u8> SAIELSR71_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR71_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR71_A::_1)
    }
}
#[doc = "Field `SAIELSR72` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR72_R = crate::BitReader<SAIELSR72_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR72_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR72_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR72_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR72_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR72_A {
        match self.bits {
            false => SAIELSR72_A::_0,
            true => SAIELSR72_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR72_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR72_A::_1
    }
}
#[doc = "Field `SAIELSR72` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR72_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR72_A, O>;
impl<'a, const O: u8> SAIELSR72_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR72_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR72_A::_1)
    }
}
#[doc = "Field `SAIELSR73` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR73_R = crate::BitReader<SAIELSR73_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR73_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR73_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR73_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR73_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR73_A {
        match self.bits {
            false => SAIELSR73_A::_0,
            true => SAIELSR73_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR73_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR73_A::_1
    }
}
#[doc = "Field `SAIELSR73` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR73_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR73_A, O>;
impl<'a, const O: u8> SAIELSR73_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR73_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR73_A::_1)
    }
}
#[doc = "Field `SAIELSR74` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR74_R = crate::BitReader<SAIELSR74_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR74_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR74_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR74_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR74_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR74_A {
        match self.bits {
            false => SAIELSR74_A::_0,
            true => SAIELSR74_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR74_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR74_A::_1
    }
}
#[doc = "Field `SAIELSR74` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR74_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR74_A, O>;
impl<'a, const O: u8> SAIELSR74_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR74_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR74_A::_1)
    }
}
#[doc = "Field `SAIELSR75` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR75_R = crate::BitReader<SAIELSR75_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR75_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR75_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR75_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR75_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR75_A {
        match self.bits {
            false => SAIELSR75_A::_0,
            true => SAIELSR75_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR75_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR75_A::_1
    }
}
#[doc = "Field `SAIELSR75` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR75_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR75_A, O>;
impl<'a, const O: u8> SAIELSR75_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR75_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR75_A::_1)
    }
}
#[doc = "Field `SAIELSR76` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR76_R = crate::BitReader<SAIELSR76_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR76_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR76_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR76_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR76_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR76_A {
        match self.bits {
            false => SAIELSR76_A::_0,
            true => SAIELSR76_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR76_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR76_A::_1
    }
}
#[doc = "Field `SAIELSR76` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR76_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR76_A, O>;
impl<'a, const O: u8> SAIELSR76_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR76_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR76_A::_1)
    }
}
#[doc = "Field `SAIELSR77` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR77_R = crate::BitReader<SAIELSR77_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR77_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR77_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR77_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR77_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR77_A {
        match self.bits {
            false => SAIELSR77_A::_0,
            true => SAIELSR77_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR77_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR77_A::_1
    }
}
#[doc = "Field `SAIELSR77` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR77_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR77_A, O>;
impl<'a, const O: u8> SAIELSR77_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR77_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR77_A::_1)
    }
}
#[doc = "Field `SAIELSR78` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR78_R = crate::BitReader<SAIELSR78_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR78_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR78_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR78_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR78_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR78_A {
        match self.bits {
            false => SAIELSR78_A::_0,
            true => SAIELSR78_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR78_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR78_A::_1
    }
}
#[doc = "Field `SAIELSR78` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR78_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR78_A, O>;
impl<'a, const O: u8> SAIELSR78_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR78_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR78_A::_1)
    }
}
#[doc = "Field `SAIELSR79` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR79_R = crate::BitReader<SAIELSR79_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR79_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR79_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR79_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR79_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR79_A {
        match self.bits {
            false => SAIELSR79_A::_0,
            true => SAIELSR79_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR79_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR79_A::_1
    }
}
#[doc = "Field `SAIELSR79` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR79_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR79_A, O>;
impl<'a, const O: u8> SAIELSR79_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR79_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR79_A::_1)
    }
}
#[doc = "Field `SAIELSR80` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR80_R = crate::BitReader<SAIELSR80_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR80_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR80_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR80_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR80_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR80_A {
        match self.bits {
            false => SAIELSR80_A::_0,
            true => SAIELSR80_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR80_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR80_A::_1
    }
}
#[doc = "Field `SAIELSR80` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR80_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR80_A, O>;
impl<'a, const O: u8> SAIELSR80_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR80_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR80_A::_1)
    }
}
#[doc = "Field `SAIELSR81` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR81_R = crate::BitReader<SAIELSR81_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR81_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR81_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR81_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR81_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR81_A {
        match self.bits {
            false => SAIELSR81_A::_0,
            true => SAIELSR81_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR81_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR81_A::_1
    }
}
#[doc = "Field `SAIELSR81` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR81_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR81_A, O>;
impl<'a, const O: u8> SAIELSR81_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR81_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR81_A::_1)
    }
}
#[doc = "Field `SAIELSR82` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR82_R = crate::BitReader<SAIELSR82_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR82_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR82_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR82_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR82_A {
        match self.bits {
            false => SAIELSR82_A::_0,
            true => SAIELSR82_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR82_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR82_A::_1
    }
}
#[doc = "Field `SAIELSR82` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR82_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR82_A, O>;
impl<'a, const O: u8> SAIELSR82_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR82_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR82_A::_1)
    }
}
#[doc = "Field `SAIELSR83` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR83_R = crate::BitReader<SAIELSR83_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR83_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR83_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR83_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR83_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR83_A {
        match self.bits {
            false => SAIELSR83_A::_0,
            true => SAIELSR83_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR83_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR83_A::_1
    }
}
#[doc = "Field `SAIELSR83` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR83_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR83_A, O>;
impl<'a, const O: u8> SAIELSR83_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR83_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR83_A::_1)
    }
}
#[doc = "Field `SAIELSR84` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR84_R = crate::BitReader<SAIELSR84_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR84_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR84_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR84_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR84_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR84_A {
        match self.bits {
            false => SAIELSR84_A::_0,
            true => SAIELSR84_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR84_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR84_A::_1
    }
}
#[doc = "Field `SAIELSR84` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR84_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR84_A, O>;
impl<'a, const O: u8> SAIELSR84_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR84_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR84_A::_1)
    }
}
#[doc = "Field `SAIELSR85` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR85_R = crate::BitReader<SAIELSR85_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR85_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR85_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR85_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR85_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR85_A {
        match self.bits {
            false => SAIELSR85_A::_0,
            true => SAIELSR85_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR85_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR85_A::_1
    }
}
#[doc = "Field `SAIELSR85` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR85_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR85_A, O>;
impl<'a, const O: u8> SAIELSR85_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR85_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR85_A::_1)
    }
}
#[doc = "Field `SAIELSR86` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR86_R = crate::BitReader<SAIELSR86_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR86_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR86_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR86_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR86_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR86_A {
        match self.bits {
            false => SAIELSR86_A::_0,
            true => SAIELSR86_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR86_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR86_A::_1
    }
}
#[doc = "Field `SAIELSR86` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR86_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR86_A, O>;
impl<'a, const O: u8> SAIELSR86_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR86_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR86_A::_1)
    }
}
#[doc = "Field `SAIELSR87` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR87_R = crate::BitReader<SAIELSR87_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR87_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR87_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR87_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR87_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR87_A {
        match self.bits {
            false => SAIELSR87_A::_0,
            true => SAIELSR87_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR87_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR87_A::_1
    }
}
#[doc = "Field `SAIELSR87` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR87_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR87_A, O>;
impl<'a, const O: u8> SAIELSR87_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR87_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR87_A::_1)
    }
}
#[doc = "Field `SAIELSR88` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR88_R = crate::BitReader<SAIELSR88_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR88_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR88_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR88_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR88_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR88_A {
        match self.bits {
            false => SAIELSR88_A::_0,
            true => SAIELSR88_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR88_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR88_A::_1
    }
}
#[doc = "Field `SAIELSR88` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR88_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR88_A, O>;
impl<'a, const O: u8> SAIELSR88_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR88_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR88_A::_1)
    }
}
#[doc = "Field `SAIELSR89` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR89_R = crate::BitReader<SAIELSR89_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR89_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR89_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR89_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR89_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR89_A {
        match self.bits {
            false => SAIELSR89_A::_0,
            true => SAIELSR89_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR89_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR89_A::_1
    }
}
#[doc = "Field `SAIELSR89` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR89_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR89_A, O>;
impl<'a, const O: u8> SAIELSR89_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR89_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR89_A::_1)
    }
}
#[doc = "Field `SAIELSR90` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR90_R = crate::BitReader<SAIELSR90_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR90_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR90_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR90_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR90_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR90_A {
        match self.bits {
            false => SAIELSR90_A::_0,
            true => SAIELSR90_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR90_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR90_A::_1
    }
}
#[doc = "Field `SAIELSR90` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR90_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR90_A, O>;
impl<'a, const O: u8> SAIELSR90_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR90_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR90_A::_1)
    }
}
#[doc = "Field `SAIELSR91` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR91_R = crate::BitReader<SAIELSR91_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR91_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR91_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR91_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR91_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR91_A {
        match self.bits {
            false => SAIELSR91_A::_0,
            true => SAIELSR91_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR91_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR91_A::_1
    }
}
#[doc = "Field `SAIELSR91` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR91_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR91_A, O>;
impl<'a, const O: u8> SAIELSR91_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR91_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR91_A::_1)
    }
}
#[doc = "Field `SAIELSR92` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR92_R = crate::BitReader<SAIELSR92_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR92_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR92_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR92_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR92_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR92_A {
        match self.bits {
            false => SAIELSR92_A::_0,
            true => SAIELSR92_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR92_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR92_A::_1
    }
}
#[doc = "Field `SAIELSR92` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR92_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR92_A, O>;
impl<'a, const O: u8> SAIELSR92_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR92_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR92_A::_1)
    }
}
#[doc = "Field `SAIELSR93` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR93_R = crate::BitReader<SAIELSR93_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR93_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR93_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR93_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR93_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR93_A {
        match self.bits {
            false => SAIELSR93_A::_0,
            true => SAIELSR93_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR93_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR93_A::_1
    }
}
#[doc = "Field `SAIELSR93` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR93_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR93_A, O>;
impl<'a, const O: u8> SAIELSR93_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR93_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR93_A::_1)
    }
}
#[doc = "Field `SAIELSR94` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR94_R = crate::BitReader<SAIELSR94_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR94_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR94_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR94_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR94_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR94_A {
        match self.bits {
            false => SAIELSR94_A::_0,
            true => SAIELSR94_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR94_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR94_A::_1
    }
}
#[doc = "Field `SAIELSR94` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR94_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR94_A, O>;
impl<'a, const O: u8> SAIELSR94_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR94_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR94_A::_1)
    }
}
#[doc = "Field `SAIELSR95` reader - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR95_R = crate::BitReader<SAIELSR95_A>;
#[doc = "Security attributes of registers for IELSR95 to IELSR64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIELSR95_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIELSR95_A> for bool {
    #[inline(always)]
    fn from(variant: SAIELSR95_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIELSR95_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIELSR95_A {
        match self.bits {
            false => SAIELSR95_A::_0,
            true => SAIELSR95_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIELSR95_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIELSR95_A::_1
    }
}
#[doc = "Field `SAIELSR95` writer - Security attributes of registers for IELSR95 to IELSR64"]
pub type SAIELSR95_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARI_SPEC, SAIELSR95_A, O>;
impl<'a, const O: u8> SAIELSR95_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIELSR95_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIELSR95_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr64(&self) -> SAIELSR64_R {
        SAIELSR64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr65(&self) -> SAIELSR65_R {
        SAIELSR65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr66(&self) -> SAIELSR66_R {
        SAIELSR66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr67(&self) -> SAIELSR67_R {
        SAIELSR67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr68(&self) -> SAIELSR68_R {
        SAIELSR68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr69(&self) -> SAIELSR69_R {
        SAIELSR69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr70(&self) -> SAIELSR70_R {
        SAIELSR70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr71(&self) -> SAIELSR71_R {
        SAIELSR71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr72(&self) -> SAIELSR72_R {
        SAIELSR72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr73(&self) -> SAIELSR73_R {
        SAIELSR73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr74(&self) -> SAIELSR74_R {
        SAIELSR74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr75(&self) -> SAIELSR75_R {
        SAIELSR75_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr76(&self) -> SAIELSR76_R {
        SAIELSR76_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr77(&self) -> SAIELSR77_R {
        SAIELSR77_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr78(&self) -> SAIELSR78_R {
        SAIELSR78_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr79(&self) -> SAIELSR79_R {
        SAIELSR79_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr80(&self) -> SAIELSR80_R {
        SAIELSR80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr81(&self) -> SAIELSR81_R {
        SAIELSR81_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr82(&self) -> SAIELSR82_R {
        SAIELSR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr83(&self) -> SAIELSR83_R {
        SAIELSR83_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr84(&self) -> SAIELSR84_R {
        SAIELSR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr85(&self) -> SAIELSR85_R {
        SAIELSR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr86(&self) -> SAIELSR86_R {
        SAIELSR86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr87(&self) -> SAIELSR87_R {
        SAIELSR87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr88(&self) -> SAIELSR88_R {
        SAIELSR88_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr89(&self) -> SAIELSR89_R {
        SAIELSR89_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr90(&self) -> SAIELSR90_R {
        SAIELSR90_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr91(&self) -> SAIELSR91_R {
        SAIELSR91_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr92(&self) -> SAIELSR92_R {
        SAIELSR92_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr93(&self) -> SAIELSR93_R {
        SAIELSR93_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr94(&self) -> SAIELSR94_R {
        SAIELSR94_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    pub fn saielsr95(&self) -> SAIELSR95_R {
        SAIELSR95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr64(&mut self) -> SAIELSR64_W<0> {
        SAIELSR64_W::new(self)
    }
    #[doc = "Bit 1 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr65(&mut self) -> SAIELSR65_W<1> {
        SAIELSR65_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr66(&mut self) -> SAIELSR66_W<2> {
        SAIELSR66_W::new(self)
    }
    #[doc = "Bit 3 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr67(&mut self) -> SAIELSR67_W<3> {
        SAIELSR67_W::new(self)
    }
    #[doc = "Bit 4 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr68(&mut self) -> SAIELSR68_W<4> {
        SAIELSR68_W::new(self)
    }
    #[doc = "Bit 5 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr69(&mut self) -> SAIELSR69_W<5> {
        SAIELSR69_W::new(self)
    }
    #[doc = "Bit 6 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr70(&mut self) -> SAIELSR70_W<6> {
        SAIELSR70_W::new(self)
    }
    #[doc = "Bit 7 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr71(&mut self) -> SAIELSR71_W<7> {
        SAIELSR71_W::new(self)
    }
    #[doc = "Bit 8 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr72(&mut self) -> SAIELSR72_W<8> {
        SAIELSR72_W::new(self)
    }
    #[doc = "Bit 9 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr73(&mut self) -> SAIELSR73_W<9> {
        SAIELSR73_W::new(self)
    }
    #[doc = "Bit 10 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr74(&mut self) -> SAIELSR74_W<10> {
        SAIELSR74_W::new(self)
    }
    #[doc = "Bit 11 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr75(&mut self) -> SAIELSR75_W<11> {
        SAIELSR75_W::new(self)
    }
    #[doc = "Bit 12 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr76(&mut self) -> SAIELSR76_W<12> {
        SAIELSR76_W::new(self)
    }
    #[doc = "Bit 13 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr77(&mut self) -> SAIELSR77_W<13> {
        SAIELSR77_W::new(self)
    }
    #[doc = "Bit 14 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr78(&mut self) -> SAIELSR78_W<14> {
        SAIELSR78_W::new(self)
    }
    #[doc = "Bit 15 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr79(&mut self) -> SAIELSR79_W<15> {
        SAIELSR79_W::new(self)
    }
    #[doc = "Bit 16 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr80(&mut self) -> SAIELSR80_W<16> {
        SAIELSR80_W::new(self)
    }
    #[doc = "Bit 17 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr81(&mut self) -> SAIELSR81_W<17> {
        SAIELSR81_W::new(self)
    }
    #[doc = "Bit 18 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr82(&mut self) -> SAIELSR82_W<18> {
        SAIELSR82_W::new(self)
    }
    #[doc = "Bit 19 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr83(&mut self) -> SAIELSR83_W<19> {
        SAIELSR83_W::new(self)
    }
    #[doc = "Bit 20 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr84(&mut self) -> SAIELSR84_W<20> {
        SAIELSR84_W::new(self)
    }
    #[doc = "Bit 21 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr85(&mut self) -> SAIELSR85_W<21> {
        SAIELSR85_W::new(self)
    }
    #[doc = "Bit 22 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr86(&mut self) -> SAIELSR86_W<22> {
        SAIELSR86_W::new(self)
    }
    #[doc = "Bit 23 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr87(&mut self) -> SAIELSR87_W<23> {
        SAIELSR87_W::new(self)
    }
    #[doc = "Bit 24 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr88(&mut self) -> SAIELSR88_W<24> {
        SAIELSR88_W::new(self)
    }
    #[doc = "Bit 25 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr89(&mut self) -> SAIELSR89_W<25> {
        SAIELSR89_W::new(self)
    }
    #[doc = "Bit 26 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr90(&mut self) -> SAIELSR90_W<26> {
        SAIELSR90_W::new(self)
    }
    #[doc = "Bit 27 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr91(&mut self) -> SAIELSR91_W<27> {
        SAIELSR91_W::new(self)
    }
    #[doc = "Bit 28 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr92(&mut self) -> SAIELSR92_W<28> {
        SAIELSR92_W::new(self)
    }
    #[doc = "Bit 29 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr93(&mut self) -> SAIELSR93_W<29> {
        SAIELSR93_W::new(self)
    }
    #[doc = "Bit 30 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr94(&mut self) -> SAIELSR94_W<30> {
        SAIELSR94_W::new(self)
    }
    #[doc = "Bit 31 - Security attributes of registers for IELSR95 to IELSR64"]
    #[inline(always)]
    #[must_use]
    pub fn saielsr95(&mut self) -> SAIELSR95_W<31> {
        SAIELSR95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register I\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusari](index.html) module"]
pub struct ICUSARI_SPEC;
impl crate::RegisterSpec for ICUSARI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusari::R](R) reader structure"]
impl crate::Readable for ICUSARI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusari::W](W) writer structure"]
impl crate::Writable for ICUSARI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARI to value 0xffff_ffff"]
impl crate::Resettable for ICUSARI_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
