#[doc = "Register `SPSR` reader"]
pub struct R(crate::R<SPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPCP` reader - SPI Command Pointer"]
pub type SPCP_R = crate::FieldReader<u8, SPCP_A>;
#[doc = "SPI Command Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPCP_A {
    #[doc = "0: SPCMD0"]
    _000 = 0,
    #[doc = "1: SPCMD1"]
    _001 = 1,
    #[doc = "2: SPCMD2"]
    _010 = 2,
    #[doc = "3: SPCMD3"]
    _011 = 3,
    #[doc = "4: SPCMD4"]
    _100 = 4,
    #[doc = "5: SPCMD5"]
    _101 = 5,
    #[doc = "6: SPCMD6"]
    _110 = 6,
    #[doc = "7: SPCMD7"]
    _111 = 7,
}
impl From<SPCP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPCP_A) -> Self {
        variant as _
    }
}
impl SPCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCP_A {
        match self.bits {
            0 => SPCP_A::_000,
            1 => SPCP_A::_001,
            2 => SPCP_A::_010,
            3 => SPCP_A::_011,
            4 => SPCP_A::_100,
            5 => SPCP_A::_101,
            6 => SPCP_A::_110,
            7 => SPCP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPCP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPCP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPCP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPCP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPCP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPCP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPCP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPCP_A::_111
    }
}
#[doc = "Field `SPECM` reader - SPI Error Command"]
pub type SPECM_R = crate::FieldReader<u8, SPECM_A>;
#[doc = "SPI Error Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPECM_A {
    #[doc = "0: SPCMD0"]
    _000 = 0,
    #[doc = "1: SPCMD1"]
    _001 = 1,
    #[doc = "2: SPCMD2"]
    _010 = 2,
    #[doc = "3: SPCMD3"]
    _011 = 3,
    #[doc = "4: SPCMD4"]
    _100 = 4,
    #[doc = "5: SPCMD5"]
    _101 = 5,
    #[doc = "6: SPCMD6"]
    _110 = 6,
    #[doc = "7: SPCMD7"]
    _111 = 7,
}
impl From<SPECM_A> for u8 {
    #[inline(always)]
    fn from(variant: SPECM_A) -> Self {
        variant as _
    }
}
impl SPECM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPECM_A {
        match self.bits {
            0 => SPECM_A::_000,
            1 => SPECM_A::_001,
            2 => SPECM_A::_010,
            3 => SPECM_A::_011,
            4 => SPECM_A::_100,
            5 => SPECM_A::_101,
            6 => SPECM_A::_110,
            7 => SPECM_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPECM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPECM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPECM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPECM_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPECM_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPECM_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPECM_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPECM_A::_111
    }
}
#[doc = "Field `SPDRF` reader - SPI Receive Data Ready Flag"]
pub type SPDRF_R = crate::BitReader<SPDRF_A>;
#[doc = "SPI Receive Data Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDRF_A {
    #[doc = "0: Receive data ready not detected"]
    _0 = 0,
    #[doc = "1: Receive data ready detected"]
    _1 = 1,
}
impl From<SPDRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDRF_A {
        match self.bits {
            false => SPDRF_A::_0,
            true => SPDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDRF_A::_1
    }
}
#[doc = "Field `OVRF` reader - Overrun Error Flag"]
pub type OVRF_R = crate::BitReader<OVRF_A>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRF_A {
    #[doc = "0: No overrun error is present."]
    _0 = 0,
    #[doc = "1: An overrun error is present."]
    _1 = 1,
}
impl From<OVRF_A> for bool {
    #[inline(always)]
    fn from(variant: OVRF_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRF_A {
        match self.bits {
            false => OVRF_A::_0,
            true => OVRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRF_A::_1
    }
}
#[doc = "Field `IDLNF` reader - SPI Idle Flag"]
pub type IDLNF_R = crate::BitReader<IDLNF_A>;
#[doc = "SPI Idle Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLNF_A {
    #[doc = "0: The SPI is in the idle state."]
    _0 = 0,
    #[doc = "1: The SPI is in the transfer state."]
    _1 = 1,
}
impl From<IDLNF_A> for bool {
    #[inline(always)]
    fn from(variant: IDLNF_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLNF_A {
        match self.bits {
            false => IDLNF_A::_0,
            true => IDLNF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDLNF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDLNF_A::_1
    }
}
#[doc = "Field `MODF` reader - Mode Fault Error Flag"]
pub type MODF_R = crate::BitReader<MODF_A>;
#[doc = "Mode Fault Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF_A {
    #[doc = "0: Neither mode fault error nor underrun error is present."]
    _0 = 0,
    #[doc = "1: A mode fault error or underrun error is present."]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODF_A::_1
    }
}
#[doc = "Field `PERF` reader - Parity Error Flag"]
pub type PERF_R = crate::BitReader<PERF_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERF_A {
    #[doc = "0: No parity error is present."]
    _0 = 0,
    #[doc = "1: A parity error is present."]
    _1 = 1,
}
impl From<PERF_A> for bool {
    #[inline(always)]
    fn from(variant: PERF_A) -> Self {
        variant as u8 != 0
    }
}
impl PERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERF_A {
        match self.bits {
            false => PERF_A::_0,
            true => PERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PERF_A::_1
    }
}
#[doc = "Field `UDRF` reader - Underrun Error Flag"]
pub type UDRF_R = crate::BitReader<UDRF_A>;
#[doc = "Underrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRF_A {
    #[doc = "0: When MODF=0, neither mode fault error nor underrun error is present. When MODF=1, a mode fault error is present."]
    _0 = 0,
    #[doc = "1: When MODF=0, neither mode fault error nor underrun error is present. When MODF=1, an underrun error is present."]
    _1 = 1,
}
impl From<UDRF_A> for bool {
    #[inline(always)]
    fn from(variant: UDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl UDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDRF_A {
        match self.bits {
            false => UDRF_A::_0,
            true => UDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDRF_A::_1
    }
}
#[doc = "Field `SPTEF` reader - SPI Transmit Buffer Empty Flag"]
pub type SPTEF_R = crate::BitReader<SPTEF_A>;
#[doc = "SPI Transmit Buffer Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPTEF_A {
    #[doc = "0: The number of empty stages in the transmit FIFO â\u{89}¤ the value set in SPDCR2.TTRG"]
    _0 = 0,
    #[doc = "1: The number of empty stages in the transmit FIFO > the value set in SPDCR2.TTRG"]
    _1 = 1,
}
impl From<SPTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SPTEF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPTEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTEF_A {
        match self.bits {
            false => SPTEF_A::_0,
            true => SPTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTEF_A::_1
    }
}
#[doc = "Field `CENDF` reader - Communication End Flag"]
pub type CENDF_R = crate::BitReader<CENDF_A>;
#[doc = "Communication End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENDF_A {
    #[doc = "0: The SPI is not communicating or communicating."]
    _0 = 0,
    #[doc = "1: The SPI communication completed."]
    _1 = 1,
}
impl From<CENDF_A> for bool {
    #[inline(always)]
    fn from(variant: CENDF_A) -> Self {
        variant as u8 != 0
    }
}
impl CENDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CENDF_A {
        match self.bits {
            false => CENDF_A::_0,
            true => CENDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CENDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CENDF_A::_1
    }
}
#[doc = "Field `SPRF` reader - SPI Receive Buffer Full Flag"]
pub type SPRF_R = crate::BitReader<SPRF_A>;
#[doc = "SPI Receive Buffer Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRF_A {
    #[doc = "0: The number of data stored in the receive FIFO â\u{89}¤ number of frames set by the SPDCR2.RTRG bit."]
    _0 = 0,
    #[doc = "1: The number of data stored in the receive FIFO > number of frames set by the SPDCR2.RTRG bit."]
    _1 = 1,
}
impl From<SPRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPRF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRF_A {
        match self.bits {
            false => SPRF_A::_0,
            true => SPRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRF_A::_1
    }
}
impl R {
    #[doc = "Bits 8:10 - SPI Command Pointer"]
    #[inline(always)]
    pub fn spcp(&self) -> SPCP_R {
        SPCP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI Error Command"]
    #[inline(always)]
    pub fn specm(&self) -> SPECM_R {
        SPECM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 23 - SPI Receive Data Ready Flag"]
    #[inline(always)]
    pub fn spdrf(&self) -> SPDRF_R {
        SPDRF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Overrun Error Flag"]
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SPI Idle Flag"]
    #[inline(always)]
    pub fn idlnf(&self) -> IDLNF_R {
        IDLNF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mode Fault Error Flag"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Parity Error Flag"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Underrun Error Flag"]
    #[inline(always)]
    pub fn udrf(&self) -> UDRF_R {
        UDRF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    pub fn sptef(&self) -> SPTEF_R {
        SPTEF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Communication End Flag"]
    #[inline(always)]
    pub fn cendf(&self) -> CENDF_R {
        CENDF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn sprf(&self) -> SPRF_R {
        SPRF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spsr](index.html) module"]
pub struct SPSR_SPEC;
impl crate::RegisterSpec for SPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spsr::R](R) reader structure"]
impl crate::Readable for SPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPSR to value 0x2000_0000"]
impl crate::Resettable for SPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
