#[doc = "Register `JCDTCU` reader"]
pub struct R(crate::R<JCDTCU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCDTCU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCDTCU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCDTCU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCU` reader - Upper bytes of the counted amount of data to be compressed The values of this register are reset before compression starts.NOTE: Read-only in Decompression."]
pub type DCU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Upper bytes of the counted amount of data to be compressed The values of this register are reset before compression starts.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn dcu(&self) -> DCU_R {
        DCU_R::new(self.bits)
    }
}
#[doc = "JPEG Code Data Count Upper Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcdtcu](index.html) module"]
pub struct JCDTCU_SPEC;
impl crate::RegisterSpec for JCDTCU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcdtcu::R](R) reader structure"]
impl crate::Readable for JCDTCU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JCDTCU to value 0"]
impl crate::Resettable for JCDTCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
