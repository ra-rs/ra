#[doc = "Register `ADFIFOSR4` reader"]
pub struct R(crate::R<ADFIFOSR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOSR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOSR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOSR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOST8` reader - Number of vacant stages in FIFO for Scan Group 8"]
pub type FIFOST8_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of vacant stages in FIFO for Scan Group 8"]
    #[inline(always)]
    pub fn fifost8(&self) -> FIFOST8_R {
        FIFOST8_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "FIFO Status Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifosr4](index.html) module"]
pub struct ADFIFOSR4_SPEC;
impl crate::RegisterSpec for ADFIFOSR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifosr4::R](R) reader structure"]
impl crate::Readable for ADFIFOSR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADFIFOSR4 to value 0x08"]
impl crate::Resettable for ADFIFOSR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
