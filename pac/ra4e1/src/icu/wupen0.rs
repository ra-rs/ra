#[doc = "Register `WUPEN0` reader"]
pub struct R(crate::R<WUPEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUPEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUPEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUPEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUPEN0` writer"]
pub struct W(crate::W<WUPEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUPEN0_SPEC>;
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
impl From<crate::W<WUPEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUPEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQWUPEN0` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN0_R = crate::BitReader<IRQWUPEN0_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN0_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN0_A {
        match self.bits {
            false => IRQWUPEN0_A::_0,
            true => IRQWUPEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN0_A::_1
    }
}
#[doc = "Field `IRQWUPEN0` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN0_A, O>;
impl<'a, const O: u8> IRQWUPEN0_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN0_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN0_A::_1)
    }
}
#[doc = "Field `IRQWUPEN1` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN1_R = crate::BitReader<IRQWUPEN1_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN1_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN1_A {
        match self.bits {
            false => IRQWUPEN1_A::_0,
            true => IRQWUPEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN1_A::_1
    }
}
#[doc = "Field `IRQWUPEN1` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN1_A, O>;
impl<'a, const O: u8> IRQWUPEN1_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN1_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN1_A::_1)
    }
}
#[doc = "Field `IRQWUPEN2` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN2_R = crate::BitReader<IRQWUPEN2_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN2_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN2_A {
        match self.bits {
            false => IRQWUPEN2_A::_0,
            true => IRQWUPEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN2_A::_1
    }
}
#[doc = "Field `IRQWUPEN2` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN2_A, O>;
impl<'a, const O: u8> IRQWUPEN2_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN2_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN2_A::_1)
    }
}
#[doc = "Field `IRQWUPEN3` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN3_R = crate::BitReader<IRQWUPEN3_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN3_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN3_A {
        match self.bits {
            false => IRQWUPEN3_A::_0,
            true => IRQWUPEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN3_A::_1
    }
}
#[doc = "Field `IRQWUPEN3` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN3_A, O>;
impl<'a, const O: u8> IRQWUPEN3_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN3_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN3_A::_1)
    }
}
#[doc = "Field `IRQWUPEN4` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN4_R = crate::BitReader<IRQWUPEN4_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN4_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN4_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN4_A {
        match self.bits {
            false => IRQWUPEN4_A::_0,
            true => IRQWUPEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN4_A::_1
    }
}
#[doc = "Field `IRQWUPEN4` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN4_A, O>;
impl<'a, const O: u8> IRQWUPEN4_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN4_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN4_A::_1)
    }
}
#[doc = "Field `IRQWUPEN5` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN5_R = crate::BitReader<IRQWUPEN5_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN5_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN5_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN5_A {
        match self.bits {
            false => IRQWUPEN5_A::_0,
            true => IRQWUPEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN5_A::_1
    }
}
#[doc = "Field `IRQWUPEN5` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN5_A, O>;
impl<'a, const O: u8> IRQWUPEN5_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN5_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN5_A::_1)
    }
}
#[doc = "Field `IRQWUPEN6` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN6_R = crate::BitReader<IRQWUPEN6_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN6_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN6_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN6_A {
        match self.bits {
            false => IRQWUPEN6_A::_0,
            true => IRQWUPEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN6_A::_1
    }
}
#[doc = "Field `IRQWUPEN6` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN6_A, O>;
impl<'a, const O: u8> IRQWUPEN6_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN6_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN6_A::_1)
    }
}
#[doc = "Field `IRQWUPEN7` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN7_R = crate::BitReader<IRQWUPEN7_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN7_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN7_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN7_A {
        match self.bits {
            false => IRQWUPEN7_A::_0,
            true => IRQWUPEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN7_A::_1
    }
}
#[doc = "Field `IRQWUPEN7` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN7_A, O>;
impl<'a, const O: u8> IRQWUPEN7_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN7_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN7_A::_1)
    }
}
#[doc = "Field `IRQWUPEN8` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN8_R = crate::BitReader<IRQWUPEN8_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN8_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN8_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN8_A {
        match self.bits {
            false => IRQWUPEN8_A::_0,
            true => IRQWUPEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN8_A::_1
    }
}
#[doc = "Field `IRQWUPEN8` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN8_A, O>;
impl<'a, const O: u8> IRQWUPEN8_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN8_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN8_A::_1)
    }
}
#[doc = "Field `IRQWUPEN9` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN9_R = crate::BitReader<IRQWUPEN9_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN9_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN9_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN9_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN9_A {
        match self.bits {
            false => IRQWUPEN9_A::_0,
            true => IRQWUPEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN9_A::_1
    }
}
#[doc = "Field `IRQWUPEN9` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
pub type IRQWUPEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN9_A, O>;
impl<'a, const O: u8> IRQWUPEN9_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN9_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN9_A::_1)
    }
}
#[doc = "Field `IRQWUPEN13` reader - IRQ13 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IRQWUPEN13_R = crate::BitReader<IRQWUPEN13_A>;
#[doc = "IRQ13 Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN13_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQ13 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQ13 interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN13_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN13_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN13_A {
        match self.bits {
            false => IRQWUPEN13_A::_0,
            true => IRQWUPEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN13_A::_1
    }
}
#[doc = "Field `IRQWUPEN13` writer - IRQ13 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IRQWUPEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IRQWUPEN13_A, O>;
impl<'a, const O: u8> IRQWUPEN13_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQ13 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN13_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQ13 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN13_A::_1)
    }
}
#[doc = "Field `IWDTWUPEN` reader - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IWDTWUPEN_R = crate::BitReader<IWDTWUPEN_A>;
#[doc = "IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IWDT interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IWDT interrupt is enabled"]
    _1 = 1,
}
impl From<IWDTWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDTWUPEN_A {
        match self.bits {
            false => IWDTWUPEN_A::_0,
            true => IWDTWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTWUPEN_A::_1
    }
}
#[doc = "Field `IWDTWUPEN` writer - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IWDTWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IWDTWUPEN_A, O>;
impl<'a, const O: u8> IWDTWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IWDTWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IWDTWUPEN_A::_1)
    }
}
#[doc = "Field `LVD1WUPEN` reader - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD1WUPEN_R = crate::BitReader<LVD1WUPEN_A>;
#[doc = "LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1WUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by LVD1 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by LVD1 interrupt is enabled"]
    _1 = 1,
}
impl From<LVD1WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD1WUPEN_A {
        match self.bits {
            false => LVD1WUPEN_A::_0,
            true => LVD1WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1WUPEN_A::_1
    }
}
#[doc = "Field `LVD1WUPEN` writer - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD1WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, LVD1WUPEN_A, O>;
impl<'a, const O: u8> LVD1WUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1WUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1WUPEN_A::_1)
    }
}
#[doc = "Field `LVD2WUPEN` reader - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD2WUPEN_R = crate::BitReader<LVD2WUPEN_A>;
#[doc = "LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2WUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by LVD2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by LVD2 interrupt is enabled"]
    _1 = 1,
}
impl From<LVD2WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD2WUPEN_A {
        match self.bits {
            false => LVD2WUPEN_A::_0,
            true => LVD2WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2WUPEN_A::_1
    }
}
#[doc = "Field `LVD2WUPEN` writer - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD2WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, LVD2WUPEN_A, O>;
impl<'a, const O: u8> LVD2WUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2WUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2WUPEN_A::_1)
    }
}
#[doc = "Field `RTCALMWUPEN` reader - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type RTCALMWUPEN_R = crate::BitReader<RTCALMWUPEN_A>;
#[doc = "RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCALMWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by RTC alarm interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by RTC alarm interrupt is enabled"]
    _1 = 1,
}
impl From<RTCALMWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCALMWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCALMWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCALMWUPEN_A {
        match self.bits {
            false => RTCALMWUPEN_A::_0,
            true => RTCALMWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCALMWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCALMWUPEN_A::_1
    }
}
#[doc = "Field `RTCALMWUPEN` writer - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type RTCALMWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, RTCALMWUPEN_A, O>;
impl<'a, const O: u8> RTCALMWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by RTC alarm interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCALMWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by RTC alarm interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCALMWUPEN_A::_1)
    }
}
#[doc = "Field `RTCPRDWUPEN` reader - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type RTCPRDWUPEN_R = crate::BitReader<RTCPRDWUPEN_A>;
#[doc = "RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCPRDWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by RTC period interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by RTC period interrupt is enabled"]
    _1 = 1,
}
impl From<RTCPRDWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCPRDWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCPRDWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPRDWUPEN_A {
        match self.bits {
            false => RTCPRDWUPEN_A::_0,
            true => RTCPRDWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCPRDWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCPRDWUPEN_A::_1
    }
}
#[doc = "Field `RTCPRDWUPEN` writer - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type RTCPRDWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, RTCPRDWUPEN_A, O>;
impl<'a, const O: u8> RTCPRDWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by RTC period interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCPRDWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by RTC period interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCPRDWUPEN_A::_1)
    }
}
#[doc = "Field `USBFS0WUPEN` reader - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type USBFS0WUPEN_R = crate::BitReader<USBFS0WUPEN_A>;
#[doc = "USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFS0WUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by USBFS0 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by USBFS0 interrupt is enabled"]
    _1 = 1,
}
impl From<USBFS0WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBFS0WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBFS0WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBFS0WUPEN_A {
        match self.bits {
            false => USBFS0WUPEN_A::_0,
            true => USBFS0WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBFS0WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBFS0WUPEN_A::_1
    }
}
#[doc = "Field `USBFS0WUPEN` writer - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type USBFS0WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, USBFS0WUPEN_A, O>;
impl<'a, const O: u8> USBFS0WUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by USBFS0 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBFS0WUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by USBFS0 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBFS0WUPEN_A::_1)
    }
}
#[doc = "Field `AGT1UDWUPEN` reader - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1UDWUPEN_R = crate::BitReader<AGT1UDWUPEN_A>;
#[doc = "AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1UDWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled"]
    _1 = 1,
}
impl From<AGT1UDWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1UDWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT1UDWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT1UDWUPEN_A {
        match self.bits {
            false => AGT1UDWUPEN_A::_0,
            true => AGT1UDWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1UDWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1UDWUPEN_A::_1
    }
}
#[doc = "Field `AGT1UDWUPEN` writer - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1UDWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, AGT1UDWUPEN_A, O>;
impl<'a, const O: u8> AGT1UDWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1UDWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1UDWUPEN_A::_1)
    }
}
#[doc = "Field `AGT1CAWUPEN` reader - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CAWUPEN_R = crate::BitReader<AGT1CAWUPEN_A>;
#[doc = "AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CAWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled"]
    _1 = 1,
}
impl From<AGT1CAWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1CAWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT1CAWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT1CAWUPEN_A {
        match self.bits {
            false => AGT1CAWUPEN_A::_0,
            true => AGT1CAWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1CAWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1CAWUPEN_A::_1
    }
}
#[doc = "Field `AGT1CAWUPEN` writer - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CAWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, AGT1CAWUPEN_A, O>;
impl<'a, const O: u8> AGT1CAWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1CAWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1CAWUPEN_A::_1)
    }
}
#[doc = "Field `AGT1CBWUPEN` reader - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CBWUPEN_R = crate::BitReader<AGT1CBWUPEN_A>;
#[doc = "AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CBWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled"]
    _1 = 1,
}
impl From<AGT1CBWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1CBWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT1CBWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT1CBWUPEN_A {
        match self.bits {
            false => AGT1CBWUPEN_A::_0,
            true => AGT1CBWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1CBWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1CBWUPEN_A::_1
    }
}
#[doc = "Field `AGT1CBWUPEN` writer - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CBWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, AGT1CBWUPEN_A, O>;
impl<'a, const O: u8> AGT1CBWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1CBWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1CBWUPEN_A::_1)
    }
}
#[doc = "Field `IIC0WUPEN` reader - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IIC0WUPEN_R = crate::BitReader<IIC0WUPEN_A>;
#[doc = "IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIC0WUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled"]
    _1 = 1,
}
impl From<IIC0WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IIC0WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IIC0WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIC0WUPEN_A {
        match self.bits {
            false => IIC0WUPEN_A::_0,
            true => IIC0WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIC0WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIC0WUPEN_A::_1
    }
}
#[doc = "Field `IIC0WUPEN` writer - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IIC0WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IIC0WUPEN_A, O>;
impl<'a, const O: u8> IIC0WUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IIC0WUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IIC0WUPEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen0(&self) -> IRQWUPEN0_R {
        IRQWUPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen1(&self) -> IRQWUPEN1_R {
        IRQWUPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen2(&self) -> IRQWUPEN2_R {
        IRQWUPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen3(&self) -> IRQWUPEN3_R {
        IRQWUPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen4(&self) -> IRQWUPEN4_R {
        IRQWUPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen5(&self) -> IRQWUPEN5_R {
        IRQWUPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen6(&self) -> IRQWUPEN6_R {
        IRQWUPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen7(&self) -> IRQWUPEN7_R {
        IRQWUPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen8(&self) -> IRQWUPEN8_R {
        IRQWUPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen9(&self) -> IRQWUPEN9_R {
        IRQWUPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - IRQ13 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn irqwupen13(&self) -> IRQWUPEN13_R {
        IRQWUPEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn iwdtwupen(&self) -> IWDTWUPEN_R {
        IWDTWUPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn lvd1wupen(&self) -> LVD1WUPEN_R {
        LVD1WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn lvd2wupen(&self) -> LVD2WUPEN_R {
        LVD2WUPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn rtcalmwupen(&self) -> RTCALMWUPEN_R {
        RTCALMWUPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn rtcprdwupen(&self) -> RTCPRDWUPEN_R {
        RTCPRDWUPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn usbfs0wupen(&self) -> USBFS0WUPEN_R {
        USBFS0WUPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1udwupen(&self) -> AGT1UDWUPEN_R {
        AGT1UDWUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1cawupen(&self) -> AGT1CAWUPEN_R {
        AGT1CAWUPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1cbwupen(&self) -> AGT1CBWUPEN_R {
        AGT1CBWUPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn iic0wupen(&self) -> IIC0WUPEN_R {
        IIC0WUPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen0(&mut self) -> IRQWUPEN0_W<0> {
        IRQWUPEN0_W::new(self)
    }
    #[doc = "Bit 1 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen1(&mut self) -> IRQWUPEN1_W<1> {
        IRQWUPEN1_W::new(self)
    }
    #[doc = "Bit 2 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen2(&mut self) -> IRQWUPEN2_W<2> {
        IRQWUPEN2_W::new(self)
    }
    #[doc = "Bit 3 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen3(&mut self) -> IRQWUPEN3_W<3> {
        IRQWUPEN3_W::new(self)
    }
    #[doc = "Bit 4 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen4(&mut self) -> IRQWUPEN4_W<4> {
        IRQWUPEN4_W::new(self)
    }
    #[doc = "Bit 5 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen5(&mut self) -> IRQWUPEN5_W<5> {
        IRQWUPEN5_W::new(self)
    }
    #[doc = "Bit 6 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen6(&mut self) -> IRQWUPEN6_W<6> {
        IRQWUPEN6_W::new(self)
    }
    #[doc = "Bit 7 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen7(&mut self) -> IRQWUPEN7_W<7> {
        IRQWUPEN7_W::new(self)
    }
    #[doc = "Bit 8 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen8(&mut self) -> IRQWUPEN8_W<8> {
        IRQWUPEN8_W::new(self)
    }
    #[doc = "Bit 9 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen9(&mut self) -> IRQWUPEN9_W<9> {
        IRQWUPEN9_W::new(self)
    }
    #[doc = "Bit 13 - IRQ13 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen13(&mut self) -> IRQWUPEN13_W<13> {
        IRQWUPEN13_W::new(self)
    }
    #[doc = "Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdtwupen(&mut self) -> IWDTWUPEN_W<16> {
        IWDTWUPEN_W::new(self)
    }
    #[doc = "Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1wupen(&mut self) -> LVD1WUPEN_W<18> {
        LVD1WUPEN_W::new(self)
    }
    #[doc = "Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2wupen(&mut self) -> LVD2WUPEN_W<19> {
        LVD2WUPEN_W::new(self)
    }
    #[doc = "Bit 24 - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtcalmwupen(&mut self) -> RTCALMWUPEN_W<24> {
        RTCALMWUPEN_W::new(self)
    }
    #[doc = "Bit 25 - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtcprdwupen(&mut self) -> RTCPRDWUPEN_W<25> {
        RTCPRDWUPEN_W::new(self)
    }
    #[doc = "Bit 27 - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbfs0wupen(&mut self) -> USBFS0WUPEN_W<27> {
        USBFS0WUPEN_W::new(self)
    }
    #[doc = "Bit 28 - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt1udwupen(&mut self) -> AGT1UDWUPEN_W<28> {
        AGT1UDWUPEN_W::new(self)
    }
    #[doc = "Bit 29 - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt1cawupen(&mut self) -> AGT1CAWUPEN_W<29> {
        AGT1CAWUPEN_W::new(self)
    }
    #[doc = "Bit 30 - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt1cbwupen(&mut self) -> AGT1CBWUPEN_W<30> {
        AGT1CBWUPEN_W::new(self)
    }
    #[doc = "Bit 31 - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iic0wupen(&mut self) -> IIC0WUPEN_W<31> {
        IIC0WUPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake Up Interrupt Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wupen0](index.html) module"]
pub struct WUPEN0_SPEC;
impl crate::RegisterSpec for WUPEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wupen0::R](R) reader structure"]
impl crate::Readable for WUPEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wupen0::W](W) writer structure"]
impl crate::Writable for WUPEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUPEN0 to value 0"]
impl crate::Resettable for WUPEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
