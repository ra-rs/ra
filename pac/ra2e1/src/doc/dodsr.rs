#[doc = "Register `DODSR` reader"]
pub struct R(crate::R<DODSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DODSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DODSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DODSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DODSR` writer"]
pub struct W(crate::W<DODSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DODSR_SPEC>;
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
impl From<crate::W<DODSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DODSR_SPEC>) -> Self {
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
#[doc = "DOC Data Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dodsr](index.html) module"]
pub struct DODSR_SPEC;
impl crate::RegisterSpec for DODSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dodsr::R](R) reader structure"]
impl crate::Readable for DODSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dodsr::W](W) writer structure"]
impl crate::Writable for DODSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DODSR to value 0"]
impl crate::Resettable for DODSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
