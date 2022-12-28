#[doc = "Register `WUPEN` reader"]
pub struct R(crate::R<WUPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUPEN` writer"]
pub struct W(crate::W<WUPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUPEN_SPEC>;
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
impl From<crate::W<WUPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQWUPEN0` reader - IRQ0 interrupt S/W standby returns enable"]
pub type IRQWUPEN0_R = crate::BitReader<IRQWUPEN0_A>;
#[doc = "IRQ0 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN0_A {
    #[doc = "0: S/W standby returns by IRQ0 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ0 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN0` writer - IRQ0 interrupt S/W standby returns enable"]
pub type IRQWUPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN0_A, O>;
impl<'a, const O: u8> IRQWUPEN0_W<'a, O> {
    #[doc = "S/W standby returns by IRQ0 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN0_A::_0)
    }
    #[doc = "S/W standby returns by IRQ0 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN0_A::_1)
    }
}
#[doc = "Field `IRQWUPEN1` reader - IRQ1 interrupt S/W standby returns enable"]
pub type IRQWUPEN1_R = crate::BitReader<IRQWUPEN1_A>;
#[doc = "IRQ1 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN1_A {
    #[doc = "0: S/W standby returns by IRQ1 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ1 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN1` writer - IRQ1 interrupt S/W standby returns enable"]
pub type IRQWUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN1_A, O>;
impl<'a, const O: u8> IRQWUPEN1_W<'a, O> {
    #[doc = "S/W standby returns by IRQ1 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN1_A::_0)
    }
    #[doc = "S/W standby returns by IRQ1 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN1_A::_1)
    }
}
#[doc = "Field `IRQWUPEN2` reader - IRQ2 interrupt S/W standby returns enable"]
pub type IRQWUPEN2_R = crate::BitReader<IRQWUPEN2_A>;
#[doc = "IRQ2 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN2_A {
    #[doc = "0: S/W standby returns by IRQ2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ2 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN2` writer - IRQ2 interrupt S/W standby returns enable"]
pub type IRQWUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN2_A, O>;
impl<'a, const O: u8> IRQWUPEN2_W<'a, O> {
    #[doc = "S/W standby returns by IRQ2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN2_A::_0)
    }
    #[doc = "S/W standby returns by IRQ2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN2_A::_1)
    }
}
#[doc = "Field `IRQWUPEN3` reader - IRQ3 interrupt S/W standby returns enable"]
pub type IRQWUPEN3_R = crate::BitReader<IRQWUPEN3_A>;
#[doc = "IRQ3 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN3_A {
    #[doc = "0: S/W standby returns by IRQ3 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ3 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN3` writer - IRQ3 interrupt S/W standby returns enable"]
pub type IRQWUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN3_A, O>;
impl<'a, const O: u8> IRQWUPEN3_W<'a, O> {
    #[doc = "S/W standby returns by IRQ3 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN3_A::_0)
    }
    #[doc = "S/W standby returns by IRQ3 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN3_A::_1)
    }
}
#[doc = "Field `IRQWUPEN4` reader - IRQ4 interrupt S/W standby returns enable"]
pub type IRQWUPEN4_R = crate::BitReader<IRQWUPEN4_A>;
#[doc = "IRQ4 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN4_A {
    #[doc = "0: S/W standby returns by IRQ4 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ4 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN4` writer - IRQ4 interrupt S/W standby returns enable"]
pub type IRQWUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN4_A, O>;
impl<'a, const O: u8> IRQWUPEN4_W<'a, O> {
    #[doc = "S/W standby returns by IRQ4 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN4_A::_0)
    }
    #[doc = "S/W standby returns by IRQ4 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN4_A::_1)
    }
}
#[doc = "Field `IRQWUPEN5` reader - IRQ5 interrupt S/W standby returns enable"]
pub type IRQWUPEN5_R = crate::BitReader<IRQWUPEN5_A>;
#[doc = "IRQ5 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN5_A {
    #[doc = "0: S/W standby returns by IRQ5 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ5 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN5` writer - IRQ5 interrupt S/W standby returns enable"]
pub type IRQWUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN5_A, O>;
impl<'a, const O: u8> IRQWUPEN5_W<'a, O> {
    #[doc = "S/W standby returns by IRQ5 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN5_A::_0)
    }
    #[doc = "S/W standby returns by IRQ5 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN5_A::_1)
    }
}
#[doc = "Field `IRQWUPEN6` reader - IRQ6 interrupt S/W standby returns enable"]
pub type IRQWUPEN6_R = crate::BitReader<IRQWUPEN6_A>;
#[doc = "IRQ6 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN6_A {
    #[doc = "0: S/W standby returns by IRQ6 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ6 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN6` writer - IRQ6 interrupt S/W standby returns enable"]
pub type IRQWUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN6_A, O>;
impl<'a, const O: u8> IRQWUPEN6_W<'a, O> {
    #[doc = "S/W standby returns by IRQ6 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN6_A::_0)
    }
    #[doc = "S/W standby returns by IRQ6 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN6_A::_1)
    }
}
#[doc = "Field `IRQWUPEN7` reader - IRQ7 interrupt S/W standby returns enable"]
pub type IRQWUPEN7_R = crate::BitReader<IRQWUPEN7_A>;
#[doc = "IRQ7 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN7_A {
    #[doc = "0: S/W standby returns by IRQ7 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ7 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN7` writer - IRQ7 interrupt S/W standby returns enable"]
pub type IRQWUPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN7_A, O>;
impl<'a, const O: u8> IRQWUPEN7_W<'a, O> {
    #[doc = "S/W standby returns by IRQ7 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN7_A::_0)
    }
    #[doc = "S/W standby returns by IRQ7 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN7_A::_1)
    }
}
#[doc = "Field `IRQWUPEN8` reader - IRQ8 interrupt S/W standby returns enable"]
pub type IRQWUPEN8_R = crate::BitReader<IRQWUPEN8_A>;
#[doc = "IRQ8 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN8_A {
    #[doc = "0: S/W standby returns by IRQ8 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ8 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN8` writer - IRQ8 interrupt S/W standby returns enable"]
pub type IRQWUPEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN8_A, O>;
impl<'a, const O: u8> IRQWUPEN8_W<'a, O> {
    #[doc = "S/W standby returns by IRQ8 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN8_A::_0)
    }
    #[doc = "S/W standby returns by IRQ8 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN8_A::_1)
    }
}
#[doc = "Field `IRQWUPEN9` reader - IRQ9 interrupt S/W standby returns enable"]
pub type IRQWUPEN9_R = crate::BitReader<IRQWUPEN9_A>;
#[doc = "IRQ9 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN9_A {
    #[doc = "0: S/W standby returns by IRQ9 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ9 interrupt is enabled"]
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
#[doc = "Field `IRQWUPEN9` writer - IRQ9 interrupt S/W standby returns enable"]
pub type IRQWUPEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN9_A, O>;
impl<'a, const O: u8> IRQWUPEN9_W<'a, O> {
    #[doc = "S/W standby returns by IRQ9 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN9_A::_0)
    }
    #[doc = "S/W standby returns by IRQ9 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN9_A::_1)
    }
}
#[doc = "Field `IRQWUPEN10` reader - IRQ10 interrupt S/W standby returns enable"]
pub type IRQWUPEN10_R = crate::BitReader<IRQWUPEN10_A>;
#[doc = "IRQ10 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN10_A {
    #[doc = "0: S/W standby returns by IRQ10 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ10 interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN10_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN10_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN10_A {
        match self.bits {
            false => IRQWUPEN10_A::_0,
            true => IRQWUPEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN10_A::_1
    }
}
#[doc = "Field `IRQWUPEN10` writer - IRQ10 interrupt S/W standby returns enable"]
pub type IRQWUPEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN10_A, O>;
impl<'a, const O: u8> IRQWUPEN10_W<'a, O> {
    #[doc = "S/W standby returns by IRQ10 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN10_A::_0)
    }
    #[doc = "S/W standby returns by IRQ10 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN10_A::_1)
    }
}
#[doc = "Field `IRQWUPEN11` reader - IRQ11 interrupt S/W standby returns enable"]
pub type IRQWUPEN11_R = crate::BitReader<IRQWUPEN11_A>;
#[doc = "IRQ11 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN11_A {
    #[doc = "0: S/W standby returns by IRQ11 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ11 interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN11_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN11_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN11_A {
        match self.bits {
            false => IRQWUPEN11_A::_0,
            true => IRQWUPEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN11_A::_1
    }
}
#[doc = "Field `IRQWUPEN11` writer - IRQ11 interrupt S/W standby returns enable"]
pub type IRQWUPEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN11_A, O>;
impl<'a, const O: u8> IRQWUPEN11_W<'a, O> {
    #[doc = "S/W standby returns by IRQ11 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN11_A::_0)
    }
    #[doc = "S/W standby returns by IRQ11 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN11_A::_1)
    }
}
#[doc = "Field `IRQWUPEN12` reader - IRQ12 interrupt S/W standby returns enable"]
pub type IRQWUPEN12_R = crate::BitReader<IRQWUPEN12_A>;
#[doc = "IRQ12 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN12_A {
    #[doc = "0: S/W standby returns by IRQ12 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ12 interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN12_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN12_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN12_A {
        match self.bits {
            false => IRQWUPEN12_A::_0,
            true => IRQWUPEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN12_A::_1
    }
}
#[doc = "Field `IRQWUPEN12` writer - IRQ12 interrupt S/W standby returns enable"]
pub type IRQWUPEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN12_A, O>;
impl<'a, const O: u8> IRQWUPEN12_W<'a, O> {
    #[doc = "S/W standby returns by IRQ12 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN12_A::_0)
    }
    #[doc = "S/W standby returns by IRQ12 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN12_A::_1)
    }
}
#[doc = "Field `IRQWUPEN14` reader - IRQ14 interrupt S/W standby returns enable"]
pub type IRQWUPEN14_R = crate::BitReader<IRQWUPEN14_A>;
#[doc = "IRQ14 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN14_A {
    #[doc = "0: S/W standby returns by IRQ14 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ14 interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN14_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN14_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN14_A {
        match self.bits {
            false => IRQWUPEN14_A::_0,
            true => IRQWUPEN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN14_A::_1
    }
}
#[doc = "Field `IRQWUPEN14` writer - IRQ14 interrupt S/W standby returns enable"]
pub type IRQWUPEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN14_A, O>;
impl<'a, const O: u8> IRQWUPEN14_W<'a, O> {
    #[doc = "S/W standby returns by IRQ14 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN14_A::_0)
    }
    #[doc = "S/W standby returns by IRQ14 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN14_A::_1)
    }
}
#[doc = "Field `IRQWUPEN15` reader - IRQ15 interrupt S/W standby returns enable"]
pub type IRQWUPEN15_R = crate::BitReader<IRQWUPEN15_A>;
#[doc = "IRQ15 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN15_A {
    #[doc = "0: S/W standby returns by IRQ15 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ15 interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN15_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN15_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQWUPEN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQWUPEN15_A {
        match self.bits {
            false => IRQWUPEN15_A::_0,
            true => IRQWUPEN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN15_A::_1
    }
}
#[doc = "Field `IRQWUPEN15` writer - IRQ15 interrupt S/W standby returns enable"]
pub type IRQWUPEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IRQWUPEN15_A, O>;
impl<'a, const O: u8> IRQWUPEN15_W<'a, O> {
    #[doc = "S/W standby returns by IRQ15 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN15_A::_0)
    }
    #[doc = "S/W standby returns by IRQ15 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN15_A::_1)
    }
}
#[doc = "Field `IWDTWUPEN` reader - IWDT interrupt S/W standby returns enable"]
pub type IWDTWUPEN_R = crate::BitReader<IWDTWUPEN_A>;
#[doc = "IWDT interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTWUPEN_A {
    #[doc = "0: S/W standby returns by IWDT interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IWDT interrupt is enabled"]
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
#[doc = "Field `IWDTWUPEN` writer - IWDT interrupt S/W standby returns enable"]
pub type IWDTWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IWDTWUPEN_A, O>;
impl<'a, const O: u8> IWDTWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by IWDT interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IWDTWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by IWDT interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IWDTWUPEN_A::_1)
    }
}
#[doc = "Field `KEYWUPEN` reader - Key interrupt S/W standby returns enable"]
pub type KEYWUPEN_R = crate::BitReader<KEYWUPEN_A>;
#[doc = "Key interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYWUPEN_A {
    #[doc = "0: S/W standby returns by KEY interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by KEY interrupt is enabled"]
    _1 = 1,
}
impl From<KEYWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: KEYWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl KEYWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYWUPEN_A {
        match self.bits {
            false => KEYWUPEN_A::_0,
            true => KEYWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KEYWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KEYWUPEN_A::_1
    }
}
#[doc = "Field `KEYWUPEN` writer - Key interrupt S/W standby returns enable"]
pub type KEYWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, KEYWUPEN_A, O>;
impl<'a, const O: u8> KEYWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by KEY interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KEYWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by KEY interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KEYWUPEN_A::_1)
    }
}
#[doc = "Field `LVD1WUPEN` reader - LVD1 interrupt S/W standby returns enable"]
pub type LVD1WUPEN_R = crate::BitReader<LVD1WUPEN_A>;
#[doc = "LVD1 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1WUPEN_A {
    #[doc = "0: S/W standby returns by LVD1 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by LVD1 interrupt is enabled"]
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
#[doc = "Field `LVD1WUPEN` writer - LVD1 interrupt S/W standby returns enable"]
pub type LVD1WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, LVD1WUPEN_A, O>;
impl<'a, const O: u8> LVD1WUPEN_W<'a, O> {
    #[doc = "S/W standby returns by LVD1 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1WUPEN_A::_0)
    }
    #[doc = "S/W standby returns by LVD1 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1WUPEN_A::_1)
    }
}
#[doc = "Field `LVD2WUPEN` reader - LVD2 interrupt S/W standby returns enable"]
pub type LVD2WUPEN_R = crate::BitReader<LVD2WUPEN_A>;
#[doc = "LVD2 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2WUPEN_A {
    #[doc = "0: S/W standby returns by LVD2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by LVD2 interrupt is enabled"]
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
#[doc = "Field `LVD2WUPEN` writer - LVD2 interrupt S/W standby returns enable"]
pub type LVD2WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, LVD2WUPEN_A, O>;
impl<'a, const O: u8> LVD2WUPEN_W<'a, O> {
    #[doc = "S/W standby returns by LVD2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2WUPEN_A::_0)
    }
    #[doc = "S/W standby returns by LVD2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2WUPEN_A::_1)
    }
}
#[doc = "Field `VBATTWUPEN` reader - VBATT monitor interrupt S/W standby returns enable"]
pub type VBATTWUPEN_R = crate::BitReader<VBATTWUPEN_A>;
#[doc = "VBATT monitor interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATTWUPEN_A {
    #[doc = "0: S/W standby returns by VBATT monitor interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by VBATT monitor interrupt is enabled"]
    _1 = 1,
}
impl From<VBATTWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATTWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATTWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATTWUPEN_A {
        match self.bits {
            false => VBATTWUPEN_A::_0,
            true => VBATTWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBATTWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBATTWUPEN_A::_1
    }
}
#[doc = "Field `VBATTWUPEN` writer - VBATT monitor interrupt S/W standby returns enable"]
pub type VBATTWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, VBATTWUPEN_A, O>;
impl<'a, const O: u8> VBATTWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by VBATT monitor interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBATTWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by VBATT monitor interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBATTWUPEN_A::_1)
    }
}
#[doc = "Field `ACMPLP0WUPEN` reader - ACMPLP0 interrupt S/W standby returns enable"]
pub type ACMPLP0WUPEN_R = crate::BitReader<ACMPLP0WUPEN_A>;
#[doc = "ACMPLP0 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMPLP0WUPEN_A {
    #[doc = "0: S/W standby returns by ACMPLP0 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by ACMPLP0 interrupt is enabled"]
    _1 = 1,
}
impl From<ACMPLP0WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMPLP0WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMPLP0WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMPLP0WUPEN_A {
        match self.bits {
            false => ACMPLP0WUPEN_A::_0,
            true => ACMPLP0WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACMPLP0WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACMPLP0WUPEN_A::_1
    }
}
#[doc = "Field `ACMPLP0WUPEN` writer - ACMPLP0 interrupt S/W standby returns enable"]
pub type ACMPLP0WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, ACMPLP0WUPEN_A, O>;
impl<'a, const O: u8> ACMPLP0WUPEN_W<'a, O> {
    #[doc = "S/W standby returns by ACMPLP0 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMPLP0WUPEN_A::_0)
    }
    #[doc = "S/W standby returns by ACMPLP0 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMPLP0WUPEN_A::_1)
    }
}
#[doc = "Field `RTCALMWUPEN` reader - RTC alarm interrupt S/W standby returns enable"]
pub type RTCALMWUPEN_R = crate::BitReader<RTCALMWUPEN_A>;
#[doc = "RTC alarm interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCALMWUPEN_A {
    #[doc = "0: S/W standby returns by RTC alarm interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by RTC alarm interrupt is enabled"]
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
#[doc = "Field `RTCALMWUPEN` writer - RTC alarm interrupt S/W standby returns enable"]
pub type RTCALMWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, RTCALMWUPEN_A, O>;
impl<'a, const O: u8> RTCALMWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by RTC alarm interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCALMWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by RTC alarm interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCALMWUPEN_A::_1)
    }
}
#[doc = "Field `RTCPRDWUPEN` reader - RCT period interrupt S/W standby returns enable"]
pub type RTCPRDWUPEN_R = crate::BitReader<RTCPRDWUPEN_A>;
#[doc = "RCT period interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCPRDWUPEN_A {
    #[doc = "0: S/W standby returns by RTC period interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by RTC period interrupt is enabled"]
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
#[doc = "Field `RTCPRDWUPEN` writer - RCT period interrupt S/W standby returns enable"]
pub type RTCPRDWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, RTCPRDWUPEN_A, O>;
impl<'a, const O: u8> RTCPRDWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by RTC period interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCPRDWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by RTC period interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCPRDWUPEN_A::_1)
    }
}
#[doc = "Field `USBFSWUPEN` reader - USBFS interrupt S/W standby returns enable"]
pub type USBFSWUPEN_R = crate::BitReader<USBFSWUPEN_A>;
#[doc = "USBFS interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFSWUPEN_A {
    #[doc = "0: S/W standby returns by USBFS interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by USBFS interrupt is enabled"]
    _1 = 1,
}
impl From<USBFSWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBFSWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBFSWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBFSWUPEN_A {
        match self.bits {
            false => USBFSWUPEN_A::_0,
            true => USBFSWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBFSWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBFSWUPEN_A::_1
    }
}
#[doc = "Field `USBFSWUPEN` writer - USBFS interrupt S/W standby returns enable"]
pub type USBFSWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, USBFSWUPEN_A, O>;
impl<'a, const O: u8> USBFSWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by USBFS interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBFSWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by USBFS interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBFSWUPEN_A::_1)
    }
}
#[doc = "Field `AGT1UDWUPEN` reader - AGT1 underflow interrupt S/W standby returns enable"]
pub type AGT1UDWUPEN_R = crate::BitReader<AGT1UDWUPEN_A>;
#[doc = "AGT1 underflow interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1UDWUPEN_A {
    #[doc = "0: S/W standby returns by AGT1 underflow interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by AGT1 underflow interrupt is enabled"]
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
#[doc = "Field `AGT1UDWUPEN` writer - AGT1 underflow interrupt S/W standby returns enable"]
pub type AGT1UDWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, AGT1UDWUPEN_A, O>;
impl<'a, const O: u8> AGT1UDWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by AGT1 underflow interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1UDWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by AGT1 underflow interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1UDWUPEN_A::_1)
    }
}
#[doc = "Field `AGT1CAWUPEN` reader - AGT1 compare match A interrupt S/W standby returns enable"]
pub type AGT1CAWUPEN_R = crate::BitReader<AGT1CAWUPEN_A>;
#[doc = "AGT1 compare match A interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CAWUPEN_A {
    #[doc = "0: S/W standby returns by AGT1 compare match A interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by AGT1 compare match A interrupt is enabled"]
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
#[doc = "Field `AGT1CAWUPEN` writer - AGT1 compare match A interrupt S/W standby returns enable"]
pub type AGT1CAWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, AGT1CAWUPEN_A, O>;
impl<'a, const O: u8> AGT1CAWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1CAWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1CAWUPEN_A::_1)
    }
}
#[doc = "Field `AGT1CBWUPEN` reader - AGT1 compare match B interrupt S/W standby returns enable"]
pub type AGT1CBWUPEN_R = crate::BitReader<AGT1CBWUPEN_A>;
#[doc = "AGT1 compare match B interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CBWUPEN_A {
    #[doc = "0: S/W standby returns by AGT1 compare match B interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by AGT1 compare match B interrupt is enabled"]
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
#[doc = "Field `AGT1CBWUPEN` writer - AGT1 compare match B interrupt S/W standby returns enable"]
pub type AGT1CBWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, AGT1CBWUPEN_A, O>;
impl<'a, const O: u8> AGT1CBWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1CBWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1CBWUPEN_A::_1)
    }
}
#[doc = "Field `IIC0WUPEN` reader - IIC0 address match interrupt S/W standby returns enable"]
pub type IIC0WUPEN_R = crate::BitReader<IIC0WUPEN_A>;
#[doc = "IIC0 address match interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIC0WUPEN_A {
    #[doc = "0: S/W standby returns by IIC0 address match interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IIC0 address match interrupt is enabled"]
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
#[doc = "Field `IIC0WUPEN` writer - IIC0 address match interrupt S/W standby returns enable"]
pub type IIC0WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN_SPEC, IIC0WUPEN_A, O>;
impl<'a, const O: u8> IIC0WUPEN_W<'a, O> {
    #[doc = "S/W standby returns by IIC0 address match interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IIC0WUPEN_A::_0)
    }
    #[doc = "S/W standby returns by IIC0 address match interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IIC0WUPEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen0(&self) -> IRQWUPEN0_R {
        IRQWUPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen1(&self) -> IRQWUPEN1_R {
        IRQWUPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQ2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen2(&self) -> IRQWUPEN2_R {
        IRQWUPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ3 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen3(&self) -> IRQWUPEN3_R {
        IRQWUPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ4 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen4(&self) -> IRQWUPEN4_R {
        IRQWUPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ5 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen5(&self) -> IRQWUPEN5_R {
        IRQWUPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ6 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen6(&self) -> IRQWUPEN6_R {
        IRQWUPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ7 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen7(&self) -> IRQWUPEN7_R {
        IRQWUPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRQ8 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen8(&self) -> IRQWUPEN8_R {
        IRQWUPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IRQ9 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen9(&self) -> IRQWUPEN9_R {
        IRQWUPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRQ10 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen10(&self) -> IRQWUPEN10_R {
        IRQWUPEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IRQ11 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen11(&self) -> IRQWUPEN11_R {
        IRQWUPEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IRQ12 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen12(&self) -> IRQWUPEN12_R {
        IRQWUPEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - IRQ14 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen14(&self) -> IRQWUPEN14_R {
        IRQWUPEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IRQ15 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen15(&self) -> IRQWUPEN15_R {
        IRQWUPEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IWDT interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn iwdtwupen(&self) -> IWDTWUPEN_R {
        IWDTWUPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Key interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn keywupen(&self) -> KEYWUPEN_R {
        KEYWUPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LVD1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn lvd1wupen(&self) -> LVD1WUPEN_R {
        LVD1WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LVD2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn lvd2wupen(&self) -> LVD2WUPEN_R {
        LVD2WUPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VBATT monitor interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn vbattwupen(&self) -> VBATTWUPEN_R {
        VBATTWUPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - ACMPLP0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn acmplp0wupen(&self) -> ACMPLP0WUPEN_R {
        ACMPLP0WUPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC alarm interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn rtcalmwupen(&self) -> RTCALMWUPEN_R {
        RTCALMWUPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCT period interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn rtcprdwupen(&self) -> RTCPRDWUPEN_R {
        RTCPRDWUPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - USBFS interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn usbfswupen(&self) -> USBFSWUPEN_R {
        USBFSWUPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AGT1 underflow interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1udwupen(&self) -> AGT1UDWUPEN_R {
        AGT1UDWUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AGT1 compare match A interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1cawupen(&self) -> AGT1CAWUPEN_R {
        AGT1CAWUPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AGT1 compare match B interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1cbwupen(&self) -> AGT1CBWUPEN_R {
        AGT1CBWUPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IIC0 address match interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn iic0wupen(&self) -> IIC0WUPEN_R {
        IIC0WUPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ0 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen0(&mut self) -> IRQWUPEN0_W<0> {
        IRQWUPEN0_W::new(self)
    }
    #[doc = "Bit 1 - IRQ1 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen1(&mut self) -> IRQWUPEN1_W<1> {
        IRQWUPEN1_W::new(self)
    }
    #[doc = "Bit 2 - IRQ2 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen2(&mut self) -> IRQWUPEN2_W<2> {
        IRQWUPEN2_W::new(self)
    }
    #[doc = "Bit 3 - IRQ3 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen3(&mut self) -> IRQWUPEN3_W<3> {
        IRQWUPEN3_W::new(self)
    }
    #[doc = "Bit 4 - IRQ4 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen4(&mut self) -> IRQWUPEN4_W<4> {
        IRQWUPEN4_W::new(self)
    }
    #[doc = "Bit 5 - IRQ5 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen5(&mut self) -> IRQWUPEN5_W<5> {
        IRQWUPEN5_W::new(self)
    }
    #[doc = "Bit 6 - IRQ6 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen6(&mut self) -> IRQWUPEN6_W<6> {
        IRQWUPEN6_W::new(self)
    }
    #[doc = "Bit 7 - IRQ7 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen7(&mut self) -> IRQWUPEN7_W<7> {
        IRQWUPEN7_W::new(self)
    }
    #[doc = "Bit 8 - IRQ8 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen8(&mut self) -> IRQWUPEN8_W<8> {
        IRQWUPEN8_W::new(self)
    }
    #[doc = "Bit 9 - IRQ9 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen9(&mut self) -> IRQWUPEN9_W<9> {
        IRQWUPEN9_W::new(self)
    }
    #[doc = "Bit 10 - IRQ10 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen10(&mut self) -> IRQWUPEN10_W<10> {
        IRQWUPEN10_W::new(self)
    }
    #[doc = "Bit 11 - IRQ11 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen11(&mut self) -> IRQWUPEN11_W<11> {
        IRQWUPEN11_W::new(self)
    }
    #[doc = "Bit 12 - IRQ12 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen12(&mut self) -> IRQWUPEN12_W<12> {
        IRQWUPEN12_W::new(self)
    }
    #[doc = "Bit 14 - IRQ14 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen14(&mut self) -> IRQWUPEN14_W<14> {
        IRQWUPEN14_W::new(self)
    }
    #[doc = "Bit 15 - IRQ15 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen15(&mut self) -> IRQWUPEN15_W<15> {
        IRQWUPEN15_W::new(self)
    }
    #[doc = "Bit 16 - IWDT interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn iwdtwupen(&mut self) -> IWDTWUPEN_W<16> {
        IWDTWUPEN_W::new(self)
    }
    #[doc = "Bit 17 - Key interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn keywupen(&mut self) -> KEYWUPEN_W<17> {
        KEYWUPEN_W::new(self)
    }
    #[doc = "Bit 18 - LVD1 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1wupen(&mut self) -> LVD1WUPEN_W<18> {
        LVD1WUPEN_W::new(self)
    }
    #[doc = "Bit 19 - LVD2 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2wupen(&mut self) -> LVD2WUPEN_W<19> {
        LVD2WUPEN_W::new(self)
    }
    #[doc = "Bit 20 - VBATT monitor interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbattwupen(&mut self) -> VBATTWUPEN_W<20> {
        VBATTWUPEN_W::new(self)
    }
    #[doc = "Bit 23 - ACMPLP0 interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmplp0wupen(&mut self) -> ACMPLP0WUPEN_W<23> {
        ACMPLP0WUPEN_W::new(self)
    }
    #[doc = "Bit 24 - RTC alarm interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcalmwupen(&mut self) -> RTCALMWUPEN_W<24> {
        RTCALMWUPEN_W::new(self)
    }
    #[doc = "Bit 25 - RCT period interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcprdwupen(&mut self) -> RTCPRDWUPEN_W<25> {
        RTCPRDWUPEN_W::new(self)
    }
    #[doc = "Bit 27 - USBFS interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbfswupen(&mut self) -> USBFSWUPEN_W<27> {
        USBFSWUPEN_W::new(self)
    }
    #[doc = "Bit 28 - AGT1 underflow interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn agt1udwupen(&mut self) -> AGT1UDWUPEN_W<28> {
        AGT1UDWUPEN_W::new(self)
    }
    #[doc = "Bit 29 - AGT1 compare match A interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn agt1cawupen(&mut self) -> AGT1CAWUPEN_W<29> {
        AGT1CAWUPEN_W::new(self)
    }
    #[doc = "Bit 30 - AGT1 compare match B interrupt S/W standby returns enable"]
    #[inline(always)]
    #[must_use]
    pub fn agt1cbwupen(&mut self) -> AGT1CBWUPEN_W<30> {
        AGT1CBWUPEN_W::new(self)
    }
    #[doc = "Bit 31 - IIC0 address match interrupt S/W standby returns enable"]
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
#[doc = "Wake Up Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wupen](index.html) module"]
pub struct WUPEN_SPEC;
impl crate::RegisterSpec for WUPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wupen::R](R) reader structure"]
impl crate::Readable for WUPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wupen::W](W) writer structure"]
impl crate::Writable for WUPEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUPEN to value 0"]
impl crate::Resettable for WUPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
