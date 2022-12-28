#[doc = "Register `CCR1` reader"]
pub struct R(crate::R<CCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR1` writer"]
pub struct W(crate::W<CCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR1_SPEC>;
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
impl From<crate::W<CCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSE` reader - CTS Enable"]
pub type CTSE_R = crate::BitReader<CTSE_A>;
#[doc = "CTS Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE_A {
    #[doc = "0: CTS function is disabled (RTS output function is enabled)."]
    _0 = 0,
    #[doc = "1: CTS function is enabled."]
    _1 = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::_0,
            true => CTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSE_A::_1
    }
}
#[doc = "Field `CTSE` writer - CTS Enable"]
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, CTSE_A, O>;
impl<'a, const O: u8> CTSE_W<'a, O> {
    #[doc = "CTS function is disabled (RTS output function is enabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSE_A::_0)
    }
    #[doc = "CTS function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSE_A::_1)
    }
}
#[doc = "Field `CTSPEN` reader - CTS external pin Enable"]
pub type CTSPEN_R = crate::BitReader<CTSPEN_A>;
#[doc = "CTS external pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSPEN_A {
    #[doc = "0: Alternate setting to use CTS and RTS functions as either one pin"]
    _0 = 0,
    #[doc = "1: Dedicated setting for separately using CTS and RTS functions with 2 pins"]
    _1 = 1,
}
impl From<CTSPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSPEN_A {
        match self.bits {
            false => CTSPEN_A::_0,
            true => CTSPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSPEN_A::_1
    }
}
#[doc = "Field `CTSPEN` writer - CTS external pin Enable"]
pub type CTSPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, CTSPEN_A, O>;
impl<'a, const O: u8> CTSPEN_W<'a, O> {
    #[doc = "Alternate setting to use CTS and RTS functions as either one pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSPEN_A::_0)
    }
    #[doc = "Dedicated setting for separately using CTS and RTS functions with 2 pins"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSPEN_A::_1)
    }
}
#[doc = "Field `SPB2DT` reader - Serial port break data select"]
pub type SPB2DT_R = crate::BitReader<SPB2DT_A>;
#[doc = "Serial port break data select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPB2DT_A {
    #[doc = "0: When TINV is 0, Low level is output in TXDn pin. When TINV is 1, High level is output in TXDn pin."]
    _0 = 0,
    #[doc = "1: When TINV is 0, High level is output in TXDn pin. When TINV is 1, Low level is output in TXDn pin."]
    _1 = 1,
}
impl From<SPB2DT_A> for bool {
    #[inline(always)]
    fn from(variant: SPB2DT_A) -> Self {
        variant as u8 != 0
    }
}
impl SPB2DT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPB2DT_A {
        match self.bits {
            false => SPB2DT_A::_0,
            true => SPB2DT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPB2DT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPB2DT_A::_1
    }
}
#[doc = "Field `SPB2DT` writer - Serial port break data select"]
pub type SPB2DT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, SPB2DT_A, O>;
impl<'a, const O: u8> SPB2DT_W<'a, O> {
    #[doc = "When TINV is 0, Low level is output in TXDn pin. When TINV is 1, High level is output in TXDn pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPB2DT_A::_0)
    }
    #[doc = "When TINV is 0, High level is output in TXDn pin. When TINV is 1, Low level is output in TXDn pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPB2DT_A::_1)
    }
}
#[doc = "Field `SPB2IO` reader - Serial port break I/O"]
pub type SPB2IO_R = crate::BitReader<SPB2IO_A>;
#[doc = "Serial port break I/O\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPB2IO_A {
    #[doc = "0: The value of SPB2DT bit is not output in TXDn pin."]
    _0 = 0,
    #[doc = "1: The value of SPB2DT bit is output in TXDn pin."]
    _1 = 1,
}
impl From<SPB2IO_A> for bool {
    #[inline(always)]
    fn from(variant: SPB2IO_A) -> Self {
        variant as u8 != 0
    }
}
impl SPB2IO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPB2IO_A {
        match self.bits {
            false => SPB2IO_A::_0,
            true => SPB2IO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPB2IO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPB2IO_A::_1
    }
}
#[doc = "Field `SPB2IO` writer - Serial port break I/O"]
pub type SPB2IO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, SPB2IO_A, O>;
impl<'a, const O: u8> SPB2IO_W<'a, O> {
    #[doc = "The value of SPB2DT bit is not output in TXDn pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPB2IO_A::_0)
    }
    #[doc = "The value of SPB2DT bit is output in TXDn pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPB2IO_A::_1)
    }
}
#[doc = "Field `PE` reader - Parity Enable"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: When transmitting: Do not add parity bit When receiving: Do not check parity bit"]
    _0 = 0,
    #[doc = "1: When transmitting: Add parity bit When receiving: Check parity bit"]
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
#[doc = "Field `PE` writer - Parity Enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, PE_A, O>;
impl<'a, const O: u8> PE_W<'a, O> {
    #[doc = "When transmitting: Do not add parity bit When receiving: Do not check parity bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PE_A::_0)
    }
    #[doc = "When transmitting: Add parity bit When receiving: Check parity bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PE_A::_1)
    }
}
#[doc = "Field `PM` reader - Parity Mode"]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: Selects even parity"]
    _0 = 0,
    #[doc = "1: Selects odd parity"]
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
#[doc = "Field `PM` writer - Parity Mode"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, PM_A, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    #[doc = "Selects even parity"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PM_A::_0)
    }
    #[doc = "Selects odd parity"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PM_A::_1)
    }
}
#[doc = "Field `TINV` reader - TXD invert"]
pub type TINV_R = crate::BitReader<TINV_A>;
#[doc = "TXD invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TINV_A {
    #[doc = "0: Transmit data is not inverted and output to TXDn."]
    _0 = 0,
    #[doc = "1: Transmit data is inverted and output to TXDn."]
    _1 = 1,
}
impl From<TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TINV_A) -> Self {
        variant as u8 != 0
    }
}
impl TINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TINV_A {
        match self.bits {
            false => TINV_A::_0,
            true => TINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TINV_A::_1
    }
}
#[doc = "Field `TINV` writer - TXD invert"]
pub type TINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, TINV_A, O>;
impl<'a, const O: u8> TINV_W<'a, O> {
    #[doc = "Transmit data is not inverted and output to TXDn."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TINV_A::_0)
    }
    #[doc = "Transmit data is inverted and output to TXDn."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TINV_A::_1)
    }
}
#[doc = "Field `RINV` reader - RXD invert"]
pub type RINV_R = crate::BitReader<RINV_A>;
#[doc = "RXD invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RINV_A {
    #[doc = "0: Received data from RXDn is not inverted and input."]
    _0 = 0,
    #[doc = "1: Received data from RXDn is inverted and input."]
    _1 = 1,
}
impl From<RINV_A> for bool {
    #[inline(always)]
    fn from(variant: RINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RINV_A {
        match self.bits {
            false => RINV_A::_0,
            true => RINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RINV_A::_1
    }
}
#[doc = "Field `RINV` writer - RXD invert"]
pub type RINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, RINV_A, O>;
impl<'a, const O: u8> RINV_W<'a, O> {
    #[doc = "Received data from RXDn is not inverted and input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RINV_A::_0)
    }
    #[doc = "Received data from RXDn is inverted and input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RINV_A::_1)
    }
}
#[doc = "Field `SPLP` reader - Loopback Control"]
pub type SPLP_R = crate::BitReader<SPLP_A>;
#[doc = "Loopback Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode"]
    _1 = 1,
}
impl From<SPLP_A> for bool {
    #[inline(always)]
    fn from(variant: SPLP_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLP_A {
        match self.bits {
            false => SPLP_A::_0,
            true => SPLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLP_A::_1
    }
}
#[doc = "Field `SPLP` writer - Loopback Control"]
pub type SPLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, SPLP_A, O>;
impl<'a, const O: u8> SPLP_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLP_A::_0)
    }
    #[doc = "Loopback mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLP_A::_1)
    }
}
#[doc = "Field `SHARPS` reader - Half-duplex communication select"]
pub type SHARPS_R = crate::BitReader<SHARPS_A>;
#[doc = "Half-duplex communication select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHARPS_A {
    #[doc = "0: TXDn pin, RXDn pin independent"]
    _0 = 0,
    #[doc = "1: TXDn / RXDn pin combination use (Half-duplex communication using TXDn pin)"]
    _1 = 1,
}
impl From<SHARPS_A> for bool {
    #[inline(always)]
    fn from(variant: SHARPS_A) -> Self {
        variant as u8 != 0
    }
}
impl SHARPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHARPS_A {
        match self.bits {
            false => SHARPS_A::_0,
            true => SHARPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHARPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHARPS_A::_1
    }
}
#[doc = "Field `SHARPS` writer - Half-duplex communication select"]
pub type SHARPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, SHARPS_A, O>;
impl<'a, const O: u8> SHARPS_W<'a, O> {
    #[doc = "TXDn pin, RXDn pin independent"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHARPS_A::_0)
    }
    #[doc = "TXDn / RXDn pin combination use (Half-duplex communication using TXDn pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHARPS_A::_1)
    }
}
#[doc = "Field `NFCS` reader - Noise Filter Clock Select"]
pub type NFCS_R = crate::FieldReader<u8, NFCS_A>;
#[doc = "Noise Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    #[doc = "0: The base clock signal divided by 1."]
    _000 = 0,
    #[doc = "1: The on-chip baud rate generator source clock divided by 1."]
    _001 = 1,
    #[doc = "2: The on-chip baud rate generator source clock divided by 2."]
    _010 = 2,
    #[doc = "3: The on-chip baud rate generator source clock divided by 4."]
    _011 = 3,
    #[doc = "4: The on-chip baud rate generator source clock divided by 8."]
    _100 = 4,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl NFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NFCS_A> {
        match self.bits {
            0 => Some(NFCS_A::_000),
            1 => Some(NFCS_A::_001),
            2 => Some(NFCS_A::_010),
            3 => Some(NFCS_A::_011),
            4 => Some(NFCS_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == NFCS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == NFCS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == NFCS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == NFCS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == NFCS_A::_100
    }
}
#[doc = "Field `NFCS` writer - Noise Filter Clock Select"]
pub type NFCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR1_SPEC, u8, NFCS_A, 3, O>;
impl<'a, const O: u8> NFCS_W<'a, O> {
    #[doc = "The base clock signal divided by 1."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(NFCS_A::_000)
    }
    #[doc = "The on-chip baud rate generator source clock divided by 1."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(NFCS_A::_001)
    }
    #[doc = "The on-chip baud rate generator source clock divided by 2."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(NFCS_A::_010)
    }
    #[doc = "The on-chip baud rate generator source clock divided by 4."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(NFCS_A::_011)
    }
    #[doc = "The on-chip baud rate generator source clock divided by 8."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(NFCS_A::_100)
    }
}
#[doc = "Field `NFEN` reader - Digital Noise Filter Function Enable"]
pub type NFEN_R = crate::BitReader<NFEN_A>;
#[doc = "Digital Noise Filter Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    #[doc = "0: In Asynchronous, Manchester, Simple LIN mode: Disable noise cancellation function for RXDn input signal In Simple IIC mode: Disable noise cancellation function for SCLn and SDAn input signals"]
    _0 = 0,
    #[doc = "1: In Asynchronous, Manchester, Simple LIN mode: Enable noise cancellation function for RXDn input signal In Simple IIC mode: Enable noise cancellation function for SCLn and SDAn input signals"]
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
#[doc = "Field `NFEN` writer - Digital Noise Filter Function Enable"]
pub type NFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1_SPEC, NFEN_A, O>;
impl<'a, const O: u8> NFEN_W<'a, O> {
    #[doc = "In Asynchronous, Manchester, Simple LIN mode: Disable noise cancellation function for RXDn input signal In Simple IIC mode: Disable noise cancellation function for SCLn and SDAn input signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFEN_A::_0)
    }
    #[doc = "In Asynchronous, Manchester, Simple LIN mode: Enable noise cancellation function for RXDn input signal In Simple IIC mode: Enable noise cancellation function for SCLn and SDAn input signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTS Enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS external pin Enable"]
    #[inline(always)]
    pub fn ctspen(&self) -> CTSPEN_R {
        CTSPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial port break data select"]
    #[inline(always)]
    pub fn spb2dt(&self) -> SPB2DT_R {
        SPB2DT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serial port break I/O"]
    #[inline(always)]
    pub fn spb2io(&self) -> SPB2IO_R {
        SPB2IO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - TXD invert"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXD invert"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Loopback Control"]
    #[inline(always)]
    pub fn splp(&self) -> SPLP_R {
        SPLP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Half-duplex communication select"]
    #[inline(always)]
    pub fn sharps(&self) -> SHARPS_R {
        SHARPS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<0> {
        CTSE_W::new(self)
    }
    #[doc = "Bit 1 - CTS external pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctspen(&mut self) -> CTSPEN_W<1> {
        CTSPEN_W::new(self)
    }
    #[doc = "Bit 4 - Serial port break data select"]
    #[inline(always)]
    #[must_use]
    pub fn spb2dt(&mut self) -> SPB2DT_W<4> {
        SPB2DT_W::new(self)
    }
    #[doc = "Bit 5 - Serial port break I/O"]
    #[inline(always)]
    #[must_use]
    pub fn spb2io(&mut self) -> SPB2IO_W<5> {
        SPB2IO_W::new(self)
    }
    #[doc = "Bit 8 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<8> {
        PE_W::new(self)
    }
    #[doc = "Bit 9 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<9> {
        PM_W::new(self)
    }
    #[doc = "Bit 12 - TXD invert"]
    #[inline(always)]
    #[must_use]
    pub fn tinv(&mut self) -> TINV_W<12> {
        TINV_W::new(self)
    }
    #[doc = "Bit 13 - RXD invert"]
    #[inline(always)]
    #[must_use]
    pub fn rinv(&mut self) -> RINV_W<13> {
        RINV_W::new(self)
    }
    #[doc = "Bit 16 - Loopback Control"]
    #[inline(always)]
    #[must_use]
    pub fn splp(&mut self) -> SPLP_W<16> {
        SPLP_W::new(self)
    }
    #[doc = "Bit 20 - Half-duplex communication select"]
    #[inline(always)]
    #[must_use]
    pub fn sharps(&mut self) -> SHARPS_W<20> {
        SHARPS_W::new(self)
    }
    #[doc = "Bits 24:26 - Noise Filter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcs(&mut self) -> NFCS_W<24> {
        NFCS_W::new(self)
    }
    #[doc = "Bit 28 - Digital Noise Filter Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfen(&mut self) -> NFEN_W<28> {
        NFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1](index.html) module"]
pub struct CCR1_SPEC;
impl crate::RegisterSpec for CCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr1::R](R) reader structure"]
impl crate::Readable for CCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr1::W](W) writer structure"]
impl crate::Writable for CCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR1 to value 0x10"]
impl crate::Resettable for CCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
