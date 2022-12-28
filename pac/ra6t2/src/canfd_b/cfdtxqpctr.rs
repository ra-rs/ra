#[doc = "Register `CFDTXQPCTR` writer"]
pub struct W(crate::W<CFDTXQPCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTXQPCTR_SPEC>;
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
impl From<crate::W<CFDTXQPCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTXQPCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXQPC` writer - TX Queue Pointer Control"]
pub type TXQPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTXQPCTR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - TX Queue Pointer Control"]
    #[inline(always)]
    #[must_use]
    pub fn txqpc(&mut self) -> TXQPC_W<0> {
        TXQPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Queue Pointer Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtxqpctr](index.html) module"]
pub struct CFDTXQPCTR_SPEC;
impl crate::RegisterSpec for CFDTXQPCTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfdtxqpctr::W](W) writer structure"]
impl crate::Writable for CFDTXQPCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTXQPCTR to value 0"]
impl crate::Resettable for CFDTXQPCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
