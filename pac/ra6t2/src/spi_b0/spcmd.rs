#[doc = "Register `SPCMD%s` reader"]
pub struct R(crate::R<SPCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCMD%s` writer"]
pub struct W(crate::W<SPCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCMD_SPEC>;
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
impl From<crate::W<SPCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPHA` reader - RSPCK Phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "RSPCK Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Data is sampled at an odd edge and changes at an even edge."]
    _0 = 0,
    #[doc = "1: Data changes at an odd edge and is sampled at an even edge."]
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
#[doc = "Field `CPHA` writer - RSPCK Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCMD_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Data is sampled at an odd edge and changes at an even edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data changes at an odd edge and is sampled at an even edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
    }
}
#[doc = "Field `CPOL` reader - RSPCK Polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "RSPCK Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: RSPCK in idle state is 0."]
    _0 = 0,
    #[doc = "1: RSPCK in idle state is 1."]
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
#[doc = "Field `CPOL` writer - RSPCK Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCMD_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "RSPCK in idle state is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "RSPCK in idle state is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
    }
}
#[doc = "Field `BRDV` reader - Bit Rate Division"]
pub type BRDV_R = crate::FieldReader<u8, BRDV_A>;
#[doc = "Bit Rate Division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRDV_A {
    #[doc = "0: The base bit rate is selected."]
    _00 = 0,
    #[doc = "1: Two-divided base bit rate is selected."]
    _01 = 1,
    #[doc = "2: Four-divided base bit rate is selected."]
    _10 = 2,
    #[doc = "3: Eight-divided base bit rate is selected."]
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
#[doc = "Field `BRDV` writer - Bit Rate Division"]
pub type BRDV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPCMD_SPEC, u8, BRDV_A, 2, O>;
impl<'a, const O: u8> BRDV_W<'a, O> {
    #[doc = "The base bit rate is selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BRDV_A::_00)
    }
    #[doc = "Two-divided base bit rate is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BRDV_A::_01)
    }
    #[doc = "Four-divided base bit rate is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BRDV_A::_10)
    }
    #[doc = "Eight-divided base bit rate is selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BRDV_A::_11)
    }
}
#[doc = "Field `SSLKP` reader - SSL Signal Level Hold"]
pub type SSLKP_R = crate::BitReader<SSLKP_A>;
#[doc = "SSL Signal Level Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSLKP_A {
    #[doc = "0: All SSL signals are negated at the end of transfer."]
    _0 = 0,
    #[doc = "1: SSL signal level is held after the transfer ends until the next access starts."]
    _1 = 1,
}
impl From<SSLKP_A> for bool {
    #[inline(always)]
    fn from(variant: SSLKP_A) -> Self {
        variant as u8 != 0
    }
}
impl SSLKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSLKP_A {
        match self.bits {
            false => SSLKP_A::_0,
            true => SSLKP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSLKP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSLKP_A::_1
    }
}
#[doc = "Field `SSLKP` writer - SSL Signal Level Hold"]
pub type SSLKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCMD_SPEC, SSLKP_A, O>;
impl<'a, const O: u8> SSLKP_W<'a, O> {
    #[doc = "All SSL signals are negated at the end of transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSLKP_A::_0)
    }
    #[doc = "SSL signal level is held after the transfer ends until the next access starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSLKP_A::_1)
    }
}
#[doc = "Field `LSBF` reader - SPI LSB First"]
pub type LSBF_R = crate::BitReader<LSBF_A>;
#[doc = "SPI LSB First\n\nValue on reset: 0"]
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
#[doc = "Field `LSBF` writer - SPI LSB First"]
pub type LSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCMD_SPEC, LSBF_A, O>;
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
#[doc = "Field `SPNDEN` reader - SPI Next-Access Delay Enable"]
pub type SPNDEN_R = crate::BitReader<SPNDEN_A>;
#[doc = "SPI Next-Access Delay Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPNDEN_A {
    #[doc = "0: Next-access delay is 1RSPCK + 5TCLK"]
    _0 = 0,
    #[doc = "1: Next-access delay is the set value of the SPI next-access delay register (SPDECR.SPNDL)."]
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
pub type SPNDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCMD_SPEC, SPNDEN_A, O>;
impl<'a, const O: u8> SPNDEN_W<'a, O> {
    #[doc = "Next-access delay is 1RSPCK + 5TCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPNDEN_A::_0)
    }
    #[doc = "Next-access delay is the set value of the SPI next-access delay register (SPDECR.SPNDL)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPNDEN_A::_1)
    }
}
#[doc = "Field `SLNDEN` reader - SSL Negation Delay Setting Enable"]
pub type SLNDEN_R = crate::BitReader<SLNDEN_A>;
#[doc = "SSL Negation Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLNDEN_A {
    #[doc = "0: \\[Master\\]
SSL negation delay is 1RSPCK. \\[Slave in the TI-SSP\\]
SSL negation delay is 1TCLK"]
    _0 = 0,
    #[doc = "1: SSL negation delay is the set value of the slave select negation delay register (SPDECR.SLNDL)."]
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
pub type SLNDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCMD_SPEC, SLNDEN_A, O>;
impl<'a, const O: u8> SLNDEN_W<'a, O> {
    #[doc = "Master\\]
SSL negation delay is 1RSPCK. \\[Slave in the TI-SSP\\]
SSL negation delay is 1TCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLNDEN_A::_0)
    }
    #[doc = "SSL negation delay is the set value of the slave select negation delay register (SPDECR.SLNDL)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLNDEN_A::_1)
    }
}
#[doc = "Field `SCKDEN` reader - RSPCK Delay Setting Enable"]
pub type SCKDEN_R = crate::BitReader<SCKDEN_A>;
#[doc = "RSPCK Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKDEN_A {
    #[doc = "0: RSPCK delay is 1 RSPCK."]
    _0 = 0,
    #[doc = "1: RSPCK delay is the set value of the RSPCK delay register (SPDECR.SCKDL)."]
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
pub type SCKDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCMD_SPEC, SCKDEN_A, O>;
impl<'a, const O: u8> SCKDEN_W<'a, O> {
    #[doc = "RSPCK delay is 1 RSPCK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCKDEN_A::_0)
    }
    #[doc = "RSPCK delay is the set value of the RSPCK delay register (SPDECR.SCKDL)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCKDEN_A::_1)
    }
}
#[doc = "Field `SPB` reader - SPI Data Length"]
pub type SPB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPB` writer - SPI Data Length"]
pub type SPB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPCMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `SSLA` reader - SSL Signal Assertion"]
pub type SSLA_R = crate::FieldReader<u8, SSLA_A>;
#[doc = "SSL Signal Assertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `SSLA` writer - SSL Signal Assertion"]
pub type SSLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPCMD_SPEC, u8, SSLA_A, 3, O>;
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
impl R {
    #[doc = "Bit 0 - RSPCK Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSPCK Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Bit Rate Division"]
    #[inline(always)]
    pub fn brdv(&self) -> BRDV_R {
        BRDV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - SSL Signal Level Hold"]
    #[inline(always)]
    pub fn sslkp(&self) -> SSLKP_R {
        SSLKP_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bits 16:20 - SPI Data Length"]
    #[inline(always)]
    pub fn spb(&self) -> SPB_R {
        SPB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - SSL Signal Assertion"]
    #[inline(always)]
    pub fn ssla(&self) -> SSLA_R {
        SSLA_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RSPCK Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - RSPCK Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    #[doc = "Bits 2:3 - Bit Rate Division"]
    #[inline(always)]
    #[must_use]
    pub fn brdv(&mut self) -> BRDV_W<2> {
        BRDV_W::new(self)
    }
    #[doc = "Bit 7 - SSL Signal Level Hold"]
    #[inline(always)]
    #[must_use]
    pub fn sslkp(&mut self) -> SSLKP_W<7> {
        SSLKP_W::new(self)
    }
    #[doc = "Bit 12 - SPI LSB First"]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LSBF_W<12> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 13 - SPI Next-Access Delay Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spnden(&mut self) -> SPNDEN_W<13> {
        SPNDEN_W::new(self)
    }
    #[doc = "Bit 14 - SSL Negation Delay Setting Enable"]
    #[inline(always)]
    #[must_use]
    pub fn slnden(&mut self) -> SLNDEN_W<14> {
        SLNDEN_W::new(self)
    }
    #[doc = "Bit 15 - RSPCK Delay Setting Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sckden(&mut self) -> SCKDEN_W<15> {
        SCKDEN_W::new(self)
    }
    #[doc = "Bits 16:20 - SPI Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn spb(&mut self) -> SPB_W<16> {
        SPB_W::new(self)
    }
    #[doc = "Bits 24:26 - SSL Signal Assertion"]
    #[inline(always)]
    #[must_use]
    pub fn ssla(&mut self) -> SSLA_W<24> {
        SSLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcmd](index.html) module"]
pub struct SPCMD_SPEC;
impl crate::RegisterSpec for SPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spcmd::R](R) reader structure"]
impl crate::Readable for SPCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcmd::W](W) writer structure"]
impl crate::Writable for SPCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCMD%s to value 0x0007_0000"]
impl crate::Resettable for SPCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0000;
}
