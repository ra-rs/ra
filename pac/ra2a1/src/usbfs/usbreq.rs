#[doc = "Register `USBREQ` reader"]
pub struct R(crate::R<USBREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BMREQUESTTYPE` reader - Request TypeThese bits store the USB request bmRequestType value."]
pub type BMREQUESTTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREQUEST` reader - RequestThese bits store the USB request bRequest value."]
pub type BREQUEST_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Request TypeThese bits store the USB request bmRequestType value."]
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RequestThese bits store the USB request bRequest value."]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "USB Request Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbreq](index.html) module"]
pub struct USBREQ_SPEC;
impl crate::RegisterSpec for USBREQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbreq::R](R) reader structure"]
impl crate::Readable for USBREQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBREQ to value 0"]
impl crate::Resettable for USBREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
