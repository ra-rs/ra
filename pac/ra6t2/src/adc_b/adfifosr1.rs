#[doc = "Register `ADFIFOSR1` reader"]
pub struct R(crate::R<ADFIFOSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOST2` reader - Number of vacant stages in FIFO for Scan Group 2"]
pub type FIFOST2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOST3` reader - Number of vacant stages in FIFO for Scan Group 3"]
pub type FIFOST3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of vacant stages in FIFO for Scan Group 2"]
    #[inline(always)]
    pub fn fifost2(&self) -> FIFOST2_R {
        FIFOST2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Number of vacant stages in FIFO for Scan Group 3"]
    #[inline(always)]
    pub fn fifost3(&self) -> FIFOST3_R {
        FIFOST3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifosr1](index.html) module"]
pub struct ADFIFOSR1_SPEC;
impl crate::RegisterSpec for ADFIFOSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifosr1::R](R) reader structure"]
impl crate::Readable for ADFIFOSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADFIFOSR1 to value 0x0008_0008"]
impl crate::Resettable for ADFIFOSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0008;
}
