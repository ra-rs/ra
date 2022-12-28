#[doc = "Register `DLISTSTART` writer"]
pub struct W(crate::W<DLISTSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLISTSTART_SPEC>;
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
impl From<crate::W<DLISTSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLISTSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLISTSTART` writer - Display list start address"]
pub type DLISTSTART_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLISTSTART_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Display list start address"]
    #[inline(always)]
    #[must_use]
    pub fn dliststart(&mut self) -> DLISTSTART_W<0> {
        DLISTSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Display List Start Address Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dliststart](index.html) module"]
pub struct DLISTSTART_SPEC;
impl crate::RegisterSpec for DLISTSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dliststart::W](W) writer structure"]
impl crate::Writable for DLISTSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLISTSTART to value 0"]
impl crate::Resettable for DLISTSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
