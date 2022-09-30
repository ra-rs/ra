#[doc = "Register `ADDR20` reader"]
pub struct R(crate::R<ADDR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "A/D Data Registers 20\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr20](index.html) module"]
pub struct ADDR20_SPEC;
impl crate::RegisterSpec for ADDR20_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [addr20::R](R) reader structure"]
impl crate::Readable for ADDR20_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDR20 to value 0"]
impl crate::Resettable for ADDR20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
