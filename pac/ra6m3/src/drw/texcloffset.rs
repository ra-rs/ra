#[doc = "Register `TEXCLOFFSET` writer"]
pub struct W(crate::W<TEXCLOFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXCLOFFSET_SPEC>;
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
impl From<crate::W<TEXCLOFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXCLOFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOFFSET` writer - Texture CLUT offset for Indexed texture format. CLOFFSET\\[7:0\\]
is or'ed with the original index"]
pub type CLOFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEXCLOFFSET_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Texture CLUT offset for Indexed texture format. CLOFFSET\\[7:0\\]
is or'ed with the original index"]
    #[inline(always)]
    #[must_use]
    pub fn cloffset(&mut self) -> CLOFFSET_W<0> {
        CLOFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLUT Offset Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [texcloffset](index.html) module"]
pub struct TEXCLOFFSET_SPEC;
impl crate::RegisterSpec for TEXCLOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [texcloffset::W](W) writer structure"]
impl crate::Writable for TEXCLOFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXCLOFFSET to value 0"]
impl crate::Resettable for TEXCLOFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
