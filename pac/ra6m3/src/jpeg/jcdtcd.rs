#[doc = "Register `JCDTCD` reader"]
pub struct R(crate::R<JCDTCD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCDTCD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCDTCD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCDTCD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCD` reader - Lower bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts.NOTE: Read-only in Decompression."]
pub type DCD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Lower bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(self.bits)
    }
}
#[doc = "JPEG Code Data Count Lower Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcdtcd](index.html) module"]
pub struct JCDTCD_SPEC;
impl crate::RegisterSpec for JCDTCD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcdtcd::R](R) reader structure"]
impl crate::Readable for JCDTCD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JCDTCD to value 0"]
impl crate::Resettable for JCDTCD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
