#[doc = "Register `USBCKCR` reader"]
pub struct R(crate::R<USBCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCKCR` writer"]
pub struct W(crate::W<USBCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCKCR_SPEC>;
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
impl From<crate::W<USBCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTS` reader - USB Clock Source Select"]
pub type HSTS_R = crate::BitReader<HSTS_A>;
#[doc = "USB Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSTS_A {
    #[doc = "0: PLL(Value after reset)"]
    _0 = 0,
    #[doc = "1: HOCO"]
    _1 = 1,
}
impl From<HSTS_A> for bool {
    #[inline(always)]
    fn from(variant: HSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl HSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSTS_A {
        match self.bits {
            false => HSTS_A::_0,
            true => HSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSTS_A::_1
    }
}
#[doc = "Field `HSTS` writer - USB Clock Source Select"]
pub type HSTS_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCKCR_SPEC, HSTS_A, O>;
impl<'a, const O: u8> HSTS_W<'a, O> {
    #[doc = "PLL(Value after reset)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSTS_A::_0)
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSTS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Source Select"]
    #[inline(always)]
    pub fn hsts(&self) -> HSTS_R {
        HSTS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsts(&mut self) -> HSTS_W<0> {
        HSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbckcr](index.html) module"]
pub struct USBCKCR_SPEC;
impl crate::RegisterSpec for USBCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbckcr::R](R) reader structure"]
impl crate::Readable for USBCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbckcr::W](W) writer structure"]
impl crate::Writable for USBCKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCKCR to value 0"]
impl crate::Resettable for USBCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
