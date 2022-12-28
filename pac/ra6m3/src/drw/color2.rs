#[doc = "Register `COLOR2` writer"]
pub struct W(crate::W<COLOR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLOR2_SPEC>;
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
impl From<crate::W<COLOR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLOR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLOR2B` writer - Blue channel of color 2"]
pub type COLOR2B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLOR2G` writer - Green channel of color 2"]
pub type COLOR2G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLOR2R` writer - Red channel of color 2"]
pub type COLOR2R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLOR2A` writer - Alpha channel of color 2(0x00: transparent. . . 0xFF: opaque)"]
pub type COLOR2A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR2_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Blue channel of color 2"]
    #[inline(always)]
    #[must_use]
    pub fn color2b(&mut self) -> COLOR2B_W<0> {
        COLOR2B_W::new(self)
    }
    #[doc = "Bits 8:15 - Green channel of color 2"]
    #[inline(always)]
    #[must_use]
    pub fn color2g(&mut self) -> COLOR2G_W<8> {
        COLOR2G_W::new(self)
    }
    #[doc = "Bits 16:23 - Red channel of color 2"]
    #[inline(always)]
    #[must_use]
    pub fn color2r(&mut self) -> COLOR2R_W<16> {
        COLOR2R_W::new(self)
    }
    #[doc = "Bits 24:31 - Alpha channel of color 2(0x00: transparent. . . 0xFF: opaque)"]
    #[inline(always)]
    #[must_use]
    pub fn color2a(&mut self) -> COLOR2A_W<24> {
        COLOR2A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secondary Color Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [color2](index.html) module"]
pub struct COLOR2_SPEC;
impl crate::RegisterSpec for COLOR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [color2::W](W) writer structure"]
impl crate::Writable for COLOR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLOR2 to value 0"]
impl crate::Resettable for COLOR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
