#[doc = "Register `L%sBAND` writer"]
pub struct W(crate::W<LBAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LBAND_SPEC>;
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
impl From<crate::W<LBAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LBAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBAND` writer - Limiter m band width parameter"]
pub type LBAND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LBAND_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Limiter m band width parameter"]
    #[inline(always)]
    #[must_use]
    pub fn lband(&mut self) -> LBAND_W<0> {
        LBAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limiter %s Band Width Parameter Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lband](index.html) module"]
pub struct LBAND_SPEC;
impl crate::RegisterSpec for LBAND_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lband::W](W) writer structure"]
impl crate::Writable for LBAND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L%sBAND to value 0"]
impl crate::Resettable for LBAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
