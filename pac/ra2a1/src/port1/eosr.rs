#[doc = "Register `EOSR` reader"]
pub struct R(crate::R<EOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EOSR` writer"]
pub struct W(crate::W<EOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EOSR_SPEC>;
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
impl From<crate::W<EOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EOSR_SPEC>) -> Self {
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
#[doc = "Event output set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eosr](index.html) module"]
pub struct EOSR_SPEC;
impl crate::RegisterSpec for EOSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eosr::R](R) reader structure"]
impl crate::Readable for EOSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eosr::W](W) writer structure"]
impl crate::Writable for EOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EOSR to value 0"]
impl crate::Resettable for EOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
