#[doc = "Register `EESIPR` reader"]
pub struct R(crate::R<EESIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EESIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EESIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EESIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EESIPR` writer"]
pub struct W(crate::W<EESIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EESIPR_SPEC>;
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
impl From<crate::W<EESIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EESIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVERIP` reader - PTP v2 Packet Receive Interrupt Request Enable"]
pub type PVERIP_R = crate::BitReader<PVERIP_A>;
#[doc = "PTP v2 Packet Receive Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVERIP_A {
    #[doc = "0: PTP v2 packet receive interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: PTP v2 packet receive interrupt request is enabled."]
    _1 = 1,
}
impl From<PVERIP_A> for bool {
    #[inline(always)]
    fn from(variant: PVERIP_A) -> Self {
        variant as u8 != 0
    }
}
impl PVERIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVERIP_A {
        match self.bits {
            false => PVERIP_A::_0,
            true => PVERIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PVERIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PVERIP_A::_1
    }
}
#[doc = "Field `PVERIP` writer - PTP v2 Packet Receive Interrupt Request Enable"]
pub type PVERIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, PVERIP_A, O>;
impl<'a, const O: u8> PVERIP_W<'a, O> {
    #[doc = "PTP v2 packet receive interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PVERIP_A::_0)
    }
    #[doc = "PTP v2 packet receive interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PVERIP_A::_1)
    }
}
#[doc = "Field `MACEIP` reader - MAC Address Mismatch Interrupt Request Enable"]
pub type MACEIP_R = crate::BitReader<MACEIP_A>;
#[doc = "MAC Address Mismatch Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MACEIP_A {
    #[doc = "0: This bit disables an interrupt request generated when the source MAC address of transmit frame data does not match the set value."]
    _0 = 0,
    #[doc = "1: This bit enables an interrupt request generated when the source MAC address of transmit frame data does not match the set value."]
    _1 = 1,
}
impl From<MACEIP_A> for bool {
    #[inline(always)]
    fn from(variant: MACEIP_A) -> Self {
        variant as u8 != 0
    }
}
impl MACEIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MACEIP_A {
        match self.bits {
            false => MACEIP_A::_0,
            true => MACEIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MACEIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MACEIP_A::_1
    }
}
#[doc = "Field `MACEIP` writer - MAC Address Mismatch Interrupt Request Enable"]
pub type MACEIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, MACEIP_A, O>;
impl<'a, const O: u8> MACEIP_W<'a, O> {
    #[doc = "This bit disables an interrupt request generated when the source MAC address of transmit frame data does not match the set value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MACEIP_A::_0)
    }
    #[doc = "This bit enables an interrupt request generated when the source MAC address of transmit frame data does not match the set value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MACEIP_A::_1)
    }
}
#[doc = "Field `RFOFIP` reader - Receive FIFO Overflow Interrupt Request Enable"]
pub type RFOFIP_R = crate::BitReader<RFOFIP_A>;
#[doc = "Receive FIFO Overflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOFIP_A {
    #[doc = "0: Overflow interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Overflow interrupt request is enabled."]
    _1 = 1,
}
impl From<RFOFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RFOFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOFIP_A {
        match self.bits {
            false => RFOFIP_A::_0,
            true => RFOFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOFIP_A::_1
    }
}
#[doc = "Field `RFOFIP` writer - Receive FIFO Overflow Interrupt Request Enable"]
pub type RFOFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RFOFIP_A, O>;
impl<'a, const O: u8> RFOFIP_W<'a, O> {
    #[doc = "Overflow interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOFIP_A::_0)
    }
    #[doc = "Overflow interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOFIP_A::_1)
    }
}
#[doc = "Field `RDEIP` reader - Receive Descriptor Empty Interrupt Request Enable"]
pub type RDEIP_R = crate::BitReader<RDEIP_A>;
#[doc = "Receive Descriptor Empty Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDEIP_A {
    #[doc = "0: Receive descriptor empty interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Receive descriptor empty interrupt request is enabled."]
    _1 = 1,
}
impl From<RDEIP_A> for bool {
    #[inline(always)]
    fn from(variant: RDEIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RDEIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDEIP_A {
        match self.bits {
            false => RDEIP_A::_0,
            true => RDEIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDEIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDEIP_A::_1
    }
}
#[doc = "Field `RDEIP` writer - Receive Descriptor Empty Interrupt Request Enable"]
pub type RDEIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RDEIP_A, O>;
impl<'a, const O: u8> RDEIP_W<'a, O> {
    #[doc = "Receive descriptor empty interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDEIP_A::_0)
    }
    #[doc = "Receive descriptor empty interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDEIP_A::_1)
    }
}
#[doc = "Field `FRIP` reader - Frame Receive Interrupt Request Enable"]
pub type FRIP_R = crate::BitReader<FRIP_A>;
#[doc = "Frame Receive Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRIP_A {
    #[doc = "0: Frame receive interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Frame receive interrupt request is enabled."]
    _1 = 1,
}
impl From<FRIP_A> for bool {
    #[inline(always)]
    fn from(variant: FRIP_A) -> Self {
        variant as u8 != 0
    }
}
impl FRIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRIP_A {
        match self.bits {
            false => FRIP_A::_0,
            true => FRIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRIP_A::_1
    }
}
#[doc = "Field `FRIP` writer - Frame Receive Interrupt Request Enable"]
pub type FRIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, FRIP_A, O>;
impl<'a, const O: u8> FRIP_W<'a, O> {
    #[doc = "Frame receive interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRIP_A::_0)
    }
    #[doc = "Frame receive interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRIP_A::_1)
    }
}
#[doc = "Field `TFUFIP` reader - Transmit FIFO Underflow Interrupt Request Enable"]
pub type TFUFIP_R = crate::BitReader<TFUFIP_A>;
#[doc = "Transmit FIFO Underflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUFIP_A {
    #[doc = "0: Underflow interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Underflow interrupt request is enabled."]
    _1 = 1,
}
impl From<TFUFIP_A> for bool {
    #[inline(always)]
    fn from(variant: TFUFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TFUFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUFIP_A {
        match self.bits {
            false => TFUFIP_A::_0,
            true => TFUFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUFIP_A::_1
    }
}
#[doc = "Field `TFUFIP` writer - Transmit FIFO Underflow Interrupt Request Enable"]
pub type TFUFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TFUFIP_A, O>;
impl<'a, const O: u8> TFUFIP_W<'a, O> {
    #[doc = "Underflow interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUFIP_A::_0)
    }
    #[doc = "Underflow interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUFIP_A::_1)
    }
}
#[doc = "Field `TDEIP` reader - Transmit Descriptor Empty Interrupt Request Enable"]
pub type TDEIP_R = crate::BitReader<TDEIP_A>;
#[doc = "Transmit Descriptor Empty Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDEIP_A {
    #[doc = "0: Transmit descriptor empty interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Transmit descriptor empty interrupt request is enabled."]
    _1 = 1,
}
impl From<TDEIP_A> for bool {
    #[inline(always)]
    fn from(variant: TDEIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TDEIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDEIP_A {
        match self.bits {
            false => TDEIP_A::_0,
            true => TDEIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDEIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDEIP_A::_1
    }
}
#[doc = "Field `TDEIP` writer - Transmit Descriptor Empty Interrupt Request Enable"]
pub type TDEIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TDEIP_A, O>;
impl<'a, const O: u8> TDEIP_W<'a, O> {
    #[doc = "Transmit descriptor empty interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDEIP_A::_0)
    }
    #[doc = "Transmit descriptor empty interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDEIP_A::_1)
    }
}
#[doc = "Field `TCIP` reader - Frame Transfer Complete Interrupt Request Enable"]
pub type TCIP_R = crate::BitReader<TCIP_A>;
#[doc = "Frame Transfer Complete Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIP_A {
    #[doc = "0: Frame transmission complete interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Frame transmission complete interrupt request is enabled."]
    _1 = 1,
}
impl From<TCIP_A> for bool {
    #[inline(always)]
    fn from(variant: TCIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIP_A {
        match self.bits {
            false => TCIP_A::_0,
            true => TCIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCIP_A::_1
    }
}
#[doc = "Field `TCIP` writer - Frame Transfer Complete Interrupt Request Enable"]
pub type TCIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TCIP_A, O>;
impl<'a, const O: u8> TCIP_W<'a, O> {
    #[doc = "Frame transmission complete interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIP_A::_0)
    }
    #[doc = "Frame transmission complete interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIP_A::_1)
    }
}
#[doc = "Field `ADEIP` reader - Address Error Interrupt Request Enable"]
pub type ADEIP_R = crate::BitReader<ADEIP_A>;
#[doc = "Address Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEIP_A {
    #[doc = "0: Address error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Address error interrupt request is enabled."]
    _1 = 1,
}
impl From<ADEIP_A> for bool {
    #[inline(always)]
    fn from(variant: ADEIP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEIP_A {
        match self.bits {
            false => ADEIP_A::_0,
            true => ADEIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEIP_A::_1
    }
}
#[doc = "Field `ADEIP` writer - Address Error Interrupt Request Enable"]
pub type ADEIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, ADEIP_A, O>;
impl<'a, const O: u8> ADEIP_W<'a, O> {
    #[doc = "Address error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEIP_A::_0)
    }
    #[doc = "Address error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEIP_A::_1)
    }
}
#[doc = "Field `RFCOFIP` reader - Receive Frame Counter Overflow Interrupt Request Enable"]
pub type RFCOFIP_R = crate::BitReader<RFCOFIP_A>;
#[doc = "Receive Frame Counter Overflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCOFIP_A {
    #[doc = "0: Receive frame counter overflow interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Receive frame counter overflow interrupt request is enabled."]
    _1 = 1,
}
impl From<RFCOFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RFCOFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RFCOFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCOFIP_A {
        match self.bits {
            false => RFCOFIP_A::_0,
            true => RFCOFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFCOFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFCOFIP_A::_1
    }
}
#[doc = "Field `RFCOFIP` writer - Receive Frame Counter Overflow Interrupt Request Enable"]
pub type RFCOFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RFCOFIP_A, O>;
impl<'a, const O: u8> RFCOFIP_W<'a, O> {
    #[doc = "Receive frame counter overflow interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFCOFIP_A::_0)
    }
    #[doc = "Receive frame counter overflow interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFCOFIP_A::_1)
    }
}
#[doc = "Field `TABTIP` reader - Transmit Abort Detect Interrupt Request Enable"]
pub type TABTIP_R = crate::BitReader<TABTIP_A>;
#[doc = "Transmit Abort Detect Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABTIP_A {
    #[doc = "0: Transmit abort detect interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Transmit abort detect interrupt request is enabled."]
    _1 = 1,
}
impl From<TABTIP_A> for bool {
    #[inline(always)]
    fn from(variant: TABTIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TABTIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABTIP_A {
        match self.bits {
            false => TABTIP_A::_0,
            true => TABTIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABTIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABTIP_A::_1
    }
}
#[doc = "Field `TABTIP` writer - Transmit Abort Detect Interrupt Request Enable"]
pub type TABTIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TABTIP_A, O>;
impl<'a, const O: u8> TABTIP_W<'a, O> {
    #[doc = "Transmit abort detect interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABTIP_A::_0)
    }
    #[doc = "Transmit abort detect interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABTIP_A::_1)
    }
}
#[doc = "Field `TWBIP` reader - Write-Back Complete Interrupt Request Enable"]
pub type TWBIP_R = crate::BitReader<TWBIP_A>;
#[doc = "Write-Back Complete Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWBIP_A {
    #[doc = "0: Write-back complete interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Write-back complete interrupt request is enabled."]
    _1 = 1,
}
impl From<TWBIP_A> for bool {
    #[inline(always)]
    fn from(variant: TWBIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TWBIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWBIP_A {
        match self.bits {
            false => TWBIP_A::_0,
            true => TWBIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWBIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWBIP_A::_1
    }
}
#[doc = "Field `TWBIP` writer - Write-Back Complete Interrupt Request Enable"]
pub type TWBIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TWBIP_A, O>;
impl<'a, const O: u8> TWBIP_W<'a, O> {
    #[doc = "Write-back complete interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWBIP_A::_0)
    }
    #[doc = "Write-back complete interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWBIP_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - PTP v2 Packet Receive Interrupt Request Enable"]
    #[inline(always)]
    pub fn pverip(&self) -> PVERIP_R {
        PVERIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MAC Address Mismatch Interrupt Request Enable"]
    #[inline(always)]
    pub fn maceip(&self) -> MACEIP_R {
        MACEIP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn rfofip(&self) -> RFOFIP_R {
        RFOFIP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn rdeip(&self) -> RDEIP_R {
        RDEIP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    pub fn frip(&self) -> FRIP_R {
        FRIP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn tfufip(&self) -> TFUFIP_R {
        TFUFIP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn tdeip(&self) -> TDEIP_R {
        TDEIP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Interrupt Request Enable"]
    #[inline(always)]
    pub fn tcip(&self) -> TCIP_R {
        TCIP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Address Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn adeip(&self) -> ADEIP_R {
        ADEIP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn rfcofip(&self) -> RFCOFIP_R {
        RFCOFIP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn tabtip(&self) -> TABTIP_R {
        TABTIP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Write-Back Complete Interrupt Request Enable"]
    #[inline(always)]
    pub fn twbip(&self) -> TWBIP_R {
        TWBIP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PTP v2 Packet Receive Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pverip(&mut self) -> PVERIP_W<4> {
        PVERIP_W::new(self)
    }
    #[doc = "Bit 8 - MAC Address Mismatch Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn maceip(&mut self) -> MACEIP_W<8> {
        MACEIP_W::new(self)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfofip(&mut self) -> RFOFIP_W<16> {
        RFOFIP_W::new(self)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdeip(&mut self) -> RDEIP_W<17> {
        RDEIP_W::new(self)
    }
    #[doc = "Bit 18 - Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frip(&mut self) -> FRIP_W<18> {
        FRIP_W::new(self)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfufip(&mut self) -> TFUFIP_W<19> {
        TFUFIP_W::new(self)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdeip(&mut self) -> TDEIP_W<20> {
        TDEIP_W::new(self)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcip(&mut self) -> TCIP_W<21> {
        TCIP_W::new(self)
    }
    #[doc = "Bit 23 - Address Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adeip(&mut self) -> ADEIP_W<23> {
        ADEIP_W::new(self)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfcofip(&mut self) -> RFCOFIP_W<24> {
        RFCOFIP_W::new(self)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tabtip(&mut self) -> TABTIP_W<26> {
        TABTIP_W::new(self)
    }
    #[doc = "Bit 30 - Write-Back Complete Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twbip(&mut self) -> TWBIP_W<30> {
        TWBIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP/EDMAC Status Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eesipr](index.html) module"]
pub struct EESIPR_SPEC;
impl crate::RegisterSpec for EESIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eesipr::R](R) reader structure"]
impl crate::Readable for EESIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eesipr::W](W) writer structure"]
impl crate::Writable for EESIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EESIPR to value 0"]
impl crate::Resettable for EESIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
