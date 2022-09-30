#[doc = "Register `ADDR0` reader"]
pub struct R(crate::R<ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "A/D Data Registers 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0](index.html) module"]
pub struct ADDR0_SPEC;
impl crate::RegisterSpec for ADDR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [addr0::R](R) reader structure"]
impl crate::Readable for ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::Resettable for ADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
