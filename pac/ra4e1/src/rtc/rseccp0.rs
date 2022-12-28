#[doc = "Register `RSECCP0` reader"]
pub struct R(crate::R<RSECCP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSECCP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSECCP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSECCP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC1` reader - 1-Second Capture"]
pub type SEC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC10` reader - 10-Second Capture"]
pub type SEC10_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - 1-Second Capture"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Second Capture"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Second Capture Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rseccp0](index.html) module"]
pub struct RSECCP0_SPEC;
impl crate::RegisterSpec for RSECCP0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rseccp0::R](R) reader structure"]
impl crate::Readable for RSECCP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSECCP0 to value 0"]
impl crate::Resettable for RSECCP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
