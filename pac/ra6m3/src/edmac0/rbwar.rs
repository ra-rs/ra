#[doc = "Register `RBWAR` reader"]
pub struct R(crate::R<RBWAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBWAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBWAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBWAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RBWAR` reader - Receive Buffer Write Address RegisterThe RBWAR register indicates the last address that the EDMAC has written data to when writing to the receive buffer.Refer to the address indicated by the RBWAR register to recognize which address in the receive buffer the EDMAC is writing data to. Note that the address that the EDMAC is outputting to the receive buffer may not match the read value of the RBWAR register during data reception."]
pub type RBWAR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Buffer Write Address RegisterThe RBWAR register indicates the last address that the EDMAC has written data to when writing to the receive buffer.Refer to the address indicated by the RBWAR register to recognize which address in the receive buffer the EDMAC is writing data to. Note that the address that the EDMAC is outputting to the receive buffer may not match the read value of the RBWAR register during data reception."]
    #[inline(always)]
    pub fn rbwar(&self) -> RBWAR_R {
        RBWAR_R::new(self.bits)
    }
}
#[doc = "Receive Buffer Write Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbwar](index.html) module"]
pub struct RBWAR_SPEC;
impl crate::RegisterSpec for RBWAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbwar::R](R) reader structure"]
impl crate::Readable for RBWAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBWAR to value 0"]
impl crate::Resettable for RBWAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
