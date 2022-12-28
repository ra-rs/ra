#[doc = "Register `CRCCR0` reader"]
pub struct R(crate::R<CRCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCR0` writer"]
pub struct W(crate::W<CRCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCR0_SPEC>;
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
impl From<crate::W<CRCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPS` reader - CRC Generating Polynomial Switching"]
pub type GPS_R = crate::FieldReader<u8, GPS_A>;
#[doc = "CRC Generating Polynomial Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPS_A {
    #[doc = "0: No calculation is executed."]
    _000 = 0,
    #[doc = "1: 8-bit CRC-8 (X8 + X2 + X + 1)"]
    _001 = 1,
    #[doc = "2: 16-bit CRC-16 (X16 + X15 + X2 + 1)"]
    _010 = 2,
    #[doc = "3: 16-bit CRC-CCITT (X16 + X12 + X5 + 1)"]
    _011 = 3,
    #[doc = "4: 32-bit CRC-32(X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)"]
    _100 = 4,
    #[doc = "5: 32-bit CRC-32C (X32+X28+X27+X26+X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)"]
    _101 = 5,
}
impl From<GPS_A> for u8 {
    #[inline(always)]
    fn from(variant: GPS_A) -> Self {
        variant as _
    }
}
impl GPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPS_A> {
        match self.bits {
            0 => Some(GPS_A::_000),
            1 => Some(GPS_A::_001),
            2 => Some(GPS_A::_010),
            3 => Some(GPS_A::_011),
            4 => Some(GPS_A::_100),
            5 => Some(GPS_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == GPS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == GPS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == GPS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == GPS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == GPS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == GPS_A::_101
    }
}
#[doc = "Field `GPS` writer - CRC Generating Polynomial Switching"]
pub type GPS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CRCCR0_SPEC, u8, GPS_A, 3, O>;
impl<'a, const O: u8> GPS_W<'a, O> {
    #[doc = "No calculation is executed."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(GPS_A::_000)
    }
    #[doc = "8-bit CRC-8 (X8 + X2 + X + 1)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(GPS_A::_001)
    }
    #[doc = "16-bit CRC-16 (X16 + X15 + X2 + 1)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(GPS_A::_010)
    }
    #[doc = "16-bit CRC-CCITT (X16 + X12 + X5 + 1)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(GPS_A::_011)
    }
    #[doc = "32-bit CRC-32(X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(GPS_A::_100)
    }
    #[doc = "32-bit CRC-32C (X32+X28+X27+X26+X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(GPS_A::_101)
    }
}
#[doc = "Field `LMS` reader - CRC Calculation Switching"]
pub type LMS_R = crate::BitReader<LMS_A>;
#[doc = "CRC Calculation Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LMS_A {
    #[doc = "0: Generates CRC for LSB first communication."]
    _0 = 0,
    #[doc = "1: Generates CRC for MSB first communication."]
    _1 = 1,
}
impl From<LMS_A> for bool {
    #[inline(always)]
    fn from(variant: LMS_A) -> Self {
        variant as u8 != 0
    }
}
impl LMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMS_A {
        match self.bits {
            false => LMS_A::_0,
            true => LMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LMS_A::_1
    }
}
#[doc = "Field `LMS` writer - CRC Calculation Switching"]
pub type LMS_W<'a, const O: u8> = crate::BitWriter<'a, u8, CRCCR0_SPEC, LMS_A, O>;
impl<'a, const O: u8> LMS_W<'a, O> {
    #[doc = "Generates CRC for LSB first communication."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LMS_A::_0)
    }
    #[doc = "Generates CRC for MSB first communication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LMS_A::_1)
    }
}
#[doc = "CRCDOR Register Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DORCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clears the CRCDOR register."]
    _1 = 1,
}
impl From<DORCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: DORCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORCLR` writer - CRCDOR Register Clear"]
pub type DORCLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CRCCR0_SPEC, DORCLR_AW, O>;
impl<'a, const O: u8> DORCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DORCLR_AW::_0)
    }
    #[doc = "Clears the CRCDOR register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DORCLR_AW::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - CRC Generating Polynomial Switching"]
    #[inline(always)]
    pub fn gps(&self) -> GPS_R {
        GPS_R::new(self.bits & 7)
    }
    #[doc = "Bit 6 - CRC Calculation Switching"]
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CRC Generating Polynomial Switching"]
    #[inline(always)]
    #[must_use]
    pub fn gps(&mut self) -> GPS_W<0> {
        GPS_W::new(self)
    }
    #[doc = "Bit 6 - CRC Calculation Switching"]
    #[inline(always)]
    #[must_use]
    pub fn lms(&mut self) -> LMS_W<6> {
        LMS_W::new(self)
    }
    #[doc = "Bit 7 - CRCDOR Register Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dorclr(&mut self) -> DORCLR_W<7> {
        DORCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccr0](index.html) module"]
pub struct CRCCR0_SPEC;
impl crate::RegisterSpec for CRCCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crccr0::R](R) reader structure"]
impl crate::Readable for CRCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crccr0::W](W) writer structure"]
impl crate::Writable for CRCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCR0 to value 0"]
impl crate::Resettable for CRCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
