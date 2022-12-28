#[doc = "Register `CFDGTSC` reader"]
pub struct R(crate::R<CFDGTSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGTSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGTSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGTSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TS` reader - Timestamp value"]
pub type TS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp value"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Global Timestamp Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgtsc](index.html) module"]
pub struct CFDGTSC_SPEC;
impl crate::RegisterSpec for CFDGTSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgtsc::R](R) reader structure"]
impl crate::Readable for CFDGTSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDGTSC to value 0"]
impl crate::Resettable for CFDGTSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
