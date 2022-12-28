#[doc = "Register `CACHECTL` writer"]
pub struct W(crate::W<CACHECTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHECTL_SPEC>;
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
impl From<crate::W<CACHECTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHECTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Framebuffer cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENABLEFX_AW {
    #[doc = "0: disable the framebuffer cache"]
    _0 = 0,
    #[doc = "1: enable the framebuffer cache"]
    _1 = 1,
}
impl From<CENABLEFX_AW> for bool {
    #[inline(always)]
    fn from(variant: CENABLEFX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENABLEFX` writer - Framebuffer cache enable"]
pub type CENABLEFX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHECTL_SPEC, CENABLEFX_AW, O>;
impl<'a, const O: u8> CENABLEFX_W<'a, O> {
    #[doc = "disable the framebuffer cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CENABLEFX_AW::_0)
    }
    #[doc = "enable the framebuffer cache"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CENABLEFX_AW::_1)
    }
}
#[doc = "Flush framebuffer cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFLUSHFX_AW {
    #[doc = "0: do not flush the framebuffer cache"]
    _0 = 0,
    #[doc = "1: flush the framebuffer cache"]
    _1 = 1,
}
impl From<CFLUSHFX_AW> for bool {
    #[inline(always)]
    fn from(variant: CFLUSHFX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLUSHFX` writer - Flush framebuffer cache"]
pub type CFLUSHFX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHECTL_SPEC, CFLUSHFX_AW, O>;
impl<'a, const O: u8> CFLUSHFX_W<'a, O> {
    #[doc = "do not flush the framebuffer cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLUSHFX_AW::_0)
    }
    #[doc = "flush the framebuffer cache"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLUSHFX_AW::_1)
    }
}
#[doc = "Texture cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENABLETX_AW {
    #[doc = "0: disable the texture cache"]
    _0 = 0,
    #[doc = "1: enable the texture cache"]
    _1 = 1,
}
impl From<CENABLETX_AW> for bool {
    #[inline(always)]
    fn from(variant: CENABLETX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENABLETX` writer - Texture cache enable"]
pub type CENABLETX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHECTL_SPEC, CENABLETX_AW, O>;
impl<'a, const O: u8> CENABLETX_W<'a, O> {
    #[doc = "disable the texture cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CENABLETX_AW::_0)
    }
    #[doc = "enable the texture cache"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CENABLETX_AW::_1)
    }
}
#[doc = "Flush texture cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFLUSHTX_AW {
    #[doc = "0: do not flush the texture cache"]
    _0 = 0,
    #[doc = "1: flush the texture cache"]
    _1 = 1,
}
impl From<CFLUSHTX_AW> for bool {
    #[inline(always)]
    fn from(variant: CFLUSHTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLUSHTX` writer - Flush texture cache"]
pub type CFLUSHTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHECTL_SPEC, CFLUSHTX_AW, O>;
impl<'a, const O: u8> CFLUSHTX_W<'a, O> {
    #[doc = "do not flush the texture cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLUSHTX_AW::_0)
    }
    #[doc = "flush the texture cache"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLUSHTX_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Framebuffer cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn cenablefx(&mut self) -> CENABLEFX_W<0> {
        CENABLEFX_W::new(self)
    }
    #[doc = "Bit 1 - Flush framebuffer cache"]
    #[inline(always)]
    #[must_use]
    pub fn cflushfx(&mut self) -> CFLUSHFX_W<1> {
        CFLUSHFX_W::new(self)
    }
    #[doc = "Bit 2 - Texture cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn cenabletx(&mut self) -> CENABLETX_W<2> {
        CENABLETX_W::new(self)
    }
    #[doc = "Bit 3 - Flush texture cache"]
    #[inline(always)]
    #[must_use]
    pub fn cflushtx(&mut self) -> CFLUSHTX_W<3> {
        CFLUSHTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachectl](index.html) module"]
pub struct CACHECTL_SPEC;
impl crate::RegisterSpec for CACHECTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cachectl::W](W) writer structure"]
impl crate::Writable for CACHECTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHECTL to value 0"]
impl crate::Resettable for CACHECTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
