#[doc = "Register `CFDCFPCTR` writer"]
pub struct W(crate::W<CFDCFPCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCFPCTR_SPEC>;
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
impl From<crate::W<CFDCFPCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCFPCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFPC` writer - Common FIFO Pointer Control"]
pub type CFPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFPCTR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Common FIFO Pointer Control"]
    #[inline(always)]
    #[must_use]
    pub fn cfpc(&mut self) -> CFPC_W<0> {
        CFPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common FIFO Pointer Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcfpctr](index.html) module"]
pub struct CFDCFPCTR_SPEC;
impl crate::RegisterSpec for CFDCFPCTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfdcfpctr::W](W) writer structure"]
impl crate::Writable for CFDCFPCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCFPCTR to value 0"]
impl crate::Resettable for CFDCFPCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
