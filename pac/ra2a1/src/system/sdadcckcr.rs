#[doc = "Register `SDADCCKCR` reader"]
pub struct R(crate::R<SDADCCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDADCCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDADCCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDADCCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDADCCKCR` writer"]
pub struct W(crate::W<SDADCCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDADCCKCR_SPEC>;
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
impl From<crate::W<SDADCCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDADCCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDADCCKSEL` reader - 24-bit Sigma-Delta A/D Converter Clock Enable"]
pub type SDADCCKSEL_R = crate::BitReader<SDADCCKSEL_A>;
#[doc = "24-bit Sigma-Delta A/D Converter Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADCCKSEL_A {
    #[doc = "0: 24-bit Sigma-Delta A/D Converter Clock is disabled"]
    _0 = 0,
    #[doc = "1: 24-bit Sigma-Delta A/D Converter Clock is enabled"]
    _1 = 1,
}
impl From<SDADCCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDADCCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADCCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADCCKSEL_A {
        match self.bits {
            false => SDADCCKSEL_A::_0,
            true => SDADCCKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADCCKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADCCKSEL_A::_1
    }
}
#[doc = "Field `SDADCCKSEL` writer - 24-bit Sigma-Delta A/D Converter Clock Enable"]
pub type SDADCCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SDADCCKCR_SPEC, SDADCCKSEL_A, O>;
impl<'a, const O: u8> SDADCCKSEL_W<'a, O> {
    #[doc = "24-bit Sigma-Delta A/D Converter Clock is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDADCCKSEL_A::_0)
    }
    #[doc = "24-bit Sigma-Delta A/D Converter Clock is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDADCCKSEL_A::_1)
    }
}
#[doc = "Field `SDADCCKEN` reader - 24-bit Sigma-Delta A/D Converter Clock Select"]
pub type SDADCCKEN_R = crate::BitReader<SDADCCKEN_A>;
#[doc = "24-bit Sigma-Delta A/D Converter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADCCKEN_A {
    #[doc = "0: MOSC is chosen by a source clock of 24-bit Sigma-Delta A/D Converter Clock"]
    _0 = 0,
    #[doc = "1: HOCO is chosen by a source clock of 24-bit Sigma-Delta A/D Converter Clock"]
    _1 = 1,
}
impl From<SDADCCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDADCCKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADCCKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADCCKEN_A {
        match self.bits {
            false => SDADCCKEN_A::_0,
            true => SDADCCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADCCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADCCKEN_A::_1
    }
}
#[doc = "Field `SDADCCKEN` writer - 24-bit Sigma-Delta A/D Converter Clock Select"]
pub type SDADCCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SDADCCKCR_SPEC, SDADCCKEN_A, O>;
impl<'a, const O: u8> SDADCCKEN_W<'a, O> {
    #[doc = "MOSC is chosen by a source clock of 24-bit Sigma-Delta A/D Converter Clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDADCCKEN_A::_0)
    }
    #[doc = "HOCO is chosen by a source clock of 24-bit Sigma-Delta A/D Converter Clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDADCCKEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - 24-bit Sigma-Delta A/D Converter Clock Enable"]
    #[inline(always)]
    pub fn sdadccksel(&self) -> SDADCCKSEL_R {
        SDADCCKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - 24-bit Sigma-Delta A/D Converter Clock Select"]
    #[inline(always)]
    pub fn sdadccken(&self) -> SDADCCKEN_R {
        SDADCCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 24-bit Sigma-Delta A/D Converter Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdadccksel(&mut self) -> SDADCCKSEL_W<0> {
        SDADCCKSEL_W::new(self)
    }
    #[doc = "Bit 7 - 24-bit Sigma-Delta A/D Converter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdadccken(&mut self) -> SDADCCKEN_W<7> {
        SDADCCKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "24-bit Sigma-Delta A/D Converter Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdadcckcr](index.html) module"]
pub struct SDADCCKCR_SPEC;
impl crate::RegisterSpec for SDADCCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdadcckcr::R](R) reader structure"]
impl crate::Readable for SDADCCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdadcckcr::W](W) writer structure"]
impl crate::Writable for SDADCCKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDADCCKCR to value 0"]
impl crate::Resettable for SDADCCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
