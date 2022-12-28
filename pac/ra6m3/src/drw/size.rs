#[doc = "Register `SIZE` writer"]
pub struct W(crate::W<SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIZE_SPEC>;
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
impl From<crate::W<SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZEX` writer - Width of the bounding box in pixelsvalid range: 0 to 1024"]
pub type SIZEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIZE_SPEC, u16, u16, 16, O>;
#[doc = "Field `SIZEY` writer - Height of the bounding box in pixelsvalid range: 0 to 1024"]
pub type SIZEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIZE_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Width of the bounding box in pixelsvalid range: 0 to 1024"]
    #[inline(always)]
    #[must_use]
    pub fn sizex(&mut self) -> SIZEX_W<0> {
        SIZEX_W::new(self)
    }
    #[doc = "Bits 16:31 - Height of the bounding box in pixelsvalid range: 0 to 1024"]
    #[inline(always)]
    #[must_use]
    pub fn sizey(&mut self) -> SIZEY_W<16> {
        SIZEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bounding Box Dimension Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [size](index.html) module"]
pub struct SIZE_SPEC;
impl crate::RegisterSpec for SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [size::W](W) writer structure"]
impl crate::Writable for SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIZE to value 0"]
impl crate::Resettable for SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
