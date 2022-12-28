#[doc = "Register `ADFIFOSR3` reader"]
pub struct R(crate::R<ADFIFOSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOST6` reader - Number of vacant stages in FIFO for Scan Group 6"]
pub type FIFOST6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOST7` reader - Number of vacant stages in FIFO for Scan Group 7"]
pub type FIFOST7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of vacant stages in FIFO for Scan Group 6"]
    #[inline(always)]
    pub fn fifost6(&self) -> FIFOST6_R {
        FIFOST6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Number of vacant stages in FIFO for Scan Group 7"]
    #[inline(always)]
    pub fn fifost7(&self) -> FIFOST7_R {
        FIFOST7_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "FIFO Status Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifosr3](index.html) module"]
pub struct ADFIFOSR3_SPEC;
impl crate::RegisterSpec for ADFIFOSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifosr3::R](R) reader structure"]
impl crate::Readable for ADFIFOSR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADFIFOSR3 to value 0x0008_0008"]
impl crate::Resettable for ADFIFOSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0008;
}
