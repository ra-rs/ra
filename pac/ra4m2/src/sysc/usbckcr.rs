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
#[doc = "Field `USBCKSEL` reader - USB Clock (USBCLK) Source Select"]
pub type USBCKSEL_R = crate::FieldReader<u8, USBCKSEL_A>;
#[doc = "USB Clock (USBCLK) Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBCKSEL_A {
    #[doc = "5: PLL"]
    _101 = 5,
    #[doc = "6: PLL2"]
    _110 = 6,
}
impl From<USBCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBCKSEL_A) -> Self {
        variant as _
    }
}
impl USBCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USBCKSEL_A> {
        match self.bits {
            5 => Some(USBCKSEL_A::_101),
            6 => Some(USBCKSEL_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == USBCKSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == USBCKSEL_A::_110
    }
}
#[doc = "Field `USBCKSEL` writer - USB Clock (USBCLK) Source Select"]
pub type USBCKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, USBCKCR_SPEC, u8, USBCKSEL_A, 3, O>;
impl<'a, const O: u8> USBCKSEL_W<'a, O> {
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(USBCKSEL_A::_101)
    }
    #[doc = "PLL2"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(USBCKSEL_A::_110)
    }
}
#[doc = "Field `USBCKSREQ` reader - USB Clock (USBCLK) Switching Request"]
pub type USBCKSREQ_R = crate::BitReader<USBCKSREQ_A>;
#[doc = "USB Clock (USBCLK) Switching Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCKSREQ_A {
    #[doc = "0: No request"]
    _0 = 0,
    #[doc = "1: Request switching."]
    _1 = 1,
}
impl From<USBCKSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBCKSREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USBCKSREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCKSREQ_A {
        match self.bits {
            false => USBCKSREQ_A::_0,
            true => USBCKSREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBCKSREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBCKSREQ_A::_1
    }
}
#[doc = "Field `USBCKSREQ` writer - USB Clock (USBCLK) Switching Request"]
pub type USBCKSREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCKCR_SPEC, USBCKSREQ_A, O>;
impl<'a, const O: u8> USBCKSREQ_W<'a, O> {
    #[doc = "No request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBCKSREQ_A::_0)
    }
    #[doc = "Request switching."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBCKSREQ_A::_1)
    }
}
#[doc = "Field `USBCKSRDY` reader - USB Clock (USBCLK) Switching Ready state flag"]
pub type USBCKSRDY_R = crate::BitReader<USBCKSRDY_A>;
#[doc = "USB Clock (USBCLK) Switching Ready state flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCKSRDY_A {
    #[doc = "0: Impossible to Switch"]
    _0 = 0,
    #[doc = "1: Possible to Switch"]
    _1 = 1,
}
impl From<USBCKSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: USBCKSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl USBCKSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCKSRDY_A {
        match self.bits {
            false => USBCKSRDY_A::_0,
            true => USBCKSRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBCKSRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBCKSRDY_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Clock (USBCLK) Source Select"]
    #[inline(always)]
    pub fn usbcksel(&self) -> USBCKSEL_R {
        USBCKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 6 - USB Clock (USBCLK) Switching Request"]
    #[inline(always)]
    pub fn usbcksreq(&self) -> USBCKSREQ_R {
        USBCKSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Clock (USBCLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn usbcksrdy(&self) -> USBCKSRDY_R {
        USBCKSRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Clock (USBCLK) Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbcksel(&mut self) -> USBCKSEL_W<0> {
        USBCKSEL_W::new(self)
    }
    #[doc = "Bit 6 - USB Clock (USBCLK) Switching Request"]
    #[inline(always)]
    #[must_use]
    pub fn usbcksreq(&mut self) -> USBCKSREQ_W<6> {
        USBCKSREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbckcr](index.html) module"]
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
#[doc = "`reset()` method sets USBCKCR to value 0x01"]
impl crate::Resettable for USBCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
