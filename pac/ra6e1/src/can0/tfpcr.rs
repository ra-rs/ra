#[doc = "Register `TFPCR` writer"]
pub struct W(crate::W<TFPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFPCR_SPEC>;
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
impl From<crate::W<TFPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFPCR_SPEC>) -> Self {
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
#[doc = "Transmit FIFO Pointer Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfpcr](index.html) module"]
pub struct TFPCR_SPEC;
impl crate::RegisterSpec for TFPCR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [tfpcr::W](W) writer structure"]
impl crate::Writable for TFPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFPCR to value 0"]
impl crate::Resettable for TFPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
