#[doc = "Register `GTINTAD` reader"]
pub struct R(crate::R<GTINTAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTINTAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTINTAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTINTAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTINTAD` writer"]
pub struct W(crate::W<GTINTAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTINTAD_SPEC>;
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
impl From<crate::W<GTINTAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTINTAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCFA` reader - GTCCRA Register Compare Match/Input Capture Source Synchronous Clear Enable"]
pub type SCFA_R = crate::BitReader<SCFA_A>;
#[doc = "GTCCRA Register Compare Match/Input Capture Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFA_A {
    #[doc = "0: Disable use of GTCCRA register compare match/input capture as a clear factor for other channels."]
    _0 = 0,
    #[doc = "1: Enable use of GTCCRA register compare match/input capture as a clear factor for other channels."]
    _1 = 1,
}
impl From<SCFA_A> for bool {
    #[inline(always)]
    fn from(variant: SCFA_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFA_A {
        match self.bits {
            false => SCFA_A::_0,
            true => SCFA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFA_A::_1
    }
}
#[doc = "Field `SCFA` writer - GTCCRA Register Compare Match/Input Capture Source Synchronous Clear Enable"]
pub type SCFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFA_A, O>;
impl<'a, const O: u8> SCFA_W<'a, O> {
    #[doc = "Disable use of GTCCRA register compare match/input capture as a clear factor for other channels."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFA_A::_0)
    }
    #[doc = "Enable use of GTCCRA register compare match/input capture as a clear factor for other channels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFA_A::_1)
    }
}
#[doc = "Field `SCFB` reader - GTCCRB Register Compare Match/Input Capture Source Synchronous Clear Enable"]
pub type SCFB_R = crate::BitReader<SCFB_A>;
#[doc = "GTCCRB Register Compare Match/Input Capture Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFB_A {
    #[doc = "0: Disable use of GTCCRB register compare match/input capture as a clear factor for other channels."]
    _0 = 0,
    #[doc = "1: Enable use of GTCCRB register compare match/input capture as a clear factor for other channels."]
    _1 = 1,
}
impl From<SCFB_A> for bool {
    #[inline(always)]
    fn from(variant: SCFB_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFB_A {
        match self.bits {
            false => SCFB_A::_0,
            true => SCFB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFB_A::_1
    }
}
#[doc = "Field `SCFB` writer - GTCCRB Register Compare Match/Input Capture Source Synchronous Clear Enable"]
pub type SCFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFB_A, O>;
impl<'a, const O: u8> SCFB_W<'a, O> {
    #[doc = "Disable use of GTCCRB register compare match/input capture as a clear factor for other channels."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFB_A::_0)
    }
    #[doc = "Enable use of GTCCRB register compare match/input capture as a clear factor for other channels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFB_A::_1)
    }
}
#[doc = "Field `SCFC` reader - GTCCRC Register Compare Match Source Synchronous Clear Enable"]
pub type SCFC_R = crate::BitReader<SCFC_A>;
#[doc = "GTCCRC Register Compare Match Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFC_A {
    #[doc = "0: Disable use of GTCCRC register compare match as a clear factor for other channels."]
    _0 = 0,
    #[doc = "1: Enable use of GTCCRC register compare match as a clear factor for other channels."]
    _1 = 1,
}
impl From<SCFC_A> for bool {
    #[inline(always)]
    fn from(variant: SCFC_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFC_A {
        match self.bits {
            false => SCFC_A::_0,
            true => SCFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFC_A::_1
    }
}
#[doc = "Field `SCFC` writer - GTCCRC Register Compare Match Source Synchronous Clear Enable"]
pub type SCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFC_A, O>;
impl<'a, const O: u8> SCFC_W<'a, O> {
    #[doc = "Disable use of GTCCRC register compare match as a clear factor for other channels."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFC_A::_0)
    }
    #[doc = "Enable use of GTCCRC register compare match as a clear factor for other channels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFC_A::_1)
    }
}
#[doc = "Field `SCFD` reader - GTCCRD Register Compare Match Source Synchronous Clear Enable"]
pub type SCFD_R = crate::BitReader<SCFD_A>;
#[doc = "GTCCRD Register Compare Match Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFD_A {
    #[doc = "0: Disable use of GTCCRD register compare match as a clear factor for other channels."]
    _0 = 0,
    #[doc = "1: Enable use of GTCCRD register compare match as a clear factor for other channels."]
    _1 = 1,
}
impl From<SCFD_A> for bool {
    #[inline(always)]
    fn from(variant: SCFD_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFD_A {
        match self.bits {
            false => SCFD_A::_0,
            true => SCFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFD_A::_1
    }
}
#[doc = "Field `SCFD` writer - GTCCRD Register Compare Match Source Synchronous Clear Enable"]
pub type SCFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFD_A, O>;
impl<'a, const O: u8> SCFD_W<'a, O> {
    #[doc = "Disable use of GTCCRD register compare match as a clear factor for other channels."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFD_A::_0)
    }
    #[doc = "Enable use of GTCCRD register compare match as a clear factor for other channels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFD_A::_1)
    }
}
#[doc = "Field `SCFE` reader - GTCCRE Register Compare Match Source Synchronous Clear Enable"]
pub type SCFE_R = crate::BitReader<SCFE_A>;
#[doc = "GTCCRE Register Compare Match Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFE_A {
    #[doc = "0: Disable use of GTCCRE register compare match as a clear factor for other channels."]
    _0 = 0,
    #[doc = "1: Enable use of GTCCRE register compare match as a clear factor for other channels"]
    _1 = 1,
}
impl From<SCFE_A> for bool {
    #[inline(always)]
    fn from(variant: SCFE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFE_A {
        match self.bits {
            false => SCFE_A::_0,
            true => SCFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFE_A::_1
    }
}
#[doc = "Field `SCFE` writer - GTCCRE Register Compare Match Source Synchronous Clear Enable"]
pub type SCFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFE_A, O>;
impl<'a, const O: u8> SCFE_W<'a, O> {
    #[doc = "Disable use of GTCCRE register compare match as a clear factor for other channels."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFE_A::_0)
    }
    #[doc = "Enable use of GTCCRE register compare match as a clear factor for other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFE_A::_1)
    }
}
#[doc = "Field `SCFF` reader - GTCCRF Register Compare Match Source Synchronous Clear Enable"]
pub type SCFF_R = crate::BitReader<SCFF_A>;
#[doc = "GTCCRF Register Compare Match Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFF_A {
    #[doc = "0: Disable use of GTCCRF register compare match as a clear factor for other channels."]
    _0 = 0,
    #[doc = "1: Enable use of GTCCRF register compare match as a clear factor for other channels"]
    _1 = 1,
}
impl From<SCFF_A> for bool {
    #[inline(always)]
    fn from(variant: SCFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFF_A {
        match self.bits {
            false => SCFF_A::_0,
            true => SCFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFF_A::_1
    }
}
#[doc = "Field `SCFF` writer - GTCCRF Register Compare Match Source Synchronous Clear Enable"]
pub type SCFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFF_A, O>;
impl<'a, const O: u8> SCFF_W<'a, O> {
    #[doc = "Disable use of GTCCRF register compare match as a clear factor for other channels."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFF_A::_0)
    }
    #[doc = "Enable use of GTCCRF register compare match as a clear factor for other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFF_A::_1)
    }
}
#[doc = "Field `SCFPO` reader - Overflow Source Synchronous Clear Enable"]
pub type SCFPO_R = crate::BitReader<SCFPO_A>;
#[doc = "Overflow Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFPO_A {
    #[doc = "0: Disable use of overflow as a clear factor for other channels."]
    _0 = 0,
    #[doc = "1: Enable use of overflow as a clear factor for other channels."]
    _1 = 1,
}
impl From<SCFPO_A> for bool {
    #[inline(always)]
    fn from(variant: SCFPO_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFPO_A {
        match self.bits {
            false => SCFPO_A::_0,
            true => SCFPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFPO_A::_1
    }
}
#[doc = "Field `SCFPO` writer - Overflow Source Synchronous Clear Enable"]
pub type SCFPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFPO_A, O>;
impl<'a, const O: u8> SCFPO_W<'a, O> {
    #[doc = "Disable use of overflow as a clear factor for other channels."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFPO_A::_0)
    }
    #[doc = "Enable use of overflow as a clear factor for other channels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFPO_A::_1)
    }
}
#[doc = "Field `SCFPU` reader - Underflow Source Synchronous Clear Enable"]
pub type SCFPU_R = crate::BitReader<SCFPU_A>;
#[doc = "Underflow Source Synchronous Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCFPU_A {
    #[doc = "0: Disable use of underflow as a clear factor for other channels"]
    _0 = 0,
    #[doc = "1: Enable use of underflow as a clear factor for other channels"]
    _1 = 1,
}
impl From<SCFPU_A> for bool {
    #[inline(always)]
    fn from(variant: SCFPU_A) -> Self {
        variant as u8 != 0
    }
}
impl SCFPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCFPU_A {
        match self.bits {
            false => SCFPU_A::_0,
            true => SCFPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCFPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCFPU_A::_1
    }
}
#[doc = "Field `SCFPU` writer - Underflow Source Synchronous Clear Enable"]
pub type SCFPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, SCFPU_A, O>;
impl<'a, const O: u8> SCFPU_W<'a, O> {
    #[doc = "Disable use of underflow as a clear factor for other channels"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCFPU_A::_0)
    }
    #[doc = "Enable use of underflow as a clear factor for other channels"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCFPU_A::_1)
    }
}
#[doc = "Field `ADTRAUEN` reader - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
pub type ADTRAUEN_R = crate::BitReader<ADTRAUEN_A>;
#[doc = "GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRAUEN_A {
    #[doc = "0: A/D conversion start request is disabled."]
    _0 = 0,
    #[doc = "1: A/D conversion start request is enabled."]
    _1 = 1,
}
impl From<ADTRAUEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRAUEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRAUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRAUEN_A {
        match self.bits {
            false => ADTRAUEN_A::_0,
            true => ADTRAUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRAUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRAUEN_A::_1
    }
}
#[doc = "Field `ADTRAUEN` writer - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
pub type ADTRAUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRAUEN_A, O>;
impl<'a, const O: u8> ADTRAUEN_W<'a, O> {
    #[doc = "A/D conversion start request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRAUEN_A::_0)
    }
    #[doc = "A/D conversion start request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRAUEN_A::_1)
    }
}
#[doc = "Field `ADTRADEN` reader - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
pub type ADTRADEN_R = crate::BitReader<ADTRADEN_A>;
#[doc = "GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRADEN_A {
    #[doc = "0: A/D conversion start request is disabled."]
    _0 = 0,
    #[doc = "1: A/D conversion start request is enabled."]
    _1 = 1,
}
impl From<ADTRADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRADEN_A {
        match self.bits {
            false => ADTRADEN_A::_0,
            true => ADTRADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRADEN_A::_1
    }
}
#[doc = "Field `ADTRADEN` writer - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
pub type ADTRADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRADEN_A, O>;
impl<'a, const O: u8> ADTRADEN_W<'a, O> {
    #[doc = "A/D conversion start request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRADEN_A::_0)
    }
    #[doc = "A/D conversion start request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRADEN_A::_1)
    }
}
#[doc = "Field `ADTRBUEN` reader - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
pub type ADTRBUEN_R = crate::BitReader<ADTRBUEN_A>;
#[doc = "GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBUEN_A {
    #[doc = "0: A/D conversion start request is disabled."]
    _0 = 0,
    #[doc = "1: A/D conversion start request is enabled."]
    _1 = 1,
}
impl From<ADTRBUEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBUEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRBUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRBUEN_A {
        match self.bits {
            false => ADTRBUEN_A::_0,
            true => ADTRBUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBUEN_A::_1
    }
}
#[doc = "Field `ADTRBUEN` writer - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
pub type ADTRBUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRBUEN_A, O>;
impl<'a, const O: u8> ADTRBUEN_W<'a, O> {
    #[doc = "A/D conversion start request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRBUEN_A::_0)
    }
    #[doc = "A/D conversion start request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRBUEN_A::_1)
    }
}
#[doc = "Field `ADTRBDEN` reader - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
pub type ADTRBDEN_R = crate::BitReader<ADTRBDEN_A>;
#[doc = "GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBDEN_A {
    #[doc = "0: A/D conversion start request is disabled."]
    _0 = 0,
    #[doc = "1: A/D conversion start request is enabled."]
    _1 = 1,
}
impl From<ADTRBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRBDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRBDEN_A {
        match self.bits {
            false => ADTRBDEN_A::_0,
            true => ADTRBDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBDEN_A::_1
    }
}
#[doc = "Field `ADTRBDEN` writer - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
pub type ADTRBDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRBDEN_A, O>;
impl<'a, const O: u8> ADTRBDEN_W<'a, O> {
    #[doc = "A/D conversion start request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRBDEN_A::_0)
    }
    #[doc = "A/D conversion start request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRBDEN_A::_1)
    }
}
#[doc = "Field `GRP` reader - Output Disable Source Select"]
pub type GRP_R = crate::FieldReader<u8, GRP_A>;
#[doc = "Output Disable Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GRP_A {
    #[doc = "0: Group A output disable source is selected Group B output disable source is selected Group C output disable source is selected Group D output disable source is selected"]
    _00 = 0,
}
impl From<GRP_A> for u8 {
    #[inline(always)]
    fn from(variant: GRP_A) -> Self {
        variant as _
    }
}
impl GRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GRP_A> {
        match self.bits {
            0 => Some(GRP_A::_00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GRP_A::_00
    }
}
#[doc = "Field `GRP` writer - Output Disable Source Select"]
pub type GRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTINTAD_SPEC, u8, GRP_A, 2, O>;
impl<'a, const O: u8> GRP_W<'a, O> {
    #[doc = "Group A output disable source is selected Group B output disable source is selected Group C output disable source is selected Group D output disable source is selected"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GRP_A::_00)
    }
}
#[doc = "Field `GRPDTE` reader - Dead Time Error Output Disable Request Enable"]
pub type GRPDTE_R = crate::BitReader<GRPDTE_A>;
#[doc = "Dead Time Error Output Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPDTE_A {
    #[doc = "0: Dead time error output disable request is disabled."]
    _0 = 0,
    #[doc = "1: Dead time error output disable request is enabled."]
    _1 = 1,
}
impl From<GRPDTE_A> for bool {
    #[inline(always)]
    fn from(variant: GRPDTE_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPDTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPDTE_A {
        match self.bits {
            false => GRPDTE_A::_0,
            true => GRPDTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPDTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPDTE_A::_1
    }
}
#[doc = "Field `GRPDTE` writer - Dead Time Error Output Disable Request Enable"]
pub type GRPDTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPDTE_A, O>;
impl<'a, const O: u8> GRPDTE_W<'a, O> {
    #[doc = "Dead time error output disable request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPDTE_A::_0)
    }
    #[doc = "Dead time error output disable request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPDTE_A::_1)
    }
}
#[doc = "Field `GRPABH` reader - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_R = crate::BitReader<GRPABH_A>;
#[doc = "Same Time Output Level High Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABH_A {
    #[doc = "0: Same time output level high disable request disabled"]
    _0 = 0,
    #[doc = "1: Same time output level high disable request enabled"]
    _1 = 1,
}
impl From<GRPABH_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABH_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPABH_A {
        match self.bits {
            false => GRPABH_A::_0,
            true => GRPABH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABH_A::_1
    }
}
#[doc = "Field `GRPABH` writer - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPABH_A, O>;
impl<'a, const O: u8> GRPABH_W<'a, O> {
    #[doc = "Same time output level high disable request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPABH_A::_0)
    }
    #[doc = "Same time output level high disable request enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPABH_A::_1)
    }
}
#[doc = "Field `GRPABL` reader - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_R = crate::BitReader<GRPABL_A>;
#[doc = "Same Time Output Level Low Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABL_A {
    #[doc = "0: Same time output level low disable request disabled"]
    _0 = 0,
    #[doc = "1: Same time output level low disable request enabled"]
    _1 = 1,
}
impl From<GRPABL_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABL_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPABL_A {
        match self.bits {
            false => GRPABL_A::_0,
            true => GRPABL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABL_A::_1
    }
}
#[doc = "Field `GRPABL` writer - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPABL_A, O>;
impl<'a, const O: u8> GRPABL_W<'a, O> {
    #[doc = "Same time output level low disable request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPABL_A::_0)
    }
    #[doc = "Same time output level low disable request enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPABL_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - GTCCRA Register Compare Match/Input Capture Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scfa(&self) -> SCFA_R {
        SCFA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTCCRB Register Compare Match/Input Capture Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scfb(&self) -> SCFB_R {
        SCFB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTCCRC Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scfc(&self) -> SCFC_R {
        SCFC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTCCRD Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scfd(&self) -> SCFD_R {
        SCFD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTCCRE Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scfe(&self) -> SCFE_R {
        SCFE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTCCRF Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scff(&self) -> SCFF_R {
        SCFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Overflow Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scfpo(&self) -> SCFPO_R {
        SCFPO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Underflow Source Synchronous Clear Enable"]
    #[inline(always)]
    pub fn scfpu(&self) -> SCFPU_R {
        SCFPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    pub fn adtrauen(&self) -> ADTRAUEN_R {
        ADTRAUEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    pub fn adtraden(&self) -> ADTRADEN_R {
        ADTRADEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    pub fn adtrbuen(&self) -> ADTRBUEN_R {
        ADTRBUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    pub fn adtrbden(&self) -> ADTRBDEN_R {
        ADTRBDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Dead Time Error Output Disable Request Enable"]
    #[inline(always)]
    pub fn grpdte(&self) -> GRPDTE_R {
        GRPDTE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(&self) -> GRPABH_R {
        GRPABH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(&self) -> GRPABL_R {
        GRPABL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - GTCCRA Register Compare Match/Input Capture Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfa(&mut self) -> SCFA_W<8> {
        SCFA_W::new(self)
    }
    #[doc = "Bit 9 - GTCCRB Register Compare Match/Input Capture Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfb(&mut self) -> SCFB_W<9> {
        SCFB_W::new(self)
    }
    #[doc = "Bit 10 - GTCCRC Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfc(&mut self) -> SCFC_W<10> {
        SCFC_W::new(self)
    }
    #[doc = "Bit 11 - GTCCRD Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfd(&mut self) -> SCFD_W<11> {
        SCFD_W::new(self)
    }
    #[doc = "Bit 12 - GTCCRE Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfe(&mut self) -> SCFE_W<12> {
        SCFE_W::new(self)
    }
    #[doc = "Bit 13 - GTCCRF Register Compare Match Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scff(&mut self) -> SCFF_W<13> {
        SCFF_W::new(self)
    }
    #[doc = "Bit 14 - Overflow Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfpo(&mut self) -> SCFPO_W<14> {
        SCFPO_W::new(self)
    }
    #[doc = "Bit 15 - Underflow Source Synchronous Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfpu(&mut self) -> SCFPU_W<15> {
        SCFPU_W::new(self)
    }
    #[doc = "Bit 16 - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtrauen(&mut self) -> ADTRAUEN_W<16> {
        ADTRAUEN_W::new(self)
    }
    #[doc = "Bit 17 - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtraden(&mut self) -> ADTRADEN_W<17> {
        ADTRADEN_W::new(self)
    }
    #[doc = "Bit 18 - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtrbuen(&mut self) -> ADTRBUEN_W<18> {
        ADTRBUEN_W::new(self)
    }
    #[doc = "Bit 19 - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtrbden(&mut self) -> ADTRBDEN_W<19> {
        ADTRBDEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn grp(&mut self) -> GRP_W<24> {
        GRP_W::new(self)
    }
    #[doc = "Bit 28 - Dead Time Error Output Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpdte(&mut self) -> GRPDTE_W<28> {
        GRPDTE_W::new(self)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabh(&mut self) -> GRPABH_W<29> {
        GRPABH_W::new(self)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabl(&mut self) -> GRPABL_W<30> {
        GRPABL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Interrupt Output Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtintad](index.html) module"]
pub struct GTINTAD_SPEC;
impl crate::RegisterSpec for GTINTAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtintad::R](R) reader structure"]
impl crate::Readable for GTINTAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtintad::W](W) writer structure"]
impl crate::Writable for GTINTAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTINTAD to value 0"]
impl crate::Resettable for GTINTAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
