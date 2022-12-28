#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMPOL` reader - Polarity of Received Manchester Code"]
pub type RMPOL_R = crate::BitReader<RMPOL_A>;
#[doc = "Polarity of Received Manchester Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMPOL_A {
    #[doc = "0: Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    _0 = 0,
    #[doc = "1: Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    _1 = 1,
}
impl From<RMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RMPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl RMPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMPOL_A {
        match self.bits {
            false => RMPOL_A::_0,
            true => RMPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMPOL_A::_1
    }
}
#[doc = "Field `RMPOL` writer - Polarity of Received Manchester Code"]
pub type RMPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RMPOL_A, O>;
impl<'a, const O: u8> RMPOL_W<'a, O> {
    #[doc = "Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMPOL_A::_0)
    }
    #[doc = "Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMPOL_A::_1)
    }
}
#[doc = "Field `TMPOL` reader - Polarity of Transmit Manchester Code"]
pub type TMPOL_R = crate::BitReader<TMPOL_A>;
#[doc = "Polarity of Transmit Manchester Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMPOL_A {
    #[doc = "0: Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    _0 = 0,
    #[doc = "1: Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    _1 = 1,
}
impl From<TMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TMPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TMPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMPOL_A {
        match self.bits {
            false => TMPOL_A::_0,
            true => TMPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMPOL_A::_1
    }
}
#[doc = "Field `TMPOL` writer - Polarity of Transmit Manchester Code"]
pub type TMPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, TMPOL_A, O>;
impl<'a, const O: u8> TMPOL_W<'a, O> {
    #[doc = "Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMPOL_A::_0)
    }
    #[doc = "Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMPOL_A::_1)
    }
}
#[doc = "Field `ERTEN` reader - Manchester Edge Retiming Enable"]
pub type ERTEN_R = crate::BitReader<ERTEN_A>;
#[doc = "Manchester Edge Retiming Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERTEN_A {
    #[doc = "0: Disables the receive retiming function"]
    _0 = 0,
    #[doc = "1: Enables the receive retiming function"]
    _1 = 1,
}
impl From<ERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ERTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERTEN_A {
        match self.bits {
            false => ERTEN_A::_0,
            true => ERTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERTEN_A::_1
    }
}
#[doc = "Field `ERTEN` writer - Manchester Edge Retiming Enable"]
pub type ERTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, ERTEN_A, O>;
impl<'a, const O: u8> ERTEN_W<'a, O> {
    #[doc = "Disables the receive retiming function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERTEN_A::_0)
    }
    #[doc = "Enables the receive retiming function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERTEN_A::_1)
    }
}
#[doc = "Field `SYNVAL` reader - SYNC value Setting"]
pub type SYNVAL_R = crate::BitReader<bool>;
#[doc = "Field `SYNVAL` writer - SYNC value Setting"]
pub type SYNVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `SYNSEL` reader - SYNC Select"]
pub type SYNSEL_R = crate::BitReader<SYNSEL_A>;
#[doc = "SYNC Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNSEL_A {
    #[doc = "0: The start bit pattern is set with the SYNVAL bit"]
    _0 = 0,
    #[doc = "1: The start bit pattern is set with the TSYNC bit."]
    _1 = 1,
}
impl From<SYNSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYNSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNSEL_A {
        match self.bits {
            false => SYNSEL_A::_0,
            true => SYNSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNSEL_A::_1
    }
}
#[doc = "Field `SYNSEL` writer - SYNC Select"]
pub type SYNSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SYNSEL_A, O>;
impl<'a, const O: u8> SYNSEL_W<'a, O> {
    #[doc = "The start bit pattern is set with the SYNVAL bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNSEL_A::_0)
    }
    #[doc = "The start bit pattern is set with the TSYNC bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNSEL_A::_1)
    }
}
#[doc = "Field `SBSEL` reader - Start Bit Select"]
pub type SBSEL_R = crate::BitReader<SBSEL_A>;
#[doc = "Start Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSEL_A {
    #[doc = "0: The start bit area consists of one bit."]
    _0 = 0,
    #[doc = "1: The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)"]
    _1 = 1,
}
impl From<SBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SBSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBSEL_A {
        match self.bits {
            false => SBSEL_A::_0,
            true => SBSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBSEL_A::_1
    }
}
#[doc = "Field `SBSEL` writer - Start Bit Select"]
pub type SBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SBSEL_A, O>;
impl<'a, const O: u8> SBSEL_W<'a, O> {
    #[doc = "The start bit area consists of one bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBSEL_A::_0)
    }
    #[doc = "The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBSEL_A::_1)
    }
}
#[doc = "Field `TPLEN` reader - Transmit preface length"]
pub type TPLEN_R = crate::FieldReader<u8, TPLEN_A>;
#[doc = "Transmit preface length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPLEN_A {
    #[doc = "0: Disables the transmit preface generation"]
    _0X0 = 0,
}
impl From<TPLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TPLEN_A) -> Self {
        variant as _
    }
}
impl TPLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPLEN_A> {
        match self.bits {
            0 => Some(TPLEN_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == TPLEN_A::_0X0
    }
}
#[doc = "Field `TPLEN` writer - Transmit preface length"]
pub type TPLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, TPLEN_A, 4, O>;
impl<'a, const O: u8> TPLEN_W<'a, O> {
    #[doc = "Disables the transmit preface generation"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(TPLEN_A::_0X0)
    }
}
#[doc = "Field `TPPAT` reader - Transmit preface pattern"]
pub type TPPAT_R = crate::FieldReader<u8, TPPAT_A>;
#[doc = "Transmit preface pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPPAT_A {
    #[doc = "0: ALL ZERO"]
    _00 = 0,
    #[doc = "1: ZERO ONE"]
    _01 = 1,
    #[doc = "2: ONE ZERO"]
    _10 = 2,
    #[doc = "3: ALL ONE"]
    _11 = 3,
}
impl From<TPPAT_A> for u8 {
    #[inline(always)]
    fn from(variant: TPPAT_A) -> Self {
        variant as _
    }
}
impl TPPAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPPAT_A {
        match self.bits {
            0 => TPPAT_A::_00,
            1 => TPPAT_A::_01,
            2 => TPPAT_A::_10,
            3 => TPPAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPPAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPPAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPPAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TPPAT_A::_11
    }
}
#[doc = "Field `TPPAT` writer - Transmit preface pattern"]
pub type TPPAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, TPPAT_A, 2, O>;
impl<'a, const O: u8> TPPAT_W<'a, O> {
    #[doc = "ALL ZERO"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPPAT_A::_00)
    }
    #[doc = "ZERO ONE"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPPAT_A::_01)
    }
    #[doc = "ONE ZERO"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPPAT_A::_10)
    }
    #[doc = "ALL ONE"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPPAT_A::_11)
    }
}
#[doc = "Field `RPLEN` reader - Receive Preface Length"]
pub type RPLEN_R = crate::FieldReader<u8, RPLEN_A>;
#[doc = "Receive Preface Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPLEN_A {
    #[doc = "0: Disables the receive preface generation"]
    _0X0 = 0,
}
impl From<RPLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RPLEN_A) -> Self {
        variant as _
    }
}
impl RPLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RPLEN_A> {
        match self.bits {
            0 => Some(RPLEN_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == RPLEN_A::_0X0
    }
}
#[doc = "Field `RPLEN` writer - Receive Preface Length"]
pub type RPLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, RPLEN_A, 4, O>;
impl<'a, const O: u8> RPLEN_W<'a, O> {
    #[doc = "Disables the receive preface generation"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(RPLEN_A::_0X0)
    }
}
#[doc = "Field `RPPAT` reader - Receive Preface Pattern"]
pub type RPPAT_R = crate::FieldReader<u8, RPPAT_A>;
#[doc = "Receive Preface Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPPAT_A {
    #[doc = "0: ALL ZERO"]
    _00 = 0,
    #[doc = "1: ZERO ONE"]
    _01 = 1,
    #[doc = "2: ONE ZERO"]
    _10 = 2,
    #[doc = "3: ALL ONE"]
    _11 = 3,
}
impl From<RPPAT_A> for u8 {
    #[inline(always)]
    fn from(variant: RPPAT_A) -> Self {
        variant as _
    }
}
impl RPPAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPPAT_A {
        match self.bits {
            0 => RPPAT_A::_00,
            1 => RPPAT_A::_01,
            2 => RPPAT_A::_10,
            3 => RPPAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RPPAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RPPAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RPPAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RPPAT_A::_11
    }
}
#[doc = "Field `RPPAT` writer - Receive Preface Pattern"]
pub type RPPAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, RPPAT_A, 2, O>;
impl<'a, const O: u8> RPPAT_W<'a, O> {
    #[doc = "ALL ZERO"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RPPAT_A::_00)
    }
    #[doc = "ZERO ONE"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RPPAT_A::_01)
    }
    #[doc = "ONE ZERO"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RPPAT_A::_10)
    }
    #[doc = "ALL ONE"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RPPAT_A::_11)
    }
}
#[doc = "Field `PFEREN` reader - Preface Error Enable"]
pub type PFEREN_R = crate::BitReader<PFEREN_A>;
#[doc = "Preface Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFEREN_A {
    #[doc = "0: Does not handle a preface error as an interrupt source"]
    _0 = 0,
    #[doc = "1: Handles a preface error as an interrupt source"]
    _1 = 1,
}
impl From<PFEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PFEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PFEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFEREN_A {
        match self.bits {
            false => PFEREN_A::_0,
            true => PFEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFEREN_A::_1
    }
}
#[doc = "Field `PFEREN` writer - Preface Error Enable"]
pub type PFEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PFEREN_A, O>;
impl<'a, const O: u8> PFEREN_W<'a, O> {
    #[doc = "Does not handle a preface error as an interrupt source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFEREN_A::_0)
    }
    #[doc = "Handles a preface error as an interrupt source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFEREN_A::_1)
    }
}
#[doc = "Field `SYEREN` reader - Receive SYNC Error Enable"]
pub type SYEREN_R = crate::BitReader<SYEREN_A>;
#[doc = "Receive SYNC Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYEREN_A {
    #[doc = "0: Does not handle a receive SYNC error as an interrupt source"]
    _0 = 0,
    #[doc = "1: Handles a receive SYNC error as an interrupt source"]
    _1 = 1,
}
impl From<SYEREN_A> for bool {
    #[inline(always)]
    fn from(variant: SYEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYEREN_A {
        match self.bits {
            false => SYEREN_A::_0,
            true => SYEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYEREN_A::_1
    }
}
#[doc = "Field `SYEREN` writer - Receive SYNC Error Enable"]
pub type SYEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SYEREN_A, O>;
impl<'a, const O: u8> SYEREN_W<'a, O> {
    #[doc = "Does not handle a receive SYNC error as an interrupt source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYEREN_A::_0)
    }
    #[doc = "Handles a receive SYNC error as an interrupt source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYEREN_A::_1)
    }
}
#[doc = "Field `SBEREN` reader - Start Bit Error Enable"]
pub type SBEREN_R = crate::BitReader<SBEREN_A>;
#[doc = "Start Bit Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBEREN_A {
    #[doc = "0: Does not handle a start bit error as an interrupt source"]
    _0 = 0,
    #[doc = "1: Handles a start bit error as an interrupt source"]
    _1 = 1,
}
impl From<SBEREN_A> for bool {
    #[inline(always)]
    fn from(variant: SBEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl SBEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBEREN_A {
        match self.bits {
            false => SBEREN_A::_0,
            true => SBEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBEREN_A::_1
    }
}
#[doc = "Field `SBEREN` writer - Start Bit Error Enable"]
pub type SBEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SBEREN_A, O>;
impl<'a, const O: u8> SBEREN_W<'a, O> {
    #[doc = "Does not handle a start bit error as an interrupt source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBEREN_A::_0)
    }
    #[doc = "Handles a start bit error as an interrupt source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBEREN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of Received Manchester Code"]
    #[inline(always)]
    pub fn rmpol(&self) -> RMPOL_R {
        RMPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity of Transmit Manchester Code"]
    #[inline(always)]
    pub fn tmpol(&self) -> TMPOL_R {
        TMPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manchester Edge Retiming Enable"]
    #[inline(always)]
    pub fn erten(&self) -> ERTEN_R {
        ERTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SYNC value Setting"]
    #[inline(always)]
    pub fn synval(&self) -> SYNVAL_R {
        SYNVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNC Select"]
    #[inline(always)]
    pub fn synsel(&self) -> SYNSEL_R {
        SYNSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start Bit Select"]
    #[inline(always)]
    pub fn sbsel(&self) -> SBSEL_R {
        SBSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit preface length"]
    #[inline(always)]
    pub fn tplen(&self) -> TPLEN_R {
        TPLEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Transmit preface pattern"]
    #[inline(always)]
    pub fn tppat(&self) -> TPPAT_R {
        TPPAT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Receive Preface Length"]
    #[inline(always)]
    pub fn rplen(&self) -> RPLEN_R {
        RPLEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Receive Preface Pattern"]
    #[inline(always)]
    pub fn rppat(&self) -> RPPAT_R {
        RPPAT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - Preface Error Enable"]
    #[inline(always)]
    pub fn pferen(&self) -> PFEREN_R {
        PFEREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive SYNC Error Enable"]
    #[inline(always)]
    pub fn syeren(&self) -> SYEREN_R {
        SYEREN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Start Bit Error Enable"]
    #[inline(always)]
    pub fn sberen(&self) -> SBEREN_R {
        SBEREN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of Received Manchester Code"]
    #[inline(always)]
    #[must_use]
    pub fn rmpol(&mut self) -> RMPOL_W<0> {
        RMPOL_W::new(self)
    }
    #[doc = "Bit 1 - Polarity of Transmit Manchester Code"]
    #[inline(always)]
    #[must_use]
    pub fn tmpol(&mut self) -> TMPOL_W<1> {
        TMPOL_W::new(self)
    }
    #[doc = "Bit 2 - Manchester Edge Retiming Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erten(&mut self) -> ERTEN_W<2> {
        ERTEN_W::new(self)
    }
    #[doc = "Bit 4 - SYNC value Setting"]
    #[inline(always)]
    #[must_use]
    pub fn synval(&mut self) -> SYNVAL_W<4> {
        SYNVAL_W::new(self)
    }
    #[doc = "Bit 5 - SYNC Select"]
    #[inline(always)]
    #[must_use]
    pub fn synsel(&mut self) -> SYNSEL_W<5> {
        SYNSEL_W::new(self)
    }
    #[doc = "Bit 6 - Start Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn sbsel(&mut self) -> SBSEL_W<6> {
        SBSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Transmit preface length"]
    #[inline(always)]
    #[must_use]
    pub fn tplen(&mut self) -> TPLEN_W<8> {
        TPLEN_W::new(self)
    }
    #[doc = "Bits 12:13 - Transmit preface pattern"]
    #[inline(always)]
    #[must_use]
    pub fn tppat(&mut self) -> TPPAT_W<12> {
        TPPAT_W::new(self)
    }
    #[doc = "Bits 16:19 - Receive Preface Length"]
    #[inline(always)]
    #[must_use]
    pub fn rplen(&mut self) -> RPLEN_W<16> {
        RPLEN_W::new(self)
    }
    #[doc = "Bits 20:21 - Receive Preface Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn rppat(&mut self) -> RPPAT_W<20> {
        RPPAT_W::new(self)
    }
    #[doc = "Bit 24 - Preface Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pferen(&mut self) -> PFEREN_W<24> {
        PFEREN_W::new(self)
    }
    #[doc = "Bit 25 - Receive SYNC Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syeren(&mut self) -> SYEREN_W<25> {
        SYEREN_W::new(self)
    }
    #[doc = "Bit 26 - Start Bit Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sberen(&mut self) -> SBEREN_W<26> {
        SBEREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
