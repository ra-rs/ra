#[doc = "Register `CTSUCFCCNT` reader"]
pub struct R(crate::R<CTSUCFCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCFCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCFCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCFCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFCCNT` reader - CTSU CFC Counter"]
pub type CFCCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU CFC Counter"]
    #[inline(always)]
    pub fn cfccnt(&self) -> CFCCNT_R {
        CFCCNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CTSU CFC Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucfccnt](index.html) module"]
pub struct CTSUCFCCNT_SPEC;
impl crate::RegisterSpec for CTSUCFCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsucfccnt::R](R) reader structure"]
impl crate::Readable for CTSUCFCCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTSUCFCCNT to value 0"]
impl crate::Resettable for CTSUCFCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
