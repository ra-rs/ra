#[doc = "Register `ADFIFOSR2` reader"]
pub struct R(crate::R<ADFIFOSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOST4` reader - Number of vacant stages in FIFO for Scan Group 4"]
pub type FIFOST4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOST5` reader - Number of vacant stages in FIFO for Scan Group 5"]
pub type FIFOST5_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of vacant stages in FIFO for Scan Group 4"]
    #[inline(always)]
    pub fn fifost4(&self) -> FIFOST4_R {
        FIFOST4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Number of vacant stages in FIFO for Scan Group 5"]
    #[inline(always)]
    pub fn fifost5(&self) -> FIFOST5_R {
        FIFOST5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "FIFO Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifosr2](index.html) module"]
pub struct ADFIFOSR2_SPEC;
impl crate::RegisterSpec for ADFIFOSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifosr2::R](R) reader structure"]
impl crate::Readable for ADFIFOSR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADFIFOSR2 to value 0x0008_0008"]
impl crate::Resettable for ADFIFOSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0008;
}
