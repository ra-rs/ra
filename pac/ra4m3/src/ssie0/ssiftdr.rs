#[doc = "Register `SSIFTDR` writer"]
pub struct W(crate::W<SSIFTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIFTDR_SPEC>;
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
impl From<crate::W<SSIFTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSIFTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSIFTDR` writer - Transmit FIFO Data"]
pub type SSIFTDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSIFTDR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Transmit FIFO Data"]
    #[inline(always)]
    #[must_use]
    pub fn ssiftdr(&mut self) -> SSIFTDR_W<0> {
        SSIFTDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiftdr](index.html) module"]
pub struct SSIFTDR_SPEC;
impl crate::RegisterSpec for SSIFTDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ssiftdr::W](W) writer structure"]
impl crate::Writable for SSIFTDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSIFTDR to value 0"]
impl crate::Resettable for SSIFTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
