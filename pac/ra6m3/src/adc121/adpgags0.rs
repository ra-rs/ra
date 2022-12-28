#[doc = "Register `ADPGAGS0` reader"]
pub struct R(crate::R<ADPGAGS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPGAGS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPGAGS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPGAGS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPGAGS0` writer"]
pub struct W(crate::W<ADPGAGS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPGAGS0_SPEC>;
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
impl From<crate::W<ADPGAGS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPGAGS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P000GAIN` reader - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0."]
pub type P000GAIN_R = crate::FieldReader<u8, P000GAIN_A>;
#[doc = "PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P000GAIN_A {
    #[doc = "0: x 2.000 (ADPGADDCR0.P000DEN=0)"]
    _0000 = 0,
    #[doc = "1: x 2.500 (ADPGADDCR0.P000DEN=0) / x 1.500 (ADPGADDCR0.P000DEN=1)"]
    _0001 = 1,
    #[doc = "2: x 2.667 (ADPGADDCR0.P000DEN=0)"]
    _0010 = 2,
    #[doc = "3: x 2.857 (ADPGADDCR0.P000DEN=0)"]
    _0011 = 3,
    #[doc = "4: x 3.077 (ADPGADDCR0.P000DEN=0)"]
    _0100 = 4,
    #[doc = "5: x 3.333 (ADPGADDCR0.P000DEN=0) / x 2.333 (ADPGADDCR0.P000DEN=1)"]
    _0101 = 5,
    #[doc = "6: x 3.636 (ADPGADDCR0.P000DEN=0)"]
    _0110 = 6,
    #[doc = "7: x 4.000 (ADPGADDCR0.P000DEN=0)"]
    _0111 = 7,
    #[doc = "8: x 4.444 (ADPGADDCR0.P000DEN=0)"]
    _1000 = 8,
    #[doc = "9: x 5.000 (ADPGADDCR0.P000DEN=0) / x 4.00 (ADPGADDCR0.P000DEN=1)"]
    _1001 = 9,
    #[doc = "10: x 5.714 (ADPGADDCR0.P000DEN=0)"]
    _1010 = 10,
    #[doc = "11: x 6.667 (ADPGADDCR0.P000DEN=0) / x 5.667 (ADPGADDCR0.P000DEN=1)"]
    _1011 = 11,
    #[doc = "12: x 8.000 (ADPGADDCR0.P000DEN=0)"]
    _1100 = 12,
    #[doc = "13: x 10.000 (ADPGADDCR0.P000DEN=0)"]
    _1101 = 13,
    #[doc = "14: x 13.333 (ADPGADDCR0.P000DEN=0)"]
    _1110 = 14,
    #[doc = "15: x 1.000 (for offset measurement) (ADPGADDCR0.P000DEN=0)"]
    _1111 = 15,
}
impl From<P000GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: P000GAIN_A) -> Self {
        variant as _
    }
}
impl P000GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P000GAIN_A {
        match self.bits {
            0 => P000GAIN_A::_0000,
            1 => P000GAIN_A::_0001,
            2 => P000GAIN_A::_0010,
            3 => P000GAIN_A::_0011,
            4 => P000GAIN_A::_0100,
            5 => P000GAIN_A::_0101,
            6 => P000GAIN_A::_0110,
            7 => P000GAIN_A::_0111,
            8 => P000GAIN_A::_1000,
            9 => P000GAIN_A::_1001,
            10 => P000GAIN_A::_1010,
            11 => P000GAIN_A::_1011,
            12 => P000GAIN_A::_1100,
            13 => P000GAIN_A::_1101,
            14 => P000GAIN_A::_1110,
            15 => P000GAIN_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == P000GAIN_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == P000GAIN_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == P000GAIN_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == P000GAIN_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == P000GAIN_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == P000GAIN_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == P000GAIN_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == P000GAIN_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == P000GAIN_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == P000GAIN_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == P000GAIN_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == P000GAIN_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == P000GAIN_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == P000GAIN_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == P000GAIN_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == P000GAIN_A::_1111
    }
}
#[doc = "Field `P000GAIN` writer - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0."]
pub type P000GAIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADPGAGS0_SPEC, u8, P000GAIN_A, 4, O>;
impl<'a, const O: u8> P000GAIN_W<'a, O> {
    #[doc = "x 2.000 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0000)
    }
    #[doc = "x 2.500 (ADPGADDCR0.P000DEN=0) / x 1.500 (ADPGADDCR0.P000DEN=1)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0001)
    }
    #[doc = "x 2.667 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0010)
    }
    #[doc = "x 2.857 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0011)
    }
    #[doc = "x 3.077 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0100)
    }
    #[doc = "x 3.333 (ADPGADDCR0.P000DEN=0) / x 2.333 (ADPGADDCR0.P000DEN=1)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0101)
    }
    #[doc = "x 3.636 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0110)
    }
    #[doc = "x 4.000 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(P000GAIN_A::_0111)
    }
    #[doc = "x 4.444 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1000)
    }
    #[doc = "x 5.000 (ADPGADDCR0.P000DEN=0) / x 4.00 (ADPGADDCR0.P000DEN=1)"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1001)
    }
    #[doc = "x 5.714 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1010)
    }
    #[doc = "x 6.667 (ADPGADDCR0.P000DEN=0) / x 5.667 (ADPGADDCR0.P000DEN=1)"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1011)
    }
    #[doc = "x 8.000 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1100)
    }
    #[doc = "x 10.000 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1101)
    }
    #[doc = "x 13.333 (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1110)
    }
    #[doc = "x 1.000 (for offset measurement) (ADPGADDCR0.P000DEN=0)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(P000GAIN_A::_1111)
    }
}
#[doc = "Field `P001GAIN` reader - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0."]
pub type P001GAIN_R = crate::FieldReader<u8, P001GAIN_A>;
#[doc = "PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P001GAIN_A {
    #[doc = "0: x 2.000 (ADPGADDCR0.P001DEN=0)"]
    _0000 = 0,
    #[doc = "1: x 2.500 (ADPGADDCR0.P001DEN=0) / x 1.500 (ADPGADDCR0.P001DEN=1)"]
    _0001 = 1,
    #[doc = "2: x 2.667 (ADPGADDCR0.P001DEN=0)"]
    _0010 = 2,
    #[doc = "3: x 2.857 (ADPGADDCR0.P001DEN=0)"]
    _0011 = 3,
    #[doc = "4: x 3.077 (ADPGADDCR0.P001DEN=0)"]
    _0100 = 4,
    #[doc = "5: x 3.333 (ADPGADDCR0.P001DEN=0) / x 2.333 (ADPGADDCR0.P001DEN=1)"]
    _0101 = 5,
    #[doc = "6: x 3.636 (ADPGADDCR0.P001DEN=0)"]
    _0110 = 6,
    #[doc = "7: x 4.000 (ADPGADDCR0.P001DEN=0)"]
    _0111 = 7,
    #[doc = "8: x 4.444 (ADPGADDCR0.P001DEN=0)"]
    _1000 = 8,
    #[doc = "9: x 5.000 (ADPGADDCR0.P001DEN=0) / x 4.00 (ADPGADDCR0.P001DEN=1)"]
    _1001 = 9,
    #[doc = "10: x 5.714 (ADPGADDCR0.P001DEN=0)"]
    _1010 = 10,
    #[doc = "11: x 6.667 (ADPGADDCR0.P001DEN=0) / x 5.667 (ADPGADDCR0.P001DEN=1)"]
    _1011 = 11,
    #[doc = "12: x 8.000 (ADPGADDCR0.P001DEN=0)"]
    _1100 = 12,
    #[doc = "13: x 10.000 (ADPGADDCR0.P001DEN=0)"]
    _1101 = 13,
    #[doc = "14: x 13.333 (ADPGADDCR0.P001DEN=0)"]
    _1110 = 14,
    #[doc = "15: x 1.000 (for offset measurement) (ADPGADDCR0.P001DEN=0)"]
    _1111 = 15,
}
impl From<P001GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: P001GAIN_A) -> Self {
        variant as _
    }
}
impl P001GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P001GAIN_A {
        match self.bits {
            0 => P001GAIN_A::_0000,
            1 => P001GAIN_A::_0001,
            2 => P001GAIN_A::_0010,
            3 => P001GAIN_A::_0011,
            4 => P001GAIN_A::_0100,
            5 => P001GAIN_A::_0101,
            6 => P001GAIN_A::_0110,
            7 => P001GAIN_A::_0111,
            8 => P001GAIN_A::_1000,
            9 => P001GAIN_A::_1001,
            10 => P001GAIN_A::_1010,
            11 => P001GAIN_A::_1011,
            12 => P001GAIN_A::_1100,
            13 => P001GAIN_A::_1101,
            14 => P001GAIN_A::_1110,
            15 => P001GAIN_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == P001GAIN_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == P001GAIN_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == P001GAIN_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == P001GAIN_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == P001GAIN_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == P001GAIN_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == P001GAIN_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == P001GAIN_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == P001GAIN_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == P001GAIN_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == P001GAIN_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == P001GAIN_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == P001GAIN_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == P001GAIN_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == P001GAIN_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == P001GAIN_A::_1111
    }
}
#[doc = "Field `P001GAIN` writer - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0."]
pub type P001GAIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADPGAGS0_SPEC, u8, P001GAIN_A, 4, O>;
impl<'a, const O: u8> P001GAIN_W<'a, O> {
    #[doc = "x 2.000 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0000)
    }
    #[doc = "x 2.500 (ADPGADDCR0.P001DEN=0) / x 1.500 (ADPGADDCR0.P001DEN=1)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0001)
    }
    #[doc = "x 2.667 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0010)
    }
    #[doc = "x 2.857 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0011)
    }
    #[doc = "x 3.077 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0100)
    }
    #[doc = "x 3.333 (ADPGADDCR0.P001DEN=0) / x 2.333 (ADPGADDCR0.P001DEN=1)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0101)
    }
    #[doc = "x 3.636 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0110)
    }
    #[doc = "x 4.000 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(P001GAIN_A::_0111)
    }
    #[doc = "x 4.444 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1000)
    }
    #[doc = "x 5.000 (ADPGADDCR0.P001DEN=0) / x 4.00 (ADPGADDCR0.P001DEN=1)"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1001)
    }
    #[doc = "x 5.714 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1010)
    }
    #[doc = "x 6.667 (ADPGADDCR0.P001DEN=0) / x 5.667 (ADPGADDCR0.P001DEN=1)"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1011)
    }
    #[doc = "x 8.000 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1100)
    }
    #[doc = "x 10.000 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1101)
    }
    #[doc = "x 13.333 (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1110)
    }
    #[doc = "x 1.000 (for offset measurement) (ADPGADDCR0.P001DEN=0)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(P001GAIN_A::_1111)
    }
}
#[doc = "Field `P002GAIN` reader - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0."]
pub type P002GAIN_R = crate::FieldReader<u8, P002GAIN_A>;
#[doc = "PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P002GAIN_A {
    #[doc = "0: x 2.000 (ADPGADDCR0.P002DEN=0)"]
    _0000 = 0,
    #[doc = "1: x 2.500 (ADPGADDCR0.P002DEN=0) / x 1.500 (ADPGADDCR0.P002DEN=1)"]
    _0001 = 1,
    #[doc = "2: x 2.667 (ADPGADDCR0.P002DEN=0)"]
    _0010 = 2,
    #[doc = "3: x 2.857 (ADPGADDCR0.P002DEN=0)"]
    _0011 = 3,
    #[doc = "4: x 3.077 (ADPGADDCR0.P002DEN=0)"]
    _0100 = 4,
    #[doc = "5: x 3.333 (ADPGADDCR0.P002DEN=0) / x 2.333 (ADPGADDCR0.P002DEN=1)"]
    _0101 = 5,
    #[doc = "6: x 3.636 (ADPGADDCR0.P002DEN=0)"]
    _0110 = 6,
    #[doc = "7: x 4.000 (ADPGADDCR0.P002DEN=0)"]
    _0111 = 7,
    #[doc = "8: x 4.444 (ADPGADDCR0.P002DEN=0)"]
    _1000 = 8,
    #[doc = "9: x 5.000 (ADPGADDCR0.P002DEN=0) / x 4.00 (ADPGADDCR0.P002DEN=1)"]
    _1001 = 9,
    #[doc = "10: x 5.714 (ADPGADDCR0.P002DEN=0)"]
    _1010 = 10,
    #[doc = "11: x 6.667 (ADPGADDCR0.P002DEN=0) / x 5.667 (ADPGADDCR0.P002DEN=1)"]
    _1011 = 11,
    #[doc = "12: x 8.000 (ADPGADDCR0.P002DEN=0)"]
    _1100 = 12,
    #[doc = "13: x 10.000 (ADPGADDCR0.P002DEN=0)"]
    _1101 = 13,
    #[doc = "14: x 13.333 (ADPGADDCR0.P002DEN=0)"]
    _1110 = 14,
    #[doc = "15: x 1.000 (for offset measurement) (ADPGADDCR0.P002DEN=0)"]
    _1111 = 15,
}
impl From<P002GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: P002GAIN_A) -> Self {
        variant as _
    }
}
impl P002GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P002GAIN_A {
        match self.bits {
            0 => P002GAIN_A::_0000,
            1 => P002GAIN_A::_0001,
            2 => P002GAIN_A::_0010,
            3 => P002GAIN_A::_0011,
            4 => P002GAIN_A::_0100,
            5 => P002GAIN_A::_0101,
            6 => P002GAIN_A::_0110,
            7 => P002GAIN_A::_0111,
            8 => P002GAIN_A::_1000,
            9 => P002GAIN_A::_1001,
            10 => P002GAIN_A::_1010,
            11 => P002GAIN_A::_1011,
            12 => P002GAIN_A::_1100,
            13 => P002GAIN_A::_1101,
            14 => P002GAIN_A::_1110,
            15 => P002GAIN_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == P002GAIN_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == P002GAIN_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == P002GAIN_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == P002GAIN_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == P002GAIN_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == P002GAIN_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == P002GAIN_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == P002GAIN_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == P002GAIN_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == P002GAIN_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == P002GAIN_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == P002GAIN_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == P002GAIN_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == P002GAIN_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == P002GAIN_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == P002GAIN_A::_1111
    }
}
#[doc = "Field `P002GAIN` writer - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0."]
pub type P002GAIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADPGAGS0_SPEC, u8, P002GAIN_A, 4, O>;
impl<'a, const O: u8> P002GAIN_W<'a, O> {
    #[doc = "x 2.000 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0000)
    }
    #[doc = "x 2.500 (ADPGADDCR0.P002DEN=0) / x 1.500 (ADPGADDCR0.P002DEN=1)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0001)
    }
    #[doc = "x 2.667 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0010)
    }
    #[doc = "x 2.857 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0011)
    }
    #[doc = "x 3.077 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0100)
    }
    #[doc = "x 3.333 (ADPGADDCR0.P002DEN=0) / x 2.333 (ADPGADDCR0.P002DEN=1)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0101)
    }
    #[doc = "x 3.636 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0110)
    }
    #[doc = "x 4.000 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(P002GAIN_A::_0111)
    }
    #[doc = "x 4.444 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1000)
    }
    #[doc = "x 5.000 (ADPGADDCR0.P002DEN=0) / x 4.00 (ADPGADDCR0.P002DEN=1)"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1001)
    }
    #[doc = "x 5.714 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1010)
    }
    #[doc = "x 6.667 (ADPGADDCR0.P002DEN=0) / x 5.667 (ADPGADDCR0.P002DEN=1)"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1011)
    }
    #[doc = "x 8.000 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1100)
    }
    #[doc = "x 10.000 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1101)
    }
    #[doc = "x 13.333 (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1110)
    }
    #[doc = "x 1.000 (for offset measurement) (ADPGADDCR0.P002DEN=0)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(P002GAIN_A::_1111)
    }
}
impl R {
    #[doc = "Bits 0:3 - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0."]
    #[inline(always)]
    pub fn p000gain(&self) -> P000GAIN_R {
        P000GAIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0."]
    #[inline(always)]
    pub fn p001gain(&self) -> P001GAIN_R {
        P001GAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0."]
    #[inline(always)]
    pub fn p002gain(&self) -> P002GAIN_R {
        P002GAIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0."]
    #[inline(always)]
    #[must_use]
    pub fn p000gain(&mut self) -> P000GAIN_W<0> {
        P000GAIN_W::new(self)
    }
    #[doc = "Bits 4:7 - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0."]
    #[inline(always)]
    #[must_use]
    pub fn p001gain(&mut self) -> P001GAIN_W<4> {
        P001GAIN_W::new(self)
    }
    #[doc = "Bits 8:11 - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0."]
    #[inline(always)]
    #[must_use]
    pub fn p002gain(&mut self) -> P002GAIN_W<8> {
        P002GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Programmable Gain Amplifier Gain Setting Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpgags0](index.html) module"]
pub struct ADPGAGS0_SPEC;
impl crate::RegisterSpec for ADPGAGS0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adpgags0::R](R) reader structure"]
impl crate::Readable for ADPGAGS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpgags0::W](W) writer structure"]
impl crate::Writable for ADPGAGS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADPGAGS0 to value 0"]
impl crate::Resettable for ADPGAGS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
