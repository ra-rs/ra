#[doc = "Register `TPRE` reader"]
pub struct R(crate::R<TPRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPRE` writer"]
pub struct W(crate::W<TPRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPRE_SPEC>;
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
impl From<crate::W<TPRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPRE_SPEC>) -> Self {
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
#[doc = "Timer Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpre](index.html) module"]
pub struct TPRE_SPEC;
impl crate::RegisterSpec for TPRE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tpre::R](R) reader structure"]
impl crate::Readable for TPRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpre::W](W) writer structure"]
impl crate::Writable for TPRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPRE to value 0xff"]
impl crate::Resettable for TPRE_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
