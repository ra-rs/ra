#[doc = "Register `JCVSZD` reader"]
pub struct R(crate::R<JCVSZD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCVSZD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCVSZD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCVSZD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCVSZD` writer"]
pub struct W(crate::W<JCVSZD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCVSZD_SPEC>;
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
impl From<crate::W<JCVSZD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCVSZD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSZD` reader - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
pub type VSZD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSZD` writer - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
pub type VSZD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, JCVSZD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn vszd(&self) -> VSZD_R {
        VSZD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn vszd(&mut self) -> VSZD_W<0> {
        VSZD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code Vertical Size Lower Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcvszd](index.html) module"]
pub struct JCVSZD_SPEC;
impl crate::RegisterSpec for JCVSZD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcvszd::R](R) reader structure"]
impl crate::Readable for JCVSZD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jcvszd::W](W) writer structure"]
impl crate::Writable for JCVSZD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCVSZD to value 0"]
impl crate::Resettable for JCVSZD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
