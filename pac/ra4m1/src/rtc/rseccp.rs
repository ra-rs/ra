#[doc = "Register `RSECCP%s` reader"]
pub struct R(crate::R<RSECCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSECCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSECCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSECCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC1` reader - 1-Second Capture Capture value for the ones place of seconds"]
pub type SEC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC10` reader - 10-Second Capture Capture value for the tens place of seconds"]
pub type SEC10_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - 1-Second Capture Capture value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Second Capture Capture value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Second Capture Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rseccp](index.html) module"]
pub struct RSECCP_SPEC;
impl crate::RegisterSpec for RSECCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rseccp::R](R) reader structure"]
impl crate::Readable for RSECCP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSECCP%s to value 0"]
impl crate::Resettable for RSECCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
