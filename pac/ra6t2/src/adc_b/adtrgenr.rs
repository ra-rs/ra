#[doc = "Register `ADTRGENR` reader"]
pub struct R(crate::R<ADTRGENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGENR` writer"]
pub struct W(crate::W<ADTRGENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGENR_SPEC>;
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
impl From<crate::W<ADTRGENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STTRGEN0` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN0_R = crate::BitReader<STTRGEN0_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN0_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN0_A {
        match self.bits {
            false => STTRGEN0_A::_0,
            true => STTRGEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN0_A::_1
    }
}
#[doc = "Field `STTRGEN0` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN0_A, O>;
impl<'a, const O: u8> STTRGEN0_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN0_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN0_A::_1)
    }
}
#[doc = "Field `STTRGEN1` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN1_R = crate::BitReader<STTRGEN1_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN1_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN1_A {
        match self.bits {
            false => STTRGEN1_A::_0,
            true => STTRGEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN1_A::_1
    }
}
#[doc = "Field `STTRGEN1` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN1_A, O>;
impl<'a, const O: u8> STTRGEN1_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN1_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN1_A::_1)
    }
}
#[doc = "Field `STTRGEN2` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN2_R = crate::BitReader<STTRGEN2_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN2_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN2_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN2_A {
        match self.bits {
            false => STTRGEN2_A::_0,
            true => STTRGEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN2_A::_1
    }
}
#[doc = "Field `STTRGEN2` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN2_A, O>;
impl<'a, const O: u8> STTRGEN2_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN2_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN2_A::_1)
    }
}
#[doc = "Field `STTRGEN3` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN3_R = crate::BitReader<STTRGEN3_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN3_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN3_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN3_A {
        match self.bits {
            false => STTRGEN3_A::_0,
            true => STTRGEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN3_A::_1
    }
}
#[doc = "Field `STTRGEN3` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN3_A, O>;
impl<'a, const O: u8> STTRGEN3_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN3_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN3_A::_1)
    }
}
#[doc = "Field `STTRGEN4` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN4_R = crate::BitReader<STTRGEN4_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN4_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN4_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN4_A {
        match self.bits {
            false => STTRGEN4_A::_0,
            true => STTRGEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN4_A::_1
    }
}
#[doc = "Field `STTRGEN4` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN4_A, O>;
impl<'a, const O: u8> STTRGEN4_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN4_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN4_A::_1)
    }
}
#[doc = "Field `STTRGEN5` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN5_R = crate::BitReader<STTRGEN5_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN5_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN5_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN5_A {
        match self.bits {
            false => STTRGEN5_A::_0,
            true => STTRGEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN5_A::_1
    }
}
#[doc = "Field `STTRGEN5` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN5_A, O>;
impl<'a, const O: u8> STTRGEN5_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN5_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN5_A::_1)
    }
}
#[doc = "Field `STTRGEN6` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN6_R = crate::BitReader<STTRGEN6_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN6_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN6_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN6_A {
        match self.bits {
            false => STTRGEN6_A::_0,
            true => STTRGEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN6_A::_1
    }
}
#[doc = "Field `STTRGEN6` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN6_A, O>;
impl<'a, const O: u8> STTRGEN6_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN6_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN6_A::_1)
    }
}
#[doc = "Field `STTRGEN7` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN7_R = crate::BitReader<STTRGEN7_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN7_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN7_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN7_A {
        match self.bits {
            false => STTRGEN7_A::_0,
            true => STTRGEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN7_A::_1
    }
}
#[doc = "Field `STTRGEN7` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN7_A, O>;
impl<'a, const O: u8> STTRGEN7_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN7_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN7_A::_1)
    }
}
#[doc = "Field `STTRGEN8` reader - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN8_R = crate::BitReader<STTRGEN8_A>;
#[doc = "Scan Group n A/D Conversion Start Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTRGEN8_A {
    #[doc = "0: Disable the A/D conversion start trigger"]
    _0 = 0,
    #[doc = "1: Enable the A/D conversion start trigger"]
    _1 = 1,
}
impl From<STTRGEN8_A> for bool {
    #[inline(always)]
    fn from(variant: STTRGEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl STTRGEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STTRGEN8_A {
        match self.bits {
            false => STTRGEN8_A::_0,
            true => STTRGEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STTRGEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STTRGEN8_A::_1
    }
}
#[doc = "Field `STTRGEN8` writer - Scan Group n A/D Conversion Start Trigger Enable"]
pub type STTRGEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGENR_SPEC, STTRGEN8_A, O>;
impl<'a, const O: u8> STTRGEN8_W<'a, O> {
    #[doc = "Disable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTRGEN8_A::_0)
    }
    #[doc = "Enable the A/D conversion start trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTRGEN8_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen0(&self) -> STTRGEN0_R {
        STTRGEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen1(&self) -> STTRGEN1_R {
        STTRGEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen2(&self) -> STTRGEN2_R {
        STTRGEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen3(&self) -> STTRGEN3_R {
        STTRGEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen4(&self) -> STTRGEN4_R {
        STTRGEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen5(&self) -> STTRGEN5_R {
        STTRGEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen6(&self) -> STTRGEN6_R {
        STTRGEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen7(&self) -> STTRGEN7_R {
        STTRGEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    pub fn sttrgen8(&self) -> STTRGEN8_R {
        STTRGEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen0(&mut self) -> STTRGEN0_W<0> {
        STTRGEN0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen1(&mut self) -> STTRGEN1_W<1> {
        STTRGEN1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen2(&mut self) -> STTRGEN2_W<2> {
        STTRGEN2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen3(&mut self) -> STTRGEN3_W<3> {
        STTRGEN3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen4(&mut self) -> STTRGEN4_W<4> {
        STTRGEN4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen5(&mut self) -> STTRGEN5_W<5> {
        STTRGEN5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen6(&mut self) -> STTRGEN6_W<6> {
        STTRGEN6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen7(&mut self) -> STTRGEN7_W<7> {
        STTRGEN7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n A/D Conversion Start Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sttrgen8(&mut self) -> STTRGEN8_W<8> {
        STTRGEN8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Trigger Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgenr](index.html) module"]
pub struct ADTRGENR_SPEC;
impl crate::RegisterSpec for ADTRGENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgenr::R](R) reader structure"]
impl crate::Readable for ADTRGENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgenr::W](W) writer structure"]
impl crate::Writable for ADTRGENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGENR to value 0"]
impl crate::Resettable for ADTRGENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
