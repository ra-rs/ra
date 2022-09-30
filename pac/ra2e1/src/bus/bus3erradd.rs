#[doc = "Register `BUS3ERRADD` reader"]
pub struct R(crate::R<BUS3ERRADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS3ERRADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS3ERRADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS3ERRADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BERAD` reader - Bus Error Address"]
pub type BERAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Error Address"]
    #[inline(always)]
    pub fn berad(&self) -> BERAD_R {
        BERAD_R::new(self.bits)
    }
}
#[doc = "Bus Error Address Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus3erradd](index.html) module"]
pub struct BUS3ERRADD_SPEC;
impl crate::RegisterSpec for BUS3ERRADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus3erradd::R](R) reader structure"]
impl crate::Readable for BUS3ERRADD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS3ERRADD to value 0"]
impl crate::Resettable for BUS3ERRADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
