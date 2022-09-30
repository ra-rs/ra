#[doc = "Register `SPCMD0` reader"]
pub struct R(crate::R<SPCMD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCMD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCMD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCMD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCMD0` writer"]
pub struct W(crate::W<SPCMD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCMD0_SPEC>;
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
impl From<crate::W<SPCMD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCMD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPHA` reader - RSPCK Phase Setting"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "RSPCK Phase Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Select data sampling on leading edge, data change on trailing edge"]
    _0 = 0,
    #[doc = "1: Select data change on leading edge, data sampling on trailing edge"]
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
#[doc = "Field `CPHA` writer - RSPCK Phase Setting"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SPCMD0_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Select data sampling on leading edge, data change on trailing edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Select data change on leading edge, data sampling on trailing edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
    }
}
#[doc = "Field `CPOL` reader - RSPCK Polarity Setting"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "RSPCK Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Set RSPCK low during idle"]
    _0 = 0,
    #[doc = "1: Set RSPCK high during idle"]
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
#[doc = "Field `CPOL` writer - RSPCK Polarity Setting"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SPCMD0_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "Set RSPCK low during idle"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "Set RSPCK high during idle"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
    }
}
#[doc = "Field `BRDV` reader - Bit Rate Division Setting"]
pub type BRDV_R = crate::FieldReader<u8, BRDV_A>;
#[doc = "Bit Rate Division Setting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRDV_A {
    #[doc = "0: Base bit rate"]
    _00 = 0,
    #[doc = "1: Base bit rate divided by 2"]
    _01 = 1,
    #[doc = "2: Base bit rate divided by 4"]
    _10 = 2,
    #[doc = "3: Base bit rate divided by 8"]
    _11 = 3,
}
impl From<BRDV_A> for u8 {
    #[inline(always)]
    fn from(variant: BRDV_A) -> Self {
        variant as _
    }
}
impl BRDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDV_A {
        match self.bits {
            0 => BRDV_A::_00,
            1 => BRDV_A::_01,
            2 => BRDV_A::_10,
            3 => BRDV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BRDV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BRDV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BRDV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BRDV_A::_11
    }
}
#[doc = "Field `BRDV` writer - Bit Rate Division Setting"]
pub type BRDV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, SPCMD0_SPEC, u8, BRDV_A, 2, O>;
impl<'a, const O: u8> BRDV_W<'a, O> {
    #[doc = "Base bit rate"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BRDV_A::_00)
    }
    #[doc = "Base bit rate divided by 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BRDV_A::_01)
    }
    #[doc = "Base bit rate divided by 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BRDV_A::_10)
    }
    #[doc = "Base bit rate divided by 8"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BRDV_A::_11)
    }
}
#[doc = "Field `SSLA` reader - SSL Signal Assertion Setting"]
pub type SSLA_R = crate::FieldReader<u8, SSLA_A>;
#[doc = "SSL Signal Assertion Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSLA_A {
    #[doc = "0: SSL0"]
    _000 = 0,
    #[doc = "1: SSL1"]
    _001 = 1,
    #[doc = "2: SSL2"]
    _010 = 2,
    #[doc = "3: SSL3"]
    _011 = 3,
}
impl From<SSLA_A> for u8 {
    #[inline(always)]
    fn from(variant: SSLA_A) -> Self {
        variant as _
    }
}
impl SSLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSLA_A> {
        match self.bits {
            0 => Some(SSLA_A::_000),
            1 => Some(SSLA_A::_001),
            2 => Some(SSLA_A::_010),
            3 => Some(SSLA_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SSLA_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SSLA_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SSLA_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SSLA_A::_011
    }
}
#[doc = "Field `SSLA` writer - SSL Signal Assertion Setting"]
pub type SSLA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SPCMD0_SPEC, u8, SSLA_A, 3, O>;
impl<'a, const O: u8> SSLA_W<'a, O> {
    #[doc = "SSL0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SSLA_A::_000)
    }
    #[doc = "SSL1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SSLA_A::_001)
    }
    #[doc = "SSL2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SSLA_A::_010)
    }
    #[doc = "SSL3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SSLA_A::_011)
    }
}
#[doc = "Field `SPB` reader - SPI Data Length Setting"]
pub type SPB_R = crate::FieldReader<u8, SPB_A>;
#[doc = "SPI Data Length Setting\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPB_A {
    #[doc = "0: 20 bits"]
    _0X0 = 0,
    #[doc = "1: 24 bits"]
    _0X1 = 1,
    #[doc = "2: 32 bits"]
    _0X2 = 2,
    #[doc = "3: 32 bits"]
    _0X3 = 3,
    #[doc = "8: 9 bits"]
    _0X8 = 8,
    #[doc = "9: 10 bits"]
    _0X9 = 9,
    #[doc = "10: 11 bits"]
    _0X_A = 10,
    #[doc = "11: 12 bits"]
    _0X_B = 11,
    #[doc = "12: 13 bits"]
    _0X_C = 12,
    #[doc = "13: 14 bits"]
    _0X_D = 13,
    #[doc = "14: 15 bits"]
    _0X_E = 14,
    #[doc = "15: 16 bits"]
    _0X_F = 15,
}
impl From<SPB_A> for u8 {
    #[inline(always)]
    fn from(variant: SPB_A) -> Self {
        variant as _
    }
}
impl SPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPB_A> {
        match self.bits {
            0 => Some(SPB_A::_0X0),
            1 => Some(SPB_A::_0X1),
            2 => Some(SPB_A::_0X2),
            3 => Some(SPB_A::_0X3),
            8 => Some(SPB_A::_0X8),
            9 => Some(SPB_A::_0X9),
            10 => Some(SPB_A::_0X_A),
            11 => Some(SPB_A::_0X_B),
            12 => Some(SPB_A::_0X_C),
            13 => Some(SPB_A::_0X_D),
            14 => Some(SPB_A::_0X_E),
            15 => Some(SPB_A::_0X_F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == SPB_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == SPB_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == SPB_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == SPB_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == SPB_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == SPB_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == SPB_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == SPB_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == SPB_A::_0X_C
    }
    #[doc = "Checks if the value of the field is `_0X_D`"]
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == SPB_A::_0X_D
    }
    #[doc = "Checks if the value of the field is `_0X_E`"]
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == SPB_A::_0X_E
    }
    #[doc = "Checks if the value of the field is `_0X_F`"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == SPB_A::_0X_F
    }
}
#[doc = "Field `SPB` writer - SPI Data Length Setting"]
pub type SPB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SPCMD0_SPEC, u8, SPB_A, 4, O>;
impl<'a, const O: u8> SPB_W<'a, O> {
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(SPB_A::_0X0)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(SPB_A::_0X1)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(SPB_A::_0X2)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(SPB_A::_0X3)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(SPB_A::_0X8)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(SPB_A::_0X9)
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(SPB_A::_0X_A)
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(SPB_A::_0X_B)
    }
    #[doc = "13 bits"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(SPB_A::_0X_C)
    }
    #[doc = "14 bits"]
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut W {
        self.variant(SPB_A::_0X_D)
    }
    #[doc = "15 bits"]
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut W {
        self.variant(SPB_A::_0X_E)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut W {
        self.variant(SPB_A::_0X_F)
    }
}
#[doc = "Field `LSBF` reader - SPI LSB First"]
pub type LSBF_R = crate::BitReader<LSBF_A>;
#[doc = "SPI LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBF_A {
    #[doc = "0: MSB-first"]
    _0 = 0,
    #[doc = "1: LSB-first"]
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
#[doc = "Field `LSBF` writer - SPI LSB First"]
pub type LSBF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SPCMD0_SPEC, LSBF_A, O>;
impl<'a, const O: u8> LSBF_W<'a, O> {
    #[doc = "MSB-first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBF_A::_0)
    }
    #[doc = "LSB-first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBF_A::_1)
    }
}
#[doc = "Field `SPNDEN` reader - SPI Next-Access Delay Enable"]
pub type SPNDEN_R = crate::BitReader<SPNDEN_A>;
#[doc = "SPI Next-Access Delay Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPNDEN_A {
    #[doc = "0: Select next-access delay of 1 RSPCK + 2 PCLKB"]
    _0 = 0,
    #[doc = "1: Select next-access delay equal to the setting in the SPI Next-Access Delay Register (SPND)"]
    _1 = 1,
}
impl From<SPNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPNDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPNDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPNDEN_A {
        match self.bits {
            false => SPNDEN_A::_0,
            true => SPNDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPNDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPNDEN_A::_1
    }
}
#[doc = "Field `SPNDEN` writer - SPI Next-Access Delay Enable"]
pub type SPNDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SPCMD0_SPEC, SPNDEN_A, O>;
impl<'a, const O: u8> SPNDEN_W<'a, O> {
    #[doc = "Select next-access delay of 1 RSPCK + 2 PCLKB"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPNDEN_A::_0)
    }
    #[doc = "Select next-access delay equal to the setting in the SPI Next-Access Delay Register (SPND)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPNDEN_A::_1)
    }
}
#[doc = "Field `SLNDEN` reader - SSL Negation Delay Setting Enable"]
pub type SLNDEN_R = crate::BitReader<SLNDEN_A>;
#[doc = "SSL Negation Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLNDEN_A {
    #[doc = "0: Select SSL negation delay of 1 RSPCK"]
    _0 = 0,
    #[doc = "1: Select SSL negation delay equal to the setting in the SPI Slave Select Negation Delay Register (SSLND)"]
    _1 = 1,
}
impl From<SLNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLNDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SLNDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLNDEN_A {
        match self.bits {
            false => SLNDEN_A::_0,
            true => SLNDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLNDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLNDEN_A::_1
    }
}
#[doc = "Field `SLNDEN` writer - SSL Negation Delay Setting Enable"]
pub type SLNDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SPCMD0_SPEC, SLNDEN_A, O>;
impl<'a, const O: u8> SLNDEN_W<'a, O> {
    #[doc = "Select SSL negation delay of 1 RSPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLNDEN_A::_0)
    }
    #[doc = "Select SSL negation delay equal to the setting in the SPI Slave Select Negation Delay Register (SSLND)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLNDEN_A::_1)
    }
}
#[doc = "Field `SCKDEN` reader - RSPCK Delay Setting Enable"]
pub type SCKDEN_R = crate::BitReader<SCKDEN_A>;
#[doc = "RSPCK Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKDEN_A {
    #[doc = "0: Select RSPCK delay of 1 RSPCK"]
    _0 = 0,
    #[doc = "1: Select RSPCK delay equal to the setting in the SPI Clock Delay Register (SPCKD)"]
    _1 = 1,
}
impl From<SCKDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCKDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKDEN_A {
        match self.bits {
            false => SCKDEN_A::_0,
            true => SCKDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKDEN_A::_1
    }
}
#[doc = "Field `SCKDEN` writer - RSPCK Delay Setting Enable"]
pub type SCKDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SPCMD0_SPEC, SCKDEN_A, O>;
impl<'a, const O: u8> SCKDEN_W<'a, O> {
    #[doc = "Select RSPCK delay of 1 RSPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCKDEN_A::_0)
    }
    #[doc = "Select RSPCK delay equal to the setting in the SPI Clock Delay Register (SPCKD)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCKDEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RSPCK Phase Setting"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSPCK Polarity Setting"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Bit Rate Division Setting"]
    #[inline(always)]
    pub fn brdv(&self) -> BRDV_R {
        BRDV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - SSL Signal Assertion Setting"]
    #[inline(always)]
    pub fn ssla(&self) -> SSLA_R {
        SSLA_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - SPI Data Length Setting"]
    #[inline(always)]
    pub fn spb(&self) -> SPB_R {
        SPB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - SPI LSB First"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI Next-Access Delay Enable"]
    #[inline(always)]
    pub fn spnden(&self) -> SPNDEN_R {
        SPNDEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SSL Negation Delay Setting Enable"]
    #[inline(always)]
    pub fn slnden(&self) -> SLNDEN_R {
        SLNDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RSPCK Delay Setting Enable"]
    #[inline(always)]
    pub fn sckden(&self) -> SCKDEN_R {
        SCKDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RSPCK Phase Setting"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - RSPCK Polarity Setting"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    #[doc = "Bits 2:3 - Bit Rate Division Setting"]
    #[inline(always)]
    pub fn brdv(&mut self) -> BRDV_W<2> {
        BRDV_W::new(self)
    }
    #[doc = "Bits 4:6 - SSL Signal Assertion Setting"]
    #[inline(always)]
    pub fn ssla(&mut self) -> SSLA_W<4> {
        SSLA_W::new(self)
    }
    #[doc = "Bits 8:11 - SPI Data Length Setting"]
    #[inline(always)]
    pub fn spb(&mut self) -> SPB_W<8> {
        SPB_W::new(self)
    }
    #[doc = "Bit 12 - SPI LSB First"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W<12> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 13 - SPI Next-Access Delay Enable"]
    #[inline(always)]
    pub fn spnden(&mut self) -> SPNDEN_W<13> {
        SPNDEN_W::new(self)
    }
    #[doc = "Bit 14 - SSL Negation Delay Setting Enable"]
    #[inline(always)]
    pub fn slnden(&mut self) -> SLNDEN_W<14> {
        SLNDEN_W::new(self)
    }
    #[doc = "Bit 15 - RSPCK Delay Setting Enable"]
    #[inline(always)]
    pub fn sckden(&mut self) -> SCKDEN_W<15> {
        SCKDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Command Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcmd0](index.html) module"]
pub struct SPCMD0_SPEC;
impl crate::RegisterSpec for SPCMD0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spcmd0::R](R) reader structure"]
impl crate::Readable for SPCMD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcmd0::W](W) writer structure"]
impl crate::Writable for SPCMD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPCMD0 to value 0x070d"]
impl crate::Resettable for SPCMD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x070d
    }
}
