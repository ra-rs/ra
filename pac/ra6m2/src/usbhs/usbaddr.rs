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
#[doc = "Field `USBADDR` reader - USB Address In device controller mode, these flags indicate the USB address assigned by the host when the USBHS processed the SET_ADDRESS request successfully."]
pub type USBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STSRECOV0` reader - Status Recovery"]
pub type STSRECOV0_R = crate::FieldReader<u8, STSRECOV0_A>;
#[doc = "Status Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STSRECOV0_A {
    #[doc = "1: Return to the full-speed state(bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (Default state)(function controller selected)"]
    _001 = 1,
    #[doc = "2: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (Address state)(function controller selected)/ Return to the low-speed state (bitsDVSTCTR0.RHST\\[2:0\\]
= 001b)(host controller is selected)"]
    _010 = 2,
    #[doc = "3: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (Configured state)(function controller selected)"]
    _011 = 3,
    #[doc = "4: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b)(host controller selected)"]
    _100 = 4,
    #[doc = "5: Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (Default state)(function controller selected)"]
    _101 = 5,
    #[doc = "6: Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (Address state)(function controller selected)/ Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b)(host controller selected)"]
    _110 = 6,
    #[doc = "7: Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (Configured state)(function controller selected)"]
    _111 = 7,
}
impl From<STSRECOV0_A> for u8 {
    #[inline(always)]
    fn from(variant: STSRECOV0_A) -> Self {
        variant as _
    }
}
impl STSRECOV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STSRECOV0_A {
        match self.bits {
            1 => STSRECOV0_A::_001,
            2 => STSRECOV0_A::_010,
            3 => STSRECOV0_A::_011,
            4 => STSRECOV0_A::_100,
            5 => STSRECOV0_A::_101,
            6 => STSRECOV0_A::_110,
            7 => STSRECOV0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == STSRECOV0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == STSRECOV0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == STSRECOV0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == STSRECOV0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == STSRECOV0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == STSRECOV0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == STSRECOV0_A::_111
    }
}
#[doc = "Field `STSRECOV0` writer - Status Recovery"]
pub type STSRECOV0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, USBADDR_SPEC, u8, STSRECOV0_A, 3, O>;
impl<'a, const O: u8> STSRECOV0_W<'a, O> {
    #[doc = "Return to the full-speed state(bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (Default state)(function controller selected)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(STSRECOV0_A::_001)
    }
    #[doc = "Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (Address state)(function controller selected)/ Return to the low-speed state (bitsDVSTCTR0.RHST\\[2:0\\]
= 001b)(host controller is selected)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(STSRECOV0_A::_010)
    }
    #[doc = "Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (Configured state)(function controller selected)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(STSRECOV0_A::_011)
    }
    #[doc = "Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b)(host controller selected)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(STSRECOV0_A::_100)
    }
    #[doc = "Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (Default state)(function controller selected)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(STSRECOV0_A::_101)
    }
    #[doc = "Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (Address state)(function controller selected)/ Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b)(host controller selected)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(STSRECOV0_A::_110)
    }
    #[doc = "Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 011b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (Configured state)(function controller selected)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(STSRECOV0_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Address In device controller mode, these flags indicate the USB address assigned by the host when the USBHS processed the SET_ADDRESS request successfully."]
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Status Recovery"]
    #[inline(always)]
    pub fn stsrecov0(&self) -> STSRECOV0_R {
        STSRECOV0_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Status Recovery"]
    #[inline(always)]
    #[must_use]
    pub fn stsrecov0(&mut self) -> STSRECOV0_W<8> {
        STSRECOV0_W::new(self)
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
