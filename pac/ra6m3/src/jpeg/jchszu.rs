#[doc = "Register `JCHSZU` reader"]
pub struct R(crate::R<JCHSZU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCHSZU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCHSZU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCHSZU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCHSZU` writer"]
pub struct W(crate::W<JCHSZU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCHSZU_SPEC>;
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
impl From<crate::W<JCHSZU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCHSZU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSZU` reader - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
pub type HSZU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSZU` writer - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
pub type HSZU_W<'a, const O: u8> = crate::FieldWriter<'a, u8, JCHSZU_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hszu(&self) -> HSZU_R {
        HSZU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn hszu(&mut self) -> HSZU_W<0> {
        HSZU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code Horizontal Size Upper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jchszu](index.html) module"]
pub struct JCHSZU_SPEC;
impl crate::RegisterSpec for JCHSZU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jchszu::R](R) reader structure"]
impl crate::Readable for JCHSZU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jchszu::W](W) writer structure"]
impl crate::Writable for JCHSZU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCHSZU to value 0"]
impl crate::Resettable for JCHSZU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
