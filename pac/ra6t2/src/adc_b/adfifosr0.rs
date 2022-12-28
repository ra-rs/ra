#[doc = "Register `ADFIFOSR0` reader"]
pub struct R(crate::R<ADFIFOSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOST0` reader - Number of vacant stages in FIFO for Scan Group 0"]
pub type FIFOST0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOST1` reader - Number of vacant stages in FIFO for Scan Group 1"]
pub type FIFOST1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of vacant stages in FIFO for Scan Group 0"]
    #[inline(always)]
    pub fn fifost0(&self) -> FIFOST0_R {
        FIFOST0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Number of vacant stages in FIFO for Scan Group 1"]
    #[inline(always)]
    pub fn fifost1(&self) -> FIFOST1_R {
        FIFOST1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "FIFO Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifosr0](index.html) module"]
pub struct ADFIFOSR0_SPEC;
impl crate::RegisterSpec for ADFIFOSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifosr0::R](R) reader structure"]
impl crate::Readable for ADFIFOSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADFIFOSR0 to value 0x0008_0008"]
impl crate::Resettable for ADFIFOSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0008;
}
