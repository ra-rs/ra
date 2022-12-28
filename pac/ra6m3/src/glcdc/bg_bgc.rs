#[doc = "Register `BG_BGC` reader"]
pub struct R(crate::R<BG_BGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_BGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_BGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_BGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BG_BGC` writer"]
pub struct W(crate::W<BG_BGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_BGC_SPEC>;
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
impl From<crate::W<BG_BGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_BGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B` reader - B value for background plane valid pixel areaUnsigned; 8-bit integer"]
pub type B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B` writer - B value for background plane valid pixel areaUnsigned; 8-bit integer"]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_BGC_SPEC, u8, u8, 8, O>;
#[doc = "Field `G` reader - G value for background plane valid pixel areaUnsigned; 8-bit integer"]
pub type G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `G` writer - G value for background plane valid pixel areaUnsigned; 8-bit integer"]
pub type G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_BGC_SPEC, u8, u8, 8, O>;
#[doc = "Field `R` reader - R value for background plane valid pixel area.Unsigned; 8-bit integer."]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R` writer - R value for background plane valid pixel area.Unsigned; 8-bit integer."]
pub type R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_BGC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - B value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - G value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - R value for background plane valid pixel area.Unsigned; 8-bit integer."]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - B value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<0> {
        B_W::new(self)
    }
    #[doc = "Bits 8:15 - G value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn g(&mut self) -> G_W<8> {
        G_W::new(self)
    }
    #[doc = "Bits 16:23 - R value for background plane valid pixel area.Unsigned; 8-bit integer."]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<16> {
        R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Plane Setting Background Color Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_bgc](index.html) module"]
pub struct BG_BGC_SPEC;
impl crate::RegisterSpec for BG_BGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_bgc::R](R) reader structure"]
impl crate::Readable for BG_BGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_bgc::W](W) writer structure"]
impl crate::Writable for BG_BGC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BG_BGC to value 0"]
impl crate::Resettable for BG_BGC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
