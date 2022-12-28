#[doc = "Register `COLKEY` writer"]
pub struct W(crate::W<COLKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLKEY_SPEC>;
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
impl From<crate::W<COLKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLKEYB` writer - Blue channel of color key"]
pub type COLKEYB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLKEY_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLKEYG` writer - Green channel of color key"]
pub type COLKEYG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLKEY_SPEC, u8, u8, 8, O>;
#[doc = "Field `COLKEYR` writer - Red channel of color key"]
pub type COLKEYR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLKEY_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Blue channel of color key"]
    #[inline(always)]
    #[must_use]
    pub fn colkeyb(&mut self) -> COLKEYB_W<0> {
        COLKEYB_W::new(self)
    }
    #[doc = "Bits 8:15 - Green channel of color key"]
    #[inline(always)]
    #[must_use]
    pub fn colkeyg(&mut self) -> COLKEYG_W<8> {
        COLKEYG_W::new(self)
    }
    #[doc = "Bits 16:23 - Red channel of color key"]
    #[inline(always)]
    #[must_use]
    pub fn colkeyr(&mut self) -> COLKEYR_W<16> {
        COLKEYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color Key Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colkey](index.html) module"]
pub struct COLKEY_SPEC;
impl crate::RegisterSpec for COLKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [colkey::W](W) writer structure"]
impl crate::Writable for COLKEY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLKEY to value 0"]
impl crate::Resettable for COLKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
