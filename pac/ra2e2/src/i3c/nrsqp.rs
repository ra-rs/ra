#[doc = "Register `NRSQP` reader"]
pub struct R(crate::R<NRSQP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRSQP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRSQP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRSQP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Normal Receive Status Queue Port Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrsqp](index.html) module"]
pub struct NRSQP_SPEC;
impl crate::RegisterSpec for NRSQP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrsqp::R](R) reader structure"]
impl crate::Readable for NRSQP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NRSQP to value 0"]
impl crate::Resettable for NRSQP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
