#[doc = "Register `FTDRL` writer"]
pub struct W(crate::W<FTDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTDRL_SPEC>;
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
impl From<crate::W<FTDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTDRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDATL` writer - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TDATL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FTDRL_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn tdatl(&mut self) -> TDATL_W<0> {
        TDATL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Data Register L\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftdrl](index.html) module"]
pub struct FTDRL_SPEC;
impl crate::RegisterSpec for FTDRL_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [ftdrl::W](W) writer structure"]
impl crate::Writable for FTDRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTDRL to value 0xff"]
impl crate::Resettable for FTDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
