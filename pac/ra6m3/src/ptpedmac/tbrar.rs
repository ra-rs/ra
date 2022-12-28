#[doc = "Register `TBRAR` reader"]
pub struct R(crate::R<TBRAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBRAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBRAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBRAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TBRAR` reader - Transmit Buffer Read Address RegisterThe TBRAR register indicates the last address that the EDMAC has read data from when reading data from the transmit buffer.Refer to the address indicated by the TBRAR register to recognize which address in the transmit buffer the EDMAC is reading from. Note that the address that the EDMAC is outputting to the transmit buffer may not match the read value of the TBRAR register."]
pub type TBRAR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Buffer Read Address RegisterThe TBRAR register indicates the last address that the EDMAC has read data from when reading data from the transmit buffer.Refer to the address indicated by the TBRAR register to recognize which address in the transmit buffer the EDMAC is reading from. Note that the address that the EDMAC is outputting to the transmit buffer may not match the read value of the TBRAR register."]
    #[inline(always)]
    pub fn tbrar(&self) -> TBRAR_R {
        TBRAR_R::new(self.bits)
    }
}
#[doc = "Transmit Buffer Read Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbrar](index.html) module"]
pub struct TBRAR_SPEC;
impl crate::RegisterSpec for TBRAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbrar::R](R) reader structure"]
impl crate::Readable for TBRAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBRAR to value 0"]
impl crate::Resettable for TBRAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
