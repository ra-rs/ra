#[doc = "Register `USBVAL` reader"]
pub struct R(crate::R<USBVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WVALUE` reader - ValueThese bits store the USB request wValue value."]
pub type WVALUE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ValueThese bits store the USB request wValue value."]
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(self.bits)
    }
}
#[doc = "USB Request Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbval](index.html) module"]
pub struct USBVAL_SPEC;
impl crate::RegisterSpec for USBVAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbval::R](R) reader structure"]
impl crate::Readable for USBVAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBVAL to value 0"]
impl crate::Resettable for USBVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
