#[doc = "Register `CTSUMCH0` reader"]
pub struct R(crate::R<CTSUMCH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUMCH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUMCH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUMCH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUMCH0` writer"]
pub struct W(crate::W<CTSUMCH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUMCH0_SPEC>;
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
impl From<crate::W<CTSUMCH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUMCH0_SPEC>) -> Self {
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
#[doc = "CTSU Measurement Channel Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsumch0](index.html) module"]
pub struct CTSUMCH0_SPEC;
impl crate::RegisterSpec for CTSUMCH0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsumch0::R](R) reader structure"]
impl crate::Readable for CTSUMCH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsumch0::W](W) writer structure"]
impl crate::Writable for CTSUMCH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUMCH0 to value 0x1f"]
impl crate::Resettable for CTSUMCH0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
