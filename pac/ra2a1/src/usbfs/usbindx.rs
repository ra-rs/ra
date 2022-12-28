#[doc = "Register `USBINDX` reader"]
pub struct R(crate::R<USBINDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBINDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBINDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBINDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WINDEX` reader - IndexThese bits store the USB request wIndex value."]
pub type WINDEX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IndexThese bits store the USB request wIndex value."]
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new(self.bits)
    }
}
#[doc = "USB Request Index Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbindx](index.html) module"]
pub struct USBINDX_SPEC;
impl crate::RegisterSpec for USBINDX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbindx::R](R) reader structure"]
impl crate::Readable for USBINDX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBINDX to value 0"]
impl crate::Resettable for USBINDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
