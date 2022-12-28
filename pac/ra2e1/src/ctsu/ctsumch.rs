#[doc = "Register `CTSUMCH` reader"]
pub struct R(crate::R<CTSUMCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUMCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUMCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUMCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUMCH` writer"]
pub struct W(crate::W<CTSUMCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUMCH_SPEC>;
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
impl From<crate::W<CTSUMCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUMCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCH0` reader - CTSU Measurement Channel 0"]
pub type MCH0_R = crate::FieldReader<u8, MCH0_A>;
#[doc = "CTSU Measurement Channel 0\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCH0_A {
    #[doc = "0: TS00"]
    _0X00 = 0,
    #[doc = "2: TS02"]
    _0X02 = 2,
    #[doc = "4: TS04"]
    _0X04 = 4,
    #[doc = "5: TS05"]
    _0X05 = 5,
    #[doc = "6: TS06"]
    _0X06 = 6,
    #[doc = "7: TS07"]
    _0X07 = 7,
    #[doc = "8: TS08"]
    _0X08 = 8,
    #[doc = "9: TS09"]
    _0X09 = 9,
    #[doc = "10: TS10"]
    _0X0A = 10,
    #[doc = "11: TS11"]
    _0X0B = 11,
    #[doc = "12: TS12"]
    _0X0C = 12,
    #[doc = "13: TS13"]
    _0X0D = 13,
    #[doc = "14: TS14"]
    _0X0E = 14,
    #[doc = "15: TS15"]
    _0X0F = 15,
    #[doc = "16: TS16"]
    _0X10 = 16,
    #[doc = "17: TS17"]
    _0X11 = 17,
    #[doc = "18: TS18"]
    _0X12 = 18,
    #[doc = "21: TS21"]
    _0X15 = 21,
    #[doc = "22: TS22"]
    _0X16 = 22,
    #[doc = "23: TS23"]
    _0X17 = 23,
    #[doc = "24: TS24"]
    _0X18 = 24,
    #[doc = "25: TS25"]
    _0X19 = 25,
    #[doc = "26: TS26"]
    _0X1A = 26,
    #[doc = "27: TS27"]
    _0X1B = 27,
    #[doc = "28: TS28"]
    _0X1C = 28,
    #[doc = "30: TS30"]
    _0X1E = 30,
    #[doc = "31: TS31"]
    _0X1F = 31,
    #[doc = "32: TS32"]
    _0X20 = 32,
    #[doc = "33: TS33"]
    _0X21 = 33,
    #[doc = "34: TS34"]
    _0X22 = 34,
    #[doc = "63: Measurement is being stopped."]
    _0X3F = 63,
}
impl From<MCH0_A> for u8 {
    #[inline(always)]
    fn from(variant: MCH0_A) -> Self {
        variant as _
    }
}
impl MCH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCH0_A> {
        match self.bits {
            0 => Some(MCH0_A::_0X00),
            2 => Some(MCH0_A::_0X02),
            4 => Some(MCH0_A::_0X04),
            5 => Some(MCH0_A::_0X05),
            6 => Some(MCH0_A::_0X06),
            7 => Some(MCH0_A::_0X07),
            8 => Some(MCH0_A::_0X08),
            9 => Some(MCH0_A::_0X09),
            10 => Some(MCH0_A::_0X0A),
            11 => Some(MCH0_A::_0X0B),
            12 => Some(MCH0_A::_0X0C),
            13 => Some(MCH0_A::_0X0D),
            14 => Some(MCH0_A::_0X0E),
            15 => Some(MCH0_A::_0X0F),
            16 => Some(MCH0_A::_0X10),
            17 => Some(MCH0_A::_0X11),
            18 => Some(MCH0_A::_0X12),
            21 => Some(MCH0_A::_0X15),
            22 => Some(MCH0_A::_0X16),
            23 => Some(MCH0_A::_0X17),
            24 => Some(MCH0_A::_0X18),
            25 => Some(MCH0_A::_0X19),
            26 => Some(MCH0_A::_0X1A),
            27 => Some(MCH0_A::_0X1B),
            28 => Some(MCH0_A::_0X1C),
            30 => Some(MCH0_A::_0X1E),
            31 => Some(MCH0_A::_0X1F),
            32 => Some(MCH0_A::_0X20),
            33 => Some(MCH0_A::_0X21),
            34 => Some(MCH0_A::_0X22),
            63 => Some(MCH0_A::_0X3F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == MCH0_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == MCH0_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == MCH0_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == MCH0_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == MCH0_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == MCH0_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == MCH0_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == MCH0_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == MCH0_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == MCH0_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == MCH0_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == MCH0_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == MCH0_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == MCH0_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == MCH0_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == MCH0_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == MCH0_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X15`"]
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == MCH0_A::_0X15
    }
    #[doc = "Checks if the value of the field is `_0X16`"]
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == MCH0_A::_0X16
    }
    #[doc = "Checks if the value of the field is `_0X17`"]
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == MCH0_A::_0X17
    }
    #[doc = "Checks if the value of the field is `_0X18`"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == MCH0_A::_0X18
    }
    #[doc = "Checks if the value of the field is `_0X19`"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == MCH0_A::_0X19
    }
    #[doc = "Checks if the value of the field is `_0X1A`"]
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == MCH0_A::_0X1A
    }
    #[doc = "Checks if the value of the field is `_0X1B`"]
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == MCH0_A::_0X1B
    }
    #[doc = "Checks if the value of the field is `_0X1C`"]
    #[inline(always)]
    pub fn is_0x1c(&self) -> bool {
        *self == MCH0_A::_0X1C
    }
    #[doc = "Checks if the value of the field is `_0X1E`"]
    #[inline(always)]
    pub fn is_0x1e(&self) -> bool {
        *self == MCH0_A::_0X1E
    }
    #[doc = "Checks if the value of the field is `_0X1F`"]
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == MCH0_A::_0X1F
    }
    #[doc = "Checks if the value of the field is `_0X20`"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == MCH0_A::_0X20
    }
    #[doc = "Checks if the value of the field is `_0X21`"]
    #[inline(always)]
    pub fn is_0x21(&self) -> bool {
        *self == MCH0_A::_0X21
    }
    #[doc = "Checks if the value of the field is `_0X22`"]
    #[inline(always)]
    pub fn is_0x22(&self) -> bool {
        *self == MCH0_A::_0X22
    }
    #[doc = "Checks if the value of the field is `_0X3F`"]
    #[inline(always)]
    pub fn is_0x3f(&self) -> bool {
        *self == MCH0_A::_0X3F
    }
}
#[doc = "Field `MCH0` writer - CTSU Measurement Channel 0"]
pub type MCH0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUMCH_SPEC, u8, MCH0_A, 6, O>;
impl<'a, const O: u8> MCH0_W<'a, O> {
    #[doc = "TS00"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(MCH0_A::_0X00)
    }
    #[doc = "TS02"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(MCH0_A::_0X02)
    }
    #[doc = "TS04"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(MCH0_A::_0X04)
    }
    #[doc = "TS05"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(MCH0_A::_0X05)
    }
    #[doc = "TS06"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(MCH0_A::_0X06)
    }
    #[doc = "TS07"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(MCH0_A::_0X07)
    }
    #[doc = "TS08"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(MCH0_A::_0X08)
    }
    #[doc = "TS09"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(MCH0_A::_0X09)
    }
    #[doc = "TS10"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(MCH0_A::_0X0A)
    }
    #[doc = "TS11"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(MCH0_A::_0X0B)
    }
    #[doc = "TS12"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(MCH0_A::_0X0C)
    }
    #[doc = "TS13"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(MCH0_A::_0X0D)
    }
    #[doc = "TS14"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(MCH0_A::_0X0E)
    }
    #[doc = "TS15"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(MCH0_A::_0X0F)
    }
    #[doc = "TS16"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(MCH0_A::_0X10)
    }
    #[doc = "TS17"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(MCH0_A::_0X11)
    }
    #[doc = "TS18"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(MCH0_A::_0X12)
    }
    #[doc = "TS21"]
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut W {
        self.variant(MCH0_A::_0X15)
    }
    #[doc = "TS22"]
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut W {
        self.variant(MCH0_A::_0X16)
    }
    #[doc = "TS23"]
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut W {
        self.variant(MCH0_A::_0X17)
    }
    #[doc = "TS24"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut W {
        self.variant(MCH0_A::_0X18)
    }
    #[doc = "TS25"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut W {
        self.variant(MCH0_A::_0X19)
    }
    #[doc = "TS26"]
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut W {
        self.variant(MCH0_A::_0X1A)
    }
    #[doc = "TS27"]
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut W {
        self.variant(MCH0_A::_0X1B)
    }
    #[doc = "TS28"]
    #[inline(always)]
    pub fn _0x1c(self) -> &'a mut W {
        self.variant(MCH0_A::_0X1C)
    }
    #[doc = "TS30"]
    #[inline(always)]
    pub fn _0x1e(self) -> &'a mut W {
        self.variant(MCH0_A::_0X1E)
    }
    #[doc = "TS31"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(MCH0_A::_0X1F)
    }
    #[doc = "TS32"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut W {
        self.variant(MCH0_A::_0X20)
    }
    #[doc = "TS33"]
    #[inline(always)]
    pub fn _0x21(self) -> &'a mut W {
        self.variant(MCH0_A::_0X21)
    }
    #[doc = "TS34"]
    #[inline(always)]
    pub fn _0x22(self) -> &'a mut W {
        self.variant(MCH0_A::_0X22)
    }
    #[doc = "Measurement is being stopped."]
    #[inline(always)]
    pub fn _0x3f(self) -> &'a mut W {
        self.variant(MCH0_A::_0X3F)
    }
}
#[doc = "Field `MCH1` reader - CTSU Measurement Channel 1"]
pub type MCH1_R = crate::FieldReader<u8, MCH1_A>;
#[doc = "CTSU Measurement Channel 1\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCH1_A {
    #[doc = "0: TS00"]
    _0X00 = 0,
    #[doc = "2: TS02"]
    _0X02 = 2,
    #[doc = "4: TS04"]
    _0X04 = 4,
    #[doc = "5: TS05"]
    _0X05 = 5,
    #[doc = "6: TS06"]
    _0X06 = 6,
    #[doc = "7: TS07"]
    _0X07 = 7,
    #[doc = "8: TS08"]
    _0X08 = 8,
    #[doc = "9: TS09"]
    _0X09 = 9,
    #[doc = "10: TS10"]
    _0X0A = 10,
    #[doc = "11: TS11"]
    _0X0B = 11,
    #[doc = "12: TS12"]
    _0X0C = 12,
    #[doc = "13: TS13"]
    _0X0D = 13,
    #[doc = "14: TS14"]
    _0X0E = 14,
    #[doc = "15: TS15"]
    _0X0F = 15,
    #[doc = "16: TS16"]
    _0X10 = 16,
    #[doc = "17: TS17"]
    _0X11 = 17,
    #[doc = "18: TS18"]
    _0X12 = 18,
    #[doc = "21: TS21"]
    _0X15 = 21,
    #[doc = "22: TS22"]
    _0X16 = 22,
    #[doc = "23: TS23"]
    _0X17 = 23,
    #[doc = "24: TS24"]
    _0X18 = 24,
    #[doc = "25: TS25"]
    _0X19 = 25,
    #[doc = "26: TS26"]
    _0X1A = 26,
    #[doc = "27: TS27"]
    _0X1B = 27,
    #[doc = "28: TS28"]
    _0X1C = 28,
    #[doc = "30: TS30"]
    _0X1E = 30,
    #[doc = "31: TS31"]
    _0X1F = 31,
    #[doc = "32: TS32"]
    _0X20 = 32,
    #[doc = "33: TS33"]
    _0X21 = 33,
    #[doc = "34: TS34"]
    _0X22 = 34,
    #[doc = "63: Measurement is being stopped."]
    _0X3F = 63,
}
impl From<MCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: MCH1_A) -> Self {
        variant as _
    }
}
impl MCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCH1_A> {
        match self.bits {
            0 => Some(MCH1_A::_0X00),
            2 => Some(MCH1_A::_0X02),
            4 => Some(MCH1_A::_0X04),
            5 => Some(MCH1_A::_0X05),
            6 => Some(MCH1_A::_0X06),
            7 => Some(MCH1_A::_0X07),
            8 => Some(MCH1_A::_0X08),
            9 => Some(MCH1_A::_0X09),
            10 => Some(MCH1_A::_0X0A),
            11 => Some(MCH1_A::_0X0B),
            12 => Some(MCH1_A::_0X0C),
            13 => Some(MCH1_A::_0X0D),
            14 => Some(MCH1_A::_0X0E),
            15 => Some(MCH1_A::_0X0F),
            16 => Some(MCH1_A::_0X10),
            17 => Some(MCH1_A::_0X11),
            18 => Some(MCH1_A::_0X12),
            21 => Some(MCH1_A::_0X15),
            22 => Some(MCH1_A::_0X16),
            23 => Some(MCH1_A::_0X17),
            24 => Some(MCH1_A::_0X18),
            25 => Some(MCH1_A::_0X19),
            26 => Some(MCH1_A::_0X1A),
            27 => Some(MCH1_A::_0X1B),
            28 => Some(MCH1_A::_0X1C),
            30 => Some(MCH1_A::_0X1E),
            31 => Some(MCH1_A::_0X1F),
            32 => Some(MCH1_A::_0X20),
            33 => Some(MCH1_A::_0X21),
            34 => Some(MCH1_A::_0X22),
            63 => Some(MCH1_A::_0X3F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == MCH1_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == MCH1_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == MCH1_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == MCH1_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == MCH1_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == MCH1_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == MCH1_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == MCH1_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == MCH1_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == MCH1_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == MCH1_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == MCH1_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == MCH1_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == MCH1_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == MCH1_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == MCH1_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == MCH1_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X15`"]
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == MCH1_A::_0X15
    }
    #[doc = "Checks if the value of the field is `_0X16`"]
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == MCH1_A::_0X16
    }
    #[doc = "Checks if the value of the field is `_0X17`"]
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == MCH1_A::_0X17
    }
    #[doc = "Checks if the value of the field is `_0X18`"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == MCH1_A::_0X18
    }
    #[doc = "Checks if the value of the field is `_0X19`"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == MCH1_A::_0X19
    }
    #[doc = "Checks if the value of the field is `_0X1A`"]
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == MCH1_A::_0X1A
    }
    #[doc = "Checks if the value of the field is `_0X1B`"]
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == MCH1_A::_0X1B
    }
    #[doc = "Checks if the value of the field is `_0X1C`"]
    #[inline(always)]
    pub fn is_0x1c(&self) -> bool {
        *self == MCH1_A::_0X1C
    }
    #[doc = "Checks if the value of the field is `_0X1E`"]
    #[inline(always)]
    pub fn is_0x1e(&self) -> bool {
        *self == MCH1_A::_0X1E
    }
    #[doc = "Checks if the value of the field is `_0X1F`"]
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == MCH1_A::_0X1F
    }
    #[doc = "Checks if the value of the field is `_0X20`"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == MCH1_A::_0X20
    }
    #[doc = "Checks if the value of the field is `_0X21`"]
    #[inline(always)]
    pub fn is_0x21(&self) -> bool {
        *self == MCH1_A::_0X21
    }
    #[doc = "Checks if the value of the field is `_0X22`"]
    #[inline(always)]
    pub fn is_0x22(&self) -> bool {
        *self == MCH1_A::_0X22
    }
    #[doc = "Checks if the value of the field is `_0X3F`"]
    #[inline(always)]
    pub fn is_0x3f(&self) -> bool {
        *self == MCH1_A::_0X3F
    }
}
#[doc = "Field `MCH1` writer - CTSU Measurement Channel 1"]
pub type MCH1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUMCH_SPEC, u8, MCH1_A, 6, O>;
impl<'a, const O: u8> MCH1_W<'a, O> {
    #[doc = "TS00"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(MCH1_A::_0X00)
    }
    #[doc = "TS02"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(MCH1_A::_0X02)
    }
    #[doc = "TS04"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(MCH1_A::_0X04)
    }
    #[doc = "TS05"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(MCH1_A::_0X05)
    }
    #[doc = "TS06"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(MCH1_A::_0X06)
    }
    #[doc = "TS07"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(MCH1_A::_0X07)
    }
    #[doc = "TS08"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(MCH1_A::_0X08)
    }
    #[doc = "TS09"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(MCH1_A::_0X09)
    }
    #[doc = "TS10"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(MCH1_A::_0X0A)
    }
    #[doc = "TS11"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(MCH1_A::_0X0B)
    }
    #[doc = "TS12"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(MCH1_A::_0X0C)
    }
    #[doc = "TS13"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(MCH1_A::_0X0D)
    }
    #[doc = "TS14"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(MCH1_A::_0X0E)
    }
    #[doc = "TS15"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(MCH1_A::_0X0F)
    }
    #[doc = "TS16"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(MCH1_A::_0X10)
    }
    #[doc = "TS17"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(MCH1_A::_0X11)
    }
    #[doc = "TS18"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(MCH1_A::_0X12)
    }
    #[doc = "TS21"]
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut W {
        self.variant(MCH1_A::_0X15)
    }
    #[doc = "TS22"]
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut W {
        self.variant(MCH1_A::_0X16)
    }
    #[doc = "TS23"]
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut W {
        self.variant(MCH1_A::_0X17)
    }
    #[doc = "TS24"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut W {
        self.variant(MCH1_A::_0X18)
    }
    #[doc = "TS25"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut W {
        self.variant(MCH1_A::_0X19)
    }
    #[doc = "TS26"]
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut W {
        self.variant(MCH1_A::_0X1A)
    }
    #[doc = "TS27"]
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut W {
        self.variant(MCH1_A::_0X1B)
    }
    #[doc = "TS28"]
    #[inline(always)]
    pub fn _0x1c(self) -> &'a mut W {
        self.variant(MCH1_A::_0X1C)
    }
    #[doc = "TS30"]
    #[inline(always)]
    pub fn _0x1e(self) -> &'a mut W {
        self.variant(MCH1_A::_0X1E)
    }
    #[doc = "TS31"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(MCH1_A::_0X1F)
    }
    #[doc = "TS32"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut W {
        self.variant(MCH1_A::_0X20)
    }
    #[doc = "TS33"]
    #[inline(always)]
    pub fn _0x21(self) -> &'a mut W {
        self.variant(MCH1_A::_0X21)
    }
    #[doc = "TS34"]
    #[inline(always)]
    pub fn _0x22(self) -> &'a mut W {
        self.variant(MCH1_A::_0X22)
    }
    #[doc = "Measurement is being stopped."]
    #[inline(always)]
    pub fn _0x3f(self) -> &'a mut W {
        self.variant(MCH1_A::_0X3F)
    }
}
#[doc = "Field `MCA0` reader - Multiple Clocks Control"]
pub type MCA0_R = crate::BitReader<MCA0_A>;
#[doc = "Multiple Clocks Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCA0_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<MCA0_A> for bool {
    #[inline(always)]
    fn from(variant: MCA0_A) -> Self {
        variant as u8 != 0
    }
}
impl MCA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCA0_A {
        match self.bits {
            false => MCA0_A::_0,
            true => MCA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCA0_A::_1
    }
}
#[doc = "Field `MCA0` writer - Multiple Clocks Control"]
pub type MCA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUMCH_SPEC, MCA0_A, O>;
impl<'a, const O: u8> MCA0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCA0_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCA0_A::_1)
    }
}
#[doc = "Field `MCA1` reader - Multiple Clocks Control"]
pub type MCA1_R = crate::BitReader<MCA1_A>;
#[doc = "Multiple Clocks Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCA1_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<MCA1_A> for bool {
    #[inline(always)]
    fn from(variant: MCA1_A) -> Self {
        variant as u8 != 0
    }
}
impl MCA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCA1_A {
        match self.bits {
            false => MCA1_A::_0,
            true => MCA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCA1_A::_1
    }
}
#[doc = "Field `MCA1` writer - Multiple Clocks Control"]
pub type MCA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUMCH_SPEC, MCA1_A, O>;
impl<'a, const O: u8> MCA1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCA1_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCA1_A::_1)
    }
}
#[doc = "Field `MCA2` reader - Multiple Clocks Control"]
pub type MCA2_R = crate::BitReader<MCA2_A>;
#[doc = "Multiple Clocks Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCA2_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<MCA2_A> for bool {
    #[inline(always)]
    fn from(variant: MCA2_A) -> Self {
        variant as u8 != 0
    }
}
impl MCA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCA2_A {
        match self.bits {
            false => MCA2_A::_0,
            true => MCA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCA2_A::_1
    }
}
#[doc = "Field `MCA2` writer - Multiple Clocks Control"]
pub type MCA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUMCH_SPEC, MCA2_A, O>;
impl<'a, const O: u8> MCA2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCA2_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCA2_A::_1)
    }
}
#[doc = "Field `MCA3` reader - Multiple Clocks Control"]
pub type MCA3_R = crate::BitReader<MCA3_A>;
#[doc = "Multiple Clocks Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCA3_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<MCA3_A> for bool {
    #[inline(always)]
    fn from(variant: MCA3_A) -> Self {
        variant as u8 != 0
    }
}
impl MCA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCA3_A {
        match self.bits {
            false => MCA3_A::_0,
            true => MCA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCA3_A::_1
    }
}
#[doc = "Field `MCA3` writer - Multiple Clocks Control"]
pub type MCA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUMCH_SPEC, MCA3_A, O>;
impl<'a, const O: u8> MCA3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCA3_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCA3_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 0"]
    #[inline(always)]
    pub fn mch0(&self) -> MCH0_R {
        MCH0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - CTSU Measurement Channel 1"]
    #[inline(always)]
    pub fn mch1(&self) -> MCH1_R {
        MCH1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca0(&self) -> MCA0_R {
        MCA0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca1(&self) -> MCA1_R {
        MCA1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca2(&self) -> MCA2_R {
        MCA2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca3(&self) -> MCA3_R {
        MCA3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn mch0(&mut self) -> MCH0_W<0> {
        MCH0_W::new(self)
    }
    #[doc = "Bits 8:13 - CTSU Measurement Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn mch1(&mut self) -> MCH1_W<8> {
        MCH1_W::new(self)
    }
    #[doc = "Bit 16 - Multiple Clocks Control"]
    #[inline(always)]
    #[must_use]
    pub fn mca0(&mut self) -> MCA0_W<16> {
        MCA0_W::new(self)
    }
    #[doc = "Bit 17 - Multiple Clocks Control"]
    #[inline(always)]
    #[must_use]
    pub fn mca1(&mut self) -> MCA1_W<17> {
        MCA1_W::new(self)
    }
    #[doc = "Bit 18 - Multiple Clocks Control"]
    #[inline(always)]
    #[must_use]
    pub fn mca2(&mut self) -> MCA2_W<18> {
        MCA2_W::new(self)
    }
    #[doc = "Bit 19 - Multiple Clocks Control"]
    #[inline(always)]
    #[must_use]
    pub fn mca3(&mut self) -> MCA3_W<19> {
        MCA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Measurement Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsumch](index.html) module"]
pub struct CTSUMCH_SPEC;
impl crate::RegisterSpec for CTSUMCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsumch::R](R) reader structure"]
impl crate::Readable for CTSUMCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsumch::W](W) writer structure"]
impl crate::Writable for CTSUMCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUMCH to value 0x3f3f"]
impl crate::Resettable for CTSUMCH_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f3f;
}
