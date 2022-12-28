#[doc = "Register `GR%s_AB8` reader"]
pub struct R(crate::R<GR_AB8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB8` writer"]
pub struct W(crate::W<GR_AB8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB8_SPEC>;
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
impl From<crate::W<GR_AB8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKKR` reader - R signal for RGB-index chroma-key processingUnsigned; 8 bits."]
pub type CKKR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKKR` writer - R signal for RGB-index chroma-key processingUnsigned; 8 bits."]
pub type CKKR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB8_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKKB` reader - B signal for RGB-index chroma-key processingUnsigned; 8 bits."]
pub type CKKB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKKB` writer - B signal for RGB-index chroma-key processingUnsigned; 8 bits."]
pub type CKKB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB8_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKKG` reader - G signal for RGB-index chroma-key processingUnsigned; 8 bits."]
pub type CKKG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKKG` writer - G signal for RGB-index chroma-key processingUnsigned; 8 bits."]
pub type CKKG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - R signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkr(&self) -> CKKR_R {
        CKKR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - B signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkb(&self) -> CKKB_R {
        CKKB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - G signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkg(&self) -> CKKG_R {
        CKKG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - R signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn ckkr(&mut self) -> CKKR_W<0> {
        CKKR_W::new(self)
    }
    #[doc = "Bits 8:15 - B signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn ckkb(&mut self) -> CKKB_W<8> {
        CKKB_W::new(self)
    }
    #[doc = "Bits 16:23 - G signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn ckkg(&mut self) -> CKKG_W<16> {
        CKKG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab8](index.html) module"]
pub struct GR_AB8_SPEC;
impl crate::RegisterSpec for GR_AB8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab8::R](R) reader structure"]
impl crate::Readable for GR_AB8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab8::W](W) writer structure"]
impl crate::Writable for GR_AB8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB8 to value 0"]
impl crate::Resettable for GR_AB8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
