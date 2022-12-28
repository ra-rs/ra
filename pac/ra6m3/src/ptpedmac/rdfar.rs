#[doc = "Register `RDFAR` reader"]
pub struct R(crate::R<RDFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDFAR` reader - Receive Descriptor Fetch Address RegisterThe RDFAR register indicates the start address of the last fetched receive descriptor when the EDMAC fetches descriptor information from the receive descriptor.Refer to the address indicated by the RDFAR register to recognize which receive descriptor information the EDMAC is using for the current processing. Note that the address of the receive descriptor that the EDMAC fetches may not match the read value of the RDFAR register during data reception."]
pub type RDFAR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Descriptor Fetch Address RegisterThe RDFAR register indicates the start address of the last fetched receive descriptor when the EDMAC fetches descriptor information from the receive descriptor.Refer to the address indicated by the RDFAR register to recognize which receive descriptor information the EDMAC is using for the current processing. Note that the address of the receive descriptor that the EDMAC fetches may not match the read value of the RDFAR register during data reception."]
    #[inline(always)]
    pub fn rdfar(&self) -> RDFAR_R {
        RDFAR_R::new(self.bits)
    }
}
#[doc = "Receive Descriptor Fetch Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdfar](index.html) module"]
pub struct RDFAR_SPEC;
impl crate::RegisterSpec for RDFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdfar::R](R) reader structure"]
impl crate::Readable for RDFAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDFAR to value 0"]
impl crate::Resettable for RDFAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
