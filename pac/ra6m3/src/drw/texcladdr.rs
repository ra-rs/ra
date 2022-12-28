#[doc = "Register `TEXCLADDR` writer"]
pub struct W(crate::W<TEXCLADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXCLADDR_SPEC>;
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
impl From<crate::W<TEXCLADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXCLADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLADDR` writer - Texture CLUT start address for indexed texture format"]
pub type CLADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEXCLADDR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Texture CLUT start address for indexed texture format"]
    #[inline(always)]
    #[must_use]
    pub fn claddr(&mut self) -> CLADDR_W<0> {
        CLADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLUT Start Address Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [texcladdr](index.html) module"]
pub struct TEXCLADDR_SPEC;
impl crate::RegisterSpec for TEXCLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [texcladdr::W](W) writer structure"]
impl crate::Writable for TEXCLADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXCLADDR to value 0"]
impl crate::Resettable for TEXCLADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
