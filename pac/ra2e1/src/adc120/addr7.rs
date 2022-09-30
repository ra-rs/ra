#[doc = "Register `ADDR7` reader"]
pub struct R(crate::R<ADDR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "A/D Data Registers 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr7](index.html) module"]
pub struct ADDR7_SPEC;
impl crate::RegisterSpec for ADDR7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [addr7::R](R) reader structure"]
impl crate::Readable for ADDR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDR7 to value 0"]
impl crate::Resettable for ADDR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
