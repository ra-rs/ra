#[doc = "Register `EORR` reader"]
pub struct R(crate::R<EORR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EORR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EORR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EORR` writer"]
pub struct W(crate::W<EORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EORR_SPEC>;
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
impl From<crate::W<EORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EORR_SPEC>) -> Self {
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
#[doc = "Event output reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eorr](index.html) module"]
pub struct EORR_SPEC;
impl crate::RegisterSpec for EORR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eorr::R](R) reader structure"]
impl crate::Readable for EORR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eorr::W](W) writer structure"]
impl crate::Writable for EORR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EORR to value 0"]
impl crate::Resettable for EORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
