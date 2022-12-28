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
impl W {
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
