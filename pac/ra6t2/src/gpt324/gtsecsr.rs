#[doc = "Register `GTSECSR` reader"]
pub struct R(crate::R<GTSECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTSECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTSECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTSECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTSECSR` writer"]
pub struct W(crate::W<GTSECSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTSECSR_SPEC>;
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
impl From<crate::W<GTSECSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTSECSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECSEL0` reader - Channel 0 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL0_R = crate::BitReader<SECSEL0_A>;
#[doc = "Channel 0 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL0_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL0_A {
        match self.bits {
            false => SECSEL0_A::_0,
            true => SECSEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL0_A::_1
    }
}
#[doc = "Field `SECSEL0` writer - Channel 0 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL0_A, O>;
impl<'a, const O: u8> SECSEL0_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL0_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL0_A::_1)
    }
}
#[doc = "Field `SECSEL1` reader - Channel 1 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL1_R = crate::BitReader<SECSEL1_A>;
#[doc = "Channel 1 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL1_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL1_A {
        match self.bits {
            false => SECSEL1_A::_0,
            true => SECSEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL1_A::_1
    }
}
#[doc = "Field `SECSEL1` writer - Channel 1 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL1_A, O>;
impl<'a, const O: u8> SECSEL1_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL1_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL1_A::_1)
    }
}
#[doc = "Field `SECSEL2` reader - Channel 2 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL2_R = crate::BitReader<SECSEL2_A>;
#[doc = "Channel 2 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL2_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL2_A {
        match self.bits {
            false => SECSEL2_A::_0,
            true => SECSEL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL2_A::_1
    }
}
#[doc = "Field `SECSEL2` writer - Channel 2 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL2_A, O>;
impl<'a, const O: u8> SECSEL2_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL2_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL2_A::_1)
    }
}
#[doc = "Field `SECSEL3` reader - Channel 3 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL3_R = crate::BitReader<SECSEL3_A>;
#[doc = "Channel 3 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL3_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL3_A {
        match self.bits {
            false => SECSEL3_A::_0,
            true => SECSEL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL3_A::_1
    }
}
#[doc = "Field `SECSEL3` writer - Channel 3 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL3_A, O>;
impl<'a, const O: u8> SECSEL3_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL3_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL3_A::_1)
    }
}
#[doc = "Field `SECSEL4` reader - Channel 4 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL4_R = crate::BitReader<SECSEL4_A>;
#[doc = "Channel 4 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL4_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL4_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL4_A {
        match self.bits {
            false => SECSEL4_A::_0,
            true => SECSEL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL4_A::_1
    }
}
#[doc = "Field `SECSEL4` writer - Channel 4 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL4_A, O>;
impl<'a, const O: u8> SECSEL4_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL4_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL4_A::_1)
    }
}
#[doc = "Field `SECSEL5` reader - Channel 5 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL5_R = crate::BitReader<SECSEL5_A>;
#[doc = "Channel 5 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL5_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL5_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL5_A {
        match self.bits {
            false => SECSEL5_A::_0,
            true => SECSEL5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL5_A::_1
    }
}
#[doc = "Field `SECSEL5` writer - Channel 5 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL5_A, O>;
impl<'a, const O: u8> SECSEL5_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL5_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL5_A::_1)
    }
}
#[doc = "Field `SECSEL6` reader - Channel 6 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL6_R = crate::BitReader<SECSEL6_A>;
#[doc = "Channel 6 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL6_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL6_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL6_A {
        match self.bits {
            false => SECSEL6_A::_0,
            true => SECSEL6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL6_A::_1
    }
}
#[doc = "Field `SECSEL6` writer - Channel 6 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL6_A, O>;
impl<'a, const O: u8> SECSEL6_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL6_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL6_A::_1)
    }
}
#[doc = "Field `SECSEL7` reader - Channel 7 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL7_R = crate::BitReader<SECSEL7_A>;
#[doc = "Channel 7 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL7_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL7_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL7_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL7_A {
        match self.bits {
            false => SECSEL7_A::_0,
            true => SECSEL7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL7_A::_1
    }
}
#[doc = "Field `SECSEL7` writer - Channel 7 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL7_A, O>;
impl<'a, const O: u8> SECSEL7_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL7_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL7_A::_1)
    }
}
#[doc = "Field `SECSEL8` reader - Channel 8 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL8_R = crate::BitReader<SECSEL8_A>;
#[doc = "Channel 8 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL8_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL8_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL8_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL8_A {
        match self.bits {
            false => SECSEL8_A::_0,
            true => SECSEL8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL8_A::_1
    }
}
#[doc = "Field `SECSEL8` writer - Channel 8 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL8_A, O>;
impl<'a, const O: u8> SECSEL8_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL8_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL8_A::_1)
    }
}
#[doc = "Field `SECSEL9` reader - Channel 9 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL9_R = crate::BitReader<SECSEL9_A>;
#[doc = "Channel 9 Operation Enable Bit Simultaneous Control Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSEL9_A {
    #[doc = "0: Disable simultaneous control"]
    _0 = 0,
    #[doc = "1: Enable simultaneous control"]
    _1 = 1,
}
impl From<SECSEL9_A> for bool {
    #[inline(always)]
    fn from(variant: SECSEL9_A) -> Self {
        variant as u8 != 0
    }
}
impl SECSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSEL9_A {
        match self.bits {
            false => SECSEL9_A::_0,
            true => SECSEL9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECSEL9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECSEL9_A::_1
    }
}
#[doc = "Field `SECSEL9` writer - Channel 9 Operation Enable Bit Simultaneous Control Channel Select"]
pub type SECSEL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECSR_SPEC, SECSEL9_A, O>;
impl<'a, const O: u8> SECSEL9_W<'a, O> {
    #[doc = "Disable simultaneous control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECSEL9_A::_0)
    }
    #[doc = "Enable simultaneous control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECSEL9_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel0(&self) -> SECSEL0_R {
        SECSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel1(&self) -> SECSEL1_R {
        SECSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel2(&self) -> SECSEL2_R {
        SECSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel3(&self) -> SECSEL3_R {
        SECSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel4(&self) -> SECSEL4_R {
        SECSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel5(&self) -> SECSEL5_R {
        SECSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel6(&self) -> SECSEL6_R {
        SECSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel7(&self) -> SECSEL7_R {
        SECSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel8(&self) -> SECSEL8_R {
        SECSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel9(&self) -> SECSEL9_R {
        SECSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel0(&mut self) -> SECSEL0_W<0> {
        SECSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel1(&mut self) -> SECSEL1_W<1> {
        SECSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel2(&mut self) -> SECSEL2_W<2> {
        SECSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel3(&mut self) -> SECSEL3_W<3> {
        SECSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel4(&mut self) -> SECSEL4_W<4> {
        SECSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel5(&mut self) -> SECSEL5_W<5> {
        SECSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel6(&mut self) -> SECSEL6_W<6> {
        SECSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel7(&mut self) -> SECSEL7_W<7> {
        SECSEL7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel8(&mut self) -> SECSEL8_W<8> {
        SECSEL8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn secsel9(&mut self) -> SECSEL9_W<9> {
        SECSEL9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Operation Enable Bit Simultaneous Control Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtsecsr](index.html) module"]
pub struct GTSECSR_SPEC;
impl crate::RegisterSpec for GTSECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtsecsr::R](R) reader structure"]
impl crate::Readable for GTSECSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtsecsr::W](W) writer structure"]
impl crate::Writable for GTSECSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSECSR to value 0"]
impl crate::Resettable for GTSECSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
