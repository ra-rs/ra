#[doc = "Register `CTSUSCNT` reader"]
pub struct R(crate::R<CTSUSCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SENSCNT` reader - CTSU Sensor Counter"]
pub type SENSCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUCKCNT` reader - CTSU SUCLK Counter"]
pub type SUCKCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU Sensor Counter"]
    #[inline(always)]
    pub fn senscnt(&self) -> SENSCNT_R {
        SENSCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CTSU SUCLK Counter"]
    #[inline(always)]
    pub fn suckcnt(&self) -> SUCKCNT_R {
        SUCKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "CTSU Sensor Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuscnt](index.html) module"]
pub struct CTSUSCNT_SPEC;
impl crate::RegisterSpec for CTSUSCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsuscnt::R](R) reader structure"]
impl crate::Readable for CTSUSCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTSUSCNT to value 0"]
impl crate::Resettable for CTSUSCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
