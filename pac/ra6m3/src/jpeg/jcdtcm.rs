#[doc = "Register `JCDTCM` reader"]
pub struct R(crate::R<JCDTCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCDTCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCDTCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCDTCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCM` reader - Middle bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts. NOTE: Read-only in Decompression."]
pub type DCM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Middle bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn dcm(&self) -> DCM_R {
        DCM_R::new(self.bits)
    }
}
#[doc = "JPEG Code Data Count Middle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcdtcm](index.html) module"]
pub struct JCDTCM_SPEC;
impl crate::RegisterSpec for JCDTCM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcdtcm::R](R) reader structure"]
impl crate::Readable for JCDTCM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JCDTCM to value 0"]
impl crate::Resettable for JCDTCM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
