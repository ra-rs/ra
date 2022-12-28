#[doc = "Register `CFSAMONA` reader"]
pub struct R(crate::R<CFSAMONA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFSAMONA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFSAMONA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFSAMONA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFS2` reader - Code Flash Secure area 2"]
pub type CFS2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 15:23 - Code Flash Secure area 2"]
    #[inline(always)]
    pub fn cfs2(&self) -> CFS2_R {
        CFS2_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
}
#[doc = "Code Flash Security Attribution Monitor Register A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsamona](index.html) module"]
pub struct CFSAMONA_SPEC;
impl crate::RegisterSpec for CFSAMONA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfsamona::R](R) reader structure"]
impl crate::Readable for CFSAMONA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFSAMONA to value 0"]
impl crate::Resettable for CFSAMONA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
