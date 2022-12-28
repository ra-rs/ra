#[doc = "Register `RFCF` reader"]
pub struct R(crate::R<RFCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPAUSE` reader - Received PAUSE Frame Count"]
pub type RPAUSE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Received PAUSE Frame Count"]
    #[inline(always)]
    pub fn rpause(&self) -> RPAUSE_R {
        RPAUSE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Received PAUSE Frame Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcf](index.html) module"]
pub struct RFCF_SPEC;
impl crate::RegisterSpec for RFCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcf::R](R) reader structure"]
impl crate::Readable for RFCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFCF to value 0"]
impl crate::Resettable for RFCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
