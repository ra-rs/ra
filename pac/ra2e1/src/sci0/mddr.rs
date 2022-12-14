#[doc = "Register `MDDR` reader"]
pub struct R(crate::R<MDDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDDR` writer"]
pub struct W(crate::W<MDDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDDR_SPEC>;
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
impl From<crate::W<MDDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDDR_SPEC>) -> Self {
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
#[doc = "Modulation Duty Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mddr](index.html) module"]
pub struct MDDR_SPEC;
impl crate::RegisterSpec for MDDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mddr::R](R) reader structure"]
impl crate::Readable for MDDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mddr::W](W) writer structure"]
impl crate::Writable for MDDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDDR to value 0xff"]
impl crate::Resettable for MDDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
