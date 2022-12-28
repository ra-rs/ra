#[doc = "Register `NTBTHCTL0` reader"]
pub struct R(crate::R<NTBTHCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTBTHCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTBTHCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTBTHCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTBTHCTL0` writer"]
pub struct W(crate::W<NTBTHCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTBTHCTL0_SPEC>;
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
impl From<crate::W<NTBTHCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTBTHCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDBTH` reader - Normal Transmit Data Buffer Threshold"]
pub type TXDBTH_R = crate::FieldReader<u8, TXDBTH_A>;
#[doc = "Normal Transmit Data Buffer Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXDBTH_A {
    #[doc = "0: Interrupt triggers at 2 Tx Buffer empties, DWORDs"]
    _000 = 0,
    #[doc = "1: Reserved"]
    _001 = 1,
}
impl From<TXDBTH_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDBTH_A) -> Self {
        variant as _
    }
}
impl TXDBTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXDBTH_A> {
        match self.bits {
            0 => Some(TXDBTH_A::_000),
            1 => Some(TXDBTH_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TXDBTH_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TXDBTH_A::_001
    }
}
#[doc = "Field `TXDBTH` writer - Normal Transmit Data Buffer Threshold"]
pub type TXDBTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NTBTHCTL0_SPEC, u8, TXDBTH_A, 3, O>;
impl<'a, const O: u8> TXDBTH_W<'a, O> {
    #[doc = "Interrupt triggers at 2 Tx Buffer empties, DWORDs"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TXDBTH_A::_000)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TXDBTH_A::_001)
    }
}
#[doc = "Field `RXDBTH` reader - Normal Receive Data Buffer Threshold"]
pub type RXDBTH_R = crate::FieldReader<u8, RXDBTH_A>;
#[doc = "Normal Receive Data Buffer Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXDBTH_A {
    #[doc = "0: Interrupt triggers at 2 Rx Buffer entries, DWORDs"]
    _000 = 0,
    #[doc = "1: Reserved"]
    _001 = 1,
}
impl From<RXDBTH_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDBTH_A) -> Self {
        variant as _
    }
}
impl RXDBTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXDBTH_A> {
        match self.bits {
            0 => Some(RXDBTH_A::_000),
            1 => Some(RXDBTH_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RXDBTH_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RXDBTH_A::_001
    }
}
#[doc = "Field `RXDBTH` writer - Normal Receive Data Buffer Threshold"]
pub type RXDBTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NTBTHCTL0_SPEC, u8, RXDBTH_A, 3, O>;
impl<'a, const O: u8> RXDBTH_W<'a, O> {
    #[doc = "Interrupt triggers at 2 Rx Buffer entries, DWORDs"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RXDBTH_A::_000)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RXDBTH_A::_001)
    }
}
#[doc = "Field `TXSTTH` reader - Normal Tx Start Threshold"]
pub type TXSTTH_R = crate::FieldReader<u8, TXSTTH_A>;
#[doc = "Normal Tx Start Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXSTTH_A {
    #[doc = "0: Wait for 2 DWORDs"]
    _000 = 0,
    #[doc = "1: Reserved"]
    _001 = 1,
}
impl From<TXSTTH_A> for u8 {
    #[inline(always)]
    fn from(variant: TXSTTH_A) -> Self {
        variant as _
    }
}
impl TXSTTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXSTTH_A> {
        match self.bits {
            0 => Some(TXSTTH_A::_000),
            1 => Some(TXSTTH_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TXSTTH_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TXSTTH_A::_001
    }
}
#[doc = "Field `TXSTTH` writer - Normal Tx Start Threshold"]
pub type TXSTTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NTBTHCTL0_SPEC, u8, TXSTTH_A, 3, O>;
impl<'a, const O: u8> TXSTTH_W<'a, O> {
    #[doc = "Wait for 2 DWORDs"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TXSTTH_A::_000)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TXSTTH_A::_001)
    }
}
#[doc = "Field `RXSTTH` reader - Normal Rx Start Threshold"]
pub type RXSTTH_R = crate::FieldReader<u8, RXSTTH_A>;
#[doc = "Normal Rx Start Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXSTTH_A {
    #[doc = "0: Wait for 2 empty DWORDs"]
    _000 = 0,
    #[doc = "1: Reserved"]
    _001 = 1,
}
impl From<RXSTTH_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSTTH_A) -> Self {
        variant as _
    }
}
impl RXSTTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXSTTH_A> {
        match self.bits {
            0 => Some(RXSTTH_A::_000),
            1 => Some(RXSTTH_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RXSTTH_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RXSTTH_A::_001
    }
}
#[doc = "Field `RXSTTH` writer - Normal Rx Start Threshold"]
pub type RXSTTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NTBTHCTL0_SPEC, u8, RXSTTH_A, 3, O>;
impl<'a, const O: u8> RXSTTH_W<'a, O> {
    #[doc = "Wait for 2 empty DWORDs"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RXSTTH_A::_000)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RXSTTH_A::_001)
    }
}
impl R {
    #[doc = "Bits 0:2 - Normal Transmit Data Buffer Threshold"]
    #[inline(always)]
    pub fn txdbth(&self) -> TXDBTH_R {
        TXDBTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Normal Receive Data Buffer Threshold"]
    #[inline(always)]
    pub fn rxdbth(&self) -> RXDBTH_R {
        RXDBTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Normal Tx Start Threshold"]
    #[inline(always)]
    pub fn txstth(&self) -> TXSTTH_R {
        TXSTTH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Normal Rx Start Threshold"]
    #[inline(always)]
    pub fn rxstth(&self) -> RXSTTH_R {
        RXSTTH_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Normal Transmit Data Buffer Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn txdbth(&mut self) -> TXDBTH_W<0> {
        TXDBTH_W::new(self)
    }
    #[doc = "Bits 8:10 - Normal Receive Data Buffer Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rxdbth(&mut self) -> RXDBTH_W<8> {
        RXDBTH_W::new(self)
    }
    #[doc = "Bits 16:18 - Normal Tx Start Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn txstth(&mut self) -> TXSTTH_W<16> {
        TXSTTH_W::new(self)
    }
    #[doc = "Bits 24:26 - Normal Rx Start Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rxstth(&mut self) -> RXSTTH_W<24> {
        RXSTTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Transfer Data Buffer Threshold Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntbthctl0](index.html) module"]
pub struct NTBTHCTL0_SPEC;
impl crate::RegisterSpec for NTBTHCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ntbthctl0::R](R) reader structure"]
impl crate::Readable for NTBTHCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntbthctl0::W](W) writer structure"]
impl crate::Writable for NTBTHCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NTBTHCTL0 to value 0x0101_0101"]
impl crate::Resettable for NTBTHCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0101;
}
