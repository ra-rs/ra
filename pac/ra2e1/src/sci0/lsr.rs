#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ORER` reader - Overrun Error Flag"]
pub type ORER_R = crate::BitReader<ORER_A>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORER_A {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: Overrun error occurred"]
    _1 = 1,
}
impl From<ORER_A> for bool {
    #[inline(always)]
    fn from(variant: ORER_A) -> Self {
        variant as u8 != 0
    }
}
impl ORER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORER_A {
        match self.bits {
            false => ORER_A::_0,
            true => ORER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORER_A::_1
    }
}
#[doc = "Field `FNUM` reader - Framing Error Count"]
pub type FNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PNUM` reader - Parity Error Count"]
pub type PNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:6 - Framing Error Count"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Parity Error Count"]
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Line Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
