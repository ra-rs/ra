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
#[doc = "Field `USBADDR` reader - USB Address"]
pub type USBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBADDR` writer - USB Address"]
pub type USBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USBADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `STSRECOV` reader - Status Recovery"]
pub type STSRECOV_R = crate::FieldReader<u8, STSRECOV_A>;
#[doc = "Status Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STSRECOV_A {
    #[doc = "4: Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the low-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 001b)"]
    _0X4 = 4,
    #[doc = "8: Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b)"]
    _0X8 = 8,
    #[doc = "9: Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (default state) Recovery in host controller mode: Setting prohibited"]
    _0X9 = 9,
    #[doc = "10: Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (address state) Recovery in host controller mode: Setting prohibited"]
    _0X_A = 10,
    #[doc = "11: Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (configured state) Recovery in host controller mode: Setting prohibited"]
    _0X_B = 11,
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
            4 => Some(STSRECOV_A::_0X4),
            8 => Some(STSRECOV_A::_0X8),
            9 => Some(STSRECOV_A::_0X9),
            10 => Some(STSRECOV_A::_0X_A),
            11 => Some(STSRECOV_A::_0X_B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == STSRECOV_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == STSRECOV_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == STSRECOV_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == STSRECOV_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == STSRECOV_A::_0X_B
    }
}
#[doc = "Field `STSRECOV` writer - Status Recovery"]
pub type STSRECOV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, USBADDR_SPEC, u8, STSRECOV_A, 4, O>;
impl<'a, const O: u8> STSRECOV_W<'a, O> {
    #[doc = "Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the low-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 001b)"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(STSRECOV_A::_0X4)
    }
    #[doc = "Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b)"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(STSRECOV_A::_0X8)
    }
    #[doc = "Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 001b (default state) Recovery in host controller mode: Setting prohibited"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(STSRECOV_A::_0X9)
    }
    #[doc = "Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 010b (address state) Recovery in host controller mode: Setting prohibited"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(STSRECOV_A::_0X_A)
    }
    #[doc = "Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\]
= 010b), bits INTSTS0.DVSQ\\[2:0\\]
= 011b (configured state) Recovery in host controller mode: Setting prohibited"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(STSRECOV_A::_0X_B)
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Address"]
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
    #[doc = "Bits 0:6 - USB Address"]
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
