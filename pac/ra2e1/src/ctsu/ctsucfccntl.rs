#[doc = "Register `CTSUCFCCNTL` reader"]
pub struct R(crate::R<CTSUCFCCNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCFCCNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCFCCNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCFCCNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CTSU CFC Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucfccntl](index.html) module"]
pub struct CTSUCFCCNTL_SPEC;
impl crate::RegisterSpec for CTSUCFCCNTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsucfccntl::R](R) reader structure"]
impl crate::Readable for CTSUCFCCNTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTSUCFCCNTL to value 0"]
impl crate::Resettable for CTSUCFCCNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
