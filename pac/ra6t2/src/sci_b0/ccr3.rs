#[doc = "Register `CCR3` reader"]
pub struct R(crate::R<CCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR3` writer"]
pub struct W(crate::W<CCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR3_SPEC>;
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
impl From<crate::W<CCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPHA` reader - Clock Phase Select"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock Phase Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Data is sampled at an odd edge and changes at an even edge. (Clock is delayed.)"]
    _0 = 0,
    #[doc = "1: Data changes at an odd edge and is sampled at an even edge. (Clock is not delayed)"]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
#[doc = "Field `CPHA` writer - Clock Phase Select"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Data is sampled at an odd edge and changes at an even edge. (Clock is delayed.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data changes at an odd edge and is sampled at an even edge. (Clock is not delayed)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity Select"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock Polarity Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: SCKn in idle state is 0."]
    _0 = 0,
    #[doc = "1: SCKn in idle state is 1."]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity Select"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "SCKn in idle state is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "SCKn in idle state is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
    }
}
#[doc = "Field `BPEN` reader - Synchronizer bypass enable"]
pub type BPEN_R = crate::BitReader<BPEN_A>;
#[doc = "Synchronizer bypass enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPEN_A {
    #[doc = "0: Synchronizer circuit is not bypassed."]
    _0 = 0,
    #[doc = "1: Synchronizer circuit is bypassed."]
    _1 = 1,
}
impl From<BPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPEN_A {
        match self.bits {
            false => BPEN_A::_0,
            true => BPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPEN_A::_1
    }
}
#[doc = "Field `BPEN` writer - Synchronizer bypass enable"]
pub type BPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, BPEN_A, O>;
impl<'a, const O: u8> BPEN_W<'a, O> {
    #[doc = "Synchronizer circuit is not bypassed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPEN_A::_0)
    }
    #[doc = "Synchronizer circuit is bypassed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPEN_A::_1)
    }
}
#[doc = "Field `CHR` reader - Character Length"]
pub type CHR_R = crate::FieldReader<u8, CHR_A>;
#[doc = "Character Length\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHR_A {
    #[doc = "0: Transmit/receive in 9-bit data length"]
    _00 = 0,
    #[doc = "1: Transmit/receive in 9-bit data length"]
    _01 = 1,
    #[doc = "2: Transmit/receive in 8-bit data length (initial value)"]
    _10 = 2,
    #[doc = "3: Transmit/receive in 7-bit data length"]
    _11 = 3,
}
impl From<CHR_A> for u8 {
    #[inline(always)]
    fn from(variant: CHR_A) -> Self {
        variant as _
    }
}
impl CHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHR_A {
        match self.bits {
            0 => CHR_A::_00,
            1 => CHR_A::_01,
            2 => CHR_A::_10,
            3 => CHR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CHR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CHR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CHR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CHR_A::_11
    }
}
#[doc = "Field `CHR` writer - Character Length"]
pub type CHR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR3_SPEC, u8, CHR_A, 2, O>;
impl<'a, const O: u8> CHR_W<'a, O> {
    #[doc = "Transmit/receive in 9-bit data length"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CHR_A::_00)
    }
    #[doc = "Transmit/receive in 9-bit data length"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CHR_A::_01)
    }
    #[doc = "Transmit/receive in 8-bit data length (initial value)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CHR_A::_10)
    }
    #[doc = "Transmit/receive in 7-bit data length"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CHR_A::_11)
    }
}
#[doc = "Field `LSBF` reader - LSB First select"]
pub type LSBF_R = crate::BitReader<LSBF_A>;
#[doc = "LSB First select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBF_A {
    #[doc = "0: MSB first"]
    _0 = 0,
    #[doc = "1: LSB first"]
    _1 = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::_0,
            true => LSBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LSBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LSBF_A::_1
    }
}
#[doc = "Field `LSBF` writer - LSB First select"]
pub type LSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, LSBF_A, O>;
impl<'a, const O: u8> LSBF_W<'a, O> {
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBF_A::_0)
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBF_A::_1)
    }
}
#[doc = "Field `SINV` reader - Transmitted/Received Data Invert"]
pub type SINV_R = crate::BitReader<SINV_A>;
#[doc = "Transmitted/Received Data Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINV_A {
    #[doc = "0: TDR contents are transmitted to TSR as they are. RSR contents are stored to RDR as they are."]
    _0 = 0,
    #[doc = "1: TDR contents are inverted before being transmitted to TSR. RSR contents are inverted and stored to RDR."]
    _1 = 1,
}
impl From<SINV_A> for bool {
    #[inline(always)]
    fn from(variant: SINV_A) -> Self {
        variant as u8 != 0
    }
}
impl SINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINV_A {
        match self.bits {
            false => SINV_A::_0,
            true => SINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SINV_A::_1
    }
}
#[doc = "Field `SINV` writer - Transmitted/Received Data Invert"]
pub type SINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, SINV_A, O>;
impl<'a, const O: u8> SINV_W<'a, O> {
    #[doc = "TDR contents are transmitted to TSR as they are. RSR contents are stored to RDR as they are."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINV_A::_0)
    }
    #[doc = "TDR contents are inverted before being transmitted to TSR. RSR contents are inverted and stored to RDR."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINV_A::_1)
    }
}
#[doc = "Field `STP` reader - Stop Bit Length"]
pub type STP_R = crate::BitReader<STP_A>;
#[doc = "Stop Bit Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STP_A {
    #[doc = "0: 1 stop bit / Break Delimiter length is 1bit"]
    _0 = 0,
    #[doc = "1: 2 stop bits / Break Delimiter length is 2bits"]
    _1 = 1,
}
impl From<STP_A> for bool {
    #[inline(always)]
    fn from(variant: STP_A) -> Self {
        variant as u8 != 0
    }
}
impl STP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STP_A {
        match self.bits {
            false => STP_A::_0,
            true => STP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STP_A::_1
    }
}
#[doc = "Field `STP` writer - Stop Bit Length"]
pub type STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, STP_A, O>;
impl<'a, const O: u8> STP_W<'a, O> {
    #[doc = "1 stop bit / Break Delimiter length is 1bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STP_A::_0)
    }
    #[doc = "2 stop bits / Break Delimiter length is 2bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STP_A::_1)
    }
}
#[doc = "Field `RXDESEL` reader - Asynchronous Start Bit Edge Detection Select"]
pub type RXDESEL_R = crate::BitReader<RXDESEL_A>;
#[doc = "Asynchronous Start Bit Edge Detection Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDESEL_A {
    #[doc = "0: The low level on the RXDn pin is detected as the start bit."]
    _0 = 0,
    #[doc = "1: A falling edge on the RXDn pin is detected as the start bit."]
    _1 = 1,
}
impl From<RXDESEL_A> for bool {
    #[inline(always)]
    fn from(variant: RXDESEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDESEL_A {
        match self.bits {
            false => RXDESEL_A::_0,
            true => RXDESEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDESEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDESEL_A::_1
    }
}
#[doc = "Field `RXDESEL` writer - Asynchronous Start Bit Edge Detection Select"]
pub type RXDESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, RXDESEL_A, O>;
impl<'a, const O: u8> RXDESEL_W<'a, O> {
    #[doc = "The low level on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDESEL_A::_0)
    }
    #[doc = "A falling edge on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDESEL_A::_1)
    }
}
#[doc = "Field `MOD` reader - Communication mode select"]
pub type MOD_R = crate::FieldReader<u8, MOD_A>;
#[doc = "Communication mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MOD_A {
    #[doc = "0: Asynchronous mode (Multi-processor mode)"]
    _000 = 0,
    #[doc = "1: Smart card interface mode"]
    _001 = 1,
    #[doc = "2: Clock synchronous mode"]
    _010 = 2,
    #[doc = "3: Simple SPI mode"]
    _011 = 3,
    #[doc = "4: Simple IIC mode"]
    _100 = 4,
    #[doc = "5: Manchester mode"]
    _101 = 5,
    #[doc = "6: Simple LIN mode"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        variant as _
    }
}
impl MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_A {
        match self.bits {
            0 => MOD_A::_000,
            1 => MOD_A::_001,
            2 => MOD_A::_010,
            3 => MOD_A::_011,
            4 => MOD_A::_100,
            5 => MOD_A::_101,
            6 => MOD_A::_110,
            7 => MOD_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MOD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MOD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MOD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MOD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MOD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MOD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MOD_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MOD_A::_111
    }
}
#[doc = "Field `MOD` writer - Communication mode select"]
pub type MOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR3_SPEC, u8, MOD_A, 3, O>;
impl<'a, const O: u8> MOD_W<'a, O> {
    #[doc = "Asynchronous mode (Multi-processor mode)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MOD_A::_000)
    }
    #[doc = "Smart card interface mode"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MOD_A::_001)
    }
    #[doc = "Clock synchronous mode"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MOD_A::_010)
    }
    #[doc = "Simple SPI mode"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MOD_A::_011)
    }
    #[doc = "Simple IIC mode"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MOD_A::_100)
    }
    #[doc = "Manchester mode"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MOD_A::_101)
    }
    #[doc = "Simple LIN mode"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MOD_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MOD_A::_111)
    }
}
#[doc = "Field `MP` reader - Multi-Processor Mode"]
pub type MP_R = crate::BitReader<MP_A>;
#[doc = "Multi-Processor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MP_A {
    #[doc = "0: Multi-processor communications function is disabled"]
    _0 = 0,
    #[doc = "1: Multi-processor communications function is enabled"]
    _1 = 1,
}
impl From<MP_A> for bool {
    #[inline(always)]
    fn from(variant: MP_A) -> Self {
        variant as u8 != 0
    }
}
impl MP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MP_A {
        match self.bits {
            false => MP_A::_0,
            true => MP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MP_A::_1
    }
}
#[doc = "Field `MP` writer - Multi-Processor Mode"]
pub type MP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, MP_A, O>;
impl<'a, const O: u8> MP_W<'a, O> {
    #[doc = "Multi-processor communications function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MP_A::_0)
    }
    #[doc = "Multi-processor communications function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MP_A::_1)
    }
}
#[doc = "Field `FM` reader - FIFO Mode select"]
pub type FM_R = crate::BitReader<FM_A>;
#[doc = "FIFO Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FM_A {
    #[doc = "0: TDR register, RDR register is non-FIFO buffer configuration"]
    _0 = 0,
    #[doc = "1: TDR register, RDR register is FIFO buffer configuration"]
    _1 = 1,
}
impl From<FM_A> for bool {
    #[inline(always)]
    fn from(variant: FM_A) -> Self {
        variant as u8 != 0
    }
}
impl FM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FM_A {
        match self.bits {
            false => FM_A::_0,
            true => FM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FM_A::_1
    }
}
#[doc = "Field `FM` writer - FIFO Mode select"]
pub type FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, FM_A, O>;
impl<'a, const O: u8> FM_W<'a, O> {
    #[doc = "TDR register, RDR register is non-FIFO buffer configuration"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FM_A::_0)
    }
    #[doc = "TDR register, RDR register is FIFO buffer configuration"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FM_A::_1)
    }
}
#[doc = "Field `DEN` reader - Driver enable"]
pub type DEN_R = crate::BitReader<DEN_A>;
#[doc = "Driver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN_A {
    #[doc = "0: RS-485 Driver control function disable."]
    _0 = 0,
    #[doc = "1: RS-485 Driver control function enable."]
    _1 = 1,
}
impl From<DEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN_A {
        match self.bits {
            false => DEN_A::_0,
            true => DEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEN_A::_1
    }
}
#[doc = "Field `DEN` writer - Driver enable"]
pub type DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, DEN_A, O>;
impl<'a, const O: u8> DEN_W<'a, O> {
    #[doc = "RS-485 Driver control function disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEN_A::_0)
    }
    #[doc = "RS-485 Driver control function enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEN_A::_1)
    }
}
#[doc = "Field `CKE` reader - Clock enable"]
pub type CKE_R = crate::FieldReader<u8, CKE_A>;
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKE_A {
    #[doc = "0: In the case of Asynchronous modeOn-chip baud rate generatorThe SCKn pin is available for use as an I/O port in accord with the I/O port settings. In the case of Manchester mode and Simple LIN modeOn-chip baud rate generatorThe SCKn pin functions as I/O port. In the case of Clock synchronous mode, Simple SPI modeInternal clock (Master operation)The SCKn pin functions as the clock output pin. In the case of Smart card interface mode and CCR3.GM = 0Output disabled (The SCKn pin is available for use as an I/O port in accord with the I/O port settings.) In the case of Smart card interface mode and CCR3.GM = 1Output fixed low"]
    _00 = 0,
    #[doc = "1: In the case of Asynchronous modeOn-chip baud rate generatorThe clock with the same frequency as the bit rate is output from the SCKn pin. In the case of Manchester mode and Simple LIN modeProhibited In the case of Clock synchronous mode, Simple SPI modeInternal clock (Master operation)The SCKn pin functions as the clock output pin. In the case of Smart card interface mode and CCR3.GM = 0Clock output In the case of Smart card interface mode and CCR3.GM = 1Clock output"]
    _01 = 1,
    #[doc = "2: In the case of Asynchronous modeExternal clock When using the external clock16 times the bit rate should be input from the SCKn pin when CCR2.ABCS bit is 0. Input a clock signal with a frequency 8 times the bit rate when the CCR2.ABCS bit is 1. In the case of Manchester mode and Simple LIN modeProhibited In the case of Clock synchronous mode, Simple SPI modeExternal clock (Slave operation)The SCKn pin functions as the clock input pin. In the case of Smart card interface mode and CCR3.GM = 0Prohibited In the case of Smart card interface mode and CCR3.GM = 1Output fixed high"]
    _10 = 2,
    #[doc = "3: In the case of Asynchronous modeExternal clock When using the external clock16 times the bit rate should be input from the SCKn pin when CCR2.ABCS bit is 0. Input a clock signal with a frequency 8 times the bit rate when the CCR2.ABCS bit is 1. In the case of Manchester mode and Simple LIN modeProhibited In the case of Clock synchronous mode, Simple SPI modeExternal clock (Slave operation)The SCKn pin functions as the clock input pin. In the case of Smart card interface mode and CCR3.GM = 0Prohibited In the case of Smart card interface mode and CCR3.GM = 1Clock output"]
    _11 = 3,
}
impl From<CKE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKE_A) -> Self {
        variant as _
    }
}
impl CKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKE_A {
        match self.bits {
            0 => CKE_A::_00,
            1 => CKE_A::_01,
            2 => CKE_A::_10,
            3 => CKE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKE_A::_11
    }
}
#[doc = "Field `CKE` writer - Clock enable"]
pub type CKE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR3_SPEC, u8, CKE_A, 2, O>;
impl<'a, const O: u8> CKE_W<'a, O> {
    #[doc = "In the case of Asynchronous modeOn-chip baud rate generatorThe SCKn pin is available for use as an I/O port in accord with the I/O port settings. In the case of Manchester mode and Simple LIN modeOn-chip baud rate generatorThe SCKn pin functions as I/O port. In the case of Clock synchronous mode, Simple SPI modeInternal clock (Master operation)The SCKn pin functions as the clock output pin. In the case of Smart card interface mode and CCR3.GM = 0Output disabled (The SCKn pin is available for use as an I/O port in accord with the I/O port settings.) In the case of Smart card interface mode and CCR3.GM = 1Output fixed low"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CKE_A::_00)
    }
    #[doc = "In the case of Asynchronous modeOn-chip baud rate generatorThe clock with the same frequency as the bit rate is output from the SCKn pin. In the case of Manchester mode and Simple LIN modeProhibited In the case of Clock synchronous mode, Simple SPI modeInternal clock (Master operation)The SCKn pin functions as the clock output pin. In the case of Smart card interface mode and CCR3.GM = 0Clock output In the case of Smart card interface mode and CCR3.GM = 1Clock output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CKE_A::_01)
    }
    #[doc = "In the case of Asynchronous modeExternal clock When using the external clock16 times the bit rate should be input from the SCKn pin when CCR2.ABCS bit is 0. Input a clock signal with a frequency 8 times the bit rate when the CCR2.ABCS bit is 1. In the case of Manchester mode and Simple LIN modeProhibited In the case of Clock synchronous mode, Simple SPI modeExternal clock (Slave operation)The SCKn pin functions as the clock input pin. In the case of Smart card interface mode and CCR3.GM = 0Prohibited In the case of Smart card interface mode and CCR3.GM = 1Output fixed high"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CKE_A::_10)
    }
    #[doc = "In the case of Asynchronous modeExternal clock When using the external clock16 times the bit rate should be input from the SCKn pin when CCR2.ABCS bit is 0. Input a clock signal with a frequency 8 times the bit rate when the CCR2.ABCS bit is 1. In the case of Manchester mode and Simple LIN modeProhibited In the case of Clock synchronous mode, Simple SPI modeExternal clock (Slave operation)The SCKn pin functions as the clock input pin. In the case of Smart card interface mode and CCR3.GM = 0Prohibited In the case of Smart card interface mode and CCR3.GM = 1Clock output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CKE_A::_11)
    }
}
#[doc = "Field `GM` reader - GSM Mode"]
pub type GM_R = crate::BitReader<GM_A>;
#[doc = "GSM Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GM_A {
    #[doc = "0: Non-GSM mode operation"]
    _0 = 0,
    #[doc = "1: GSM mode operation"]
    _1 = 1,
}
impl From<GM_A> for bool {
    #[inline(always)]
    fn from(variant: GM_A) -> Self {
        variant as u8 != 0
    }
}
impl GM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GM_A {
        match self.bits {
            false => GM_A::_0,
            true => GM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GM_A::_1
    }
}
#[doc = "Field `GM` writer - GSM Mode"]
pub type GM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, GM_A, O>;
impl<'a, const O: u8> GM_W<'a, O> {
    #[doc = "Non-GSM mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GM_A::_0)
    }
    #[doc = "GSM mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GM_A::_1)
    }
}
#[doc = "Field `BLK` reader - Block Transfer Mode"]
pub type BLK_R = crate::BitReader<BLK_A>;
#[doc = "Block Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK_A {
    #[doc = "0: Non-block transfer mode operation"]
    _0 = 0,
    #[doc = "1: Block transfer mode operation"]
    _1 = 1,
}
impl From<BLK_A> for bool {
    #[inline(always)]
    fn from(variant: BLK_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK_A {
        match self.bits {
            false => BLK_A::_0,
            true => BLK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLK_A::_1
    }
}
#[doc = "Field `BLK` writer - Block Transfer Mode"]
pub type BLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, BLK_A, O>;
impl<'a, const O: u8> BLK_W<'a, O> {
    #[doc = "Non-block transfer mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLK_A::_0)
    }
    #[doc = "Block transfer mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLK_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Phase Select"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity Select"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronizer bypass enable"]
    #[inline(always)]
    pub fn bpen(&self) -> BPEN_R {
        BPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Character Length"]
    #[inline(always)]
    pub fn chr(&self) -> CHR_R {
        CHR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - LSB First select"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmitted/Received Data Invert"]
    #[inline(always)]
    pub fn sinv(&self) -> SINV_R {
        SINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop Bit Length"]
    #[inline(always)]
    pub fn stp(&self) -> STP_R {
        STP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(&self) -> RXDESEL_R {
        RXDESEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Communication mode select"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mp(&self) -> MP_R {
        MP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FIFO Mode select"]
    #[inline(always)]
    pub fn fm(&self) -> FM_R {
        FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Driver enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Clock enable"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - GSM Mode"]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(&self) -> BLK_R {
        BLK_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - Clock Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 7 - Synchronizer bypass enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpen(&mut self) -> BPEN_W<7> {
        BPEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn chr(&mut self) -> CHR_W<8> {
        CHR_W::new(self)
    }
    #[doc = "Bit 12 - LSB First select"]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LSBF_W<12> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 13 - Transmitted/Received Data Invert"]
    #[inline(always)]
    #[must_use]
    pub fn sinv(&mut self) -> SINV_W<13> {
        SINV_W::new(self)
    }
    #[doc = "Bit 14 - Stop Bit Length"]
    #[inline(always)]
    #[must_use]
    pub fn stp(&mut self) -> STP_W<14> {
        STP_W::new(self)
    }
    #[doc = "Bit 15 - Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxdesel(&mut self) -> RXDESEL_W<15> {
        RXDESEL_W::new(self)
    }
    #[doc = "Bits 16:18 - Communication mode select"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<16> {
        MOD_W::new(self)
    }
    #[doc = "Bit 19 - Multi-Processor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mp(&mut self) -> MP_W<19> {
        MP_W::new(self)
    }
    #[doc = "Bit 20 - FIFO Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fm(&mut self) -> FM_W<20> {
        FM_W::new(self)
    }
    #[doc = "Bit 21 - Driver enable"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<21> {
        DEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<24> {
        CKE_W::new(self)
    }
    #[doc = "Bit 28 - GSM Mode"]
    #[inline(always)]
    #[must_use]
    pub fn gm(&mut self) -> GM_W<28> {
        GM_W::new(self)
    }
    #[doc = "Bit 29 - Block Transfer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn blk(&mut self) -> BLK_W<29> {
        BLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr3](index.html) module"]
pub struct CCR3_SPEC;
impl crate::RegisterSpec for CCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr3::R](R) reader structure"]
impl crate::Readable for CCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr3::W](W) writer structure"]
impl crate::Writable for CCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR3 to value 0x1203"]
impl crate::Resettable for CCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1203;
}
