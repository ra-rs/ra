#[doc = "Register `GTIOR` reader"]
pub struct R(crate::R<GTIOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTIOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTIOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTIOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTIOR` writer"]
pub struct W(crate::W<GTIOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTIOR_SPEC>;
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
impl From<crate::W<GTIOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTIOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTIOA` reader - GTIOCA Pin Function Select"]
pub type GTIOA_R = crate::FieldReader<u8, GTIOA_A>;
#[doc = "GTIOCA Pin Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GTIOA_A {
    #[doc = "0: Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match."]
    _00000 = 0,
    #[doc = "1: Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match."]
    _00001 = 1,
    #[doc = "2: Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match."]
    _00010 = 2,
    #[doc = "3: Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    _00011 = 3,
    #[doc = "4: Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match."]
    _00100 = 4,
    #[doc = "5: Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match."]
    _00101 = 5,
    #[doc = "6: Initial output is Low. Low output at cycle end. High output at GTCCRA compare match."]
    _00110 = 6,
    #[doc = "7: Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match."]
    _00111 = 7,
    #[doc = "8: Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match."]
    _01000 = 8,
    #[doc = "9: Initial output is Low. High output at cycle end. Low output at GTCCRA compare match."]
    _01001 = 9,
    #[doc = "10: Initial output is Low. High output at cycle end. High output at GTCCRA compare match."]
    _01010 = 10,
    #[doc = "11: Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match."]
    _01011 = 11,
    #[doc = "12: Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    _01100 = 12,
    #[doc = "13: Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match."]
    _01101 = 13,
    #[doc = "14: Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match."]
    _01110 = 14,
    #[doc = "15: Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    _01111 = 15,
    #[doc = "16: Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match."]
    _10000 = 16,
    #[doc = "17: Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match."]
    _10001 = 17,
    #[doc = "18: Initial output is High. Output retained at cycle end. High output at GTCCRA compare match."]
    _10010 = 18,
    #[doc = "19: Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    _10011 = 19,
    #[doc = "20: Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match."]
    _10100 = 20,
    #[doc = "21: Initial output is High. Low output at cycle end. Low output at GTCCRA compare match."]
    _10101 = 21,
    #[doc = "22: Initial output is High. Low output at cycle end. High output at GTCCRA compare match."]
    _10110 = 22,
    #[doc = "23: Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match."]
    _10111 = 23,
    #[doc = "24: Initial output is High. High output at cycle end. Output retained at GTCCRA compare match."]
    _11000 = 24,
    #[doc = "25: Initial output is High. High output at cycle end. Low output at GTCCRA compare match."]
    _11001 = 25,
    #[doc = "26: Initial output is High. High output at cycle end. High output at GTCCRA compare match."]
    _11010 = 26,
    #[doc = "27: Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match."]
    _11011 = 27,
    #[doc = "28: Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    _11100 = 28,
    #[doc = "29: Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match."]
    _11101 = 29,
    #[doc = "30: Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match."]
    _11110 = 30,
    #[doc = "31: Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    _11111 = 31,
}
impl From<GTIOA_A> for u8 {
    #[inline(always)]
    fn from(variant: GTIOA_A) -> Self {
        variant as _
    }
}
impl GTIOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTIOA_A {
        match self.bits {
            0 => GTIOA_A::_00000,
            1 => GTIOA_A::_00001,
            2 => GTIOA_A::_00010,
            3 => GTIOA_A::_00011,
            4 => GTIOA_A::_00100,
            5 => GTIOA_A::_00101,
            6 => GTIOA_A::_00110,
            7 => GTIOA_A::_00111,
            8 => GTIOA_A::_01000,
            9 => GTIOA_A::_01001,
            10 => GTIOA_A::_01010,
            11 => GTIOA_A::_01011,
            12 => GTIOA_A::_01100,
            13 => GTIOA_A::_01101,
            14 => GTIOA_A::_01110,
            15 => GTIOA_A::_01111,
            16 => GTIOA_A::_10000,
            17 => GTIOA_A::_10001,
            18 => GTIOA_A::_10010,
            19 => GTIOA_A::_10011,
            20 => GTIOA_A::_10100,
            21 => GTIOA_A::_10101,
            22 => GTIOA_A::_10110,
            23 => GTIOA_A::_10111,
            24 => GTIOA_A::_11000,
            25 => GTIOA_A::_11001,
            26 => GTIOA_A::_11010,
            27 => GTIOA_A::_11011,
            28 => GTIOA_A::_11100,
            29 => GTIOA_A::_11101,
            30 => GTIOA_A::_11110,
            31 => GTIOA_A::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == GTIOA_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == GTIOA_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == GTIOA_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == GTIOA_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == GTIOA_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == GTIOA_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == GTIOA_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == GTIOA_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == GTIOA_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == GTIOA_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == GTIOA_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == GTIOA_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == GTIOA_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == GTIOA_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == GTIOA_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == GTIOA_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == GTIOA_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == GTIOA_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == GTIOA_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == GTIOA_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == GTIOA_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == GTIOA_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == GTIOA_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == GTIOA_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == GTIOA_A::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == GTIOA_A::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == GTIOA_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == GTIOA_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == GTIOA_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == GTIOA_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == GTIOA_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == GTIOA_A::_11111
    }
}
#[doc = "Field `GTIOA` writer - GTIOCA Pin Function Select"]
pub type GTIOA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, GTIOA_A, 5, O>;
impl<'a, const O: u8> GTIOA_W<'a, O> {
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(GTIOA_A::_00000)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(GTIOA_A::_00001)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(GTIOA_A::_00010)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(GTIOA_A::_00011)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(GTIOA_A::_00100)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(GTIOA_A::_00101)
    }
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(GTIOA_A::_00110)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(GTIOA_A::_00111)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(GTIOA_A::_01000)
    }
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(GTIOA_A::_01001)
    }
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(GTIOA_A::_01010)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(GTIOA_A::_01011)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(GTIOA_A::_01100)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(GTIOA_A::_01101)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(GTIOA_A::_01110)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(GTIOA_A::_01111)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(GTIOA_A::_10000)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(GTIOA_A::_10001)
    }
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(GTIOA_A::_10010)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(GTIOA_A::_10011)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(GTIOA_A::_10100)
    }
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(GTIOA_A::_10101)
    }
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(GTIOA_A::_10110)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(GTIOA_A::_10111)
    }
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut W {
        self.variant(GTIOA_A::_11000)
    }
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut W {
        self.variant(GTIOA_A::_11001)
    }
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(GTIOA_A::_11010)
    }
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(GTIOA_A::_11011)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(GTIOA_A::_11100)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(GTIOA_A::_11101)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(GTIOA_A::_11110)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(GTIOA_A::_11111)
    }
}
#[doc = "Field `OADFLT` reader - GTIOCA Pin Output Value Setting at the Count Stop"]
pub type OADFLT_R = crate::BitReader<OADFLT_A>;
#[doc = "GTIOCA Pin Output Value Setting at the Count Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OADFLT_A {
    #[doc = "0: The GTIOCA pin outputs low when counting is stopped."]
    _0 = 0,
    #[doc = "1: The GTIOCA pin outputs high when counting is stopped."]
    _1 = 1,
}
impl From<OADFLT_A> for bool {
    #[inline(always)]
    fn from(variant: OADFLT_A) -> Self {
        variant as u8 != 0
    }
}
impl OADFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OADFLT_A {
        match self.bits {
            false => OADFLT_A::_0,
            true => OADFLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OADFLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OADFLT_A::_1
    }
}
#[doc = "Field `OADFLT` writer - GTIOCA Pin Output Value Setting at the Count Stop"]
pub type OADFLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OADFLT_A, O>;
impl<'a, const O: u8> OADFLT_W<'a, O> {
    #[doc = "The GTIOCA pin outputs low when counting is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OADFLT_A::_0)
    }
    #[doc = "The GTIOCA pin outputs high when counting is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OADFLT_A::_1)
    }
}
#[doc = "Field `OAHLD` reader - GTIOCA Pin Output Setting at the Start/Stop Count"]
pub type OAHLD_R = crate::BitReader<OAHLD_A>;
#[doc = "GTIOCA Pin Output Setting at the Start/Stop Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAHLD_A {
    #[doc = "0: The GTIOCA pin output level at start/stop of counting depends on the register setting."]
    _0 = 0,
    #[doc = "1: The GTIOCA pin output level is retained at start/stop of counting."]
    _1 = 1,
}
impl From<OAHLD_A> for bool {
    #[inline(always)]
    fn from(variant: OAHLD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAHLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAHLD_A {
        match self.bits {
            false => OAHLD_A::_0,
            true => OAHLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAHLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAHLD_A::_1
    }
}
#[doc = "Field `OAHLD` writer - GTIOCA Pin Output Setting at the Start/Stop Count"]
pub type OAHLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OAHLD_A, O>;
impl<'a, const O: u8> OAHLD_W<'a, O> {
    #[doc = "The GTIOCA pin output level at start/stop of counting depends on the register setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAHLD_A::_0)
    }
    #[doc = "The GTIOCA pin output level is retained at start/stop of counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAHLD_A::_1)
    }
}
#[doc = "Field `OAE` reader - GTIOCA Pin Output Enable"]
pub type OAE_R = crate::BitReader<OAE_A>;
#[doc = "GTIOCA Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAE_A {
    #[doc = "0: Output is disabled"]
    _0 = 0,
    #[doc = "1: Output is enabled"]
    _1 = 1,
}
impl From<OAE_A> for bool {
    #[inline(always)]
    fn from(variant: OAE_A) -> Self {
        variant as u8 != 0
    }
}
impl OAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAE_A {
        match self.bits {
            false => OAE_A::_0,
            true => OAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAE_A::_1
    }
}
#[doc = "Field `OAE` writer - GTIOCA Pin Output Enable"]
pub type OAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OAE_A, O>;
impl<'a, const O: u8> OAE_W<'a, O> {
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAE_A::_0)
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAE_A::_1)
    }
}
#[doc = "Field `OADF` reader - GTIOCA Pin Disable Value Setting"]
pub type OADF_R = crate::FieldReader<u8, OADF_A>;
#[doc = "GTIOCA Pin Disable Value Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OADF_A {
    #[doc = "0: Output disable is prohibited."]
    _00 = 0,
    #[doc = "1: GTIOCA pin is set to Hi-Z when output disable is performed."]
    _01 = 1,
    #[doc = "2: GTIOCA pin is set to 0 when output disable is performed."]
    _10 = 2,
    #[doc = "3: GTIOCA pin is set to 1 when output disable is performed."]
    _11 = 3,
}
impl From<OADF_A> for u8 {
    #[inline(always)]
    fn from(variant: OADF_A) -> Self {
        variant as _
    }
}
impl OADF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OADF_A {
        match self.bits {
            0 => OADF_A::_00,
            1 => OADF_A::_01,
            2 => OADF_A::_10,
            3 => OADF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OADF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OADF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OADF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OADF_A::_11
    }
}
#[doc = "Field `OADF` writer - GTIOCA Pin Disable Value Setting"]
pub type OADF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, OADF_A, 2, O>;
impl<'a, const O: u8> OADF_W<'a, O> {
    #[doc = "Output disable is prohibited."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OADF_A::_00)
    }
    #[doc = "GTIOCA pin is set to Hi-Z when output disable is performed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OADF_A::_01)
    }
    #[doc = "GTIOCA pin is set to 0 when output disable is performed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OADF_A::_10)
    }
    #[doc = "GTIOCA pin is set to 1 when output disable is performed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OADF_A::_11)
    }
}
#[doc = "Field `NFAEN` reader - Noise Filter A Enable"]
pub type NFAEN_R = crate::BitReader<NFAEN_A>;
#[doc = "Noise Filter A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFAEN_A {
    #[doc = "0: The noise filter for the GTIOCA pin is disabled."]
    _0 = 0,
    #[doc = "1: The noise filter for the GTIOCA pin is enabled."]
    _1 = 1,
}
impl From<NFAEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFAEN_A {
        match self.bits {
            false => NFAEN_A::_0,
            true => NFAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFAEN_A::_1
    }
}
#[doc = "Field `NFAEN` writer - Noise Filter A Enable"]
pub type NFAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, NFAEN_A, O>;
impl<'a, const O: u8> NFAEN_W<'a, O> {
    #[doc = "The noise filter for the GTIOCA pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFAEN_A::_0)
    }
    #[doc = "The noise filter for the GTIOCA pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFAEN_A::_1)
    }
}
#[doc = "Field `NFCSA` reader - Noise Filter A Sampling Clock Select"]
pub type NFCSA_R = crate::FieldReader<u8, NFCSA_A>;
#[doc = "Noise Filter A Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCSA_A {
    #[doc = "0: PCLK/1"]
    _00 = 0,
    #[doc = "1: PCLK/4"]
    _01 = 1,
    #[doc = "2: PCLK/16"]
    _10 = 2,
    #[doc = "3: PCLK/64"]
    _11 = 3,
}
impl From<NFCSA_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCSA_A) -> Self {
        variant as _
    }
}
impl NFCSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCSA_A {
        match self.bits {
            0 => NFCSA_A::_00,
            1 => NFCSA_A::_01,
            2 => NFCSA_A::_10,
            3 => NFCSA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCSA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCSA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCSA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCSA_A::_11
    }
}
#[doc = "Field `NFCSA` writer - Noise Filter A Sampling Clock Select"]
pub type NFCSA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, NFCSA_A, 2, O>;
impl<'a, const O: u8> NFCSA_W<'a, O> {
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCSA_A::_00)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCSA_A::_01)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCSA_A::_10)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCSA_A::_11)
    }
}
#[doc = "Field `GTIOB` reader - GTIOCB Pin Function Select"]
pub type GTIOB_R = crate::FieldReader<u8, GTIOB_A>;
#[doc = "GTIOCB Pin Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GTIOB_A {
    #[doc = "0: Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match."]
    _00000 = 0,
    #[doc = "1: Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match."]
    _00001 = 1,
    #[doc = "2: Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match."]
    _00010 = 2,
    #[doc = "3: Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    _00011 = 3,
    #[doc = "4: Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match."]
    _00100 = 4,
    #[doc = "5: Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match."]
    _00101 = 5,
    #[doc = "6: Initial output is Low. Low output at cycle end. High output at GTCCRB compare match."]
    _00110 = 6,
    #[doc = "7: Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match."]
    _00111 = 7,
    #[doc = "8: Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match."]
    _01000 = 8,
    #[doc = "9: Initial output is Low. High output at cycle end. Low output at GTCCRB compare match."]
    _01001 = 9,
    #[doc = "10: Initial output is Low. High output at cycle end. High output at GTCCRB compare match."]
    _01010 = 10,
    #[doc = "11: Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match."]
    _01011 = 11,
    #[doc = "12: Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    _01100 = 12,
    #[doc = "13: Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match."]
    _01101 = 13,
    #[doc = "14: Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match."]
    _01110 = 14,
    #[doc = "15: Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    _01111 = 15,
    #[doc = "16: Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match."]
    _10000 = 16,
    #[doc = "17: Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match."]
    _10001 = 17,
    #[doc = "18: Initial output is High. Output retained at cycle end. High output at GTCCRB compare match."]
    _10010 = 18,
    #[doc = "19: Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    _10011 = 19,
    #[doc = "20: Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match."]
    _10100 = 20,
    #[doc = "21: Initial output is High. Low output at cycle end. Low output at GTCCRB compare match."]
    _10101 = 21,
    #[doc = "22: Initial output is High. Low output at cycle end. High output at GTCCRB compare match."]
    _10110 = 22,
    #[doc = "23: Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match."]
    _10111 = 23,
    #[doc = "24: Initial output is High. High output at cycle end. Output retained at GTCCRB compare match."]
    _11000 = 24,
    #[doc = "25: Initial output is High. High output at cycle end. Low output at GTCCRB compare match."]
    _11001 = 25,
    #[doc = "26: Initial output is High. High output at cycle end. High output at GTCCRB compare match."]
    _11010 = 26,
    #[doc = "27: Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match."]
    _11011 = 27,
    #[doc = "28: Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    _11100 = 28,
    #[doc = "29: Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match."]
    _11101 = 29,
    #[doc = "30: Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match."]
    _11110 = 30,
    #[doc = "31: Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    _11111 = 31,
}
impl From<GTIOB_A> for u8 {
    #[inline(always)]
    fn from(variant: GTIOB_A) -> Self {
        variant as _
    }
}
impl GTIOB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTIOB_A {
        match self.bits {
            0 => GTIOB_A::_00000,
            1 => GTIOB_A::_00001,
            2 => GTIOB_A::_00010,
            3 => GTIOB_A::_00011,
            4 => GTIOB_A::_00100,
            5 => GTIOB_A::_00101,
            6 => GTIOB_A::_00110,
            7 => GTIOB_A::_00111,
            8 => GTIOB_A::_01000,
            9 => GTIOB_A::_01001,
            10 => GTIOB_A::_01010,
            11 => GTIOB_A::_01011,
            12 => GTIOB_A::_01100,
            13 => GTIOB_A::_01101,
            14 => GTIOB_A::_01110,
            15 => GTIOB_A::_01111,
            16 => GTIOB_A::_10000,
            17 => GTIOB_A::_10001,
            18 => GTIOB_A::_10010,
            19 => GTIOB_A::_10011,
            20 => GTIOB_A::_10100,
            21 => GTIOB_A::_10101,
            22 => GTIOB_A::_10110,
            23 => GTIOB_A::_10111,
            24 => GTIOB_A::_11000,
            25 => GTIOB_A::_11001,
            26 => GTIOB_A::_11010,
            27 => GTIOB_A::_11011,
            28 => GTIOB_A::_11100,
            29 => GTIOB_A::_11101,
            30 => GTIOB_A::_11110,
            31 => GTIOB_A::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == GTIOB_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == GTIOB_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == GTIOB_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == GTIOB_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == GTIOB_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == GTIOB_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == GTIOB_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == GTIOB_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == GTIOB_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == GTIOB_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == GTIOB_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == GTIOB_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == GTIOB_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == GTIOB_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == GTIOB_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == GTIOB_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == GTIOB_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == GTIOB_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == GTIOB_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == GTIOB_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == GTIOB_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == GTIOB_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == GTIOB_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == GTIOB_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == GTIOB_A::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == GTIOB_A::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == GTIOB_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == GTIOB_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == GTIOB_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == GTIOB_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == GTIOB_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == GTIOB_A::_11111
    }
}
#[doc = "Field `GTIOB` writer - GTIOCB Pin Function Select"]
pub type GTIOB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, GTIOB_A, 5, O>;
impl<'a, const O: u8> GTIOB_W<'a, O> {
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(GTIOB_A::_00000)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(GTIOB_A::_00001)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(GTIOB_A::_00010)
    }
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(GTIOB_A::_00011)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(GTIOB_A::_00100)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(GTIOB_A::_00101)
    }
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(GTIOB_A::_00110)
    }
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(GTIOB_A::_00111)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(GTIOB_A::_01000)
    }
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(GTIOB_A::_01001)
    }
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(GTIOB_A::_01010)
    }
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(GTIOB_A::_01011)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(GTIOB_A::_01100)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(GTIOB_A::_01101)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(GTIOB_A::_01110)
    }
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(GTIOB_A::_01111)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(GTIOB_A::_10000)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(GTIOB_A::_10001)
    }
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(GTIOB_A::_10010)
    }
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(GTIOB_A::_10011)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(GTIOB_A::_10100)
    }
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(GTIOB_A::_10101)
    }
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(GTIOB_A::_10110)
    }
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(GTIOB_A::_10111)
    }
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut W {
        self.variant(GTIOB_A::_11000)
    }
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut W {
        self.variant(GTIOB_A::_11001)
    }
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(GTIOB_A::_11010)
    }
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(GTIOB_A::_11011)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(GTIOB_A::_11100)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(GTIOB_A::_11101)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(GTIOB_A::_11110)
    }
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(GTIOB_A::_11111)
    }
}
#[doc = "Field `OBDFLT` reader - GTIOCB Pin Output Value Setting at the Count Stop"]
pub type OBDFLT_R = crate::BitReader<OBDFLT_A>;
#[doc = "GTIOCB Pin Output Value Setting at the Count Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBDFLT_A {
    #[doc = "0: The GTIOCB pin outputs low when counting is stopped."]
    _0 = 0,
    #[doc = "1: The GTIOCB pin outputs high when counting is stopped."]
    _1 = 1,
}
impl From<OBDFLT_A> for bool {
    #[inline(always)]
    fn from(variant: OBDFLT_A) -> Self {
        variant as u8 != 0
    }
}
impl OBDFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBDFLT_A {
        match self.bits {
            false => OBDFLT_A::_0,
            true => OBDFLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBDFLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBDFLT_A::_1
    }
}
#[doc = "Field `OBDFLT` writer - GTIOCB Pin Output Value Setting at the Count Stop"]
pub type OBDFLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OBDFLT_A, O>;
impl<'a, const O: u8> OBDFLT_W<'a, O> {
    #[doc = "The GTIOCB pin outputs low when counting is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBDFLT_A::_0)
    }
    #[doc = "The GTIOCB pin outputs high when counting is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBDFLT_A::_1)
    }
}
#[doc = "Field `OBHLD` reader - GTIOCB Pin Output Setting at the Start/Stop Count"]
pub type OBHLD_R = crate::BitReader<OBHLD_A>;
#[doc = "GTIOCB Pin Output Setting at the Start/Stop Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBHLD_A {
    #[doc = "0: The GTIOCB pin output level at start/stop of counting depends on the register setting."]
    _0 = 0,
    #[doc = "1: The GTIOCB pin output level is retained at start/stop of counting."]
    _1 = 1,
}
impl From<OBHLD_A> for bool {
    #[inline(always)]
    fn from(variant: OBHLD_A) -> Self {
        variant as u8 != 0
    }
}
impl OBHLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBHLD_A {
        match self.bits {
            false => OBHLD_A::_0,
            true => OBHLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBHLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBHLD_A::_1
    }
}
#[doc = "Field `OBHLD` writer - GTIOCB Pin Output Setting at the Start/Stop Count"]
pub type OBHLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OBHLD_A, O>;
impl<'a, const O: u8> OBHLD_W<'a, O> {
    #[doc = "The GTIOCB pin output level at start/stop of counting depends on the register setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBHLD_A::_0)
    }
    #[doc = "The GTIOCB pin output level is retained at start/stop of counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBHLD_A::_1)
    }
}
#[doc = "Field `OBE` reader - GTIOCB Pin Output Enable"]
pub type OBE_R = crate::BitReader<OBE_A>;
#[doc = "GTIOCB Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBE_A {
    #[doc = "0: Output is disabled"]
    _0 = 0,
    #[doc = "1: Output is enabled"]
    _1 = 1,
}
impl From<OBE_A> for bool {
    #[inline(always)]
    fn from(variant: OBE_A) -> Self {
        variant as u8 != 0
    }
}
impl OBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBE_A {
        match self.bits {
            false => OBE_A::_0,
            true => OBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBE_A::_1
    }
}
#[doc = "Field `OBE` writer - GTIOCB Pin Output Enable"]
pub type OBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, OBE_A, O>;
impl<'a, const O: u8> OBE_W<'a, O> {
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBE_A::_0)
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBE_A::_1)
    }
}
#[doc = "Field `OBDF` reader - GTIOCB Pin Disable Value Setting"]
pub type OBDF_R = crate::FieldReader<u8, OBDF_A>;
#[doc = "GTIOCB Pin Disable Value Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OBDF_A {
    #[doc = "0: Output disable is prohibited."]
    _00 = 0,
    #[doc = "1: GTIOCB pin is set to Hi-Z when output disable is performed."]
    _01 = 1,
    #[doc = "2: GTIOCB pin is set to 0 when output disable is performed."]
    _10 = 2,
    #[doc = "3: GTIOCB pin is set to 1 when output disable is performed."]
    _11 = 3,
}
impl From<OBDF_A> for u8 {
    #[inline(always)]
    fn from(variant: OBDF_A) -> Self {
        variant as _
    }
}
impl OBDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBDF_A {
        match self.bits {
            0 => OBDF_A::_00,
            1 => OBDF_A::_01,
            2 => OBDF_A::_10,
            3 => OBDF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OBDF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OBDF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OBDF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OBDF_A::_11
    }
}
#[doc = "Field `OBDF` writer - GTIOCB Pin Disable Value Setting"]
pub type OBDF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, OBDF_A, 2, O>;
impl<'a, const O: u8> OBDF_W<'a, O> {
    #[doc = "Output disable is prohibited."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OBDF_A::_00)
    }
    #[doc = "GTIOCB pin is set to Hi-Z when output disable is performed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OBDF_A::_01)
    }
    #[doc = "GTIOCB pin is set to 0 when output disable is performed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OBDF_A::_10)
    }
    #[doc = "GTIOCB pin is set to 1 when output disable is performed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OBDF_A::_11)
    }
}
#[doc = "Field `NFBEN` reader - Noise Filter B Enable"]
pub type NFBEN_R = crate::BitReader<NFBEN_A>;
#[doc = "Noise Filter B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFBEN_A {
    #[doc = "0: The noise filter for the GTIOCB pin is disabled."]
    _0 = 0,
    #[doc = "1: The noise filter for the GTIOCB pin is enabled."]
    _1 = 1,
}
impl From<NFBEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFBEN_A {
        match self.bits {
            false => NFBEN_A::_0,
            true => NFBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFBEN_A::_1
    }
}
#[doc = "Field `NFBEN` writer - Noise Filter B Enable"]
pub type NFBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTIOR_SPEC, NFBEN_A, O>;
impl<'a, const O: u8> NFBEN_W<'a, O> {
    #[doc = "The noise filter for the GTIOCB pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFBEN_A::_0)
    }
    #[doc = "The noise filter for the GTIOCB pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFBEN_A::_1)
    }
}
#[doc = "Field `NFCSB` reader - Noise Filter B Sampling Clock Select"]
pub type NFCSB_R = crate::FieldReader<u8, NFCSB_A>;
#[doc = "Noise Filter B Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCSB_A {
    #[doc = "0: PCLK/1"]
    _00 = 0,
    #[doc = "1: PCLK/4"]
    _01 = 1,
    #[doc = "2: PCLK/16"]
    _10 = 2,
    #[doc = "3: PCLK/64"]
    _11 = 3,
}
impl From<NFCSB_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCSB_A) -> Self {
        variant as _
    }
}
impl NFCSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCSB_A {
        match self.bits {
            0 => NFCSB_A::_00,
            1 => NFCSB_A::_01,
            2 => NFCSB_A::_10,
            3 => NFCSB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCSB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCSB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCSB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCSB_A::_11
    }
}
#[doc = "Field `NFCSB` writer - Noise Filter B Sampling Clock Select"]
pub type NFCSB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTIOR_SPEC, u8, NFCSB_A, 2, O>;
impl<'a, const O: u8> NFCSB_W<'a, O> {
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCSB_A::_00)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCSB_A::_01)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCSB_A::_10)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCSB_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - GTIOCA Pin Function Select"]
    #[inline(always)]
    pub fn gtioa(&self) -> GTIOA_R {
        GTIOA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - GTIOCA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn oadflt(&self) -> OADFLT_R {
        OADFLT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn oahld(&self) -> OAHLD_R {
        OAHLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Output Enable"]
    #[inline(always)]
    pub fn oae(&self) -> OAE_R {
        OAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - GTIOCA Pin Disable Value Setting"]
    #[inline(always)]
    pub fn oadf(&self) -> OADF_R {
        OADF_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 13 - Noise Filter A Enable"]
    #[inline(always)]
    pub fn nfaen(&self) -> NFAEN_R {
        NFAEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsa(&self) -> NFCSA_R {
        NFCSA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20 - GTIOCB Pin Function Select"]
    #[inline(always)]
    pub fn gtiob(&self) -> GTIOB_R {
        GTIOB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - GTIOCB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn obdflt(&self) -> OBDFLT_R {
        OBDFLT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn obhld(&self) -> OBHLD_R {
        OBHLD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GTIOCB Pin Output Enable"]
    #[inline(always)]
    pub fn obe(&self) -> OBE_R {
        OBE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - GTIOCB Pin Disable Value Setting"]
    #[inline(always)]
    pub fn obdf(&self) -> OBDF_R {
        OBDF_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 29 - Noise Filter B Enable"]
    #[inline(always)]
    pub fn nfben(&self) -> NFBEN_R {
        NFBEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsb(&self) -> NFCSB_R {
        NFCSB_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GTIOCA Pin Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn gtioa(&mut self) -> GTIOA_W<0> {
        GTIOA_W::new(self)
    }
    #[doc = "Bit 6 - GTIOCA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    #[must_use]
    pub fn oadflt(&mut self) -> OADFLT_W<6> {
        OADFLT_W::new(self)
    }
    #[doc = "Bit 7 - GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    #[must_use]
    pub fn oahld(&mut self) -> OAHLD_W<7> {
        OAHLD_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCA Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oae(&mut self) -> OAE_W<8> {
        OAE_W::new(self)
    }
    #[doc = "Bits 9:10 - GTIOCA Pin Disable Value Setting"]
    #[inline(always)]
    #[must_use]
    pub fn oadf(&mut self) -> OADF_W<9> {
        OADF_W::new(self)
    }
    #[doc = "Bit 13 - Noise Filter A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfaen(&mut self) -> NFAEN_W<13> {
        NFAEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcsa(&mut self) -> NFCSA_W<14> {
        NFCSA_W::new(self)
    }
    #[doc = "Bits 16:20 - GTIOCB Pin Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn gtiob(&mut self) -> GTIOB_W<16> {
        GTIOB_W::new(self)
    }
    #[doc = "Bit 22 - GTIOCB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    #[must_use]
    pub fn obdflt(&mut self) -> OBDFLT_W<22> {
        OBDFLT_W::new(self)
    }
    #[doc = "Bit 23 - GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    #[must_use]
    pub fn obhld(&mut self) -> OBHLD_W<23> {
        OBHLD_W::new(self)
    }
    #[doc = "Bit 24 - GTIOCB Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn obe(&mut self) -> OBE_W<24> {
        OBE_W::new(self)
    }
    #[doc = "Bits 25:26 - GTIOCB Pin Disable Value Setting"]
    #[inline(always)]
    #[must_use]
    pub fn obdf(&mut self) -> OBDF_W<25> {
        OBDF_W::new(self)
    }
    #[doc = "Bit 29 - Noise Filter B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfben(&mut self) -> NFBEN_W<29> {
        NFBEN_W::new(self)
    }
    #[doc = "Bits 30:31 - Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcsb(&mut self) -> NFCSB_W<30> {
        NFCSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer I/O Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtior](index.html) module"]
pub struct GTIOR_SPEC;
impl crate::RegisterSpec for GTIOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtior::R](R) reader structure"]
impl crate::Readable for GTIOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtior::W](W) writer structure"]
impl crate::Writable for GTIOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTIOR to value 0"]
impl crate::Resettable for GTIOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
