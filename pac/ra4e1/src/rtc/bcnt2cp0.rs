#[doc = "Register `BCNT2CP0` reader"]
pub struct R(crate::R<BCNT2CP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT2CP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT2CP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT2CP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "BCNT2 Capture Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt2cp0](index.html) module"]
pub struct BCNT2CP0_SPEC;
impl crate::RegisterSpec for BCNT2CP0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcnt2cp0::R](R) reader structure"]
impl crate::Readable for BCNT2CP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCNT2CP0 to value 0"]
impl crate::Resettable for BCNT2CP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
