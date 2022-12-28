#[doc = "Register `JCHSZD` reader"]
pub struct R(crate::R<JCHSZD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCHSZD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCHSZD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCHSZD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCHSZD` writer"]
pub struct W(crate::W<JCHSZD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCHSZD_SPEC>;
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
impl From<crate::W<JCHSZD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCHSZD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSZD` reader - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
pub type HSZD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSZD` writer - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
pub type HSZD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, JCHSZD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hszd(&self) -> HSZD_R {
        HSZD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn hszd(&mut self) -> HSZD_W<0> {
        HSZD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Coded Horizontal Size Lower Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jchszd](index.html) module"]
pub struct JCHSZD_SPEC;
impl crate::RegisterSpec for JCHSZD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jchszd::R](R) reader structure"]
impl crate::Readable for JCHSZD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jchszd::W](W) writer structure"]
impl crate::Writable for JCHSZD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCHSZD to value 0"]
impl crate::Resettable for JCHSZD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
