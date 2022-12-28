#[doc = "Register `LCCVRU` reader"]
pub struct R(crate::R<LCCVRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCVRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCVRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCVRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCCVRU` reader - These bits are for reading the higher-order 16 bits of the integer portion of the local timer counter's value."]
pub type LCCVRU_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - These bits are for reading the higher-order 16 bits of the integer portion of the local timer counter's value."]
    #[inline(always)]
    pub fn lccvru(&self) -> LCCVRU_R {
        LCCVRU_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Local Time Counters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lccvru](index.html) module"]
pub struct LCCVRU_SPEC;
impl crate::RegisterSpec for LCCVRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lccvru::R](R) reader structure"]
impl crate::Readable for LCCVRU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCCVRU to value 0"]
impl crate::Resettable for LCCVRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
