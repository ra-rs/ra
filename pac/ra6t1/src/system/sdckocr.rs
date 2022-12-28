#[doc = "Register `SDCKOCR` reader"]
pub struct R(crate::R<SDCKOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCKOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCKOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCKOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCKOCR` writer"]
pub struct W(crate::W<SDCKOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCKOCR_SPEC>;
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
impl From<crate::W<SDCKOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCKOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDCKOEN` reader - SDCLK Pin Output Control"]
pub type SDCKOEN_R = crate::BitReader<SDCKOEN_A>;
#[doc = "SDCLK Pin Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCKOEN_A {
    #[doc = "0: Disable SDCLK pin output (fixed high)"]
    _0 = 0,
    #[doc = "1: Enable SDCLK pin output"]
    _1 = 1,
}
impl From<SDCKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCKOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCKOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCKOEN_A {
        match self.bits {
            false => SDCKOEN_A::_0,
            true => SDCKOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCKOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCKOEN_A::_1
    }
}
#[doc = "Field `SDCKOEN` writer - SDCLK Pin Output Control"]
pub type SDCKOEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SDCKOCR_SPEC, SDCKOEN_A, O>;
impl<'a, const O: u8> SDCKOEN_W<'a, O> {
    #[doc = "Disable SDCLK pin output (fixed high)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDCKOEN_A::_0)
    }
    #[doc = "Enable SDCLK pin output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCKOEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SDCLK Pin Output Control"]
    #[inline(always)]
    pub fn sdckoen(&self) -> SDCKOEN_R {
        SDCKOEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDCLK Pin Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn sdckoen(&mut self) -> SDCKOEN_W<0> {
        SDCKOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Clock Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdckocr](index.html) module"]
pub struct SDCKOCR_SPEC;
impl crate::RegisterSpec for SDCKOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdckocr::R](R) reader structure"]
impl crate::Readable for SDCKOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdckocr::W](W) writer structure"]
impl crate::Writable for SDCKOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDCKOCR to value 0"]
impl crate::Resettable for SDCKOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
