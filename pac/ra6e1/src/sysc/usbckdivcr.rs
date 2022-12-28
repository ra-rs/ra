#[doc = "Register `USBCKDIVCR` reader"]
pub struct R(crate::R<USBCKDIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCKDIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCKDIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCKDIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCKDIVCR` writer"]
pub struct W(crate::W<USBCKDIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCKDIVCR_SPEC>;
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
impl From<crate::W<USBCKDIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCKDIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBCKDIV` reader - USB Clock (USBCLK) Division Select"]
pub type USBCKDIV_R = crate::FieldReader<u8, USBCKDIV_A>;
#[doc = "USB Clock (USBCLK) Division Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBCKDIV_A {
    #[doc = "2: ∕ 4"]
    _010 = 2,
    #[doc = "5: ∕ 3"]
    _101 = 5,
    #[doc = "6: ∕ 5"]
    _110 = 6,
}
impl From<USBCKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: USBCKDIV_A) -> Self {
        variant as _
    }
}
impl USBCKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USBCKDIV_A> {
        match self.bits {
            2 => Some(USBCKDIV_A::_010),
            5 => Some(USBCKDIV_A::_101),
            6 => Some(USBCKDIV_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == USBCKDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == USBCKDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == USBCKDIV_A::_110
    }
}
#[doc = "Field `USBCKDIV` writer - USB Clock (USBCLK) Division Select"]
pub type USBCKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, USBCKDIVCR_SPEC, u8, USBCKDIV_A, 3, O>;
impl<'a, const O: u8> USBCKDIV_W<'a, O> {
    #[doc = "∕ 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(USBCKDIV_A::_010)
    }
    #[doc = "∕ 3"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(USBCKDIV_A::_101)
    }
    #[doc = "∕ 5"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(USBCKDIV_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Clock (USBCLK) Division Select"]
    #[inline(always)]
    pub fn usbckdiv(&self) -> USBCKDIV_R {
        USBCKDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Clock (USBCLK) Division Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbckdiv(&mut self) -> USBCKDIV_W<0> {
        USBCKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Division Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbckdivcr](index.html) module"]
pub struct USBCKDIVCR_SPEC;
impl crate::RegisterSpec for USBCKDIVCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbckdivcr::R](R) reader structure"]
impl crate::Readable for USBCKDIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbckdivcr::W](W) writer structure"]
impl crate::Writable for USBCKDIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCKDIVCR to value 0"]
impl crate::Resettable for USBCKDIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
