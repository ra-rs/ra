#[doc = "Register `CTSUSR` reader"]
pub struct R(crate::R<CTSUSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUSR` writer"]
pub struct W(crate::W<CTSUSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUSR_SPEC>;
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
impl From<crate::W<CTSUSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFC` reader - CTSU Multi-Clock Counter"]
pub type MFC_R = crate::FieldReader<u8, MFC_A>;
#[doc = "CTSU Multi-Clock Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MFC_A {
    #[doc = "0: Multi-clock 0"]
    _00 = 0,
    #[doc = "1: Multi-clock 1"]
    _01 = 1,
    #[doc = "2: Multi-clock 2"]
    _10 = 2,
    #[doc = "3: Multi-clock 3"]
    _11 = 3,
}
impl From<MFC_A> for u8 {
    #[inline(always)]
    fn from(variant: MFC_A) -> Self {
        variant as _
    }
}
impl MFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MFC_A {
        match self.bits {
            0 => MFC_A::_00,
            1 => MFC_A::_01,
            2 => MFC_A::_10,
            3 => MFC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MFC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MFC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MFC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MFC_A::_11
    }
}
#[doc = "Field `MFC` writer - CTSU Multi-Clock Counter"]
pub type MFC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTSUSR_SPEC, u8, MFC_A, 2, O>;
impl<'a, const O: u8> MFC_W<'a, O> {
    #[doc = "Multi-clock 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MFC_A::_00)
    }
    #[doc = "Multi-clock 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MFC_A::_01)
    }
    #[doc = "Multi-clock 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MFC_A::_10)
    }
    #[doc = "Multi-clock 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MFC_A::_11)
    }
}
#[doc = "Field `ICOMPRST` writer - CTSU CTSUICOMP1 Flag Reset"]
pub type ICOMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUSR_SPEC, bool, O>;
#[doc = "Field `ICOMP1` reader - CTSU Sense Current Error Monitor"]
pub type ICOMP1_R = crate::BitReader<ICOMP1_A>;
#[doc = "CTSU Sense Current Error Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICOMP1_A {
    #[doc = "0: Normal sensor current"]
    _0 = 0,
    #[doc = "1: Abnormal sensor current"]
    _1 = 1,
}
impl From<ICOMP1_A> for bool {
    #[inline(always)]
    fn from(variant: ICOMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl ICOMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICOMP1_A {
        match self.bits {
            false => ICOMP1_A::_0,
            true => ICOMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICOMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICOMP1_A::_1
    }
}
#[doc = "Field `ICOMP0` reader - TSCAP Voltage Error Monitor"]
pub type ICOMP0_R = crate::BitReader<ICOMP0_A>;
#[doc = "TSCAP Voltage Error Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICOMP0_A {
    #[doc = "0: Normal TSCAP voltage"]
    _0 = 0,
    #[doc = "1: Abnormal TSCAP voltage"]
    _1 = 1,
}
impl From<ICOMP0_A> for bool {
    #[inline(always)]
    fn from(variant: ICOMP0_A) -> Self {
        variant as u8 != 0
    }
}
impl ICOMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICOMP0_A {
        match self.bits {
            false => ICOMP0_A::_0,
            true => ICOMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICOMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICOMP0_A::_1
    }
}
#[doc = "Field `STC` reader - CTSU Measurement Status Counter"]
pub type STC_R = crate::FieldReader<u8, STC_A>;
#[doc = "CTSU Measurement Status Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STC_A {
    #[doc = "0: Status 0"]
    _000 = 0,
    #[doc = "1: Status 1"]
    _001 = 1,
    #[doc = "2: Status 2"]
    _010 = 2,
    #[doc = "3: Status 3"]
    _011 = 3,
    #[doc = "4: Status 4"]
    _100 = 4,
    #[doc = "5: Status 5"]
    _101 = 5,
}
impl From<STC_A> for u8 {
    #[inline(always)]
    fn from(variant: STC_A) -> Self {
        variant as _
    }
}
impl STC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STC_A> {
        match self.bits {
            0 => Some(STC_A::_000),
            1 => Some(STC_A::_001),
            2 => Some(STC_A::_010),
            3 => Some(STC_A::_011),
            4 => Some(STC_A::_100),
            5 => Some(STC_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == STC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == STC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == STC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == STC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == STC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == STC_A::_101
    }
}
#[doc = "Field `DTSR` reader - CTSU Data Transfer Status Flag"]
pub type DTSR_R = crate::BitReader<DTSR_A>;
#[doc = "CTSU Data Transfer Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSR_A {
    #[doc = "0: Read"]
    _0 = 0,
    #[doc = "1: Not read"]
    _1 = 1,
}
impl From<DTSR_A> for bool {
    #[inline(always)]
    fn from(variant: DTSR_A) -> Self {
        variant as u8 != 0
    }
}
impl DTSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTSR_A {
        match self.bits {
            false => DTSR_A::_0,
            true => DTSR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTSR_A::_1
    }
}
#[doc = "Field `SENSOVF` reader - CTSU Sensor Counter Overflow Flag"]
pub type SENSOVF_R = crate::BitReader<SENSOVF_A>;
#[doc = "CTSU Sensor Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENSOVF_A {
    #[doc = "0: No overflow occurred"]
    _0 = 0,
    #[doc = "1: Overflow occurred"]
    _1 = 1,
}
impl From<SENSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: SENSOVF_A) -> Self {
        variant as u8 != 0
    }
}
impl SENSOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SENSOVF_A {
        match self.bits {
            false => SENSOVF_A::_0,
            true => SENSOVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SENSOVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SENSOVF_A::_1
    }
}
#[doc = "Field `SENSOVF` writer - CTSU Sensor Counter Overflow Flag"]
pub type SENSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUSR_SPEC, SENSOVF_A, O>;
impl<'a, const O: u8> SENSOVF_W<'a, O> {
    #[doc = "No overflow occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SENSOVF_A::_0)
    }
    #[doc = "Overflow occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SENSOVF_A::_1)
    }
}
#[doc = "Field `SUOVF` reader - CTSU SUCLK Counter Overflow Flag"]
pub type SUOVF_R = crate::BitReader<SUOVF_A>;
#[doc = "CTSU SUCLK Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUOVF_A {
    #[doc = "0: No overflow occurred"]
    _0 = 0,
    #[doc = "1: Overflow occurred"]
    _1 = 1,
}
impl From<SUOVF_A> for bool {
    #[inline(always)]
    fn from(variant: SUOVF_A) -> Self {
        variant as u8 != 0
    }
}
impl SUOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUOVF_A {
        match self.bits {
            false => SUOVF_A::_0,
            true => SUOVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUOVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUOVF_A::_1
    }
}
#[doc = "Field `SUOVF` writer - CTSU SUCLK Counter Overflow Flag"]
pub type SUOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUSR_SPEC, SUOVF_A, O>;
impl<'a, const O: u8> SUOVF_W<'a, O> {
    #[doc = "No overflow occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUOVF_A::_0)
    }
    #[doc = "Overflow occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUOVF_A::_1)
    }
}
#[doc = "Field `PS` reader - CTSU Mutual Capacitance Status Flag"]
pub type PS_R = crate::BitReader<PS_A>;
#[doc = "CTSU Mutual Capacitance Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS_A {
    #[doc = "0: First measurement"]
    _0 = 0,
    #[doc = "1: Second measurement"]
    _1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::_0,
            true => PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PS_A::_1
    }
}
#[doc = "Field `CFCRDCH` reader - CTSU CFC Read Channel Select"]
pub type CFCRDCH_R = crate::FieldReader<u8, CFCRDCH_A>;
#[doc = "CTSU CFC Read Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFCRDCH_A {
    #[doc = "0: TS00"]
    _0X00 = 0,
    #[doc = "2: TS02 (CFC)"]
    _0X02 = 2,
    #[doc = "4: TS04"]
    _0X04 = 4,
    #[doc = "5: TS05"]
    _0X05 = 5,
    #[doc = "6: TS06"]
    _0X06 = 6,
    #[doc = "7: TS07"]
    _0X07 = 7,
    #[doc = "8: TS08 (CFC)"]
    _0X08 = 8,
    #[doc = "9: TS09 (CFC)"]
    _0X09 = 9,
    #[doc = "10: TS10 (CFC)"]
    _0X0A = 10,
    #[doc = "11: TS11 (CFC)"]
    _0X0B = 11,
    #[doc = "12: TS12 (CFC)"]
    _0X0C = 12,
    #[doc = "13: TS13 (CFC)"]
    _0X0D = 13,
    #[doc = "14: TS14 (CFC)"]
    _0X0E = 14,
    #[doc = "15: TS15 (CFC)"]
    _0X0F = 15,
    #[doc = "16: TS16 (CFC)"]
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
    #[doc = "26: TS26 (CFC)"]
    _0X1A = 26,
    #[doc = "27: TS27 (CFC)"]
    _0X1B = 27,
    #[doc = "28: TS28 (CFC)"]
    _0X1C = 28,
    #[doc = "29: TS29 (CFC)"]
    _0X1D = 29,
    #[doc = "30: TS30 (CFC)"]
    _0X1E = 30,
    #[doc = "31: TS31 (CFC)"]
    _0X1F = 31,
    #[doc = "32: TS32 (CFC)"]
    _0X20 = 32,
    #[doc = "33: TS33 (CFC)"]
    _0X21 = 33,
    #[doc = "34: TS34 (CFC)"]
    _0X22 = 34,
    #[doc = "35: TS35 (CFC)"]
    _0X23 = 35,
}
impl From<CFCRDCH_A> for u8 {
    #[inline(always)]
    fn from(variant: CFCRDCH_A) -> Self {
        variant as _
    }
}
impl CFCRDCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFCRDCH_A> {
        match self.bits {
            0 => Some(CFCRDCH_A::_0X00),
            2 => Some(CFCRDCH_A::_0X02),
            4 => Some(CFCRDCH_A::_0X04),
            5 => Some(CFCRDCH_A::_0X05),
            6 => Some(CFCRDCH_A::_0X06),
            7 => Some(CFCRDCH_A::_0X07),
            8 => Some(CFCRDCH_A::_0X08),
            9 => Some(CFCRDCH_A::_0X09),
            10 => Some(CFCRDCH_A::_0X0A),
            11 => Some(CFCRDCH_A::_0X0B),
            12 => Some(CFCRDCH_A::_0X0C),
            13 => Some(CFCRDCH_A::_0X0D),
            14 => Some(CFCRDCH_A::_0X0E),
            15 => Some(CFCRDCH_A::_0X0F),
            16 => Some(CFCRDCH_A::_0X10),
            17 => Some(CFCRDCH_A::_0X11),
            18 => Some(CFCRDCH_A::_0X12),
            21 => Some(CFCRDCH_A::_0X15),
            22 => Some(CFCRDCH_A::_0X16),
            23 => Some(CFCRDCH_A::_0X17),
            24 => Some(CFCRDCH_A::_0X18),
            25 => Some(CFCRDCH_A::_0X19),
            26 => Some(CFCRDCH_A::_0X1A),
            27 => Some(CFCRDCH_A::_0X1B),
            28 => Some(CFCRDCH_A::_0X1C),
            29 => Some(CFCRDCH_A::_0X1D),
            30 => Some(CFCRDCH_A::_0X1E),
            31 => Some(CFCRDCH_A::_0X1F),
            32 => Some(CFCRDCH_A::_0X20),
            33 => Some(CFCRDCH_A::_0X21),
            34 => Some(CFCRDCH_A::_0X22),
            35 => Some(CFCRDCH_A::_0X23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CFCRDCH_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == CFCRDCH_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == CFCRDCH_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == CFCRDCH_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == CFCRDCH_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == CFCRDCH_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == CFCRDCH_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == CFCRDCH_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == CFCRDCH_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == CFCRDCH_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == CFCRDCH_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == CFCRDCH_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == CFCRDCH_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == CFCRDCH_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == CFCRDCH_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == CFCRDCH_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == CFCRDCH_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X15`"]
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == CFCRDCH_A::_0X15
    }
    #[doc = "Checks if the value of the field is `_0X16`"]
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == CFCRDCH_A::_0X16
    }
    #[doc = "Checks if the value of the field is `_0X17`"]
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == CFCRDCH_A::_0X17
    }
    #[doc = "Checks if the value of the field is `_0X18`"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == CFCRDCH_A::_0X18
    }
    #[doc = "Checks if the value of the field is `_0X19`"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == CFCRDCH_A::_0X19
    }
    #[doc = "Checks if the value of the field is `_0X1A`"]
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == CFCRDCH_A::_0X1A
    }
    #[doc = "Checks if the value of the field is `_0X1B`"]
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == CFCRDCH_A::_0X1B
    }
    #[doc = "Checks if the value of the field is `_0X1C`"]
    #[inline(always)]
    pub fn is_0x1c(&self) -> bool {
        *self == CFCRDCH_A::_0X1C
    }
    #[doc = "Checks if the value of the field is `_0X1D`"]
    #[inline(always)]
    pub fn is_0x1d(&self) -> bool {
        *self == CFCRDCH_A::_0X1D
    }
    #[doc = "Checks if the value of the field is `_0X1E`"]
    #[inline(always)]
    pub fn is_0x1e(&self) -> bool {
        *self == CFCRDCH_A::_0X1E
    }
    #[doc = "Checks if the value of the field is `_0X1F`"]
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == CFCRDCH_A::_0X1F
    }
    #[doc = "Checks if the value of the field is `_0X20`"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == CFCRDCH_A::_0X20
    }
    #[doc = "Checks if the value of the field is `_0X21`"]
    #[inline(always)]
    pub fn is_0x21(&self) -> bool {
        *self == CFCRDCH_A::_0X21
    }
    #[doc = "Checks if the value of the field is `_0X22`"]
    #[inline(always)]
    pub fn is_0x22(&self) -> bool {
        *self == CFCRDCH_A::_0X22
    }
    #[doc = "Checks if the value of the field is `_0X23`"]
    #[inline(always)]
    pub fn is_0x23(&self) -> bool {
        *self == CFCRDCH_A::_0X23
    }
}
#[doc = "Field `CFCRDCH` writer - CTSU CFC Read Channel Select"]
pub type CFCRDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSR_SPEC, u8, CFCRDCH_A, 6, O>;
impl<'a, const O: u8> CFCRDCH_W<'a, O> {
    #[doc = "TS00"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X00)
    }
    #[doc = "TS02 (CFC)"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X02)
    }
    #[doc = "TS04"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X04)
    }
    #[doc = "TS05"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X05)
    }
    #[doc = "TS06"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X06)
    }
    #[doc = "TS07"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X07)
    }
    #[doc = "TS08 (CFC)"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X08)
    }
    #[doc = "TS09 (CFC)"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X09)
    }
    #[doc = "TS10 (CFC)"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X0A)
    }
    #[doc = "TS11 (CFC)"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X0B)
    }
    #[doc = "TS12 (CFC)"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X0C)
    }
    #[doc = "TS13 (CFC)"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X0D)
    }
    #[doc = "TS14 (CFC)"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X0E)
    }
    #[doc = "TS15 (CFC)"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X0F)
    }
    #[doc = "TS16 (CFC)"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X10)
    }
    #[doc = "TS17"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X11)
    }
    #[doc = "TS18"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X12)
    }
    #[doc = "TS21"]
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X15)
    }
    #[doc = "TS22"]
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X16)
    }
    #[doc = "TS23"]
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X17)
    }
    #[doc = "TS24"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X18)
    }
    #[doc = "TS25"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X19)
    }
    #[doc = "TS26 (CFC)"]
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X1A)
    }
    #[doc = "TS27 (CFC)"]
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X1B)
    }
    #[doc = "TS28 (CFC)"]
    #[inline(always)]
    pub fn _0x1c(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X1C)
    }
    #[doc = "TS29 (CFC)"]
    #[inline(always)]
    pub fn _0x1d(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X1D)
    }
    #[doc = "TS30 (CFC)"]
    #[inline(always)]
    pub fn _0x1e(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X1E)
    }
    #[doc = "TS31 (CFC)"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X1F)
    }
    #[doc = "TS32 (CFC)"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X20)
    }
    #[doc = "TS33 (CFC)"]
    #[inline(always)]
    pub fn _0x21(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X21)
    }
    #[doc = "TS34 (CFC)"]
    #[inline(always)]
    pub fn _0x22(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X22)
    }
    #[doc = "TS35 (CFC)"]
    #[inline(always)]
    pub fn _0x23(self) -> &'a mut W {
        self.variant(CFCRDCH_A::_0X23)
    }
}
impl R {
    #[doc = "Bits 0:1 - CTSU Multi-Clock Counter"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - CTSU Sense Current Error Monitor"]
    #[inline(always)]
    pub fn icomp1(&self) -> ICOMP1_R {
        ICOMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TSCAP Voltage Error Monitor"]
    #[inline(always)]
    pub fn icomp0(&self) -> ICOMP0_R {
        ICOMP0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - CTSU Measurement Status Counter"]
    #[inline(always)]
    pub fn stc(&self) -> STC_R {
        STC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - CTSU Data Transfer Status Flag"]
    #[inline(always)]
    pub fn dtsr(&self) -> DTSR_R {
        DTSR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    pub fn sensovf(&self) -> SENSOVF_R {
        SENSOVF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CTSU SUCLK Counter Overflow Flag"]
    #[inline(always)]
    pub fn suovf(&self) -> SUOVF_R {
        SUOVF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTSU Mutual Capacitance Status Flag"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - CTSU CFC Read Channel Select"]
    #[inline(always)]
    pub fn cfcrdch(&self) -> CFCRDCH_R {
        CFCRDCH_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTSU Multi-Clock Counter"]
    #[inline(always)]
    #[must_use]
    pub fn mfc(&mut self) -> MFC_W<0> {
        MFC_W::new(self)
    }
    #[doc = "Bit 5 - CTSU CTSUICOMP1 Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn icomprst(&mut self) -> ICOMPRST_W<5> {
        ICOMPRST_W::new(self)
    }
    #[doc = "Bit 13 - CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sensovf(&mut self) -> SENSOVF_W<13> {
        SENSOVF_W::new(self)
    }
    #[doc = "Bit 14 - CTSU SUCLK Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn suovf(&mut self) -> SUOVF_W<14> {
        SUOVF_W::new(self)
    }
    #[doc = "Bits 16:21 - CTSU CFC Read Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn cfcrdch(&mut self) -> CFCRDCH_W<16> {
        CFCRDCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsusr](index.html) module"]
pub struct CTSUSR_SPEC;
impl crate::RegisterSpec for CTSUSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsusr::R](R) reader structure"]
impl crate::Readable for CTSUSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsusr::W](W) writer structure"]
impl crate::Writable for CTSUSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSR to value 0"]
impl crate::Resettable for CTSUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
