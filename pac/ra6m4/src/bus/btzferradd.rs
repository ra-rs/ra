#[doc = "Register `BTZF%sERRADD` reader"]
pub struct R(crate::R<BTZFERRADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTZFERRADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTZFERRADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTZFERRADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BTZFERAD` reader - Bus TrustZone Filter Error Address"]
pub type BTZFERAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bus TrustZone Filter Error Address"]
    #[inline(always)]
    pub fn btzferad(&self) -> BTZFERAD_R {
        BTZFERAD_R::new(self.bits)
    }
}
#[doc = "BUS TZF Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btzferradd](index.html) module"]
pub struct BTZFERRADD_SPEC;
impl crate::RegisterSpec for BTZFERRADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btzferradd::R](R) reader structure"]
impl crate::Readable for BTZFERRADD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BTZF%sERRADD to value 0"]
impl crate::Resettable for BTZFERRADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
