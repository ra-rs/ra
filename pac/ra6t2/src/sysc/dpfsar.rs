#[doc = "Register `DPFSAR` reader"]
pub struct R(crate::R<DPFSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPFSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPFSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPFSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPFSAR` writer"]
pub struct W(crate::W<DPFSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPFSAR_SPEC>;
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
impl From<crate::W<DPFSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPFSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPFSA0` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA0_R = crate::BitReader<DPFSA0_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA0_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA0_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA0_A {
        match self.bits {
            false => DPFSA0_A::_0,
            true => DPFSA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA0_A::_1
    }
}
#[doc = "Field `DPFSA0` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA0_A, O>;
impl<'a, const O: u8> DPFSA0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA0_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA0_A::_1)
    }
}
#[doc = "Field `DPFSA1` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA1_R = crate::BitReader<DPFSA1_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA1_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA1_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA1_A {
        match self.bits {
            false => DPFSA1_A::_0,
            true => DPFSA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA1_A::_1
    }
}
#[doc = "Field `DPFSA1` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA1_A, O>;
impl<'a, const O: u8> DPFSA1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA1_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA1_A::_1)
    }
}
#[doc = "Field `DPFSA2` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA2_R = crate::BitReader<DPFSA2_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA2_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA2_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA2_A {
        match self.bits {
            false => DPFSA2_A::_0,
            true => DPFSA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA2_A::_1
    }
}
#[doc = "Field `DPFSA2` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA2_A, O>;
impl<'a, const O: u8> DPFSA2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA2_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA2_A::_1)
    }
}
#[doc = "Field `DPFSA3` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA3_R = crate::BitReader<DPFSA3_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA3_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA3_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA3_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA3_A {
        match self.bits {
            false => DPFSA3_A::_0,
            true => DPFSA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA3_A::_1
    }
}
#[doc = "Field `DPFSA3` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA3_A, O>;
impl<'a, const O: u8> DPFSA3_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA3_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA3_A::_1)
    }
}
#[doc = "Field `DPFSA4` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA4_R = crate::BitReader<DPFSA4_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA4_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA4_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA4_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA4_A {
        match self.bits {
            false => DPFSA4_A::_0,
            true => DPFSA4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA4_A::_1
    }
}
#[doc = "Field `DPFSA4` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA4_A, O>;
impl<'a, const O: u8> DPFSA4_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA4_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA4_A::_1)
    }
}
#[doc = "Field `DPFSA5` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA5_R = crate::BitReader<DPFSA5_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA5_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA5_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA5_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA5_A {
        match self.bits {
            false => DPFSA5_A::_0,
            true => DPFSA5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA5_A::_1
    }
}
#[doc = "Field `DPFSA5` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA5_A, O>;
impl<'a, const O: u8> DPFSA5_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA5_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA5_A::_1)
    }
}
#[doc = "Field `DPFSA6` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA6_R = crate::BitReader<DPFSA6_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA6_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA6_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA6_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA6_A {
        match self.bits {
            false => DPFSA6_A::_0,
            true => DPFSA6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA6_A::_1
    }
}
#[doc = "Field `DPFSA6` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA6_A, O>;
impl<'a, const O: u8> DPFSA6_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA6_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA6_A::_1)
    }
}
#[doc = "Field `DPFSA7` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA7_R = crate::BitReader<DPFSA7_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA7_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA7_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA7_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA7_A {
        match self.bits {
            false => DPFSA7_A::_0,
            true => DPFSA7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA7_A::_1
    }
}
#[doc = "Field `DPFSA7` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
pub type DPFSA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA7_A, O>;
impl<'a, const O: u8> DPFSA7_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA7_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA7_A::_1)
    }
}
#[doc = "Field `DPFSA8` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA8_R = crate::BitReader<DPFSA8_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA8_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA8_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA8_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA8_A {
        match self.bits {
            false => DPFSA8_A::_0,
            true => DPFSA8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA8_A::_1
    }
}
#[doc = "Field `DPFSA8` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA8_A, O>;
impl<'a, const O: u8> DPFSA8_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA8_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA8_A::_1)
    }
}
#[doc = "Field `DPFSA9` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA9_R = crate::BitReader<DPFSA9_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA9_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA9_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA9_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA9_A {
        match self.bits {
            false => DPFSA9_A::_0,
            true => DPFSA9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA9_A::_1
    }
}
#[doc = "Field `DPFSA9` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA9_A, O>;
impl<'a, const O: u8> DPFSA9_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA9_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA9_A::_1)
    }
}
#[doc = "Field `DPFSA10` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA10_R = crate::BitReader<DPFSA10_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA10_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA10_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA10_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA10_A {
        match self.bits {
            false => DPFSA10_A::_0,
            true => DPFSA10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA10_A::_1
    }
}
#[doc = "Field `DPFSA10` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA10_A, O>;
impl<'a, const O: u8> DPFSA10_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA10_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA10_A::_1)
    }
}
#[doc = "Field `DPFSA11` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA11_R = crate::BitReader<DPFSA11_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA11_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA11_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA11_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA11_A {
        match self.bits {
            false => DPFSA11_A::_0,
            true => DPFSA11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA11_A::_1
    }
}
#[doc = "Field `DPFSA11` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA11_A, O>;
impl<'a, const O: u8> DPFSA11_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA11_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA11_A::_1)
    }
}
#[doc = "Field `DPFSA12` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA12_R = crate::BitReader<DPFSA12_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA12_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA12_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA12_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA12_A {
        match self.bits {
            false => DPFSA12_A::_0,
            true => DPFSA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA12_A::_1
    }
}
#[doc = "Field `DPFSA12` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA12_A, O>;
impl<'a, const O: u8> DPFSA12_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA12_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA12_A::_1)
    }
}
#[doc = "Field `DPFSA13` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA13_R = crate::BitReader<DPFSA13_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA13_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA13_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA13_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA13_A {
        match self.bits {
            false => DPFSA13_A::_0,
            true => DPFSA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA13_A::_1
    }
}
#[doc = "Field `DPFSA13` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA13_A, O>;
impl<'a, const O: u8> DPFSA13_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA13_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA13_A::_1)
    }
}
#[doc = "Field `DPFSA14` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA14_R = crate::BitReader<DPFSA14_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA14_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA14_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA14_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA14_A {
        match self.bits {
            false => DPFSA14_A::_0,
            true => DPFSA14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA14_A::_1
    }
}
#[doc = "Field `DPFSA14` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA14_A, O>;
impl<'a, const O: u8> DPFSA14_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA14_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA14_A::_1)
    }
}
#[doc = "Field `DPFSA15` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA15_R = crate::BitReader<DPFSA15_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA15_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA15_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA15_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA15_A {
        match self.bits {
            false => DPFSA15_A::_0,
            true => DPFSA15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA15_A::_1
    }
}
#[doc = "Field `DPFSA15` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
pub type DPFSA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA15_A, O>;
impl<'a, const O: u8> DPFSA15_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA15_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA15_A::_1)
    }
}
#[doc = "Field `DPFSA16` reader - Deep Software Standby Interrupt Factor Security Attribute bit 16"]
pub type DPFSA16_R = crate::BitReader<DPFSA16_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA16_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA16_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA16_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA16_A {
        match self.bits {
            false => DPFSA16_A::_0,
            true => DPFSA16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA16_A::_1
    }
}
#[doc = "Field `DPFSA16` writer - Deep Software Standby Interrupt Factor Security Attribute bit 16"]
pub type DPFSA16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA16_A, O>;
impl<'a, const O: u8> DPFSA16_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA16_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA16_A::_1)
    }
}
#[doc = "Field `DPFSA17` reader - Deep Software Standby Interrupt Factor Security Attribute bit 17"]
pub type DPFSA17_R = crate::BitReader<DPFSA17_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA17_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA17_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA17_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA17_A {
        match self.bits {
            false => DPFSA17_A::_0,
            true => DPFSA17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA17_A::_1
    }
}
#[doc = "Field `DPFSA17` writer - Deep Software Standby Interrupt Factor Security Attribute bit 17"]
pub type DPFSA17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA17_A, O>;
impl<'a, const O: u8> DPFSA17_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA17_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA17_A::_1)
    }
}
#[doc = "Field `DPFSA20` reader - Deep Software Standby Interrupt Factor Security Attribute bit 20"]
pub type DPFSA20_R = crate::BitReader<DPFSA20_A>;
#[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPFSA20_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<DPFSA20_A> for bool {
    #[inline(always)]
    fn from(variant: DPFSA20_A) -> Self {
        variant as u8 != 0
    }
}
impl DPFSA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPFSA20_A {
        match self.bits {
            false => DPFSA20_A::_0,
            true => DPFSA20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPFSA20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPFSA20_A::_1
    }
}
#[doc = "Field `DPFSA20` writer - Deep Software Standby Interrupt Factor Security Attribute bit 20"]
pub type DPFSA20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPFSAR_SPEC, DPFSA20_A, O>;
impl<'a, const O: u8> DPFSA20_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPFSA20_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPFSA20_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa0(&self) -> DPFSA0_R {
        DPFSA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa1(&self) -> DPFSA1_R {
        DPFSA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa2(&self) -> DPFSA2_R {
        DPFSA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa3(&self) -> DPFSA3_R {
        DPFSA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa4(&self) -> DPFSA4_R {
        DPFSA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa5(&self) -> DPFSA5_R {
        DPFSA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa6(&self) -> DPFSA6_R {
        DPFSA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    pub fn dpfsa7(&self) -> DPFSA7_R {
        DPFSA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa8(&self) -> DPFSA8_R {
        DPFSA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa9(&self) -> DPFSA9_R {
        DPFSA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa10(&self) -> DPFSA10_R {
        DPFSA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa11(&self) -> DPFSA11_R {
        DPFSA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa12(&self) -> DPFSA12_R {
        DPFSA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa13(&self) -> DPFSA13_R {
        DPFSA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa14(&self) -> DPFSA14_R {
        DPFSA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    pub fn dpfsa15(&self) -> DPFSA15_R {
        DPFSA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Deep Software Standby Interrupt Factor Security Attribute bit 16"]
    #[inline(always)]
    pub fn dpfsa16(&self) -> DPFSA16_R {
        DPFSA16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Deep Software Standby Interrupt Factor Security Attribute bit 17"]
    #[inline(always)]
    pub fn dpfsa17(&self) -> DPFSA17_R {
        DPFSA17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Deep Software Standby Interrupt Factor Security Attribute bit 20"]
    #[inline(always)]
    pub fn dpfsa20(&self) -> DPFSA20_R {
        DPFSA20_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa0(&mut self) -> DPFSA0_W<0> {
        DPFSA0_W::new(self)
    }
    #[doc = "Bit 1 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa1(&mut self) -> DPFSA1_W<1> {
        DPFSA1_W::new(self)
    }
    #[doc = "Bit 2 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa2(&mut self) -> DPFSA2_W<2> {
        DPFSA2_W::new(self)
    }
    #[doc = "Bit 3 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa3(&mut self) -> DPFSA3_W<3> {
        DPFSA3_W::new(self)
    }
    #[doc = "Bit 4 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa4(&mut self) -> DPFSA4_W<4> {
        DPFSA4_W::new(self)
    }
    #[doc = "Bit 5 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa5(&mut self) -> DPFSA5_W<5> {
        DPFSA5_W::new(self)
    }
    #[doc = "Bit 6 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa6(&mut self) -> DPFSA6_W<6> {
        DPFSA6_W::new(self)
    }
    #[doc = "Bit 7 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa7(&mut self) -> DPFSA7_W<7> {
        DPFSA7_W::new(self)
    }
    #[doc = "Bit 8 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa8(&mut self) -> DPFSA8_W<8> {
        DPFSA8_W::new(self)
    }
    #[doc = "Bit 9 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa9(&mut self) -> DPFSA9_W<9> {
        DPFSA9_W::new(self)
    }
    #[doc = "Bit 10 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa10(&mut self) -> DPFSA10_W<10> {
        DPFSA10_W::new(self)
    }
    #[doc = "Bit 11 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa11(&mut self) -> DPFSA11_W<11> {
        DPFSA11_W::new(self)
    }
    #[doc = "Bit 12 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa12(&mut self) -> DPFSA12_W<12> {
        DPFSA12_W::new(self)
    }
    #[doc = "Bit 13 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa13(&mut self) -> DPFSA13_W<13> {
        DPFSA13_W::new(self)
    }
    #[doc = "Bit 14 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa14(&mut self) -> DPFSA14_W<14> {
        DPFSA14_W::new(self)
    }
    #[doc = "Bit 15 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa15(&mut self) -> DPFSA15_W<15> {
        DPFSA15_W::new(self)
    }
    #[doc = "Bit 16 - Deep Software Standby Interrupt Factor Security Attribute bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa16(&mut self) -> DPFSA16_W<16> {
        DPFSA16_W::new(self)
    }
    #[doc = "Bit 17 - Deep Software Standby Interrupt Factor Security Attribute bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa17(&mut self) -> DPFSA17_W<17> {
        DPFSA17_W::new(self)
    }
    #[doc = "Bit 20 - Deep Software Standby Interrupt Factor Security Attribute bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn dpfsa20(&mut self) -> DPFSA20_W<20> {
        DPFSA20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Software Standby Interrupt Factor Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpfsar](index.html) module"]
pub struct DPFSAR_SPEC;
impl crate::RegisterSpec for DPFSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpfsar::R](R) reader structure"]
impl crate::Readable for DPFSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpfsar::W](W) writer structure"]
impl crate::Writable for DPFSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPFSAR to value 0xffff_ffff"]
impl crate::Resettable for DPFSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
