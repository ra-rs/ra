#[doc = "Register `USBLENG` reader"]
pub struct R(crate::R<USBLENG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBLENG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBLENG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBLENG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WLENGTUH` reader - LengthThese bits store the USB request wLength value."]
pub type WLENGTUH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - LengthThese bits store the USB request wLength value."]
    #[inline(always)]
    pub fn wlengtuh(&self) -> WLENGTUH_R {
        WLENGTUH_R::new(self.bits)
    }
}
#[doc = "USB Request Length Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbleng](index.html) module"]
pub struct USBLENG_SPEC;
impl crate::RegisterSpec for USBLENG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbleng::R](R) reader structure"]
impl crate::Readable for USBLENG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBLENG to value 0"]
impl crate::Resettable for USBLENG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
