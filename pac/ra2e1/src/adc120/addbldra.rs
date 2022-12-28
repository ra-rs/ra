#[doc = "Register `ADDBLDRA` reader"]
pub struct R(crate::R<ADDBLDRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDBLDRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDBLDRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDBLDRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDBLDRn` reader - Converted Value 15 to 0"]
pub type ADDBLDRN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Value 15 to 0"]
    #[inline(always)]
    pub fn addbldrn(&self) -> ADDBLDRN_R {
        ADDBLDRN_R::new(self.bits)
    }
}
#[doc = "A/D Data Duplexing Register A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addbldra](index.html) module"]
pub struct ADDBLDRA_SPEC;
impl crate::RegisterSpec for ADDBLDRA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [addbldra::R](R) reader structure"]
impl crate::Readable for ADDBLDRA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDBLDRA to value 0"]
impl crate::Resettable for ADDBLDRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
