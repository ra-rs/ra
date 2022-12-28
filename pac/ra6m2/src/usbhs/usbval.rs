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
#[doc = "Register `USBVAL` writer"]
pub struct W(crate::W<USBVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBVAL_SPEC>;
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
impl From<crate::W<USBVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVALUE` reader - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write"]
pub type WVALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVALUE` writer - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write"]
pub type WVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USBVAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    #[must_use]
    pub fn wvalue(&mut self) -> WVALUE_W<0> {
        WVALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Request Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbval](index.html) module"]
pub struct USBVAL_SPEC;
impl crate::RegisterSpec for USBVAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbval::R](R) reader structure"]
impl crate::Readable for USBVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbval::W](W) writer structure"]
impl crate::Writable for USBVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBVAL to value 0"]
impl crate::Resettable for USBVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
