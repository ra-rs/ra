#[doc = "Register `STC1` reader"]
pub struct R(crate::R<STC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STC1` writer"]
pub struct W(crate::W<STC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STC1_SPEC>;
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
impl From<crate::W<STC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - SDADC24 reference clock division select"]
pub type CLKDIV_R = crate::FieldReader<u8, CLKDIV_A>;
#[doc = "SDADC24 reference clock division select\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: SDADCCLK (no division)"]
    _0000 = 0,
    #[doc = "1: SDADCCLK/2 (1/2)"]
    _0001 = 1,
    #[doc = "2: SDADCCLK/3 (1/3)"]
    _0010 = 2,
    #[doc = "3: SDADCCLK/4 (1/4)"]
    _0011 = 3,
    #[doc = "4: SDADCCLK/5 (1/5)"]
    _0100 = 4,
    #[doc = "5: SDADCCLK/6 (1/6)"]
    _0101 = 5,
    #[doc = "6: SDADCCLK/8 (1/8)"]
    _0110 = 6,
    #[doc = "7: SDADCCLK/12 (1/12)"]
    _0111 = 7,
    #[doc = "8: SDADCCLK/16 (1/16)."]
    _1000 = 8,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKDIV_A> {
        match self.bits {
            0 => Some(CLKDIV_A::_0000),
            1 => Some(CLKDIV_A::_0001),
            2 => Some(CLKDIV_A::_0010),
            3 => Some(CLKDIV_A::_0011),
            4 => Some(CLKDIV_A::_0100),
            5 => Some(CLKDIV_A::_0101),
            6 => Some(CLKDIV_A::_0110),
            7 => Some(CLKDIV_A::_0111),
            8 => Some(CLKDIV_A::_1000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CLKDIV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CLKDIV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CLKDIV_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CLKDIV_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CLKDIV_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CLKDIV_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CLKDIV_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CLKDIV_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CLKDIV_A::_1000
    }
}
#[doc = "Field `CLKDIV` writer - SDADC24 reference clock division select"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STC1_SPEC, u8, CLKDIV_A, 4, O>;
impl<'a, const O: u8> CLKDIV_W<'a, O> {
    #[doc = "SDADCCLK (no division)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0000)
    }
    #[doc = "SDADCCLK/2 (1/2)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0001)
    }
    #[doc = "SDADCCLK/3 (1/3)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0010)
    }
    #[doc = "SDADCCLK/4 (1/4)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0011)
    }
    #[doc = "SDADCCLK/5 (1/5)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0100)
    }
    #[doc = "SDADCCLK/6 (1/6)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0101)
    }
    #[doc = "SDADCCLK/8 (1/8)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0110)
    }
    #[doc = "SDADCCLK/12 (1/12)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CLKDIV_A::_0111)
    }
    #[doc = "SDADCCLK/16 (1/16)."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CLKDIV_A::_1000)
    }
}
#[doc = "Field `SDADLPM` reader - A/D conversion operation mode select"]
pub type SDADLPM_R = crate::BitReader<SDADLPM_A>;
#[doc = "A/D conversion operation mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADLPM_A {
    #[doc = "0: Normal A/D conversion mode: SDADC24 reference clock:4 MHz, Oversampling clock:1 MHz"]
    _0 = 0,
    #[doc = "1: Low-power A/D conversion mode(1/8 of the clock in normal A/D conversion mode): SDADC24 reference clock:500 kHz,Oversampling clock: 125 kHz"]
    _1 = 1,
}
impl From<SDADLPM_A> for bool {
    #[inline(always)]
    fn from(variant: SDADLPM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADLPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADLPM_A {
        match self.bits {
            false => SDADLPM_A::_0,
            true => SDADLPM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADLPM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADLPM_A::_1
    }
}
#[doc = "Field `SDADLPM` writer - A/D conversion operation mode select"]
pub type SDADLPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, STC1_SPEC, SDADLPM_A, O>;
impl<'a, const O: u8> SDADLPM_W<'a, O> {
    #[doc = "Normal A/D conversion mode: SDADC24 reference clock:4 MHz, Oversampling clock:1 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDADLPM_A::_0)
    }
    #[doc = "Low-power A/D conversion mode(1/8 of the clock in normal A/D conversion mode): SDADC24 reference clock:500 kHz,Oversampling clock: 125 kHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDADLPM_A::_1)
    }
}
#[doc = "Field `VSBIAS` reader - Reference voltage select"]
pub type VSBIAS_R = crate::FieldReader<u8, VSBIAS_A>;
#[doc = "Reference voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VSBIAS_A {
    #[doc = "0: 0.8 V"]
    _0000 = 0,
    #[doc = "1: 1.0 V"]
    _0001 = 1,
    #[doc = "2: 1.2 V"]
    _0010 = 2,
    #[doc = "3: 1.4 V"]
    _0011 = 3,
    #[doc = "4: 1.6 V"]
    _0100 = 4,
    #[doc = "5: 1.8 V"]
    _0101 = 5,
    #[doc = "6: 2.0 V"]
    _0110 = 6,
    #[doc = "7: 2.2 V"]
    _0111 = 7,
    #[doc = "15: 2.4 V (This voltage can be set only if VREFSEL = 1. When VREFSEL = 0, 2.2 V is set (rather than 2.4 V))"]
    _1111 = 15,
}
impl From<VSBIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: VSBIAS_A) -> Self {
        variant as _
    }
}
impl VSBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VSBIAS_A> {
        match self.bits {
            0 => Some(VSBIAS_A::_0000),
            1 => Some(VSBIAS_A::_0001),
            2 => Some(VSBIAS_A::_0010),
            3 => Some(VSBIAS_A::_0011),
            4 => Some(VSBIAS_A::_0100),
            5 => Some(VSBIAS_A::_0101),
            6 => Some(VSBIAS_A::_0110),
            7 => Some(VSBIAS_A::_0111),
            15 => Some(VSBIAS_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == VSBIAS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == VSBIAS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == VSBIAS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == VSBIAS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == VSBIAS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == VSBIAS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == VSBIAS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == VSBIAS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == VSBIAS_A::_1111
    }
}
#[doc = "Field `VSBIAS` writer - Reference voltage select"]
pub type VSBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STC1_SPEC, u8, VSBIAS_A, 4, O>;
impl<'a, const O: u8> VSBIAS_W<'a, O> {
    #[doc = "0.8 V"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0000)
    }
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0001)
    }
    #[doc = "1.2 V"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0010)
    }
    #[doc = "1.4 V"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0011)
    }
    #[doc = "1.6 V"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0100)
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0101)
    }
    #[doc = "2.0 V"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0110)
    }
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(VSBIAS_A::_0111)
    }
    #[doc = "2.4 V (This voltage can be set only if VREFSEL = 1. When VREFSEL = 0, 2.2 V is set (rather than 2.4 V))"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(VSBIAS_A::_1111)
    }
}
#[doc = "Field `VREFSEL` reader - VREF mode select"]
pub type VREFSEL_R = crate::BitReader<VREFSEL_A>;
#[doc = "VREF mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFSEL_A {
    #[doc = "0: Internal VREF mode"]
    _0 = 0,
    #[doc = "1: External VREF mode"]
    _1 = 1,
}
impl From<VREFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VREFSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFSEL_A {
        match self.bits {
            false => VREFSEL_A::_0,
            true => VREFSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFSEL_A::_1
    }
}
#[doc = "Field `VREFSEL` writer - VREF mode select"]
pub type VREFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, STC1_SPEC, VREFSEL_A, O>;
impl<'a, const O: u8> VREFSEL_W<'a, O> {
    #[doc = "Internal VREF mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFSEL_A::_0)
    }
    #[doc = "External VREF mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFSEL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - SDADC24 reference clock division select"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - A/D conversion operation mode select"]
    #[inline(always)]
    pub fn sdadlpm(&self) -> SDADLPM_R {
        SDADLPM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Reference voltage select"]
    #[inline(always)]
    pub fn vsbias(&self) -> VSBIAS_R {
        VSBIAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - VREF mode select"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SDADC24 reference clock division select"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 7 - A/D conversion operation mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sdadlpm(&mut self) -> SDADLPM_W<7> {
        SDADLPM_W::new(self)
    }
    #[doc = "Bits 8:11 - Reference voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn vsbias(&mut self) -> VSBIAS_W<8> {
        VSBIAS_W::new(self)
    }
    #[doc = "Bit 15 - VREF mode select"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<15> {
        VREFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Startup Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stc1](index.html) module"]
pub struct STC1_SPEC;
impl crate::RegisterSpec for STC1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stc1::R](R) reader structure"]
impl crate::Readable for STC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stc1::W](W) writer structure"]
impl crate::Writable for STC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STC1 to value 0x8008"]
impl crate::Resettable for STC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8008;
}
