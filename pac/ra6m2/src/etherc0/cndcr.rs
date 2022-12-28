#[doc = "Register `CNDCR` reader"]
pub struct R(crate::R<CNDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDCR` writer"]
pub struct W(crate::W<CNDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDCR_SPEC>;
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
impl From<crate::W<CNDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNDCR` reader - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission."]
pub type CNDCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNDCR` writer - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission."]
pub type CNDCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission."]
    #[inline(always)]
    pub fn cndcr(&self) -> CNDCR_R {
        CNDCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission."]
    #[inline(always)]
    #[must_use]
    pub fn cndcr(&mut self) -> CNDCR_W<0> {
        CNDCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Carrier Not Detect Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndcr](index.html) module"]
pub struct CNDCR_SPEC;
impl crate::RegisterSpec for CNDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndcr::R](R) reader structure"]
impl crate::Readable for CNDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndcr::W](W) writer structure"]
impl crate::Writable for CNDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNDCR to value 0"]
impl crate::Resettable for CNDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
