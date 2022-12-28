#[doc = "Register `PGAC%s` reader"]
pub struct R(crate::R<PGAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGAC%s` writer"]
pub struct W(crate::W<PGAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGAC_SPEC>;
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
impl From<crate::W<PGAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGAGC` reader - Gain selection of a programmable gain instrumentation amplifier ( Gset1, Gset2, Gtotal )"]
pub type PGAGC_R = crate::FieldReader<u8, PGAGC_A>;
#[doc = "Gain selection of a programmable gain instrumentation amplifier ( Gset1, Gset2, Gtotal )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGAGC_A {
    #[doc = "0: (1, 1, 1)"]
    _00000 = 0,
    #[doc = "4: (2, 1, 2)"]
    _00100 = 4,
    #[doc = "8: (3, 1, 3)"]
    _01000 = 8,
    #[doc = "12: (4, 1, 4)"]
    _01100 = 12,
    #[doc = "16: (8, 1, 8)"]
    _10000 = 16,
    #[doc = "1: (1, 2, 2)"]
    _00001 = 1,
    #[doc = "5: (2, 2, 4)"]
    _00101 = 5,
    #[doc = "9: (3, 2, 6)"]
    _01001 = 9,
    #[doc = "13: (4, 2, 8)"]
    _01101 = 13,
    #[doc = "17: (8, 2, 16)"]
    _10001 = 17,
    #[doc = "2: (1, 4, 4)"]
    _00010 = 2,
    #[doc = "6: (2, 4, 8)"]
    _00110 = 6,
    #[doc = "10: (3, 4, 12)"]
    _01010 = 10,
    #[doc = "14: (4, 4, 16)"]
    _01110 = 14,
    #[doc = "18: (8, 4, 32)"]
    _10010 = 18,
    #[doc = "3: (1, 8, 8)"]
    _00011 = 3,
    #[doc = "7: (2, 8, 16)"]
    _00111 = 7,
    #[doc = "11: (3, 8, 24)"]
    _01011 = 11,
    #[doc = "15: (4, 8, 32)."]
    _01111 = 15,
}
impl From<PGAGC_A> for u8 {
    #[inline(always)]
    fn from(variant: PGAGC_A) -> Self {
        variant as _
    }
}
impl PGAGC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PGAGC_A> {
        match self.bits {
            0 => Some(PGAGC_A::_00000),
            4 => Some(PGAGC_A::_00100),
            8 => Some(PGAGC_A::_01000),
            12 => Some(PGAGC_A::_01100),
            16 => Some(PGAGC_A::_10000),
            1 => Some(PGAGC_A::_00001),
            5 => Some(PGAGC_A::_00101),
            9 => Some(PGAGC_A::_01001),
            13 => Some(PGAGC_A::_01101),
            17 => Some(PGAGC_A::_10001),
            2 => Some(PGAGC_A::_00010),
            6 => Some(PGAGC_A::_00110),
            10 => Some(PGAGC_A::_01010),
            14 => Some(PGAGC_A::_01110),
            18 => Some(PGAGC_A::_10010),
            3 => Some(PGAGC_A::_00011),
            7 => Some(PGAGC_A::_00111),
            11 => Some(PGAGC_A::_01011),
            15 => Some(PGAGC_A::_01111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == PGAGC_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == PGAGC_A::_00100
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == PGAGC_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == PGAGC_A::_01100
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == PGAGC_A::_10000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == PGAGC_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == PGAGC_A::_00101
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == PGAGC_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == PGAGC_A::_01101
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == PGAGC_A::_10001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == PGAGC_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == PGAGC_A::_00110
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == PGAGC_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == PGAGC_A::_01110
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == PGAGC_A::_10010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == PGAGC_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == PGAGC_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == PGAGC_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == PGAGC_A::_01111
    }
}
#[doc = "Field `PGAGC` writer - Gain selection of a programmable gain instrumentation amplifier ( Gset1, Gset2, Gtotal )"]
pub type PGAGC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGAC_SPEC, u8, PGAGC_A, 5, O>;
impl<'a, const O: u8> PGAGC_W<'a, O> {
    #[doc = "(1, 1, 1)"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(PGAGC_A::_00000)
    }
    #[doc = "(2, 1, 2)"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(PGAGC_A::_00100)
    }
    #[doc = "(3, 1, 3)"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(PGAGC_A::_01000)
    }
    #[doc = "(4, 1, 4)"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(PGAGC_A::_01100)
    }
    #[doc = "(8, 1, 8)"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(PGAGC_A::_10000)
    }
    #[doc = "(1, 2, 2)"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(PGAGC_A::_00001)
    }
    #[doc = "(2, 2, 4)"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(PGAGC_A::_00101)
    }
    #[doc = "(3, 2, 6)"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(PGAGC_A::_01001)
    }
    #[doc = "(4, 2, 8)"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(PGAGC_A::_01101)
    }
    #[doc = "(8, 2, 16)"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(PGAGC_A::_10001)
    }
    #[doc = "(1, 4, 4)"]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(PGAGC_A::_00010)
    }
    #[doc = "(2, 4, 8)"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(PGAGC_A::_00110)
    }
    #[doc = "(3, 4, 12)"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(PGAGC_A::_01010)
    }
    #[doc = "(4, 4, 16)"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(PGAGC_A::_01110)
    }
    #[doc = "(8, 4, 32)"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(PGAGC_A::_10010)
    }
    #[doc = "(1, 8, 8)"]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(PGAGC_A::_00011)
    }
    #[doc = "(2, 8, 16)"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(PGAGC_A::_00111)
    }
    #[doc = "(3, 8, 24)"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(PGAGC_A::_01011)
    }
    #[doc = "(4, 8, 32)."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(PGAGC_A::_01111)
    }
}
#[doc = "Field `PGAOSR` reader - Oversampling ratio select"]
pub type PGAOSR_R = crate::FieldReader<u8, PGAOSR_A>;
#[doc = "Oversampling ratio select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGAOSR_A {
    #[doc = "0: 64"]
    _000 = 0,
    #[doc = "1: 128"]
    _001 = 1,
    #[doc = "2: 256"]
    _010 = 2,
    #[doc = "3: 512"]
    _011 = 3,
    #[doc = "4: 1024"]
    _100 = 4,
    #[doc = "5: 2048"]
    _101 = 5,
}
impl From<PGAOSR_A> for u8 {
    #[inline(always)]
    fn from(variant: PGAOSR_A) -> Self {
        variant as _
    }
}
impl PGAOSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PGAOSR_A> {
        match self.bits {
            0 => Some(PGAOSR_A::_000),
            1 => Some(PGAOSR_A::_001),
            2 => Some(PGAOSR_A::_010),
            3 => Some(PGAOSR_A::_011),
            4 => Some(PGAOSR_A::_100),
            5 => Some(PGAOSR_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PGAOSR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PGAOSR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PGAOSR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PGAOSR_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PGAOSR_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PGAOSR_A::_101
    }
}
#[doc = "Field `PGAOSR` writer - Oversampling ratio select"]
pub type PGAOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGAC_SPEC, u8, PGAOSR_A, 3, O>;
impl<'a, const O: u8> PGAOSR_W<'a, O> {
    #[doc = "64"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PGAOSR_A::_000)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PGAOSR_A::_001)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PGAOSR_A::_010)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PGAOSR_A::_011)
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PGAOSR_A::_100)
    }
    #[doc = "2048"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PGAOSR_A::_101)
    }
}
#[doc = "Field `PGAOFS` reader - Offset voltage select"]
pub type PGAOFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGAOFS` writer - Offset voltage select"]
pub type PGAOFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGAC_SPEC, u8, u8, 5, O>;
#[doc = "Field `PGAPOL` reader - Polarity select"]
pub type PGAPOL_R = crate::BitReader<PGAPOL_A>;
#[doc = "Polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAPOL_A {
    #[doc = "0: Positive-side single-end input"]
    _0 = 0,
    #[doc = "1: Negative-side single-end input"]
    _1 = 1,
}
impl From<PGAPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PGAPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAPOL_A {
        match self.bits {
            false => PGAPOL_A::_0,
            true => PGAPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGAPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGAPOL_A::_1
    }
}
#[doc = "Field `PGAPOL` writer - Polarity select"]
pub type PGAPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PGAC_SPEC, PGAPOL_A, O>;
impl<'a, const O: u8> PGAPOL_W<'a, O> {
    #[doc = "Positive-side single-end input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGAPOL_A::_0)
    }
    #[doc = "Negative-side single-end input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGAPOL_A::_1)
    }
}
#[doc = "Field `PGASEL` reader - Analog Channel Input Mode Select"]
pub type PGASEL_R = crate::BitReader<PGASEL_A>;
#[doc = "Analog Channel Input Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGASEL_A {
    #[doc = "0: Differential input mode"]
    _0 = 0,
    #[doc = "1: Single-end input mode"]
    _1 = 1,
}
impl From<PGASEL_A> for bool {
    #[inline(always)]
    fn from(variant: PGASEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PGASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGASEL_A {
        match self.bits {
            false => PGASEL_A::_0,
            true => PGASEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGASEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGASEL_A::_1
    }
}
#[doc = "Field `PGASEL` writer - Analog Channel Input Mode Select"]
pub type PGASEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PGAC_SPEC, PGASEL_A, O>;
impl<'a, const O: u8> PGASEL_W<'a, O> {
    #[doc = "Differential input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGASEL_A::_0)
    }
    #[doc = "Single-end input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGASEL_A::_1)
    }
}
#[doc = "Field `PGACTM` reader - Coefficient (m) selection of the A/D conversion count (N) in AUTOSCAN"]
pub type PGACTM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGACTM` writer - Coefficient (m) selection of the A/D conversion count (N) in AUTOSCAN"]
pub type PGACTM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGAC_SPEC, u8, u8, 5, O>;
#[doc = "Field `PGACTN` reader - Coefficient (n) selection of the A/D conversion count (N) in AUTOSCAN"]
pub type PGACTN_R = crate::FieldReader<u8, PGACTN_A>;
#[doc = "Coefficient (n) selection of the A/D conversion count (N) in AUTOSCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGACTN_A {
    #[doc = "0: 0"]
    _000 = 0,
    #[doc = "1: 1"]
    _001 = 1,
    #[doc = "2: 2"]
    _010 = 2,
    #[doc = "3: 3"]
    _011 = 3,
    #[doc = "4: 4"]
    _100 = 4,
    #[doc = "5: 5"]
    _101 = 5,
    #[doc = "6: 6"]
    _110 = 6,
    #[doc = "7: 7"]
    _111 = 7,
}
impl From<PGACTN_A> for u8 {
    #[inline(always)]
    fn from(variant: PGACTN_A) -> Self {
        variant as _
    }
}
impl PGACTN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGACTN_A {
        match self.bits {
            0 => PGACTN_A::_000,
            1 => PGACTN_A::_001,
            2 => PGACTN_A::_010,
            3 => PGACTN_A::_011,
            4 => PGACTN_A::_100,
            5 => PGACTN_A::_101,
            6 => PGACTN_A::_110,
            7 => PGACTN_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PGACTN_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PGACTN_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PGACTN_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PGACTN_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PGACTN_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PGACTN_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PGACTN_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PGACTN_A::_111
    }
}
#[doc = "Field `PGACTN` writer - Coefficient (n) selection of the A/D conversion count (N) in AUTOSCAN"]
pub type PGACTN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PGAC_SPEC, u8, PGACTN_A, 3, O>;
impl<'a, const O: u8> PGACTN_W<'a, O> {
    #[doc = "0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PGACTN_A::_000)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PGACTN_A::_001)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PGACTN_A::_010)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PGACTN_A::_011)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PGACTN_A::_100)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PGACTN_A::_101)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PGACTN_A::_110)
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PGACTN_A::_111)
    }
}
#[doc = "Field `PGAAVN` reader - Selection of the number of data to be averaged"]
pub type PGAAVN_R = crate::FieldReader<u8, PGAAVN_A>;
#[doc = "Selection of the number of data to be averaged\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGAAVN_A {
    #[doc = "0: 8"]
    _00 = 0,
    #[doc = "1: 16"]
    _01 = 1,
    #[doc = "2: 32"]
    _10 = 2,
    #[doc = "3: 64"]
    _11 = 3,
}
impl From<PGAAVN_A> for u8 {
    #[inline(always)]
    fn from(variant: PGAAVN_A) -> Self {
        variant as _
    }
}
impl PGAAVN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAAVN_A {
        match self.bits {
            0 => PGAAVN_A::_00,
            1 => PGAAVN_A::_01,
            2 => PGAAVN_A::_10,
            3 => PGAAVN_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PGAAVN_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PGAAVN_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PGAAVN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PGAAVN_A::_11
    }
}
#[doc = "Field `PGAAVN` writer - Selection of the number of data to be averaged"]
pub type PGAAVN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PGAC_SPEC, u8, PGAAVN_A, 2, O>;
impl<'a, const O: u8> PGAAVN_W<'a, O> {
    #[doc = "8"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PGAAVN_A::_00)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PGAAVN_A::_01)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PGAAVN_A::_10)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PGAAVN_A::_11)
    }
}
#[doc = "Field `PGAAVE` reader - Selection of averaging processing"]
pub type PGAAVE_R = crate::FieldReader<u8, PGAAVE_A>;
#[doc = "Selection of averaging processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGAAVE_A {
    #[doc = "0: Do not average the A/D conversion results"]
    _00 = 0,
    #[doc = "1: Do not average the A/D conversion results"]
    _01 = 1,
    #[doc = "2: Average the A/D conversion results and generates SDADC_ADI each time an A/D conversion occurs"]
    _10 = 2,
    #[doc = "3: Perform averaging, and generate SDADC_ADI at each time of average value output (A/D conversion is performed N times)."]
    _11 = 3,
}
impl From<PGAAVE_A> for u8 {
    #[inline(always)]
    fn from(variant: PGAAVE_A) -> Self {
        variant as _
    }
}
impl PGAAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAAVE_A {
        match self.bits {
            0 => PGAAVE_A::_00,
            1 => PGAAVE_A::_01,
            2 => PGAAVE_A::_10,
            3 => PGAAVE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PGAAVE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PGAAVE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PGAAVE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PGAAVE_A::_11
    }
}
#[doc = "Field `PGAAVE` writer - Selection of averaging processing"]
pub type PGAAVE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PGAC_SPEC, u8, PGAAVE_A, 2, O>;
impl<'a, const O: u8> PGAAVE_W<'a, O> {
    #[doc = "Do not average the A/D conversion results"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PGAAVE_A::_00)
    }
    #[doc = "Do not average the A/D conversion results"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PGAAVE_A::_01)
    }
    #[doc = "Average the A/D conversion results and generates SDADC_ADI each time an A/D conversion occurs"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PGAAVE_A::_10)
    }
    #[doc = "Perform averaging, and generate SDADC_ADI at each time of average value output (A/D conversion is performed N times)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PGAAVE_A::_11)
    }
}
#[doc = "Field `PGAREV` reader - Single-End Input A/D Converted Data Inversion Select"]
pub type PGAREV_R = crate::BitReader<PGAREV_A>;
#[doc = "Single-End Input A/D Converted Data Inversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAREV_A {
    #[doc = "0: Do not invert the conversion result data"]
    _0 = 0,
    #[doc = "1: Invert the conversion result data"]
    _1 = 1,
}
impl From<PGAREV_A> for bool {
    #[inline(always)]
    fn from(variant: PGAREV_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAREV_A {
        match self.bits {
            false => PGAREV_A::_0,
            true => PGAREV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGAREV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGAREV_A::_1
    }
}
#[doc = "Field `PGAREV` writer - Single-End Input A/D Converted Data Inversion Select"]
pub type PGAREV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PGAC_SPEC, PGAREV_A, O>;
impl<'a, const O: u8> PGAREV_W<'a, O> {
    #[doc = "Do not invert the conversion result data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGAREV_A::_0)
    }
    #[doc = "Invert the conversion result data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGAREV_A::_1)
    }
}
#[doc = "Field `PGACVE` reader - Calibration enable"]
pub type PGACVE_R = crate::BitReader<PGACVE_A>;
#[doc = "Calibration enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGACVE_A {
    #[doc = "0: Do not calculate the calibration correction factor"]
    _0 = 0,
    #[doc = "1: Calculate the calibration correction factor"]
    _1 = 1,
}
impl From<PGACVE_A> for bool {
    #[inline(always)]
    fn from(variant: PGACVE_A) -> Self {
        variant as u8 != 0
    }
}
impl PGACVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGACVE_A {
        match self.bits {
            false => PGACVE_A::_0,
            true => PGACVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGACVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGACVE_A::_1
    }
}
#[doc = "Field `PGACVE` writer - Calibration enable"]
pub type PGACVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PGAC_SPEC, PGACVE_A, O>;
impl<'a, const O: u8> PGACVE_W<'a, O> {
    #[doc = "Do not calculate the calibration correction factor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGACVE_A::_0)
    }
    #[doc = "Calculate the calibration correction factor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGACVE_A::_1)
    }
}
#[doc = "Field `PGAASN` reader - Selection of the mode for specifying the number of A/D conversions in ADSCAN"]
pub type PGAASN_R = crate::BitReader<PGAASN_A>;
#[doc = "Selection of the mode for specifying the number of A/D conversions in ADSCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAASN_A {
    #[doc = "0: Specify 1 to 8,032 times by using the value set in the PGACTN\\[2:0\\]
and PGACTM\\[4:0\\]
bits"]
    _0 = 0,
    #[doc = "1: Specify 1 to 255 times linearly by using the value set in the PGACTN\\[2:0\\]
and PGACTM\\[4:0\\]
bits"]
    _1 = 1,
}
impl From<PGAASN_A> for bool {
    #[inline(always)]
    fn from(variant: PGAASN_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAASN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAASN_A {
        match self.bits {
            false => PGAASN_A::_0,
            true => PGAASN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGAASN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGAASN_A::_1
    }
}
#[doc = "Field `PGAASN` writer - Selection of the mode for specifying the number of A/D conversions in ADSCAN"]
pub type PGAASN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PGAC_SPEC, PGAASN_A, O>;
impl<'a, const O: u8> PGAASN_W<'a, O> {
    #[doc = "Specify 1 to 8,032 times by using the value set in the PGACTN\\[2:0\\]
and PGACTM\\[4:0\\]
bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGAASN_A::_0)
    }
    #[doc = "Specify 1 to 255 times linearly by using the value set in the PGACTN\\[2:0\\]
and PGACTM\\[4:0\\]
bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGAASN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Gain selection of a programmable gain instrumentation amplifier ( Gset1, Gset2, Gtotal )"]
    #[inline(always)]
    pub fn pgagc(&self) -> PGAGC_R {
        PGAGC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Oversampling ratio select"]
    #[inline(always)]
    pub fn pgaosr(&self) -> PGAOSR_R {
        PGAOSR_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Offset voltage select"]
    #[inline(always)]
    pub fn pgaofs(&self) -> PGAOFS_R {
        PGAOFS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Polarity select"]
    #[inline(always)]
    pub fn pgapol(&self) -> PGAPOL_R {
        PGAPOL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Channel Input Mode Select"]
    #[inline(always)]
    pub fn pgasel(&self) -> PGASEL_R {
        PGASEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Coefficient (m) selection of the A/D conversion count (N) in AUTOSCAN"]
    #[inline(always)]
    pub fn pgactm(&self) -> PGACTM_R {
        PGACTM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Coefficient (n) selection of the A/D conversion count (N) in AUTOSCAN"]
    #[inline(always)]
    pub fn pgactn(&self) -> PGACTN_R {
        PGACTN_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Selection of the number of data to be averaged"]
    #[inline(always)]
    pub fn pgaavn(&self) -> PGAAVN_R {
        PGAAVN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Selection of averaging processing"]
    #[inline(always)]
    pub fn pgaave(&self) -> PGAAVE_R {
        PGAAVE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Single-End Input A/D Converted Data Inversion Select"]
    #[inline(always)]
    pub fn pgarev(&self) -> PGAREV_R {
        PGAREV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Calibration enable"]
    #[inline(always)]
    pub fn pgacve(&self) -> PGACVE_R {
        PGACVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Selection of the mode for specifying the number of A/D conversions in ADSCAN"]
    #[inline(always)]
    pub fn pgaasn(&self) -> PGAASN_R {
        PGAASN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Gain selection of a programmable gain instrumentation amplifier ( Gset1, Gset2, Gtotal )"]
    #[inline(always)]
    #[must_use]
    pub fn pgagc(&mut self) -> PGAGC_W<0> {
        PGAGC_W::new(self)
    }
    #[doc = "Bits 5:7 - Oversampling ratio select"]
    #[inline(always)]
    #[must_use]
    pub fn pgaosr(&mut self) -> PGAOSR_W<5> {
        PGAOSR_W::new(self)
    }
    #[doc = "Bits 8:12 - Offset voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn pgaofs(&mut self) -> PGAOFS_W<8> {
        PGAOFS_W::new(self)
    }
    #[doc = "Bit 14 - Polarity select"]
    #[inline(always)]
    #[must_use]
    pub fn pgapol(&mut self) -> PGAPOL_W<14> {
        PGAPOL_W::new(self)
    }
    #[doc = "Bit 15 - Analog Channel Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn pgasel(&mut self) -> PGASEL_W<15> {
        PGASEL_W::new(self)
    }
    #[doc = "Bits 16:20 - Coefficient (m) selection of the A/D conversion count (N) in AUTOSCAN"]
    #[inline(always)]
    #[must_use]
    pub fn pgactm(&mut self) -> PGACTM_W<16> {
        PGACTM_W::new(self)
    }
    #[doc = "Bits 21:23 - Coefficient (n) selection of the A/D conversion count (N) in AUTOSCAN"]
    #[inline(always)]
    #[must_use]
    pub fn pgactn(&mut self) -> PGACTN_W<21> {
        PGACTN_W::new(self)
    }
    #[doc = "Bits 24:25 - Selection of the number of data to be averaged"]
    #[inline(always)]
    #[must_use]
    pub fn pgaavn(&mut self) -> PGAAVN_W<24> {
        PGAAVN_W::new(self)
    }
    #[doc = "Bits 26:27 - Selection of averaging processing"]
    #[inline(always)]
    #[must_use]
    pub fn pgaave(&mut self) -> PGAAVE_W<26> {
        PGAAVE_W::new(self)
    }
    #[doc = "Bit 28 - Single-End Input A/D Converted Data Inversion Select"]
    #[inline(always)]
    #[must_use]
    pub fn pgarev(&mut self) -> PGAREV_W<28> {
        PGAREV_W::new(self)
    }
    #[doc = "Bit 30 - Calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgacve(&mut self) -> PGACVE_W<30> {
        PGACVE_W::new(self)
    }
    #[doc = "Bit 31 - Selection of the mode for specifying the number of A/D conversions in ADSCAN"]
    #[inline(always)]
    #[must_use]
    pub fn pgaasn(&mut self) -> PGAASN_W<31> {
        PGAASN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Multiplexer %s Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgac](index.html) module"]
pub struct PGAC_SPEC;
impl crate::RegisterSpec for PGAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgac::R](R) reader structure"]
impl crate::Readable for PGAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgac::W](W) writer structure"]
impl crate::Writable for PGAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGAC%s to value 0x0001_0040"]
impl crate::Resettable for PGAC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0040;
}
