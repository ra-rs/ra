#[doc = "Register `TDFAR` reader"]
pub struct R(crate::R<TDFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TDFAR` reader - Transmit Descriptor Fetch Address RegisterThe TDFAR register indicates the start address of the last fetched transmit descriptor when the EDMAC fetches descriptor information from the transmit descriptor.Refer to the address indicated by the TDFAR register to recognize which transmit descriptor information the EDMAC is using for the current processing. Note that the address of the transmit descriptor that the EDMAC fetches may not match the read value of the TDFAR register."]
pub type TDFAR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Descriptor Fetch Address RegisterThe TDFAR register indicates the start address of the last fetched transmit descriptor when the EDMAC fetches descriptor information from the transmit descriptor.Refer to the address indicated by the TDFAR register to recognize which transmit descriptor information the EDMAC is using for the current processing. Note that the address of the transmit descriptor that the EDMAC fetches may not match the read value of the TDFAR register."]
    #[inline(always)]
    pub fn tdfar(&self) -> TDFAR_R {
        TDFAR_R::new(self.bits)
    }
}
#[doc = "Transmit Descriptor Fetch Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdfar](index.html) module"]
pub struct TDFAR_SPEC;
impl crate::RegisterSpec for TDFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdfar::R](R) reader structure"]
impl crate::Readable for TDFAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TDFAR to value 0"]
impl crate::Resettable for TDFAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
