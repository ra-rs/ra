#[doc = "Register `ADTRGELC%s` reader"]
pub struct R(crate::R<ADTRGELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTRGELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTRGELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTRGELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTRGELC%s` writer"]
pub struct W(crate::W<ADTRGELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTRGELC_SPEC>;
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
impl From<crate::W<ADTRGELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTRGELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGELC0` reader - ELC Trigger m Enable"]
pub type TRGELC0_R = crate::BitReader<TRGELC0_A>;
#[doc = "ELC Trigger m Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGELC0_A {
    #[doc = "0: Disable ELC Trigger m"]
    _0 = 0,
    #[doc = "1: Enable ELC Trigger m"]
    _1 = 1,
}
impl From<TRGELC0_A> for bool {
    #[inline(always)]
    fn from(variant: TRGELC0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGELC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGELC0_A {
        match self.bits {
            false => TRGELC0_A::_0,
            true => TRGELC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGELC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGELC0_A::_1
    }
}
#[doc = "Field `TRGELC0` writer - ELC Trigger m Enable"]
pub type TRGELC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGELC_SPEC, TRGELC0_A, O>;
impl<'a, const O: u8> TRGELC0_W<'a, O> {
    #[doc = "Disable ELC Trigger m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGELC0_A::_0)
    }
    #[doc = "Enable ELC Trigger m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGELC0_A::_1)
    }
}
#[doc = "Field `TRGELC1` reader - ELC Trigger m Enable"]
pub type TRGELC1_R = crate::BitReader<TRGELC1_A>;
#[doc = "ELC Trigger m Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGELC1_A {
    #[doc = "0: Disable ELC Trigger m"]
    _0 = 0,
    #[doc = "1: Enable ELC Trigger m"]
    _1 = 1,
}
impl From<TRGELC1_A> for bool {
    #[inline(always)]
    fn from(variant: TRGELC1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGELC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGELC1_A {
        match self.bits {
            false => TRGELC1_A::_0,
            true => TRGELC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGELC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGELC1_A::_1
    }
}
#[doc = "Field `TRGELC1` writer - ELC Trigger m Enable"]
pub type TRGELC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGELC_SPEC, TRGELC1_A, O>;
impl<'a, const O: u8> TRGELC1_W<'a, O> {
    #[doc = "Disable ELC Trigger m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGELC1_A::_0)
    }
    #[doc = "Enable ELC Trigger m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGELC1_A::_1)
    }
}
#[doc = "Field `TRGELC2` reader - ELC Trigger m Enable"]
pub type TRGELC2_R = crate::BitReader<TRGELC2_A>;
#[doc = "ELC Trigger m Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGELC2_A {
    #[doc = "0: Disable ELC Trigger m"]
    _0 = 0,
    #[doc = "1: Enable ELC Trigger m"]
    _1 = 1,
}
impl From<TRGELC2_A> for bool {
    #[inline(always)]
    fn from(variant: TRGELC2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGELC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGELC2_A {
        match self.bits {
            false => TRGELC2_A::_0,
            true => TRGELC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGELC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGELC2_A::_1
    }
}
#[doc = "Field `TRGELC2` writer - ELC Trigger m Enable"]
pub type TRGELC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGELC_SPEC, TRGELC2_A, O>;
impl<'a, const O: u8> TRGELC2_W<'a, O> {
    #[doc = "Disable ELC Trigger m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGELC2_A::_0)
    }
    #[doc = "Enable ELC Trigger m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGELC2_A::_1)
    }
}
#[doc = "Field `TRGELC3` reader - ELC Trigger m Enable"]
pub type TRGELC3_R = crate::BitReader<TRGELC3_A>;
#[doc = "ELC Trigger m Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGELC3_A {
    #[doc = "0: Disable ELC Trigger m"]
    _0 = 0,
    #[doc = "1: Enable ELC Trigger m"]
    _1 = 1,
}
impl From<TRGELC3_A> for bool {
    #[inline(always)]
    fn from(variant: TRGELC3_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGELC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGELC3_A {
        match self.bits {
            false => TRGELC3_A::_0,
            true => TRGELC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGELC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGELC3_A::_1
    }
}
#[doc = "Field `TRGELC3` writer - ELC Trigger m Enable"]
pub type TRGELC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGELC_SPEC, TRGELC3_A, O>;
impl<'a, const O: u8> TRGELC3_W<'a, O> {
    #[doc = "Disable ELC Trigger m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGELC3_A::_0)
    }
    #[doc = "Enable ELC Trigger m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGELC3_A::_1)
    }
}
#[doc = "Field `TRGELC4` reader - ELC Trigger m Enable"]
pub type TRGELC4_R = crate::BitReader<TRGELC4_A>;
#[doc = "ELC Trigger m Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGELC4_A {
    #[doc = "0: Disable ELC Trigger m"]
    _0 = 0,
    #[doc = "1: Enable ELC Trigger m"]
    _1 = 1,
}
impl From<TRGELC4_A> for bool {
    #[inline(always)]
    fn from(variant: TRGELC4_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGELC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGELC4_A {
        match self.bits {
            false => TRGELC4_A::_0,
            true => TRGELC4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGELC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGELC4_A::_1
    }
}
#[doc = "Field `TRGELC4` writer - ELC Trigger m Enable"]
pub type TRGELC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGELC_SPEC, TRGELC4_A, O>;
impl<'a, const O: u8> TRGELC4_W<'a, O> {
    #[doc = "Disable ELC Trigger m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGELC4_A::_0)
    }
    #[doc = "Enable ELC Trigger m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGELC4_A::_1)
    }
}
#[doc = "Field `TRGELC5` reader - ELC Trigger m Enable"]
pub type TRGELC5_R = crate::BitReader<TRGELC5_A>;
#[doc = "ELC Trigger m Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGELC5_A {
    #[doc = "0: Disable ELC Trigger m"]
    _0 = 0,
    #[doc = "1: Enable ELC Trigger m"]
    _1 = 1,
}
impl From<TRGELC5_A> for bool {
    #[inline(always)]
    fn from(variant: TRGELC5_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGELC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGELC5_A {
        match self.bits {
            false => TRGELC5_A::_0,
            true => TRGELC5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGELC5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGELC5_A::_1
    }
}
#[doc = "Field `TRGELC5` writer - ELC Trigger m Enable"]
pub type TRGELC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTRGELC_SPEC, TRGELC5_A, O>;
impl<'a, const O: u8> TRGELC5_W<'a, O> {
    #[doc = "Disable ELC Trigger m"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGELC5_A::_0)
    }
    #[doc = "Enable ELC Trigger m"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGELC5_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ELC Trigger m Enable"]
    #[inline(always)]
    pub fn trgelc0(&self) -> TRGELC0_R {
        TRGELC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ELC Trigger m Enable"]
    #[inline(always)]
    pub fn trgelc1(&self) -> TRGELC1_R {
        TRGELC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ELC Trigger m Enable"]
    #[inline(always)]
    pub fn trgelc2(&self) -> TRGELC2_R {
        TRGELC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ELC Trigger m Enable"]
    #[inline(always)]
    pub fn trgelc3(&self) -> TRGELC3_R {
        TRGELC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ELC Trigger m Enable"]
    #[inline(always)]
    pub fn trgelc4(&self) -> TRGELC4_R {
        TRGELC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ELC Trigger m Enable"]
    #[inline(always)]
    pub fn trgelc5(&self) -> TRGELC5_R {
        TRGELC5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ELC Trigger m Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgelc0(&mut self) -> TRGELC0_W<0> {
        TRGELC0_W::new(self)
    }
    #[doc = "Bit 1 - ELC Trigger m Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgelc1(&mut self) -> TRGELC1_W<1> {
        TRGELC1_W::new(self)
    }
    #[doc = "Bit 2 - ELC Trigger m Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgelc2(&mut self) -> TRGELC2_W<2> {
        TRGELC2_W::new(self)
    }
    #[doc = "Bit 3 - ELC Trigger m Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgelc3(&mut self) -> TRGELC3_W<3> {
        TRGELC3_W::new(self)
    }
    #[doc = "Bit 4 - ELC Trigger m Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgelc4(&mut self) -> TRGELC4_W<4> {
        TRGELC4_W::new(self)
    }
    #[doc = "Bit 5 - ELC Trigger m Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgelc5(&mut self) -> TRGELC5_W<5> {
        TRGELC5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ELC Trigger Enable Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtrgelc](index.html) module"]
pub struct ADTRGELC_SPEC;
impl crate::RegisterSpec for ADTRGELC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtrgelc::R](R) reader structure"]
impl crate::Readable for ADTRGELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtrgelc::W](W) writer structure"]
impl crate::Writable for ADTRGELC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADTRGELC%s to value 0"]
impl crate::Resettable for ADTRGELC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
