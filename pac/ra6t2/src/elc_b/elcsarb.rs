#[doc = "Register `ELCSARB` reader"]
pub struct R(crate::R<ELCSARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELCSARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELCSARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELCSARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELCSARB` writer"]
pub struct W(crate::W<ELCSARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELCSARB_SPEC>;
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
impl From<crate::W<ELCSARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELCSARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELSR0` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR0_R = crate::BitReader<ELSR0_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR0_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR0_A {
        match self.bits {
            false => ELSR0_A::_0,
            true => ELSR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR0_A::_1
    }
}
#[doc = "Field `ELSR0` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR0_A, O>;
impl<'a, const O: u8> ELSR0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR0_A::_1)
    }
}
#[doc = "Field `ELSR1` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR1_R = crate::BitReader<ELSR1_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR1_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR1_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR1_A {
        match self.bits {
            false => ELSR1_A::_0,
            true => ELSR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR1_A::_1
    }
}
#[doc = "Field `ELSR1` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR1_A, O>;
impl<'a, const O: u8> ELSR1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR1_A::_1)
    }
}
#[doc = "Field `ELSR2` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR2_R = crate::BitReader<ELSR2_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR2_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR2_A {
        match self.bits {
            false => ELSR2_A::_0,
            true => ELSR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR2_A::_1
    }
}
#[doc = "Field `ELSR2` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR2_A, O>;
impl<'a, const O: u8> ELSR2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR2_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR2_A::_1)
    }
}
#[doc = "Field `ELSR3` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR3_R = crate::BitReader<ELSR3_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR3_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR3_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR3_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR3_A {
        match self.bits {
            false => ELSR3_A::_0,
            true => ELSR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR3_A::_1
    }
}
#[doc = "Field `ELSR3` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR3_A, O>;
impl<'a, const O: u8> ELSR3_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR3_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR3_A::_1)
    }
}
#[doc = "Field `ELSR4` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR4_R = crate::BitReader<ELSR4_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR4_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR4_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR4_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR4_A {
        match self.bits {
            false => ELSR4_A::_0,
            true => ELSR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR4_A::_1
    }
}
#[doc = "Field `ELSR4` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR4_A, O>;
impl<'a, const O: u8> ELSR4_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR4_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR4_A::_1)
    }
}
#[doc = "Field `ELSR5` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR5_R = crate::BitReader<ELSR5_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR5_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR5_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR5_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR5_A {
        match self.bits {
            false => ELSR5_A::_0,
            true => ELSR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR5_A::_1
    }
}
#[doc = "Field `ELSR5` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR5_A, O>;
impl<'a, const O: u8> ELSR5_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR5_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR5_A::_1)
    }
}
#[doc = "Field `ELSR6` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR6_R = crate::BitReader<ELSR6_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR6_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR6_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR6_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR6_A {
        match self.bits {
            false => ELSR6_A::_0,
            true => ELSR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR6_A::_1
    }
}
#[doc = "Field `ELSR6` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR6_A, O>;
impl<'a, const O: u8> ELSR6_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR6_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR6_A::_1)
    }
}
#[doc = "Field `ELSR7` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR7_R = crate::BitReader<ELSR7_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR7_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR7_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR7_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR7_A {
        match self.bits {
            false => ELSR7_A::_0,
            true => ELSR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR7_A::_1
    }
}
#[doc = "Field `ELSR7` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR7_A, O>;
impl<'a, const O: u8> ELSR7_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR7_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR7_A::_1)
    }
}
#[doc = "Field `ELSR12` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR12_R = crate::BitReader<ELSR12_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR12_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR12_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR12_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR12_A {
        match self.bits {
            false => ELSR12_A::_0,
            true => ELSR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR12_A::_1
    }
}
#[doc = "Field `ELSR12` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR12_A, O>;
impl<'a, const O: u8> ELSR12_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR12_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR12_A::_1)
    }
}
#[doc = "Field `ELSR13` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR13_R = crate::BitReader<ELSR13_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR13_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR13_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR13_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR13_A {
        match self.bits {
            false => ELSR13_A::_0,
            true => ELSR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR13_A::_1
    }
}
#[doc = "Field `ELSR13` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR13_A, O>;
impl<'a, const O: u8> ELSR13_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR13_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR13_A::_1)
    }
}
#[doc = "Field `ELSR14` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR14_R = crate::BitReader<ELSR14_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR14_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR14_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR14_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR14_A {
        match self.bits {
            false => ELSR14_A::_0,
            true => ELSR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR14_A::_1
    }
}
#[doc = "Field `ELSR14` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR14_A, O>;
impl<'a, const O: u8> ELSR14_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR14_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR14_A::_1)
    }
}
#[doc = "Field `ELSR15` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR15_R = crate::BitReader<ELSR15_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR15_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR15_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR15_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR15_A {
        match self.bits {
            false => ELSR15_A::_0,
            true => ELSR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR15_A::_1
    }
}
#[doc = "Field `ELSR15` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR15_A, O>;
impl<'a, const O: u8> ELSR15_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR15_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR15_A::_1)
    }
}
#[doc = "Field `ELSR16` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR16_R = crate::BitReader<ELSR16_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR16_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR16_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR16_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR16_A {
        match self.bits {
            false => ELSR16_A::_0,
            true => ELSR16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR16_A::_1
    }
}
#[doc = "Field `ELSR16` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR16_A, O>;
impl<'a, const O: u8> ELSR16_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR16_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR16_A::_1)
    }
}
#[doc = "Field `ELSR17` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR17_R = crate::BitReader<ELSR17_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR17_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR17_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR17_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR17_A {
        match self.bits {
            false => ELSR17_A::_0,
            true => ELSR17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR17_A::_1
    }
}
#[doc = "Field `ELSR17` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR17_A, O>;
impl<'a, const O: u8> ELSR17_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR17_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR17_A::_1)
    }
}
#[doc = "Field `ELSR19` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR19_R = crate::BitReader<ELSR19_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR19_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR19_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR19_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR19_A {
        match self.bits {
            false => ELSR19_A::_0,
            true => ELSR19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR19_A::_1
    }
}
#[doc = "Field `ELSR19` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR19_A, O>;
impl<'a, const O: u8> ELSR19_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR19_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR19_A::_1)
    }
}
#[doc = "Field `ELSR20` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR20_R = crate::BitReader<ELSR20_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR20_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR20_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR20_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR20_A {
        match self.bits {
            false => ELSR20_A::_0,
            true => ELSR20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR20_A::_1
    }
}
#[doc = "Field `ELSR20` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR20_A, O>;
impl<'a, const O: u8> ELSR20_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR20_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR20_A::_1)
    }
}
#[doc = "Field `ELSR21` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR21_R = crate::BitReader<ELSR21_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR21_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR21_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR21_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR21_A {
        match self.bits {
            false => ELSR21_A::_0,
            true => ELSR21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR21_A::_1
    }
}
#[doc = "Field `ELSR21` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR21_A, O>;
impl<'a, const O: u8> ELSR21_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR21_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR21_A::_1)
    }
}
#[doc = "Field `ELSR22` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR22_R = crate::BitReader<ELSR22_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR22_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR22_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR22_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR22_A {
        match self.bits {
            false => ELSR22_A::_0,
            true => ELSR22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR22_A::_1
    }
}
#[doc = "Field `ELSR22` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR22_A, O>;
impl<'a, const O: u8> ELSR22_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR22_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR22_A::_1)
    }
}
#[doc = "Field `ELSR23` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR23_R = crate::BitReader<ELSR23_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR23_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR23_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR23_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR23_A {
        match self.bits {
            false => ELSR23_A::_0,
            true => ELSR23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR23_A::_1
    }
}
#[doc = "Field `ELSR23` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR23_A, O>;
impl<'a, const O: u8> ELSR23_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR23_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR23_A::_1)
    }
}
#[doc = "Field `ELSR24` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR24_R = crate::BitReader<ELSR24_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR24_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR24_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR24_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR24_A {
        match self.bits {
            false => ELSR24_A::_0,
            true => ELSR24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR24_A::_1
    }
}
#[doc = "Field `ELSR24` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR24_A, O>;
impl<'a, const O: u8> ELSR24_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR24_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR24_A::_1)
    }
}
#[doc = "Field `ELSR28` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR28_R = crate::BitReader<ELSR28_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR28_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR28_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR28_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR28_A {
        match self.bits {
            false => ELSR28_A::_0,
            true => ELSR28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR28_A::_1
    }
}
#[doc = "Field `ELSR28` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR28_A, O>;
impl<'a, const O: u8> ELSR28_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR28_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR28_A::_1)
    }
}
#[doc = "Field `ELSR29` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR29_R = crate::BitReader<ELSR29_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSR29_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR29_A> for bool {
    #[inline(always)]
    fn from(variant: ELSR29_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSR29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSR29_A {
        match self.bits {
            false => ELSR29_A::_0,
            true => ELSR29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR29_A::_1
    }
}
#[doc = "Field `ELSR29` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARB_SPEC, ELSR29_A, O>;
impl<'a, const O: u8> ELSR29_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR29_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR29_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr0(&self) -> ELSR0_R {
        ELSR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr1(&self) -> ELSR1_R {
        ELSR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr2(&self) -> ELSR2_R {
        ELSR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr3(&self) -> ELSR3_R {
        ELSR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr4(&self) -> ELSR4_R {
        ELSR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr5(&self) -> ELSR5_R {
        ELSR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr6(&self) -> ELSR6_R {
        ELSR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr7(&self) -> ELSR7_R {
        ELSR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr12(&self) -> ELSR12_R {
        ELSR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr13(&self) -> ELSR13_R {
        ELSR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr14(&self) -> ELSR14_R {
        ELSR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr15(&self) -> ELSR15_R {
        ELSR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr16(&self) -> ELSR16_R {
        ELSR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr17(&self) -> ELSR17_R {
        ELSR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr19(&self) -> ELSR19_R {
        ELSR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr20(&self) -> ELSR20_R {
        ELSR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr21(&self) -> ELSR21_R {
        ELSR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr22(&self) -> ELSR22_R {
        ELSR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr23(&self) -> ELSR23_R {
        ELSR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr24(&self) -> ELSR24_R {
        ELSR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr28(&self) -> ELSR28_R {
        ELSR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr29(&self) -> ELSR29_R {
        ELSR29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr0(&mut self) -> ELSR0_W<0> {
        ELSR0_W::new(self)
    }
    #[doc = "Bit 1 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr1(&mut self) -> ELSR1_W<1> {
        ELSR1_W::new(self)
    }
    #[doc = "Bit 2 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr2(&mut self) -> ELSR2_W<2> {
        ELSR2_W::new(self)
    }
    #[doc = "Bit 3 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr3(&mut self) -> ELSR3_W<3> {
        ELSR3_W::new(self)
    }
    #[doc = "Bit 4 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr4(&mut self) -> ELSR4_W<4> {
        ELSR4_W::new(self)
    }
    #[doc = "Bit 5 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr5(&mut self) -> ELSR5_W<5> {
        ELSR5_W::new(self)
    }
    #[doc = "Bit 6 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr6(&mut self) -> ELSR6_W<6> {
        ELSR6_W::new(self)
    }
    #[doc = "Bit 7 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr7(&mut self) -> ELSR7_W<7> {
        ELSR7_W::new(self)
    }
    #[doc = "Bit 12 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr12(&mut self) -> ELSR12_W<12> {
        ELSR12_W::new(self)
    }
    #[doc = "Bit 13 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr13(&mut self) -> ELSR13_W<13> {
        ELSR13_W::new(self)
    }
    #[doc = "Bit 14 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr14(&mut self) -> ELSR14_W<14> {
        ELSR14_W::new(self)
    }
    #[doc = "Bit 15 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr15(&mut self) -> ELSR15_W<15> {
        ELSR15_W::new(self)
    }
    #[doc = "Bit 16 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr16(&mut self) -> ELSR16_W<16> {
        ELSR16_W::new(self)
    }
    #[doc = "Bit 17 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr17(&mut self) -> ELSR17_W<17> {
        ELSR17_W::new(self)
    }
    #[doc = "Bit 19 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr19(&mut self) -> ELSR19_W<19> {
        ELSR19_W::new(self)
    }
    #[doc = "Bit 20 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr20(&mut self) -> ELSR20_W<20> {
        ELSR20_W::new(self)
    }
    #[doc = "Bit 21 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr21(&mut self) -> ELSR21_W<21> {
        ELSR21_W::new(self)
    }
    #[doc = "Bit 22 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr22(&mut self) -> ELSR22_W<22> {
        ELSR22_W::new(self)
    }
    #[doc = "Bit 23 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr23(&mut self) -> ELSR23_W<23> {
        ELSR23_W::new(self)
    }
    #[doc = "Bit 24 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr24(&mut self) -> ELSR24_W<24> {
        ELSR24_W::new(self)
    }
    #[doc = "Bit 28 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr28(&mut self) -> ELSR28_W<28> {
        ELSR28_W::new(self)
    }
    #[doc = "Bit 29 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr29(&mut self) -> ELSR29_W<29> {
        ELSR29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Link Controller Security Attribution Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elcsarb](index.html) module"]
pub struct ELCSARB_SPEC;
impl crate::RegisterSpec for ELCSARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elcsarb::R](R) reader structure"]
impl crate::Readable for ELCSARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elcsarb::W](W) writer structure"]
impl crate::Writable for ELCSARB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELCSARB to value 0xffff_ffff"]
impl crate::Resettable for ELCSARB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
