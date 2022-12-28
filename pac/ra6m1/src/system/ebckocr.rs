#[doc = "Register `EBCKOCR` reader"]
pub struct R(crate::R<EBCKOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBCKOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBCKOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBCKOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBCKOCR` writer"]
pub struct W(crate::W<EBCKOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBCKOCR_SPEC>;
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
impl From<crate::W<EBCKOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBCKOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EBCKOEN` reader - BCLK Pin Output Control"]
pub type EBCKOEN_R = crate::BitReader<EBCKOEN_A>;
#[doc = "BCLK Pin Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBCKOEN_A {
    #[doc = "0: Disable EBCLK pin output (fixed high)"]
    _0 = 0,
    #[doc = "1: Enable EBCLK pin output"]
    _1 = 1,
}
impl From<EBCKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: EBCKOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EBCKOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBCKOEN_A {
        match self.bits {
            false => EBCKOEN_A::_0,
            true => EBCKOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EBCKOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EBCKOEN_A::_1
    }
}
#[doc = "Field `EBCKOEN` writer - BCLK Pin Output Control"]
pub type EBCKOEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, EBCKOCR_SPEC, EBCKOEN_A, O>;
impl<'a, const O: u8> EBCKOEN_W<'a, O> {
    #[doc = "Disable EBCLK pin output (fixed high)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EBCKOEN_A::_0)
    }
    #[doc = "Enable EBCLK pin output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EBCKOEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BCLK Pin Output Control"]
    #[inline(always)]
    pub fn ebckoen(&self) -> EBCKOEN_R {
        EBCKOEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BCLK Pin Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn ebckoen(&mut self) -> EBCKOEN_W<0> {
        EBCKOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Bus Clock Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebckocr](index.html) module"]
pub struct EBCKOCR_SPEC;
impl crate::RegisterSpec for EBCKOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ebckocr::R](R) reader structure"]
impl crate::Readable for EBCKOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebckocr::W](W) writer structure"]
impl crate::Writable for EBCKOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBCKOCR to value 0"]
impl crate::Resettable for EBCKOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
