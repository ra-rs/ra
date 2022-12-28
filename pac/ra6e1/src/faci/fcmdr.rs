#[doc = "Register `FCMDR` reader"]
pub struct R(crate::R<FCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCMDR` reader - Pre-command Flag"]
pub type PCMDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDR` reader - Command Flag"]
pub type CMDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Pre-command Flag"]
    #[inline(always)]
    pub fn pcmdr(&self) -> PCMDR_R {
        PCMDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command Flag"]
    #[inline(always)]
    pub fn cmdr(&self) -> CMDR_R {
        CMDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "FACI Command Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcmdr](index.html) module"]
pub struct FCMDR_SPEC;
impl crate::RegisterSpec for FCMDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fcmdr::R](R) reader structure"]
impl crate::Readable for FCMDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCMDR to value 0"]
impl crate::Resettable for FCMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
