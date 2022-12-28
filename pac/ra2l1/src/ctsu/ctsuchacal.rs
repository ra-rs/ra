#[doc = "Register `CTSUCHACAL` reader"]
pub struct R(crate::R<CTSUCHACAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHACAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHACAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHACAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHACAL` writer"]
pub struct W(crate::W<CTSUCHACAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHACAL_SPEC>;
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
impl From<crate::W<CTSUCHACAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHACAL_SPEC>) -> Self {
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
#[doc = "CTSU Channel Enable Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchacal](index.html) module"]
pub struct CTSUCHACAL_SPEC;
impl crate::RegisterSpec for CTSUCHACAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsuchacal::R](R) reader structure"]
impl crate::Readable for CTSUCHACAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchacal::W](W) writer structure"]
impl crate::Writable for CTSUCHACAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHACAL to value 0"]
impl crate::Resettable for CTSUCHACAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
