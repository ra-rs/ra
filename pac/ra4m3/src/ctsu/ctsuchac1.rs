#[doc = "Register `CTSUCHAC1` reader"]
pub struct R(crate::R<CTSUCHAC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHAC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHAC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHAC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHAC1` writer"]
pub struct W(crate::W<CTSUCHAC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHAC1_SPEC>;
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
impl From<crate::W<CTSUCHAC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHAC1_SPEC>) -> Self {
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
#[doc = "CTSU Channel Enable Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchac1](index.html) module"]
pub struct CTSUCHAC1_SPEC;
impl crate::RegisterSpec for CTSUCHAC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsuchac1::R](R) reader structure"]
impl crate::Readable for CTSUCHAC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchac1::W](W) writer structure"]
impl crate::Writable for CTSUCHAC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC1 to value 0"]
impl crate::Resettable for CTSUCHAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
