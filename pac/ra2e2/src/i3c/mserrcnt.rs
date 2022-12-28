#[doc = "Register `MSERRCNT` reader"]
pub struct R(crate::R<MSERRCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSERRCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSERRCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSERRCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M2ECNT` reader - M2 Error Counter"]
pub type M2ECNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - M2 Error Counter"]
    #[inline(always)]
    pub fn m2ecnt(&self) -> M2ECNT_R {
        M2ECNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Master Error Counters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mserrcnt](index.html) module"]
pub struct MSERRCNT_SPEC;
impl crate::RegisterSpec for MSERRCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mserrcnt::R](R) reader structure"]
impl crate::Readable for MSERRCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSERRCNT to value 0"]
impl crate::Resettable for MSERRCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
