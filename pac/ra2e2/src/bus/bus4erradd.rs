#[doc = "Register `BUS4ERRADD` reader"]
pub struct R(crate::R<BUS4ERRADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS4ERRADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS4ERRADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS4ERRADD_SPEC>) -> Self {
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
#[doc = "Bus Error Address Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus4erradd](index.html) module"]
pub struct BUS4ERRADD_SPEC;
impl crate::RegisterSpec for BUS4ERRADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus4erradd::R](R) reader structure"]
impl crate::Readable for BUS4ERRADD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS4ERRADD to value 0"]
impl crate::Resettable for BUS4ERRADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
