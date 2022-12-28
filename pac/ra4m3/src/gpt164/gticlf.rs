#[doc = "Register `GTICLF` reader"]
pub struct R(crate::R<GTICLF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTICLF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTICLF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTICLF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTICLF` writer"]
pub struct W(crate::W<GTICLF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTICLF_SPEC>;
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
impl From<crate::W<GTICLF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTICLF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICLFA` reader - GTIOCnA Output Logical Operation Function Select"]
pub type ICLFA_R = crate::FieldReader<u8, ICLFA_A>;
#[doc = "GTIOCnA Output Logical Operation Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLFA_A {
    #[doc = "0: A (no delay)"]
    _000 = 0,
    #[doc = "1: NOT A (no delay)"]
    _001 = 1,
    #[doc = "2: C (1PCLKD delay)"]
    _010 = 2,
    #[doc = "3: NOT C (1PCLKD delay)"]
    _011 = 3,
    #[doc = "4: A AND C (1PCLKD delay)"]
    _100 = 4,
    #[doc = "5: A OR C (1PCLKD delay)"]
    _101 = 5,
    #[doc = "6: A EXOR C (1PCLKD delay)"]
    _110 = 6,
    #[doc = "7: A NOR C (1PCLKD delay)"]
    _111 = 7,
}
impl From<ICLFA_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLFA_A) -> Self {
        variant as _
    }
}
impl ICLFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLFA_A {
        match self.bits {
            0 => ICLFA_A::_000,
            1 => ICLFA_A::_001,
            2 => ICLFA_A::_010,
            3 => ICLFA_A::_011,
            4 => ICLFA_A::_100,
            5 => ICLFA_A::_101,
            6 => ICLFA_A::_110,
            7 => ICLFA_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ICLFA_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ICLFA_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ICLFA_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ICLFA_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ICLFA_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ICLFA_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ICLFA_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == ICLFA_A::_111
    }
}
#[doc = "Field `ICLFA` writer - GTIOCnA Output Logical Operation Function Select"]
pub type ICLFA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTICLF_SPEC, u8, ICLFA_A, 3, O>;
impl<'a, const O: u8> ICLFA_W<'a, O> {
    #[doc = "A (no delay)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ICLFA_A::_000)
    }
    #[doc = "NOT A (no delay)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ICLFA_A::_001)
    }
    #[doc = "C (1PCLKD delay)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ICLFA_A::_010)
    }
    #[doc = "NOT C (1PCLKD delay)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ICLFA_A::_011)
    }
    #[doc = "A AND C (1PCLKD delay)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ICLFA_A::_100)
    }
    #[doc = "A OR C (1PCLKD delay)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ICLFA_A::_101)
    }
    #[doc = "A EXOR C (1PCLKD delay)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ICLFA_A::_110)
    }
    #[doc = "A NOR C (1PCLKD delay)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(ICLFA_A::_111)
    }
}
#[doc = "Field `ICLFSELC` reader - Inter Channel Signal C Select"]
pub type ICLFSELC_R = crate::FieldReader<u8, ICLFSELC_A>;
#[doc = "Inter Channel Signal C Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLFSELC_A {
    #[doc = "0: GTIOC0A"]
    _0X00 = 0,
    #[doc = "1: GTIOC0B"]
    _0X01 = 1,
    #[doc = "2: GTIOC1A"]
    _0X02 = 2,
    #[doc = "3: GTIOC1B"]
    _0X03 = 3,
    #[doc = "4: GTIOC2A"]
    _0X04 = 4,
    #[doc = "5: GTIOC2B"]
    _0X05 = 5,
    #[doc = "6: GTIOC3A"]
    _0X06 = 6,
    #[doc = "7: GTIOC3B"]
    _0X07 = 7,
    #[doc = "8: GTIOC4A"]
    _0X08 = 8,
    #[doc = "9: GTIOC4B"]
    _0X09 = 9,
    #[doc = "10: GTIOC5A"]
    _0X0A = 10,
    #[doc = "11: GTIOC5B"]
    _0X0B = 11,
    #[doc = "12: GTIOC6A"]
    _0X0C = 12,
    #[doc = "13: GTIOC6B"]
    _0X0D = 13,
    #[doc = "14: GTIOC7A"]
    _0X0E = 14,
    #[doc = "15: GTIOC7B"]
    _0X0F = 15,
    #[doc = "16: GTIOC8A"]
    _0X10 = 16,
    #[doc = "17: GTIOC8B"]
    _0X11 = 17,
    #[doc = "18: GTIOC9A"]
    _0X12 = 18,
    #[doc = "19: GTIOC9B"]
    _0X13 = 19,
}
impl From<ICLFSELC_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLFSELC_A) -> Self {
        variant as _
    }
}
impl ICLFSELC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ICLFSELC_A> {
        match self.bits {
            0 => Some(ICLFSELC_A::_0X00),
            1 => Some(ICLFSELC_A::_0X01),
            2 => Some(ICLFSELC_A::_0X02),
            3 => Some(ICLFSELC_A::_0X03),
            4 => Some(ICLFSELC_A::_0X04),
            5 => Some(ICLFSELC_A::_0X05),
            6 => Some(ICLFSELC_A::_0X06),
            7 => Some(ICLFSELC_A::_0X07),
            8 => Some(ICLFSELC_A::_0X08),
            9 => Some(ICLFSELC_A::_0X09),
            10 => Some(ICLFSELC_A::_0X0A),
            11 => Some(ICLFSELC_A::_0X0B),
            12 => Some(ICLFSELC_A::_0X0C),
            13 => Some(ICLFSELC_A::_0X0D),
            14 => Some(ICLFSELC_A::_0X0E),
            15 => Some(ICLFSELC_A::_0X0F),
            16 => Some(ICLFSELC_A::_0X10),
            17 => Some(ICLFSELC_A::_0X11),
            18 => Some(ICLFSELC_A::_0X12),
            19 => Some(ICLFSELC_A::_0X13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == ICLFSELC_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == ICLFSELC_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == ICLFSELC_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == ICLFSELC_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == ICLFSELC_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == ICLFSELC_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == ICLFSELC_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == ICLFSELC_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == ICLFSELC_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == ICLFSELC_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == ICLFSELC_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == ICLFSELC_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == ICLFSELC_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == ICLFSELC_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == ICLFSELC_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == ICLFSELC_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == ICLFSELC_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == ICLFSELC_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == ICLFSELC_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X13`"]
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == ICLFSELC_A::_0X13
    }
}
#[doc = "Field `ICLFSELC` writer - Inter Channel Signal C Select"]
pub type ICLFSELC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GTICLF_SPEC, u8, ICLFSELC_A, 6, O>;
impl<'a, const O: u8> ICLFSELC_W<'a, O> {
    #[doc = "GTIOC0A"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X00)
    }
    #[doc = "GTIOC0B"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X01)
    }
    #[doc = "GTIOC1A"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X02)
    }
    #[doc = "GTIOC1B"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X03)
    }
    #[doc = "GTIOC2A"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X04)
    }
    #[doc = "GTIOC2B"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X05)
    }
    #[doc = "GTIOC3A"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X06)
    }
    #[doc = "GTIOC3B"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X07)
    }
    #[doc = "GTIOC4A"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X08)
    }
    #[doc = "GTIOC4B"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X09)
    }
    #[doc = "GTIOC5A"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X0A)
    }
    #[doc = "GTIOC5B"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X0B)
    }
    #[doc = "GTIOC6A"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X0C)
    }
    #[doc = "GTIOC6B"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X0D)
    }
    #[doc = "GTIOC7A"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X0E)
    }
    #[doc = "GTIOC7B"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X0F)
    }
    #[doc = "GTIOC8A"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X10)
    }
    #[doc = "GTIOC8B"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X11)
    }
    #[doc = "GTIOC9A"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X12)
    }
    #[doc = "GTIOC9B"]
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut W {
        self.variant(ICLFSELC_A::_0X13)
    }
}
#[doc = "Field `ICLFB` reader - GTIOCnB Output Logical Operation Function Select"]
pub type ICLFB_R = crate::FieldReader<u8, ICLFB_A>;
#[doc = "GTIOCnB Output Logical Operation Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLFB_A {
    #[doc = "0: B (no delay)"]
    _000 = 0,
    #[doc = "1: NOT B (no delay)"]
    _001 = 1,
    #[doc = "2: D (1PCLKD delay)"]
    _010 = 2,
    #[doc = "3: NOT D (1PCLKD delay)"]
    _011 = 3,
    #[doc = "4: B AND D (1PCLKD delay)"]
    _100 = 4,
    #[doc = "5: B OR D (1PCLKDn delay)"]
    _101 = 5,
    #[doc = "6: B EXOR D (1PCLKD delay)"]
    _110 = 6,
    #[doc = "7: B NOR D (1PCLKD delay)"]
    _111 = 7,
}
impl From<ICLFB_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLFB_A) -> Self {
        variant as _
    }
}
impl ICLFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLFB_A {
        match self.bits {
            0 => ICLFB_A::_000,
            1 => ICLFB_A::_001,
            2 => ICLFB_A::_010,
            3 => ICLFB_A::_011,
            4 => ICLFB_A::_100,
            5 => ICLFB_A::_101,
            6 => ICLFB_A::_110,
            7 => ICLFB_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ICLFB_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ICLFB_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ICLFB_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ICLFB_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ICLFB_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ICLFB_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ICLFB_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == ICLFB_A::_111
    }
}
#[doc = "Field `ICLFB` writer - GTIOCnB Output Logical Operation Function Select"]
pub type ICLFB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTICLF_SPEC, u8, ICLFB_A, 3, O>;
impl<'a, const O: u8> ICLFB_W<'a, O> {
    #[doc = "B (no delay)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ICLFB_A::_000)
    }
    #[doc = "NOT B (no delay)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ICLFB_A::_001)
    }
    #[doc = "D (1PCLKD delay)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ICLFB_A::_010)
    }
    #[doc = "NOT D (1PCLKD delay)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ICLFB_A::_011)
    }
    #[doc = "B AND D (1PCLKD delay)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ICLFB_A::_100)
    }
    #[doc = "B OR D (1PCLKDn delay)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ICLFB_A::_101)
    }
    #[doc = "B EXOR D (1PCLKD delay)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ICLFB_A::_110)
    }
    #[doc = "B NOR D (1PCLKD delay)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(ICLFB_A::_111)
    }
}
#[doc = "Field `ICLFSELD` reader - Inter Channel Signal D Select"]
pub type ICLFSELD_R = crate::FieldReader<u8, ICLFSELD_A>;
#[doc = "Inter Channel Signal D Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLFSELD_A {
    #[doc = "0: GTIOC0A"]
    _0X00 = 0,
    #[doc = "1: GTIOC0B"]
    _0X01 = 1,
    #[doc = "2: GTIOC1A"]
    _0X02 = 2,
    #[doc = "3: GTIOC1B"]
    _0X03 = 3,
    #[doc = "4: GTIOC2A"]
    _0X04 = 4,
    #[doc = "5: GTIOC2B"]
    _0X05 = 5,
    #[doc = "6: GTIOC3A"]
    _0X06 = 6,
    #[doc = "7: GTIOC3B"]
    _0X07 = 7,
    #[doc = "8: GTIOC4A"]
    _0X08 = 8,
    #[doc = "9: GTIOC4B"]
    _0X09 = 9,
    #[doc = "10: GTIOC5A"]
    _0X0A = 10,
    #[doc = "11: GTIOC5B"]
    _0X0B = 11,
    #[doc = "12: GTIOC6A"]
    _0X0C = 12,
    #[doc = "13: GTIOC6B"]
    _0X0D = 13,
    #[doc = "14: GTIOC7A"]
    _0X0E = 14,
    #[doc = "15: GTIOC7B"]
    _0X0F = 15,
    #[doc = "16: GTIOC8A"]
    _0X10 = 16,
    #[doc = "17: GTIOC8B"]
    _0X11 = 17,
    #[doc = "18: GTIOC9A"]
    _0X12 = 18,
    #[doc = "19: GTIOC9B"]
    _0X13 = 19,
}
impl From<ICLFSELD_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLFSELD_A) -> Self {
        variant as _
    }
}
impl ICLFSELD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ICLFSELD_A> {
        match self.bits {
            0 => Some(ICLFSELD_A::_0X00),
            1 => Some(ICLFSELD_A::_0X01),
            2 => Some(ICLFSELD_A::_0X02),
            3 => Some(ICLFSELD_A::_0X03),
            4 => Some(ICLFSELD_A::_0X04),
            5 => Some(ICLFSELD_A::_0X05),
            6 => Some(ICLFSELD_A::_0X06),
            7 => Some(ICLFSELD_A::_0X07),
            8 => Some(ICLFSELD_A::_0X08),
            9 => Some(ICLFSELD_A::_0X09),
            10 => Some(ICLFSELD_A::_0X0A),
            11 => Some(ICLFSELD_A::_0X0B),
            12 => Some(ICLFSELD_A::_0X0C),
            13 => Some(ICLFSELD_A::_0X0D),
            14 => Some(ICLFSELD_A::_0X0E),
            15 => Some(ICLFSELD_A::_0X0F),
            16 => Some(ICLFSELD_A::_0X10),
            17 => Some(ICLFSELD_A::_0X11),
            18 => Some(ICLFSELD_A::_0X12),
            19 => Some(ICLFSELD_A::_0X13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == ICLFSELD_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == ICLFSELD_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == ICLFSELD_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == ICLFSELD_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == ICLFSELD_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == ICLFSELD_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == ICLFSELD_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == ICLFSELD_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == ICLFSELD_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == ICLFSELD_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == ICLFSELD_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == ICLFSELD_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == ICLFSELD_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == ICLFSELD_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == ICLFSELD_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == ICLFSELD_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == ICLFSELD_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == ICLFSELD_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == ICLFSELD_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X13`"]
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == ICLFSELD_A::_0X13
    }
}
#[doc = "Field `ICLFSELD` writer - Inter Channel Signal D Select"]
pub type ICLFSELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GTICLF_SPEC, u8, ICLFSELD_A, 6, O>;
impl<'a, const O: u8> ICLFSELD_W<'a, O> {
    #[doc = "GTIOC0A"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X00)
    }
    #[doc = "GTIOC0B"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X01)
    }
    #[doc = "GTIOC1A"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X02)
    }
    #[doc = "GTIOC1B"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X03)
    }
    #[doc = "GTIOC2A"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X04)
    }
    #[doc = "GTIOC2B"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X05)
    }
    #[doc = "GTIOC3A"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X06)
    }
    #[doc = "GTIOC3B"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X07)
    }
    #[doc = "GTIOC4A"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X08)
    }
    #[doc = "GTIOC4B"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X09)
    }
    #[doc = "GTIOC5A"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X0A)
    }
    #[doc = "GTIOC5B"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X0B)
    }
    #[doc = "GTIOC6A"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X0C)
    }
    #[doc = "GTIOC6B"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X0D)
    }
    #[doc = "GTIOC7A"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X0E)
    }
    #[doc = "GTIOC7B"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X0F)
    }
    #[doc = "GTIOC8A"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X10)
    }
    #[doc = "GTIOC8B"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X11)
    }
    #[doc = "GTIOC9A"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X12)
    }
    #[doc = "GTIOC9B"]
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut W {
        self.variant(ICLFSELD_A::_0X13)
    }
}
impl R {
    #[doc = "Bits 0:2 - GTIOCnA Output Logical Operation Function Select"]
    #[inline(always)]
    pub fn iclfa(&self) -> ICLFA_R {
        ICLFA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:9 - Inter Channel Signal C Select"]
    #[inline(always)]
    pub fn iclfselc(&self) -> ICLFSELC_R {
        ICLFSELC_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - GTIOCnB Output Logical Operation Function Select"]
    #[inline(always)]
    pub fn iclfb(&self) -> ICLFB_R {
        ICLFB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:25 - Inter Channel Signal D Select"]
    #[inline(always)]
    pub fn iclfseld(&self) -> ICLFSELD_R {
        ICLFSELD_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GTIOCnA Output Logical Operation Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfa(&mut self) -> ICLFA_W<0> {
        ICLFA_W::new(self)
    }
    #[doc = "Bits 4:9 - Inter Channel Signal C Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfselc(&mut self) -> ICLFSELC_W<4> {
        ICLFSELC_W::new(self)
    }
    #[doc = "Bits 16:18 - GTIOCnB Output Logical Operation Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfb(&mut self) -> ICLFB_W<16> {
        ICLFB_W::new(self)
    }
    #[doc = "Bits 20:25 - Inter Channel Signal D Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfseld(&mut self) -> ICLFSELD_W<20> {
        ICLFSELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Inter Channel Logical Operation Function Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gticlf](index.html) module"]
pub struct GTICLF_SPEC;
impl crate::RegisterSpec for GTICLF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gticlf::R](R) reader structure"]
impl crate::Readable for GTICLF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gticlf::W](W) writer structure"]
impl crate::Writable for GTICLF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTICLF to value 0"]
impl crate::Resettable for GTICLF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
