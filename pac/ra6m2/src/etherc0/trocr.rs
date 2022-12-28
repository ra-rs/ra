#[doc = "Register `TROCR` reader"]
pub struct R(crate::R<TROCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TROCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TROCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TROCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TROCR` writer"]
pub struct W(crate::W<TROCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TROCR_SPEC>;
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
impl From<crate::W<TROCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TROCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TROCR` reader - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted."]
pub type TROCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TROCR` writer - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted."]
pub type TROCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TROCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted."]
    #[inline(always)]
    pub fn trocr(&self) -> TROCR_R {
        TROCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted."]
    #[inline(always)]
    #[must_use]
    pub fn trocr(&mut self) -> TROCR_W<0> {
        TROCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Retry Over Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trocr](index.html) module"]
pub struct TROCR_SPEC;
impl crate::RegisterSpec for TROCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trocr::R](R) reader structure"]
impl crate::Readable for TROCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trocr::W](W) writer structure"]
impl crate::Writable for TROCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TROCR to value 0"]
impl crate::Resettable for TROCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
