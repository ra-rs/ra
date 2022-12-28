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
#[doc = "Register `USBINDX` writer"]
pub struct W(crate::W<USBINDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBINDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USBINDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBINDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINDEX` reader - Index These bits store the USB request wIndex value."]
pub type WINDEX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINDEX` writer - Index These bits store the USB request wIndex value."]
pub type WINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USBINDX_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Index These bits store the USB request wIndex value."]
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Index These bits store the USB request wIndex value."]
    #[inline(always)]
    #[must_use]
    pub fn windex(&mut self) -> WINDEX_W<0> {
        WINDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Request Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbindx](index.html) module"]
pub struct USBINDX_SPEC;
impl crate::RegisterSpec for USBINDX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbindx::R](R) reader structure"]
impl crate::Readable for USBINDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbindx::W](W) writer structure"]
impl crate::Writable for USBINDX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBINDX to value 0"]
impl crate::Resettable for USBINDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
