#[doc = "Register `USBADDR` reader"]
pub struct R(crate::R<USBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBADDR` writer"]
pub struct W(crate::W<USBADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBADDR_SPEC>;
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
impl From<crate::W<USBADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBADDR` reader - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed."]
pub type USBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBADDR` writer - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed."]
pub type USBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USBADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `STSRECOV` reader - Status Recovery"]
pub type STSRECOV_R = crate::FieldReader<u8, STSRECOV_A>;
#[doc = "Status Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STSRECOV_A {
    #[doc = "4: Return to the low-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 001b;(Recovery when the host controller is selected))"]
    _0100 = 4,
    #[doc = "8: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b;(Recovery when the host controller is selected))"]
    _1000 = 8,
    #[doc = "9: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (Default state);(Recovery when the function controller is selected)"]
    _1001 = 9,
    #[doc = "10: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (Address state);(Recovery when the function controller is selected)"]
    _1010 = 10,
    #[doc = "11: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (Configured state);(Recovery when the function controller is selected)"]
    _1011 = 11,
}
impl From<STSRECOV_A> for u8 {
    #[inline(always)]
    fn from(variant: STSRECOV_A) -> Self {
        variant as _
    }
}
impl STSRECOV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STSRECOV_A> {
        match self.bits {
            4 => Some(STSRECOV_A::_0100),
            8 => Some(STSRECOV_A::_1000),
            9 => Some(STSRECOV_A::_1001),
            10 => Some(STSRECOV_A::_1010),
            11 => Some(STSRECOV_A::_1011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == STSRECOV_A::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == STSRECOV_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == STSRECOV_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == STSRECOV_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == STSRECOV_A::_1011
    }
}
#[doc = "Field `STSRECOV` writer - Status Recovery"]
pub type STSRECOV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, USBADDR_SPEC, u8, STSRECOV_A, 4, O>;
impl<'a, const O: u8> STSRECOV_W<'a, O> {
    #[doc = "Return to the low-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 001b;(Recovery when the host controller is selected))"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(STSRECOV_A::_0100)
    }
    #[doc = "Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b;(Recovery when the host controller is selected))"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(STSRECOV_A::_1000)
    }
    #[doc = "Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (Default state);(Recovery when the function controller is selected)"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(STSRECOV_A::_1001)
    }
    #[doc = "Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (Address state);(Recovery when the function controller is selected)"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(STSRECOV_A::_1010)
    }
    #[doc = "Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (Configured state);(Recovery when the function controller is selected)"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(STSRECOV_A::_1011)
    }
}
impl R {
    #[doc = "Bits 0:6 - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed."]
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Status Recovery"]
    #[inline(always)]
    pub fn stsrecov(&self) -> STSRECOV_R {
        STSRECOV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed."]
    #[inline(always)]
    #[must_use]
    pub fn usbaddr(&mut self) -> USBADDR_W<0> {
        USBADDR_W::new(self)
    }
    #[doc = "Bits 8:11 - Status Recovery"]
    #[inline(always)]
    #[must_use]
    pub fn stsrecov(&mut self) -> STSRECOV_W<8> {
        STSRECOV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbaddr](index.html) module"]
pub struct USBADDR_SPEC;
impl crate::RegisterSpec for USBADDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbaddr::R](R) reader structure"]
impl crate::Readable for USBADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbaddr::W](W) writer structure"]
impl crate::Writable for USBADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBADDR to value 0"]
impl crate::Resettable for USBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
