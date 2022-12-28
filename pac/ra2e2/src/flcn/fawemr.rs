#[doc = "Register `FAWEMR` reader"]
pub struct R(crate::R<FAWEMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAWEMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAWEMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAWEMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FAWE` reader - Access Window End Address"]
pub type FAWE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SASMF` reader - Startup Area Setting Monitor Flag"]
pub type SASMF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - Access Window End Address"]
    #[inline(always)]
    pub fn fawe(&self) -> FAWE_R {
        FAWE_R::new(self.bits & 0x07ff)
    }
    #[doc = "Bit 15 - Startup Area Setting Monitor Flag"]
    #[inline(always)]
    pub fn sasmf(&self) -> SASMF_R {
        SASMF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Flash Access Window End Address Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fawemr](index.html) module"]
pub struct FAWEMR_SPEC;
impl crate::RegisterSpec for FAWEMR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fawemr::R](R) reader structure"]
impl crate::Readable for FAWEMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FAWEMR to value 0"]
impl crate::Resettable for FAWEMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
