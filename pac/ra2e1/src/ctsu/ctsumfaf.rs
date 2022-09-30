#[doc = "Register `CTSUMFAF` reader"]
pub struct R(crate::R<CTSUMFAF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUMFAF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUMFAF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUMFAF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUMFAF` writer"]
pub struct W(crate::W<CTSUMFAF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUMFAF_SPEC>;
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
impl From<crate::W<CTSUMFAF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUMFAF_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Measurement Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsumfaf](index.html) module"]
pub struct CTSUMFAF_SPEC;
impl crate::RegisterSpec for CTSUMFAF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsumfaf::R](R) reader structure"]
impl crate::Readable for CTSUMFAF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsumfaf::W](W) writer structure"]
impl crate::Writable for CTSUMFAF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUMFAF to value 0x3f"]
impl crate::Resettable for CTSUMFAF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
