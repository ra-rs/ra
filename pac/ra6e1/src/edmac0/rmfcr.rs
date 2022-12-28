#[doc = "Register `RMFCR` reader"]
pub struct R(crate::R<RMFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMFCR` writer"]
pub struct W(crate::W<RMFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMFCR_SPEC>;
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
impl From<crate::W<RMFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFC` reader - Missed-Frame Counter"]
pub type MFC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFC` writer - Missed-Frame Counter"]
pub type MFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RMFCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Missed-Frame Counter"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Missed-Frame Counter"]
    #[inline(always)]
    #[must_use]
    pub fn mfc(&mut self) -> MFC_W<0> {
        MFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Missed-Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmfcr](index.html) module"]
pub struct RMFCR_SPEC;
impl crate::RegisterSpec for RMFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmfcr::R](R) reader structure"]
impl crate::Readable for RMFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmfcr::W](W) writer structure"]
impl crate::Writable for RMFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMFCR to value 0"]
impl crate::Resettable for RMFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
