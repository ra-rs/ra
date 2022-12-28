#[doc = "Register `ADTRGGPT%s` reader"]
pub struct R(crate::R<ADTRGGPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGGPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGGPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGGPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGGPT%s` writer"]
pub struct W(crate::W<ADTRGGPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGGPT_SPEC>;
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
impl From<crate::W<ADTRGGPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGGPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGGPTA0` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA0_R = crate::BitReader<TRGGPTA0_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA0_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA0_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA0_A {
        match self.bits {
            false => TRGGPTA0_A::_0,
            true => TRGGPTA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA0_A::_1
    }
}
#[doc = "Field `TRGGPTA0` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA0_A, O>;
impl<'a, const O: u8> TRGGPTA0_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA0_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA0_A::_1)
    }
}
#[doc = "Field `TRGGPTA1` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA1_R = crate::BitReader<TRGGPTA1_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA1_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA1_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA1_A {
        match self.bits {
            false => TRGGPTA1_A::_0,
            true => TRGGPTA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA1_A::_1
    }
}
#[doc = "Field `TRGGPTA1` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA1_A, O>;
impl<'a, const O: u8> TRGGPTA1_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA1_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA1_A::_1)
    }
}
#[doc = "Field `TRGGPTA2` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA2_R = crate::BitReader<TRGGPTA2_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA2_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA2_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA2_A {
        match self.bits {
            false => TRGGPTA2_A::_0,
            true => TRGGPTA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA2_A::_1
    }
}
#[doc = "Field `TRGGPTA2` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA2_A, O>;
impl<'a, const O: u8> TRGGPTA2_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA2_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA2_A::_1)
    }
}
#[doc = "Field `TRGGPTA3` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA3_R = crate::BitReader<TRGGPTA3_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA3_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA3_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA3_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA3_A {
        match self.bits {
            false => TRGGPTA3_A::_0,
            true => TRGGPTA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA3_A::_1
    }
}
#[doc = "Field `TRGGPTA3` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA3_A, O>;
impl<'a, const O: u8> TRGGPTA3_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA3_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA3_A::_1)
    }
}
#[doc = "Field `TRGGPTA4` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA4_R = crate::BitReader<TRGGPTA4_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA4_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA4_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA4_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA4_A {
        match self.bits {
            false => TRGGPTA4_A::_0,
            true => TRGGPTA4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA4_A::_1
    }
}
#[doc = "Field `TRGGPTA4` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA4_A, O>;
impl<'a, const O: u8> TRGGPTA4_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA4_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA4_A::_1)
    }
}
#[doc = "Field `TRGGPTA5` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA5_R = crate::BitReader<TRGGPTA5_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA5_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA5_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA5_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA5_A {
        match self.bits {
            false => TRGGPTA5_A::_0,
            true => TRGGPTA5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA5_A::_1
    }
}
#[doc = "Field `TRGGPTA5` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA5_A, O>;
impl<'a, const O: u8> TRGGPTA5_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA5_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA5_A::_1)
    }
}
#[doc = "Field `TRGGPTA6` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA6_R = crate::BitReader<TRGGPTA6_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA6_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA6_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA6_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA6_A {
        match self.bits {
            false => TRGGPTA6_A::_0,
            true => TRGGPTA6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA6_A::_1
    }
}
#[doc = "Field `TRGGPTA6` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA6_A, O>;
impl<'a, const O: u8> TRGGPTA6_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA6_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA6_A::_1)
    }
}
#[doc = "Field `TRGGPTA7` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA7_R = crate::BitReader<TRGGPTA7_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA7_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA7_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA7_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA7_A {
        match self.bits {
            false => TRGGPTA7_A::_0,
            true => TRGGPTA7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA7_A::_1
    }
}
#[doc = "Field `TRGGPTA7` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA7_A, O>;
impl<'a, const O: u8> TRGGPTA7_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA7_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA7_A::_1)
    }
}
#[doc = "Field `TRGGPTA8` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA8_R = crate::BitReader<TRGGPTA8_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA8_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA8_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA8_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA8_A {
        match self.bits {
            false => TRGGPTA8_A::_0,
            true => TRGGPTA8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA8_A::_1
    }
}
#[doc = "Field `TRGGPTA8` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA8_A, O>;
impl<'a, const O: u8> TRGGPTA8_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA8_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA8_A::_1)
    }
}
#[doc = "Field `TRGGPTA9` reader - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA9_R = crate::BitReader<TRGGPTA9_A>;
#[doc = "GPT channel m A/D Conversion Starting Request A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTA9_A {
    #[doc = "0: Disable the A/D conversion stating request A from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request A from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTA9_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTA9_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTA9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTA9_A {
        match self.bits {
            false => TRGGPTA9_A::_0,
            true => TRGGPTA9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTA9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTA9_A::_1
    }
}
#[doc = "Field `TRGGPTA9` writer - GPT channel m A/D Conversion Starting Request A Enable"]
pub type TRGGPTA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTA9_A, O>;
impl<'a, const O: u8> TRGGPTA9_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTA9_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request A from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTA9_A::_1)
    }
}
#[doc = "Field `TRGGPTB0` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB0_R = crate::BitReader<TRGGPTB0_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB0_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB0_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB0_A {
        match self.bits {
            false => TRGGPTB0_A::_0,
            true => TRGGPTB0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB0_A::_1
    }
}
#[doc = "Field `TRGGPTB0` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB0_A, O>;
impl<'a, const O: u8> TRGGPTB0_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB0_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB0_A::_1)
    }
}
#[doc = "Field `TRGGPTB1` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB1_R = crate::BitReader<TRGGPTB1_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB1_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB1_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB1_A {
        match self.bits {
            false => TRGGPTB1_A::_0,
            true => TRGGPTB1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB1_A::_1
    }
}
#[doc = "Field `TRGGPTB1` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB1_A, O>;
impl<'a, const O: u8> TRGGPTB1_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB1_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB1_A::_1)
    }
}
#[doc = "Field `TRGGPTB2` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB2_R = crate::BitReader<TRGGPTB2_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB2_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB2_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB2_A {
        match self.bits {
            false => TRGGPTB2_A::_0,
            true => TRGGPTB2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB2_A::_1
    }
}
#[doc = "Field `TRGGPTB2` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB2_A, O>;
impl<'a, const O: u8> TRGGPTB2_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB2_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB2_A::_1)
    }
}
#[doc = "Field `TRGGPTB3` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB3_R = crate::BitReader<TRGGPTB3_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB3_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB3_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB3_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB3_A {
        match self.bits {
            false => TRGGPTB3_A::_0,
            true => TRGGPTB3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB3_A::_1
    }
}
#[doc = "Field `TRGGPTB3` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB3_A, O>;
impl<'a, const O: u8> TRGGPTB3_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB3_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB3_A::_1)
    }
}
#[doc = "Field `TRGGPTB4` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB4_R = crate::BitReader<TRGGPTB4_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB4_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB4_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB4_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB4_A {
        match self.bits {
            false => TRGGPTB4_A::_0,
            true => TRGGPTB4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB4_A::_1
    }
}
#[doc = "Field `TRGGPTB4` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB4_A, O>;
impl<'a, const O: u8> TRGGPTB4_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB4_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB4_A::_1)
    }
}
#[doc = "Field `TRGGPTB5` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB5_R = crate::BitReader<TRGGPTB5_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB5_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB5_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB5_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB5_A {
        match self.bits {
            false => TRGGPTB5_A::_0,
            true => TRGGPTB5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB5_A::_1
    }
}
#[doc = "Field `TRGGPTB5` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB5_A, O>;
impl<'a, const O: u8> TRGGPTB5_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB5_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB5_A::_1)
    }
}
#[doc = "Field `TRGGPTB6` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB6_R = crate::BitReader<TRGGPTB6_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB6_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB6_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB6_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB6_A {
        match self.bits {
            false => TRGGPTB6_A::_0,
            true => TRGGPTB6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB6_A::_1
    }
}
#[doc = "Field `TRGGPTB6` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB6_A, O>;
impl<'a, const O: u8> TRGGPTB6_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB6_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB6_A::_1)
    }
}
#[doc = "Field `TRGGPTB7` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB7_R = crate::BitReader<TRGGPTB7_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB7_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB7_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB7_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB7_A {
        match self.bits {
            false => TRGGPTB7_A::_0,
            true => TRGGPTB7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB7_A::_1
    }
}
#[doc = "Field `TRGGPTB7` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB7_A, O>;
impl<'a, const O: u8> TRGGPTB7_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB7_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB7_A::_1)
    }
}
#[doc = "Field `TRGGPTB8` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB8_R = crate::BitReader<TRGGPTB8_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB8_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB8_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB8_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB8_A {
        match self.bits {
            false => TRGGPTB8_A::_0,
            true => TRGGPTB8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB8_A::_1
    }
}
#[doc = "Field `TRGGPTB8` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB8_A, O>;
impl<'a, const O: u8> TRGGPTB8_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB8_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB8_A::_1)
    }
}
#[doc = "Field `TRGGPTB9` reader - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB9_R = crate::BitReader<TRGGPTB9_A>;
#[doc = "GPT channel m A/D Conversion Starting Request B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGGPTB9_A {
    #[doc = "0: Disable the A/D conversion stating request B from GPT channel m"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion stating request B from GPT channel m"]
    _1 = 1,
}
impl From<TRGGPTB9_A> for bool {
    #[inline(always)]
    fn from(variant: TRGGPTB9_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGGPTB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGGPTB9_A {
        match self.bits {
            false => TRGGPTB9_A::_0,
            true => TRGGPTB9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGGPTB9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGGPTB9_A::_1
    }
}
#[doc = "Field `TRGGPTB9` writer - GPT channel m A/D Conversion Starting Request B Enable"]
pub type TRGGPTB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGGPT_SPEC, TRGGPTB9_A, O>;
impl<'a, const O: u8> TRGGPTB9_W<'a, O> {
    #[doc = "Disable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGGPTB9_A::_0)
    }
    #[doc = "Enable the A/D conversion stating request B from GPT channel m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGGPTB9_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta0(&self) -> TRGGPTA0_R {
        TRGGPTA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta1(&self) -> TRGGPTA1_R {
        TRGGPTA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta2(&self) -> TRGGPTA2_R {
        TRGGPTA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta3(&self) -> TRGGPTA3_R {
        TRGGPTA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta4(&self) -> TRGGPTA4_R {
        TRGGPTA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta5(&self) -> TRGGPTA5_R {
        TRGGPTA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta6(&self) -> TRGGPTA6_R {
        TRGGPTA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta7(&self) -> TRGGPTA7_R {
        TRGGPTA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta8(&self) -> TRGGPTA8_R {
        TRGGPTA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    pub fn trggpta9(&self) -> TRGGPTA9_R {
        TRGGPTA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb0(&self) -> TRGGPTB0_R {
        TRGGPTB0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb1(&self) -> TRGGPTB1_R {
        TRGGPTB1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb2(&self) -> TRGGPTB2_R {
        TRGGPTB2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb3(&self) -> TRGGPTB3_R {
        TRGGPTB3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb4(&self) -> TRGGPTB4_R {
        TRGGPTB4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb5(&self) -> TRGGPTB5_R {
        TRGGPTB5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb6(&self) -> TRGGPTB6_R {
        TRGGPTB6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb7(&self) -> TRGGPTB7_R {
        TRGGPTB7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb8(&self) -> TRGGPTB8_R {
        TRGGPTB8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    pub fn trggptb9(&self) -> TRGGPTB9_R {
        TRGGPTB9_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta0(&mut self) -> TRGGPTA0_W<0> {
        TRGGPTA0_W::new(self)
    }
    #[doc = "Bit 1 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta1(&mut self) -> TRGGPTA1_W<1> {
        TRGGPTA1_W::new(self)
    }
    #[doc = "Bit 2 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta2(&mut self) -> TRGGPTA2_W<2> {
        TRGGPTA2_W::new(self)
    }
    #[doc = "Bit 3 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta3(&mut self) -> TRGGPTA3_W<3> {
        TRGGPTA3_W::new(self)
    }
    #[doc = "Bit 4 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta4(&mut self) -> TRGGPTA4_W<4> {
        TRGGPTA4_W::new(self)
    }
    #[doc = "Bit 5 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta5(&mut self) -> TRGGPTA5_W<5> {
        TRGGPTA5_W::new(self)
    }
    #[doc = "Bit 6 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta6(&mut self) -> TRGGPTA6_W<6> {
        TRGGPTA6_W::new(self)
    }
    #[doc = "Bit 7 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta7(&mut self) -> TRGGPTA7_W<7> {
        TRGGPTA7_W::new(self)
    }
    #[doc = "Bit 8 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta8(&mut self) -> TRGGPTA8_W<8> {
        TRGGPTA8_W::new(self)
    }
    #[doc = "Bit 9 - GPT channel m A/D Conversion Starting Request A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggpta9(&mut self) -> TRGGPTA9_W<9> {
        TRGGPTA9_W::new(self)
    }
    #[doc = "Bit 16 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb0(&mut self) -> TRGGPTB0_W<16> {
        TRGGPTB0_W::new(self)
    }
    #[doc = "Bit 17 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb1(&mut self) -> TRGGPTB1_W<17> {
        TRGGPTB1_W::new(self)
    }
    #[doc = "Bit 18 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb2(&mut self) -> TRGGPTB2_W<18> {
        TRGGPTB2_W::new(self)
    }
    #[doc = "Bit 19 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb3(&mut self) -> TRGGPTB3_W<19> {
        TRGGPTB3_W::new(self)
    }
    #[doc = "Bit 20 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb4(&mut self) -> TRGGPTB4_W<20> {
        TRGGPTB4_W::new(self)
    }
    #[doc = "Bit 21 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb5(&mut self) -> TRGGPTB5_W<21> {
        TRGGPTB5_W::new(self)
    }
    #[doc = "Bit 22 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb6(&mut self) -> TRGGPTB6_W<22> {
        TRGGPTB6_W::new(self)
    }
    #[doc = "Bit 23 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb7(&mut self) -> TRGGPTB7_W<23> {
        TRGGPTB7_W::new(self)
    }
    #[doc = "Bit 24 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb8(&mut self) -> TRGGPTB8_W<24> {
        TRGGPTB8_W::new(self)
    }
    #[doc = "Bit 25 - GPT channel m A/D Conversion Starting Request B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trggptb9(&mut self) -> TRGGPTB9_W<25> {
        TRGGPTB9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Trigger Enable Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrggpt](index.html) module"]
pub struct ADTRGGPT_SPEC;
impl crate::RegisterSpec for ADTRGGPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrggpt::R](R) reader structure"]
impl crate::Readable for ADTRGGPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrggpt::W](W) writer structure"]
impl crate::Writable for ADTRGGPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGGPT%s to value 0"]
impl crate::Resettable for ADTRGGPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
