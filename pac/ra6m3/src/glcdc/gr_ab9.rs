#[doc = "Register `GR%s_AB9` reader"]
pub struct R(crate::R<GR_AB9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB9` writer"]
pub struct W(crate::W<GR_AB9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB9_SPEC>;
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
impl From<crate::W<GR_AB9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKR` reader - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
pub type CKR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKR` writer - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
pub type CKR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB9_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKB` reader - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
pub type CKB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKB` writer - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
pub type CKB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB9_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKG` reader - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
pub type CKG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKG` writer - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
pub type CKG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB9_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKA` reader - A value after RGB-index chroma-key processing replacement."]
pub type CKA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKA` writer - A value after RGB-index chroma-key processing replacement."]
pub type CKA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB9_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckr(&self) -> CKR_R {
        CKR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckb(&self) -> CKB_R {
        CKB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - A value after RGB-index chroma-key processing replacement."]
    #[inline(always)]
    pub fn cka(&self) -> CKA_R {
        CKA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn ckr(&mut self) -> CKR_W<0> {
        CKR_W::new(self)
    }
    #[doc = "Bits 8:15 - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn ckb(&mut self) -> CKB_W<8> {
        CKB_W::new(self)
    }
    #[doc = "Bits 16:23 - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn ckg(&mut self) -> CKG_W<16> {
        CKG_W::new(self)
    }
    #[doc = "Bits 24:31 - A value after RGB-index chroma-key processing replacement."]
    #[inline(always)]
    #[must_use]
    pub fn cka(&mut self) -> CKA_W<24> {
        CKA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab9](index.html) module"]
pub struct GR_AB9_SPEC;
impl crate::RegisterSpec for GR_AB9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab9::R](R) reader structure"]
impl crate::Readable for GR_AB9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab9::W](W) writer structure"]
impl crate::Writable for GR_AB9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB9 to value 0"]
impl crate::Resettable for GR_AB9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
