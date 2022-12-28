#[doc = "Register `COLOR1` writer"]
pub struct W(crate::W<COLOR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLOR1_SPEC>;
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
impl From<crate::W<COLOR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLOR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLOR1B` writer - Blue channel of color 1"]
pub type COLOR1B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLOR1G` writer - Green channel of color 1"]
pub type COLOR1G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLOR1R` writer - Red channel of color 1"]
pub type COLOR1R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLOR1A` writer - Alpha channel of color 1(0x00: transparent. . . 0xFF: opaque)"]
pub type COLOR1A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLOR1_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Blue channel of color 1"]
    #[inline(always)]
    #[must_use]
    pub fn color1b(&mut self) -> COLOR1B_W<0> {
        COLOR1B_W::new(self)
    }
    #[doc = "Bits 8:15 - Green channel of color 1"]
    #[inline(always)]
    #[must_use]
    pub fn color1g(&mut self) -> COLOR1G_W<8> {
        COLOR1G_W::new(self)
    }
    #[doc = "Bits 16:23 - Red channel of color 1"]
    #[inline(always)]
    #[must_use]
    pub fn color1r(&mut self) -> COLOR1R_W<16> {
        COLOR1R_W::new(self)
    }
    #[doc = "Bits 24:31 - Alpha channel of color 1(0x00: transparent. . . 0xFF: opaque)"]
    #[inline(always)]
    #[must_use]
    pub fn color1a(&mut self) -> COLOR1A_W<24> {
        COLOR1A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base Color Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [color1](index.html) module"]
pub struct COLOR1_SPEC;
impl crate::RegisterSpec for COLOR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [color1::W](W) writer structure"]
impl crate::Writable for COLOR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLOR1 to value 0"]
impl crate::Resettable for COLOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
