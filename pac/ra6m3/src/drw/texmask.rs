#[doc = "Register `TEXMASK` writer"]
pub struct W(crate::W<TEXMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXMASK_SPEC>;
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
impl From<crate::W<TEXMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXUMASK` writer - U maskSet TEXUMASK\\[10:0\\]
= texture_width -1In texture wrapping mode (CONTROL2.TEXTURECLAMPX = 0): texture_width must be a power of 2.In texture clamping mode (CONTROL2.TEXTURECLAMPX = 1):all widths up to 2048 are allowed."]
pub type TEXUMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEXMASK_SPEC, u16, u16, 11, O>;
#[doc = "Field `TEXVMASK` writer - V maskSet TEXVMASK\\[20:0\\]
= TEXPITCH * (texture_height - 1).In texture wrapping mode (CONTROL2.TEXTURECLAMPY = 0): texture_height must be a power of 2In texture clamping mode (CONTROL2.TEXTURECLAMPY = 1):all heights up to 1024 are allowed."]
pub type TEXVMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEXMASK_SPEC, u32, u32, 21, O>;
impl W {
    #[doc = "Bits 0:10 - U maskSet TEXUMASK\\[10:0\\]
= texture_width -1In texture wrapping mode (CONTROL2.TEXTURECLAMPX = 0): texture_width must be a power of 2.In texture clamping mode (CONTROL2.TEXTURECLAMPX = 1):all widths up to 2048 are allowed."]
    #[inline(always)]
    #[must_use]
    pub fn texumask(&mut self) -> TEXUMASK_W<0> {
        TEXUMASK_W::new(self)
    }
    #[doc = "Bits 11:31 - V maskSet TEXVMASK\\[20:0\\]
= TEXPITCH * (texture_height - 1).In texture wrapping mode (CONTROL2.TEXTURECLAMPY = 0): texture_height must be a power of 2In texture clamping mode (CONTROL2.TEXTURECLAMPY = 1):all heights up to 1024 are allowed."]
    #[inline(always)]
    #[must_use]
    pub fn texvmask(&mut self) -> TEXVMASK_W<11> {
        TEXVMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Texture Size or Texture Address Mask Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [texmask](index.html) module"]
pub struct TEXMASK_SPEC;
impl crate::RegisterSpec for TEXMASK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [texmask::W](W) writer structure"]
impl crate::Writable for TEXMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXMASK to value 0"]
impl crate::Resettable for TEXMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
