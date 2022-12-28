#[doc = "Register `CRR` reader"]
pub struct R(crate::R<CRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD0` reader - Read data 0"]
pub type RD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD1` reader - Read data 1"]
pub type RD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD2` reader - Read data 2"]
pub type RD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD3` reader - Read data 3"]
pub type RD3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Read data 0"]
    #[inline(always)]
    pub fn rd0(&self) -> RD0_R {
        RD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read data 1"]
    #[inline(always)]
    pub fn rd1(&self) -> RD1_R {
        RD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read data 2"]
    #[inline(always)]
    pub fn rd2(&self) -> RD2_R {
        RD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read data 3"]
    #[inline(always)]
    pub fn rd3(&self) -> RD3_R {
        RD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Configure Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crr](index.html) module"]
pub struct CRR_SPEC;
impl crate::RegisterSpec for CRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crr::R](R) reader structure"]
impl crate::Readable for CRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRR to value 0"]
impl crate::Resettable for CRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
