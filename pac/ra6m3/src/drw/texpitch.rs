#[doc = "Register `TEXPITCH` writer"]
pub struct W(crate::W<TEXPITCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXPITCH_SPEC>;
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
impl From<crate::W<TEXPITCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXPITCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXPITCH` writer - Texels per texture linevalid range: 0 to 2048"]
pub type TEXPITCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEXPITCH_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Texels per texture linevalid range: 0 to 2048"]
    #[inline(always)]
    #[must_use]
    pub fn texpitch(&mut self) -> TEXPITCH_W<0> {
        TEXPITCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Texels Per Texture Line Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [texpitch](index.html) module"]
pub struct TEXPITCH_SPEC;
impl crate::RegisterSpec for TEXPITCH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [texpitch::W](W) writer structure"]
impl crate::Writable for TEXPITCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXPITCH to value 0"]
impl crate::Resettable for TEXPITCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
