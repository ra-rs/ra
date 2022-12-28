#[doc = "Register `GTICCR` reader"]
pub struct R(crate::R<GTICCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTICCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTICCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTICCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTICCR` writer"]
pub struct W(crate::W<GTICCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTICCR_SPEC>;
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
impl From<crate::W<GTICCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTICCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICAFA` reader - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFA_R = crate::BitReader<ICAFA_A>;
#[doc = "Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFA_A {
    #[doc = "0: Disable forwarding GTCCRA register compare match/input capture to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRA register compare match/input capture to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFA_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFA_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFA_A {
        match self.bits {
            false => ICAFA_A::_0,
            true => ICAFA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFA_A::_1
    }
}
#[doc = "Field `ICAFA` writer - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFA_A, O>;
impl<'a, const O: u8> ICAFA_W<'a, O> {
    #[doc = "Disable forwarding GTCCRA register compare match/input capture to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFA_A::_0)
    }
    #[doc = "Enable forwarding GTCCRA register compare match/input capture to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFA_A::_1)
    }
}
#[doc = "Field `ICAFB` reader - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFB_R = crate::BitReader<ICAFB_A>;
#[doc = "Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFB_A {
    #[doc = "0: Disable forwarding GTCCRB register compare match/input capture to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRB register compare match/input capture to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFB_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFB_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFB_A {
        match self.bits {
            false => ICAFB_A::_0,
            true => ICAFB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFB_A::_1
    }
}
#[doc = "Field `ICAFB` writer - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFB_A, O>;
impl<'a, const O: u8> ICAFB_W<'a, O> {
    #[doc = "Disable forwarding GTCCRB register compare match/input capture to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFB_A::_0)
    }
    #[doc = "Enable forwarding GTCCRB register compare match/input capture to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFB_A::_1)
    }
}
#[doc = "Field `ICAFC` reader - Forwarding GTCCRC register Compare Match to Other Channel GTCCRA Input Source Capture Enable"]
pub type ICAFC_R = crate::BitReader<ICAFC_A>;
#[doc = "Forwarding GTCCRC register Compare Match to Other Channel GTCCRA Input Source Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFC_A {
    #[doc = "0: Disable forwarding GTCCRC register compare match to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRC register compare match to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFC_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFC_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFC_A {
        match self.bits {
            false => ICAFC_A::_0,
            true => ICAFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFC_A::_1
    }
}
#[doc = "Field `ICAFC` writer - Forwarding GTCCRC register Compare Match to Other Channel GTCCRA Input Source Capture Enable"]
pub type ICAFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFC_A, O>;
impl<'a, const O: u8> ICAFC_W<'a, O> {
    #[doc = "Disable forwarding GTCCRC register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFC_A::_0)
    }
    #[doc = "Enable forwarding GTCCRC register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFC_A::_1)
    }
}
#[doc = "Field `ICAFD` reader - Forwarding GTCCRD register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFD_R = crate::BitReader<ICAFD_A>;
#[doc = "Forwarding GTCCRD register Compare Match to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFD_A {
    #[doc = "0: Disable forwarding GTCCRD register compare match to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRD register compare match to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFD_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFD_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFD_A {
        match self.bits {
            false => ICAFD_A::_0,
            true => ICAFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFD_A::_1
    }
}
#[doc = "Field `ICAFD` writer - Forwarding GTCCRD register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFD_A, O>;
impl<'a, const O: u8> ICAFD_W<'a, O> {
    #[doc = "Disable forwarding GTCCRD register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFD_A::_0)
    }
    #[doc = "Enable forwarding GTCCRD register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFD_A::_1)
    }
}
#[doc = "Field `ICAFE` reader - Forwarding GTCCRE register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFE_R = crate::BitReader<ICAFE_A>;
#[doc = "Forwarding GTCCRE register Compare Match to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFE_A {
    #[doc = "0: Disable forwarding GTCCRE register compare match to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRE register compare match to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFE_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFE_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFE_A {
        match self.bits {
            false => ICAFE_A::_0,
            true => ICAFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFE_A::_1
    }
}
#[doc = "Field `ICAFE` writer - Forwarding GTCCRE register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFE_A, O>;
impl<'a, const O: u8> ICAFE_W<'a, O> {
    #[doc = "Disable forwarding GTCCRE register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFE_A::_0)
    }
    #[doc = "Enable forwarding GTCCRE register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFE_A::_1)
    }
}
#[doc = "Field `ICAFF` reader - Forwarding GTCCRF register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFF_R = crate::BitReader<ICAFF_A>;
#[doc = "Forwarding GTCCRF register Compare Match to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFF_A {
    #[doc = "0: Disable forwarding GTCCRF register compare match to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRF register compare match to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFF_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFF_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFF_A {
        match self.bits {
            false => ICAFF_A::_0,
            true => ICAFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFF_A::_1
    }
}
#[doc = "Field `ICAFF` writer - Forwarding GTCCRF register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFF_A, O>;
impl<'a, const O: u8> ICAFF_W<'a, O> {
    #[doc = "Disable forwarding GTCCRF register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFF_A::_0)
    }
    #[doc = "Enable forwarding GTCCRF register compare match to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFF_A::_1)
    }
}
#[doc = "Field `ICAFPO` reader - Forwarding Overflow to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFPO_R = crate::BitReader<ICAFPO_A>;
#[doc = "Forwarding Overflow to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFPO_A {
    #[doc = "0: Disable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFPO_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFPO_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFPO_A {
        match self.bits {
            false => ICAFPO_A::_0,
            true => ICAFPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFPO_A::_1
    }
}
#[doc = "Field `ICAFPO` writer - Forwarding Overflow to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFPO_A, O>;
impl<'a, const O: u8> ICAFPO_W<'a, O> {
    #[doc = "Disable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFPO_A::_0)
    }
    #[doc = "Enable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFPO_A::_1)
    }
}
#[doc = "Field `ICAFPU` reader - Forwarding Underflow to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFPU_R = crate::BitReader<ICAFPU_A>;
#[doc = "Forwarding Underflow to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAFPU_A {
    #[doc = "0: Disable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICAFPU_A> for bool {
    #[inline(always)]
    fn from(variant: ICAFPU_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAFPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAFPU_A {
        match self.bits {
            false => ICAFPU_A::_0,
            true => ICAFPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICAFPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICAFPU_A::_1
    }
}
#[doc = "Field `ICAFPU` writer - Forwarding Underflow to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICAFPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICAFPU_A, O>;
impl<'a, const O: u8> ICAFPU_W<'a, O> {
    #[doc = "Disable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICAFPU_A::_0)
    }
    #[doc = "Enable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICAFPU_A::_1)
    }
}
#[doc = "Field `ICACLK` reader - Forwarding Count Clock to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICACLK_R = crate::BitReader<ICACLK_A>;
#[doc = "Forwarding Count Clock to Other Channel GTCCRA Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICACLK_A {
    #[doc = "0: Disable forwarding count clock to GTCCRA input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding count clock to GTCCRA input capture source of other channels"]
    _1 = 1,
}
impl From<ICACLK_A> for bool {
    #[inline(always)]
    fn from(variant: ICACLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ICACLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACLK_A {
        match self.bits {
            false => ICACLK_A::_0,
            true => ICACLK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICACLK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICACLK_A::_1
    }
}
#[doc = "Field `ICACLK` writer - Forwarding Count Clock to Other Channel GTCCRA Input Capture Source Enable"]
pub type ICACLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICACLK_A, O>;
impl<'a, const O: u8> ICACLK_W<'a, O> {
    #[doc = "Disable forwarding count clock to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICACLK_A::_0)
    }
    #[doc = "Enable forwarding count clock to GTCCRA input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICACLK_A::_1)
    }
}
#[doc = "Field `ICAGRP` reader - GTCCRA Input Capture Group Select"]
pub type ICAGRP_R = crate::FieldReader<u8, ICAGRP_A>;
#[doc = "GTCCRA Input Capture Group Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICAGRP_A {
    #[doc = "0: Select group A"]
    _00 = 0,
    #[doc = "1: Select group B"]
    _01 = 1,
    #[doc = "2: Select group C"]
    _10 = 2,
    #[doc = "3: Select group D"]
    _11 = 3,
}
impl From<ICAGRP_A> for u8 {
    #[inline(always)]
    fn from(variant: ICAGRP_A) -> Self {
        variant as _
    }
}
impl ICAGRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAGRP_A {
        match self.bits {
            0 => ICAGRP_A::_00,
            1 => ICAGRP_A::_01,
            2 => ICAGRP_A::_10,
            3 => ICAGRP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ICAGRP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ICAGRP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ICAGRP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ICAGRP_A::_11
    }
}
#[doc = "Field `ICAGRP` writer - GTCCRA Input Capture Group Select"]
pub type ICAGRP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTICCR_SPEC, u8, ICAGRP_A, 2, O>;
impl<'a, const O: u8> ICAGRP_W<'a, O> {
    #[doc = "Select group A"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ICAGRP_A::_00)
    }
    #[doc = "Select group B"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ICAGRP_A::_01)
    }
    #[doc = "Select group C"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ICAGRP_A::_10)
    }
    #[doc = "Select group D"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ICAGRP_A::_11)
    }
}
#[doc = "Field `ICBFA` reader - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFA_R = crate::BitReader<ICBFA_A>;
#[doc = "Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFA_A {
    #[doc = "0: Disable forwarding GTCCRA register compare match/input capture to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRA register compare match/input capture to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFA_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFA_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFA_A {
        match self.bits {
            false => ICBFA_A::_0,
            true => ICBFA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFA_A::_1
    }
}
#[doc = "Field `ICBFA` writer - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFA_A, O>;
impl<'a, const O: u8> ICBFA_W<'a, O> {
    #[doc = "Disable forwarding GTCCRA register compare match/input capture to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFA_A::_0)
    }
    #[doc = "Enable forwarding GTCCRA register compare match/input capture to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFA_A::_1)
    }
}
#[doc = "Field `ICBFB` reader - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFB_R = crate::BitReader<ICBFB_A>;
#[doc = "Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFB_A {
    #[doc = "0: Disable forwarding GTCCRB register compare match/input capture to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRB register compare match/input capture to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFB_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFB_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFB_A {
        match self.bits {
            false => ICBFB_A::_0,
            true => ICBFB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFB_A::_1
    }
}
#[doc = "Field `ICBFB` writer - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFB_A, O>;
impl<'a, const O: u8> ICBFB_W<'a, O> {
    #[doc = "Disable forwarding GTCCRB register compare match/input capture to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFB_A::_0)
    }
    #[doc = "Enable forwarding GTCCRB register compare match/input capture to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFB_A::_1)
    }
}
#[doc = "Field `ICBFC` reader - Forwarding GTCCRC register Compare Match to Other Channel GTCCRB Input Source Capture Enable"]
pub type ICBFC_R = crate::BitReader<ICBFC_A>;
#[doc = "Forwarding GTCCRC register Compare Match to Other Channel GTCCRB Input Source Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFC_A {
    #[doc = "0: Disable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFC_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFC_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFC_A {
        match self.bits {
            false => ICBFC_A::_0,
            true => ICBFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFC_A::_1
    }
}
#[doc = "Field `ICBFC` writer - Forwarding GTCCRC register Compare Match to Other Channel GTCCRB Input Source Capture Enable"]
pub type ICBFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFC_A, O>;
impl<'a, const O: u8> ICBFC_W<'a, O> {
    #[doc = "Disable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFC_A::_0)
    }
    #[doc = "Enable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFC_A::_1)
    }
}
#[doc = "Field `ICBFD` reader - Forwarding GTCCRD register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFD_R = crate::BitReader<ICBFD_A>;
#[doc = "Forwarding GTCCRD register Compare Match to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFD_A {
    #[doc = "0: Disable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFD_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFD_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFD_A {
        match self.bits {
            false => ICBFD_A::_0,
            true => ICBFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFD_A::_1
    }
}
#[doc = "Field `ICBFD` writer - Forwarding GTCCRD register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFD_A, O>;
impl<'a, const O: u8> ICBFD_W<'a, O> {
    #[doc = "Disable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFD_A::_0)
    }
    #[doc = "Enable forwarding GTCCRD register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFD_A::_1)
    }
}
#[doc = "Field `ICBFE` reader - Forwarding GTCCRE register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFE_R = crate::BitReader<ICBFE_A>;
#[doc = "Forwarding GTCCRE register Compare Match to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFE_A {
    #[doc = "0: Disable forwarding GTCCRE register compare match to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRE register compare match to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFE_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFE_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFE_A {
        match self.bits {
            false => ICBFE_A::_0,
            true => ICBFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFE_A::_1
    }
}
#[doc = "Field `ICBFE` writer - Forwarding GTCCRE register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFE_A, O>;
impl<'a, const O: u8> ICBFE_W<'a, O> {
    #[doc = "Disable forwarding GTCCRE register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFE_A::_0)
    }
    #[doc = "Enable forwarding GTCCRE register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFE_A::_1)
    }
}
#[doc = "Field `ICBFF` reader - Forwarding GTCCRF register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFF_R = crate::BitReader<ICBFF_A>;
#[doc = "Forwarding GTCCRF register Compare Match to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFF_A {
    #[doc = "0: Disable forwarding GTCCRF register compare match to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding GTCCRF register compare match to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFF_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFF_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFF_A {
        match self.bits {
            false => ICBFF_A::_0,
            true => ICBFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFF_A::_1
    }
}
#[doc = "Field `ICBFF` writer - Forwarding GTCCRF register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFF_A, O>;
impl<'a, const O: u8> ICBFF_W<'a, O> {
    #[doc = "Disable forwarding GTCCRF register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFF_A::_0)
    }
    #[doc = "Enable forwarding GTCCRF register compare match to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFF_A::_1)
    }
}
#[doc = "Field `ICBFPO` reader - Forwarding Overflow to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFPO_R = crate::BitReader<ICBFPO_A>;
#[doc = "Forwarding Overflow to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFPO_A {
    #[doc = "0: Disable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFPO_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFPO_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFPO_A {
        match self.bits {
            false => ICBFPO_A::_0,
            true => ICBFPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFPO_A::_1
    }
}
#[doc = "Field `ICBFPO` writer - Forwarding Overflow to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFPO_A, O>;
impl<'a, const O: u8> ICBFPO_W<'a, O> {
    #[doc = "Disable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFPO_A::_0)
    }
    #[doc = "Enable forwarding overflow in saw-waves or the crest in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFPO_A::_1)
    }
}
#[doc = "Field `ICBFPU` reader - Forwarding Underflow to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFPU_R = crate::BitReader<ICBFPU_A>;
#[doc = "Forwarding Underflow to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBFPU_A {
    #[doc = "0: Disable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBFPU_A> for bool {
    #[inline(always)]
    fn from(variant: ICBFPU_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBFPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBFPU_A {
        match self.bits {
            false => ICBFPU_A::_0,
            true => ICBFPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBFPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBFPU_A::_1
    }
}
#[doc = "Field `ICBFPU` writer - Forwarding Underflow to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBFPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBFPU_A, O>;
impl<'a, const O: u8> ICBFPU_W<'a, O> {
    #[doc = "Disable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBFPU_A::_0)
    }
    #[doc = "Enable forwarding underflow in saw-waves or the trough in triangle-waves or complementary PWM mode to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBFPU_A::_1)
    }
}
#[doc = "Field `ICBCLK` reader - Forwarding Count Clock to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBCLK_R = crate::BitReader<ICBCLK_A>;
#[doc = "Forwarding Count Clock to Other Channel GTCCRB Input Capture Source Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICBCLK_A {
    #[doc = "0: Disable forwarding count clock to GTCCRB input capture source of other channels"]
    _0 = 0,
    #[doc = "1: Enable forwarding count clock to GTCCRB input capture source of other channels"]
    _1 = 1,
}
impl From<ICBCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ICBCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ICBCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBCLK_A {
        match self.bits {
            false => ICBCLK_A::_0,
            true => ICBCLK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICBCLK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICBCLK_A::_1
    }
}
#[doc = "Field `ICBCLK` writer - Forwarding Count Clock to Other Channel GTCCRB Input Capture Source Enable"]
pub type ICBCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICCR_SPEC, ICBCLK_A, O>;
impl<'a, const O: u8> ICBCLK_W<'a, O> {
    #[doc = "Disable forwarding count clock to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICBCLK_A::_0)
    }
    #[doc = "Enable forwarding count clock to GTCCRB input capture source of other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICBCLK_A::_1)
    }
}
#[doc = "Field `ICBGRP` reader - GTCCRB Input Capture Group Select"]
pub type ICBGRP_R = crate::FieldReader<u8, ICBGRP_A>;
#[doc = "GTCCRB Input Capture Group Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICBGRP_A {
    #[doc = "0: Select group A"]
    _00 = 0,
    #[doc = "1: Select group B"]
    _01 = 1,
    #[doc = "2: Select group C"]
    _10 = 2,
    #[doc = "3: Select group D"]
    _11 = 3,
}
impl From<ICBGRP_A> for u8 {
    #[inline(always)]
    fn from(variant: ICBGRP_A) -> Self {
        variant as _
    }
}
impl ICBGRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBGRP_A {
        match self.bits {
            0 => ICBGRP_A::_00,
            1 => ICBGRP_A::_01,
            2 => ICBGRP_A::_10,
            3 => ICBGRP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ICBGRP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ICBGRP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ICBGRP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ICBGRP_A::_11
    }
}
#[doc = "Field `ICBGRP` writer - GTCCRB Input Capture Group Select"]
pub type ICBGRP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTICCR_SPEC, u8, ICBGRP_A, 2, O>;
impl<'a, const O: u8> ICBGRP_W<'a, O> {
    #[doc = "Select group A"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ICBGRP_A::_00)
    }
    #[doc = "Select group B"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ICBGRP_A::_01)
    }
    #[doc = "Select group C"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ICBGRP_A::_10)
    }
    #[doc = "Select group D"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ICBGRP_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icafa(&self) -> ICAFA_R {
        ICAFA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icafb(&self) -> ICAFB_R {
        ICAFB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Forwarding GTCCRC register Compare Match to Other Channel GTCCRA Input Source Capture Enable"]
    #[inline(always)]
    pub fn icafc(&self) -> ICAFC_R {
        ICAFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Forwarding GTCCRD register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icafd(&self) -> ICAFD_R {
        ICAFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Forwarding GTCCRE register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icafe(&self) -> ICAFE_R {
        ICAFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Forwarding GTCCRF register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icaff(&self) -> ICAFF_R {
        ICAFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Forwarding Overflow to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icafpo(&self) -> ICAFPO_R {
        ICAFPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forwarding Underflow to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icafpu(&self) -> ICAFPU_R {
        ICAFPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Forwarding Count Clock to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    pub fn icaclk(&self) -> ICACLK_R {
        ICACLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 14:15 - GTCCRA Input Capture Group Select"]
    #[inline(always)]
    pub fn icagrp(&self) -> ICAGRP_R {
        ICAGRP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbfa(&self) -> ICBFA_R {
        ICBFA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbfb(&self) -> ICBFB_R {
        ICBFB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Forwarding GTCCRC register Compare Match to Other Channel GTCCRB Input Source Capture Enable"]
    #[inline(always)]
    pub fn icbfc(&self) -> ICBFC_R {
        ICBFC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Forwarding GTCCRD register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbfd(&self) -> ICBFD_R {
        ICBFD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Forwarding GTCCRE register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbfe(&self) -> ICBFE_R {
        ICBFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Forwarding GTCCRF register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbff(&self) -> ICBFF_R {
        ICBFF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Forwarding Overflow to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbfpo(&self) -> ICBFPO_R {
        ICBFPO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Forwarding Underflow to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbfpu(&self) -> ICBFPU_R {
        ICBFPU_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Forwarding Count Clock to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    pub fn icbclk(&self) -> ICBCLK_R {
        ICBCLK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 30:31 - GTCCRB Input Capture Group Select"]
    #[inline(always)]
    pub fn icbgrp(&self) -> ICBGRP_R {
        ICBGRP_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icafa(&mut self) -> ICAFA_W<0> {
        ICAFA_W::new(self)
    }
    #[doc = "Bit 1 - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icafb(&mut self) -> ICAFB_W<1> {
        ICAFB_W::new(self)
    }
    #[doc = "Bit 2 - Forwarding GTCCRC register Compare Match to Other Channel GTCCRA Input Source Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icafc(&mut self) -> ICAFC_W<2> {
        ICAFC_W::new(self)
    }
    #[doc = "Bit 3 - Forwarding GTCCRD register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icafd(&mut self) -> ICAFD_W<3> {
        ICAFD_W::new(self)
    }
    #[doc = "Bit 4 - Forwarding GTCCRE register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icafe(&mut self) -> ICAFE_W<4> {
        ICAFE_W::new(self)
    }
    #[doc = "Bit 5 - Forwarding GTCCRF register Compare Match to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icaff(&mut self) -> ICAFF_W<5> {
        ICAFF_W::new(self)
    }
    #[doc = "Bit 6 - Forwarding Overflow to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icafpo(&mut self) -> ICAFPO_W<6> {
        ICAFPO_W::new(self)
    }
    #[doc = "Bit 7 - Forwarding Underflow to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icafpu(&mut self) -> ICAFPU_W<7> {
        ICAFPU_W::new(self)
    }
    #[doc = "Bit 8 - Forwarding Count Clock to Other Channel GTCCRA Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icaclk(&mut self) -> ICACLK_W<8> {
        ICACLK_W::new(self)
    }
    #[doc = "Bits 14:15 - GTCCRA Input Capture Group Select"]
    #[inline(always)]
    #[must_use]
    pub fn icagrp(&mut self) -> ICAGRP_W<14> {
        ICAGRP_W::new(self)
    }
    #[doc = "Bit 16 - Forwarding GTCCRA register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbfa(&mut self) -> ICBFA_W<16> {
        ICBFA_W::new(self)
    }
    #[doc = "Bit 17 - Forwarding GTCCRB register Compare Match/Input Capture to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbfb(&mut self) -> ICBFB_W<17> {
        ICBFB_W::new(self)
    }
    #[doc = "Bit 18 - Forwarding GTCCRC register Compare Match to Other Channel GTCCRB Input Source Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbfc(&mut self) -> ICBFC_W<18> {
        ICBFC_W::new(self)
    }
    #[doc = "Bit 19 - Forwarding GTCCRD register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbfd(&mut self) -> ICBFD_W<19> {
        ICBFD_W::new(self)
    }
    #[doc = "Bit 20 - Forwarding GTCCRE register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbfe(&mut self) -> ICBFE_W<20> {
        ICBFE_W::new(self)
    }
    #[doc = "Bit 21 - Forwarding GTCCRF register Compare Match to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbff(&mut self) -> ICBFF_W<21> {
        ICBFF_W::new(self)
    }
    #[doc = "Bit 22 - Forwarding Overflow to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbfpo(&mut self) -> ICBFPO_W<22> {
        ICBFPO_W::new(self)
    }
    #[doc = "Bit 23 - Forwarding Underflow to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbfpu(&mut self) -> ICBFPU_W<23> {
        ICBFPU_W::new(self)
    }
    #[doc = "Bit 24 - Forwarding Count Clock to Other Channel GTCCRB Input Capture Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbclk(&mut self) -> ICBCLK_W<24> {
        ICBCLK_W::new(self)
    }
    #[doc = "Bits 30:31 - GTCCRB Input Capture Group Select"]
    #[inline(always)]
    #[must_use]
    pub fn icbgrp(&mut self) -> ICBGRP_W<30> {
        ICBGRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Inter Channel Cooperation Input Capture Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gticcr](index.html) module"]
pub struct GTICCR_SPEC;
impl crate::RegisterSpec for GTICCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gticcr::R](R) reader structure"]
impl crate::Readable for GTICCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gticcr::W](W) writer structure"]
impl crate::Writable for GTICCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTICCR to value 0"]
impl crate::Resettable for GTICCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
