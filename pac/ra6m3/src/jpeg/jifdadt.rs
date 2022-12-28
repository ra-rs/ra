#[doc = "Register `JIFDADT` reader"]
pub struct R(crate::R<JIFDADT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFDADT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFDADT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFDADT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFDADT` writer"]
pub struct W(crate::W<JIFDADT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFDADT_SPEC>;
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
impl From<crate::W<JIFDADT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFDADT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALPHA` reader - Setting of the alpha value for output in ARGB8888 format."]
pub type ALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALPHA` writer - Setting of the alpha value for output in ARGB8888 format."]
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFDADT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Setting of the alpha value for output in ARGB8888 format."]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Setting of the alpha value for output in ARGB8888 format."]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<0> {
        ALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Decompression alpha Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifdadt](index.html) module"]
pub struct JIFDADT_SPEC;
impl crate::RegisterSpec for JIFDADT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifdadt::R](R) reader structure"]
impl crate::Readable for JIFDADT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifdadt::W](W) writer structure"]
impl crate::Writable for JIFDADT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFDADT to value 0"]
impl crate::Resettable for JIFDADT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
