#[doc = "Register `IELSR%s` reader"]
pub struct R(crate::R<IELSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IELSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IELSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IELSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IELSR%s` writer"]
pub struct W(crate::W<IELSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IELSR_SPEC>;
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
impl From<crate::W<IELSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IELSR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICU Event Link Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ielsr](index.html) module"]
pub struct IELSR_SPEC;
impl crate::RegisterSpec for IELSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ielsr::R](R) reader structure"]
impl crate::Readable for IELSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ielsr::W](W) writer structure"]
impl crate::Writable for IELSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IELSR%s to value 0"]
impl crate::Resettable for IELSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
