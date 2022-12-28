#[doc = "Register `SPRFSR` reader"]
pub struct R(crate::R<SPRFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPRFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPRFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPRFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFDN` reader - Receive FIFO data store stage number"]
pub type RFDN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Receive FIFO data store stage number"]
    #[inline(always)]
    pub fn rfdn(&self) -> RFDN_R {
        RFDN_R::new((self.bits & 7) as u8)
    }
}
#[doc = "SPI Receive FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sprfsr](index.html) module"]
pub struct SPRFSR_SPEC;
impl crate::RegisterSpec for SPRFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sprfsr::R](R) reader structure"]
impl crate::Readable for SPRFSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPRFSR to value 0x04"]
impl crate::Resettable for SPRFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
