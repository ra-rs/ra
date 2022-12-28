#[doc = "Register `TCNT` reader"]
pub struct R(crate::R<TCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCNT` writer"]
pub struct W(crate::W<TCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCNT_SPEC>;
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
impl From<crate::W<TCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCNT_SPEC>) -> Self {
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
#[doc = "Timer Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcnt](index.html) module"]
pub struct TCNT_SPEC;
impl crate::RegisterSpec for TCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcnt::R](R) reader structure"]
impl crate::Readable for TCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcnt::W](W) writer structure"]
impl crate::Writable for TCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT to value 0xff"]
impl crate::Resettable for TCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
