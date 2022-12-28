#[doc = "Register `BUS%sERRADD` reader"]
pub struct R(crate::R<BUSERRADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSERRADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSERRADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSERRADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BERAD` reader - Bus Error AddressWhen a bus error occurs, It stores an error address."]
pub type BERAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Error AddressWhen a bus error occurs, It stores an error address."]
    #[inline(always)]
    pub fn berad(&self) -> BERAD_R {
        BERAD_R::new(self.bits)
    }
}
#[doc = "Bus Error Address Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buserradd](index.html) module"]
pub struct BUSERRADD_SPEC;
impl crate::RegisterSpec for BUSERRADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buserradd::R](R) reader structure"]
impl crate::Readable for BUSERRADD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS%sERRADD to value 0"]
impl crate::Resettable for BUSERRADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
