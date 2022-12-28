#[doc = "Register `SSAMONA` reader"]
pub struct R(crate::R<SSAMONA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSAMONA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSAMONA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSAMONA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SS2` reader - SRAM Secure area 2"]
pub type SS2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 13:20 - SRAM Secure area 2"]
    #[inline(always)]
    pub fn ss2(&self) -> SS2_R {
        SS2_R::new(((self.bits >> 13) & 0xff) as u8)
    }
}
#[doc = "SRAM Security Attribution Monitor Register A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssamona](index.html) module"]
pub struct SSAMONA_SPEC;
impl crate::RegisterSpec for SSAMONA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssamona::R](R) reader structure"]
impl crate::Readable for SSAMONA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSAMONA to value 0"]
impl crate::Resettable for SSAMONA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
