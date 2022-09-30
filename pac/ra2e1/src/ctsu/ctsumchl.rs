#[doc = "Register `CTSUMCHL` reader"]
pub struct R(crate::R<CTSUMCHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUMCHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUMCHL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUMCHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUMCHL` writer"]
pub struct W(crate::W<CTSUMCHL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUMCHL_SPEC>;
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
impl From<crate::W<CTSUMCHL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUMCHL_SPEC>) -> Self {
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
#[doc = "CTSU Measurement Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsumchl](index.html) module"]
pub struct CTSUMCHL_SPEC;
impl crate::RegisterSpec for CTSUMCHL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsumchl::R](R) reader structure"]
impl crate::Readable for CTSUMCHL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsumchl::W](W) writer structure"]
impl crate::Writable for CTSUMCHL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUMCHL to value 0"]
impl crate::Resettable for CTSUMCHL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
