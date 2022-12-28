#[doc = "Register `AGTCMA` reader"]
pub struct R(crate::R<AGTCMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTCMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTCMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTCMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTCMA` writer"]
pub struct W(crate::W<AGTCMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTCMA_SPEC>;
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
impl From<crate::W<AGTCMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTCMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGTCMA` reader - AGT Compare Match A data is stored.NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
pub type AGTCMA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AGTCMA` writer - AGT Compare Match A data is stored.NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
pub type AGTCMA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AGTCMA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - AGT Compare Match A data is stored.NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcma(&self) -> AGTCMA_R {
        AGTCMA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - AGT Compare Match A data is stored.NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[inline(always)]
    #[must_use]
    pub fn agtcma(&mut self) -> AGTCMA_W<0> {
        AGTCMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Compare Match A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtcma](index.html) module"]
pub struct AGTCMA_SPEC;
impl crate::RegisterSpec for AGTCMA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [agtcma::R](R) reader structure"]
impl crate::Readable for AGTCMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtcma::W](W) writer structure"]
impl crate::Writable for AGTCMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTCMA to value 0xffff"]
impl crate::Resettable for AGTCMA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
