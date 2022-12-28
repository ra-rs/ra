#[doc = "Register `SSICR` reader"]
pub struct R(crate::R<SSICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSICR` writer"]
pub struct W(crate::W<SSICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSICR_SPEC>;
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
impl From<crate::W<SSICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REN` reader - Receive Enable"]
pub type REN_R = crate::BitReader<REN_A>;
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REN_A {
    #[doc = "0: Disables the receive operation."]
    _0 = 0,
    #[doc = "1: Enables the receive operation."]
    _1 = 1,
}
impl From<REN_A> for bool {
    #[inline(always)]
    fn from(variant: REN_A) -> Self {
        variant as u8 != 0
    }
}
impl REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REN_A {
        match self.bits {
            false => REN_A::_0,
            true => REN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REN_A::_1
    }
}
#[doc = "Field `REN` writer - Receive Enable"]
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, REN_A, O>;
impl<'a, const O: u8> REN_W<'a, O> {
    #[doc = "Disables the receive operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REN_A::_0)
    }
    #[doc = "Enables the receive operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REN_A::_1)
    }
}
#[doc = "Field `TEN` reader - Transmit Enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: Disables the transmit operation."]
    _0 = 0,
    #[doc = "1: Enables the transmit operation."]
    _1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::_0,
            true => TEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEN_A::_1
    }
}
#[doc = "Field `TEN` writer - Transmit Enable"]
pub type TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, TEN_A, O>;
impl<'a, const O: u8> TEN_W<'a, O> {
    #[doc = "Disables the transmit operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEN_A::_0)
    }
    #[doc = "Enables the transmit operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEN_A::_1)
    }
}
#[doc = "Field `MUEN` reader - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing."]
pub type MUEN_R = crate::BitReader<MUEN_A>;
#[doc = "Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUEN_A {
    #[doc = "0: This module is not muted."]
    _0 = 0,
    #[doc = "1: This module is muted."]
    _1 = 1,
}
impl From<MUEN_A> for bool {
    #[inline(always)]
    fn from(variant: MUEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUEN_A {
        match self.bits {
            false => MUEN_A::_0,
            true => MUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MUEN_A::_1
    }
}
#[doc = "Field `MUEN` writer - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing."]
pub type MUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, MUEN_A, O>;
impl<'a, const O: u8> MUEN_W<'a, O> {
    #[doc = "This module is not muted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUEN_A::_0)
    }
    #[doc = "This module is muted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUEN_A::_1)
    }
}
#[doc = "Field `CKDV` reader - Serial Oversampling Clock Division Ratio"]
pub type CKDV_R = crate::FieldReader<u8, CKDV_A>;
#[doc = "Serial Oversampling Clock Division Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKDV_A {
    #[doc = "0: CLK"]
    _0X0 = 0,
    #[doc = "1: CLK/2"]
    _0X1 = 1,
    #[doc = "2: CLK/4"]
    _0X2 = 2,
    #[doc = "3: CLK/8"]
    _0X3 = 3,
    #[doc = "4: CLK/16"]
    _0X4 = 4,
    #[doc = "5: CLK/32"]
    _0X5 = 5,
    #[doc = "6: CLK/64"]
    _0X6 = 6,
    #[doc = "7: CLK/128"]
    _0X7 = 7,
    #[doc = "8: CLK/6"]
    _0X8 = 8,
    #[doc = "9: CLK/12 (These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
    _0X9 = 9,
    #[doc = "10: CLK/24"]
    _0X_A = 10,
    #[doc = "11: CLK/48(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
    _0X_B = 11,
    #[doc = "12: CLK/96(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
    _0X_C = 12,
}
impl From<CKDV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKDV_A) -> Self {
        variant as _
    }
}
impl CKDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKDV_A> {
        match self.bits {
            0 => Some(CKDV_A::_0X0),
            1 => Some(CKDV_A::_0X1),
            2 => Some(CKDV_A::_0X2),
            3 => Some(CKDV_A::_0X3),
            4 => Some(CKDV_A::_0X4),
            5 => Some(CKDV_A::_0X5),
            6 => Some(CKDV_A::_0X6),
            7 => Some(CKDV_A::_0X7),
            8 => Some(CKDV_A::_0X8),
            9 => Some(CKDV_A::_0X9),
            10 => Some(CKDV_A::_0X_A),
            11 => Some(CKDV_A::_0X_B),
            12 => Some(CKDV_A::_0X_C),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CKDV_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == CKDV_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == CKDV_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == CKDV_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == CKDV_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == CKDV_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == CKDV_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == CKDV_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == CKDV_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == CKDV_A::_0X9
    }
    #[doc = "Checks if the value of the field is `_0X_A`"]
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == CKDV_A::_0X_A
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == CKDV_A::_0X_B
    }
    #[doc = "Checks if the value of the field is `_0X_C`"]
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == CKDV_A::_0X_C
    }
}
#[doc = "Field `CKDV` writer - Serial Oversampling Clock Division Ratio"]
pub type CKDV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICR_SPEC, u8, CKDV_A, 4, O>;
impl<'a, const O: u8> CKDV_W<'a, O> {
    #[doc = "CLK"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CKDV_A::_0X0)
    }
    #[doc = "CLK/2"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(CKDV_A::_0X1)
    }
    #[doc = "CLK/4"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(CKDV_A::_0X2)
    }
    #[doc = "CLK/8"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(CKDV_A::_0X3)
    }
    #[doc = "CLK/16"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(CKDV_A::_0X4)
    }
    #[doc = "CLK/32"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(CKDV_A::_0X5)
    }
    #[doc = "CLK/64"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(CKDV_A::_0X6)
    }
    #[doc = "CLK/128"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(CKDV_A::_0X7)
    }
    #[doc = "CLK/6"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(CKDV_A::_0X8)
    }
    #[doc = "CLK/12 (These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(CKDV_A::_0X9)
    }
    #[doc = "CLK/24"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(CKDV_A::_0X_A)
    }
    #[doc = "CLK/48(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(CKDV_A::_0X_B)
    }
    #[doc = "CLK/96(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(CKDV_A::_0X_C)
    }
}
#[doc = "Field `DEL` reader - Serial Data Delay"]
pub type DEL_R = crate::BitReader<DEL_A>;
#[doc = "Serial Data Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEL_A {
    #[doc = "0: 1 clock cycle delay between SSIWS and SSIDATA"]
    _0 = 0,
    #[doc = "1: No delay between SSIWS and SSIDATA"]
    _1 = 1,
}
impl From<DEL_A> for bool {
    #[inline(always)]
    fn from(variant: DEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEL_A {
        match self.bits {
            false => DEL_A::_0,
            true => DEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEL_A::_1
    }
}
#[doc = "Field `DEL` writer - Serial Data Delay"]
pub type DEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, DEL_A, O>;
impl<'a, const O: u8> DEL_W<'a, O> {
    #[doc = "1 clock cycle delay between SSIWS and SSIDATA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEL_A::_0)
    }
    #[doc = "No delay between SSIWS and SSIDATA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEL_A::_1)
    }
}
#[doc = "Field `PDTA` reader - Parallel Data Alignment"]
pub type PDTA_R = crate::BitReader<PDTA_A>;
#[doc = "Parallel Data Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDTA_A {
    #[doc = "0: The lower bits of parallel data (SSITDR, SSIRDR) are transferred prior to the upper bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is left-aligned.(When data word length is 18, 20, 22, or 24 bits)"]
    _0 = 0,
    #[doc = "1: The upper bits of parallel data (SSITDR, SSIRDR) are transferred prior to the lower bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is right-aligned.(When data word length is 18, 20, 22, or 24 bits)"]
    _1 = 1,
}
impl From<PDTA_A> for bool {
    #[inline(always)]
    fn from(variant: PDTA_A) -> Self {
        variant as u8 != 0
    }
}
impl PDTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDTA_A {
        match self.bits {
            false => PDTA_A::_0,
            true => PDTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDTA_A::_1
    }
}
#[doc = "Field `PDTA` writer - Parallel Data Alignment"]
pub type PDTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, PDTA_A, O>;
impl<'a, const O: u8> PDTA_W<'a, O> {
    #[doc = "The lower bits of parallel data (SSITDR, SSIRDR) are transferred prior to the upper bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is left-aligned.(When data word length is 18, 20, 22, or 24 bits)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDTA_A::_0)
    }
    #[doc = "The upper bits of parallel data (SSITDR, SSIRDR) are transferred prior to the lower bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is right-aligned.(When data word length is 18, 20, 22, or 24 bits)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDTA_A::_1)
    }
}
#[doc = "Field `SDTA` reader - Serial Data Alignment"]
pub type SDTA_R = crate::BitReader<SDTA_A>;
#[doc = "Serial Data Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTA_A {
    #[doc = "0: Transmitting and receiving in the order of serial data and padding bits"]
    _0 = 0,
    #[doc = "1: Transmitting and receiving in the order of padding bits and serial data"]
    _1 = 1,
}
impl From<SDTA_A> for bool {
    #[inline(always)]
    fn from(variant: SDTA_A) -> Self {
        variant as u8 != 0
    }
}
impl SDTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDTA_A {
        match self.bits {
            false => SDTA_A::_0,
            true => SDTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDTA_A::_1
    }
}
#[doc = "Field `SDTA` writer - Serial Data Alignment"]
pub type SDTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, SDTA_A, O>;
impl<'a, const O: u8> SDTA_W<'a, O> {
    #[doc = "Transmitting and receiving in the order of serial data and padding bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDTA_A::_0)
    }
    #[doc = "Transmitting and receiving in the order of padding bits and serial data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDTA_A::_1)
    }
}
#[doc = "Field `SPDP` reader - Serial Padding Polarity"]
pub type SPDP_R = crate::BitReader<SPDP_A>;
#[doc = "Serial Padding Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDP_A {
    #[doc = "0: Padding bits are low."]
    _0 = 0,
    #[doc = "1: Padding bits are high."]
    _1 = 1,
}
impl From<SPDP_A> for bool {
    #[inline(always)]
    fn from(variant: SPDP_A) -> Self {
        variant as u8 != 0
    }
}
impl SPDP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDP_A {
        match self.bits {
            false => SPDP_A::_0,
            true => SPDP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDP_A::_1
    }
}
#[doc = "Field `SPDP` writer - Serial Padding Polarity"]
pub type SPDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, SPDP_A, O>;
impl<'a, const O: u8> SPDP_W<'a, O> {
    #[doc = "Padding bits are low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDP_A::_0)
    }
    #[doc = "Padding bits are high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDP_A::_1)
    }
}
#[doc = "Field `SWSP` reader - Serial WS Polarity"]
pub type SWSP_R = crate::BitReader<SWSP_A>;
#[doc = "Serial WS Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSP_A {
    #[doc = "0: SSIWS is low for 1st channel, high for 2nd channel."]
    _0 = 0,
    #[doc = "1: SSIWS is high for 1st channel, low for 2nd channel."]
    _1 = 1,
}
impl From<SWSP_A> for bool {
    #[inline(always)]
    fn from(variant: SWSP_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSP_A {
        match self.bits {
            false => SWSP_A::_0,
            true => SWSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSP_A::_1
    }
}
#[doc = "Field `SWSP` writer - Serial WS Polarity"]
pub type SWSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, SWSP_A, O>;
impl<'a, const O: u8> SWSP_W<'a, O> {
    #[doc = "SSIWS is low for 1st channel, high for 2nd channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSP_A::_0)
    }
    #[doc = "SSIWS is high for 1st channel, low for 2nd channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSP_A::_1)
    }
}
#[doc = "Field `SCKP` reader - Serial Bit Clock Polarity"]
pub type SCKP_R = crate::BitReader<SCKP_A>;
#[doc = "Serial Bit Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKP_A {
    #[doc = "0: SSIWS and SSIDATA change at the SSISCK falling edge (sampled at the SCK rising edge)."]
    _0 = 0,
    #[doc = "1: SSIWS and SSIDATA change at the SSISCK rising edge (sampled at the SCK falling edge)."]
    _1 = 1,
}
impl From<SCKP_A> for bool {
    #[inline(always)]
    fn from(variant: SCKP_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKP_A {
        match self.bits {
            false => SCKP_A::_0,
            true => SCKP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKP_A::_1
    }
}
#[doc = "Field `SCKP` writer - Serial Bit Clock Polarity"]
pub type SCKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, SCKP_A, O>;
impl<'a, const O: u8> SCKP_W<'a, O> {
    #[doc = "SSIWS and SSIDATA change at the SSISCK falling edge (sampled at the SCK rising edge)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCKP_A::_0)
    }
    #[doc = "SSIWS and SSIDATA change at the SSISCK rising edge (sampled at the SCK falling edge)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCKP_A::_1)
    }
}
#[doc = "Field `SWSD` reader - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited."]
pub type SWSD_R = crate::BitReader<SWSD_A>;
#[doc = "Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSD_A {
    #[doc = "0: Serial word select is input, slave mode."]
    _0 = 0,
    #[doc = "1: Serial word select is output, master mode."]
    _1 = 1,
}
impl From<SWSD_A> for bool {
    #[inline(always)]
    fn from(variant: SWSD_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSD_A {
        match self.bits {
            false => SWSD_A::_0,
            true => SWSD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSD_A::_1
    }
}
#[doc = "Field `SWSD` writer - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited."]
pub type SWSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, SWSD_A, O>;
impl<'a, const O: u8> SWSD_W<'a, O> {
    #[doc = "Serial word select is input, slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSD_A::_0)
    }
    #[doc = "Serial word select is output, master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSD_A::_1)
    }
}
#[doc = "Field `SWL` reader - System Word LengthSet the system word length to the bit clock frequency/2 fs."]
pub type SWL_R = crate::FieldReader<u8, SWL_A>;
#[doc = "System Word LengthSet the system word length to the bit clock frequency/2 fs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWL_A {
    #[doc = "0: 8 bits (serial bit clock frequency = 16fs )"]
    _000 = 0,
    #[doc = "1: 16 bits (serial bit clock frequency = 32fs )"]
    _001 = 1,
    #[doc = "2: 24 bits (serial bit clock frequency = 48fs )"]
    _010 = 2,
    #[doc = "3: 32 bits (serial bit clock frequency = 64fs )"]
    _011 = 3,
}
impl From<SWL_A> for u8 {
    #[inline(always)]
    fn from(variant: SWL_A) -> Self {
        variant as _
    }
}
impl SWL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWL_A> {
        match self.bits {
            0 => Some(SWL_A::_000),
            1 => Some(SWL_A::_001),
            2 => Some(SWL_A::_010),
            3 => Some(SWL_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SWL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SWL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SWL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SWL_A::_011
    }
}
#[doc = "Field `SWL` writer - System Word LengthSet the system word length to the bit clock frequency/2 fs."]
pub type SWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICR_SPEC, u8, SWL_A, 3, O>;
impl<'a, const O: u8> SWL_W<'a, O> {
    #[doc = "8 bits (serial bit clock frequency = 16fs )"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SWL_A::_000)
    }
    #[doc = "16 bits (serial bit clock frequency = 32fs )"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SWL_A::_001)
    }
    #[doc = "24 bits (serial bit clock frequency = 48fs )"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SWL_A::_010)
    }
    #[doc = "32 bits (serial bit clock frequency = 64fs )"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SWL_A::_011)
    }
}
#[doc = "Field `DWL` reader - Data Word Length"]
pub type DWL_R = crate::FieldReader<u8, DWL_A>;
#[doc = "Data Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DWL_A {
    #[doc = "0: 8 bits"]
    _000 = 0,
    #[doc = "1: 16 bits"]
    _001 = 1,
    #[doc = "2: 18 bits"]
    _010 = 2,
    #[doc = "3: 20 bits"]
    _011 = 3,
    #[doc = "4: 22 bits"]
    _100 = 4,
    #[doc = "5: 24 bits"]
    _101 = 5,
}
impl From<DWL_A> for u8 {
    #[inline(always)]
    fn from(variant: DWL_A) -> Self {
        variant as _
    }
}
impl DWL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DWL_A> {
        match self.bits {
            0 => Some(DWL_A::_000),
            1 => Some(DWL_A::_001),
            2 => Some(DWL_A::_010),
            3 => Some(DWL_A::_011),
            4 => Some(DWL_A::_100),
            5 => Some(DWL_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DWL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DWL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DWL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DWL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DWL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DWL_A::_101
    }
}
#[doc = "Field `DWL` writer - Data Word Length"]
pub type DWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICR_SPEC, u8, DWL_A, 3, O>;
impl<'a, const O: u8> DWL_W<'a, O> {
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DWL_A::_000)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DWL_A::_001)
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DWL_A::_010)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DWL_A::_011)
    }
    #[doc = "22 bits"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DWL_A::_100)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DWL_A::_101)
    }
}
#[doc = "Field `CHNL` reader - Channels"]
pub type CHNL_R = crate::FieldReader<u8, CHNL_A>;
#[doc = "Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHNL_A {
    #[doc = "0: One channel"]
    _00 = 0,
}
impl From<CHNL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHNL_A) -> Self {
        variant as _
    }
}
impl CHNL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHNL_A> {
        match self.bits {
            0 => Some(CHNL_A::_00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CHNL_A::_00
    }
}
#[doc = "Field `CHNL` writer - Channels"]
pub type CHNL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICR_SPEC, u8, CHNL_A, 2, O>;
impl<'a, const O: u8> CHNL_W<'a, O> {
    #[doc = "One channel"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CHNL_A::_00)
    }
}
#[doc = "Field `IIEN` reader - Idle Mode Interrupt Enable"]
pub type IIEN_R = crate::BitReader<IIEN_A>;
#[doc = "Idle Mode Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIEN_A {
    #[doc = "0: Disables an idle mode interrupt."]
    _0 = 0,
    #[doc = "1: Enables an idle mode interrupt."]
    _1 = 1,
}
impl From<IIEN_A> for bool {
    #[inline(always)]
    fn from(variant: IIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIEN_A {
        match self.bits {
            false => IIEN_A::_0,
            true => IIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIEN_A::_1
    }
}
#[doc = "Field `IIEN` writer - Idle Mode Interrupt Enable"]
pub type IIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, IIEN_A, O>;
impl<'a, const O: u8> IIEN_W<'a, O> {
    #[doc = "Disables an idle mode interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IIEN_A::_0)
    }
    #[doc = "Enables an idle mode interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IIEN_A::_1)
    }
}
#[doc = "Field `ROIEN` reader - Receive Overflow Interrupt Enable"]
pub type ROIEN_R = crate::BitReader<ROIEN_A>;
#[doc = "Receive Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIEN_A {
    #[doc = "0: Disables an overflow interrupt."]
    _0 = 0,
    #[doc = "1: Enables an overflow interrupt."]
    _1 = 1,
}
impl From<ROIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ROIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ROIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROIEN_A {
        match self.bits {
            false => ROIEN_A::_0,
            true => ROIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROIEN_A::_1
    }
}
#[doc = "Field `ROIEN` writer - Receive Overflow Interrupt Enable"]
pub type ROIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, ROIEN_A, O>;
impl<'a, const O: u8> ROIEN_W<'a, O> {
    #[doc = "Disables an overflow interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROIEN_A::_0)
    }
    #[doc = "Enables an overflow interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROIEN_A::_1)
    }
}
#[doc = "Field `RUIEN` reader - Receive Underflow Interrupt Enable"]
pub type RUIEN_R = crate::BitReader<RUIEN_A>;
#[doc = "Receive Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIEN_A {
    #[doc = "0: Disables an underflow interrupt."]
    _0 = 0,
    #[doc = "1: Enables an underflow interrupt."]
    _1 = 1,
}
impl From<RUIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RUIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RUIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUIEN_A {
        match self.bits {
            false => RUIEN_A::_0,
            true => RUIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RUIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RUIEN_A::_1
    }
}
#[doc = "Field `RUIEN` writer - Receive Underflow Interrupt Enable"]
pub type RUIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, RUIEN_A, O>;
impl<'a, const O: u8> RUIEN_W<'a, O> {
    #[doc = "Disables an underflow interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RUIEN_A::_0)
    }
    #[doc = "Enables an underflow interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RUIEN_A::_1)
    }
}
#[doc = "Field `TOIEN` reader - Transmit Overflow Interrupt Enable"]
pub type TOIEN_R = crate::BitReader<TOIEN_A>;
#[doc = "Transmit Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIEN_A {
    #[doc = "0: Disables an overflow interrupt."]
    _0 = 0,
    #[doc = "1: Enables an overflow interrupt."]
    _1 = 1,
}
impl From<TOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TOIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIEN_A {
        match self.bits {
            false => TOIEN_A::_0,
            true => TOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIEN_A::_1
    }
}
#[doc = "Field `TOIEN` writer - Transmit Overflow Interrupt Enable"]
pub type TOIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, TOIEN_A, O>;
impl<'a, const O: u8> TOIEN_W<'a, O> {
    #[doc = "Disables an overflow interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIEN_A::_0)
    }
    #[doc = "Enables an overflow interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIEN_A::_1)
    }
}
#[doc = "Field `TUIEN` reader - Transmit Underflow Interrupt Enable"]
pub type TUIEN_R = crate::BitReader<TUIEN_A>;
#[doc = "Transmit Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUIEN_A {
    #[doc = "0: Disables an underflow interrupt."]
    _0 = 0,
    #[doc = "1: Enables an underflow interrupt."]
    _1 = 1,
}
impl From<TUIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TUIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TUIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUIEN_A {
        match self.bits {
            false => TUIEN_A::_0,
            true => TUIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUIEN_A::_1
    }
}
#[doc = "Field `TUIEN` writer - Transmit Underflow Interrupt Enable"]
pub type TUIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, TUIEN_A, O>;
impl<'a, const O: u8> TUIEN_W<'a, O> {
    #[doc = "Disables an underflow interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TUIEN_A::_0)
    }
    #[doc = "Enables an underflow interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TUIEN_A::_1)
    }
}
#[doc = "Field `CKS` reader - Oversampling Clock Select"]
pub type CKS_R = crate::BitReader<CKS_A>;
#[doc = "Oversampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKS_A {
    #[doc = "0: AUDIO_CLK input"]
    _0 = 0,
    #[doc = "1: Setting prohibited"]
    _1 = 1,
}
impl From<CKS_A> for bool {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as u8 != 0
    }
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKS_A {
        match self.bits {
            false => CKS_A::_0,
            true => CKS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKS_A::_1
    }
}
#[doc = "Field `CKS` writer - Oversampling Clock Select"]
pub type CKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, CKS_A, O>;
impl<'a, const O: u8> CKS_W<'a, O> {
    #[doc = "AUDIO_CLK input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CKS_A::_0)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CKS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing."]
    #[inline(always)]
    pub fn muen(&self) -> MUEN_R {
        MUEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Serial Oversampling Clock Division Ratio"]
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Serial Data Delay"]
    #[inline(always)]
    pub fn del(&self) -> DEL_R {
        DEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parallel Data Alignment"]
    #[inline(always)]
    pub fn pdta(&self) -> PDTA_R {
        PDTA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Serial Data Alignment"]
    #[inline(always)]
    pub fn sdta(&self) -> SDTA_R {
        SDTA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Serial Padding Polarity"]
    #[inline(always)]
    pub fn spdp(&self) -> SPDP_R {
        SPDP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Serial WS Polarity"]
    #[inline(always)]
    pub fn swsp(&self) -> SWSP_R {
        SWSP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Serial Bit Clock Polarity"]
    #[inline(always)]
    pub fn sckp(&self) -> SCKP_R {
        SCKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited."]
    #[inline(always)]
    pub fn swsd(&self) -> SWSD_R {
        SWSD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - System Word LengthSet the system word length to the bit clock frequency/2 fs."]
    #[inline(always)]
    pub fn swl(&self) -> SWL_R {
        SWL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Data Word Length"]
    #[inline(always)]
    pub fn dwl(&self) -> DWL_R {
        DWL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Channels"]
    #[inline(always)]
    pub fn chnl(&self) -> CHNL_R {
        CHNL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 25 - Idle Mode Interrupt Enable"]
    #[inline(always)]
    pub fn iien(&self) -> IIEN_R {
        IIEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receive Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn roien(&self) -> ROIEN_R {
        ROIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn ruien(&self) -> RUIEN_R {
        RUIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toien(&self) -> TOIEN_R {
        TOIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn tuien(&self) -> TUIEN_R {
        TUIEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Oversampling Clock Select"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<0> {
        REN_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<1> {
        TEN_W::new(self)
    }
    #[doc = "Bit 3 - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing."]
    #[inline(always)]
    #[must_use]
    pub fn muen(&mut self) -> MUEN_W<3> {
        MUEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Serial Oversampling Clock Division Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ckdv(&mut self) -> CKDV_W<4> {
        CKDV_W::new(self)
    }
    #[doc = "Bit 8 - Serial Data Delay"]
    #[inline(always)]
    #[must_use]
    pub fn del(&mut self) -> DEL_W<8> {
        DEL_W::new(self)
    }
    #[doc = "Bit 9 - Parallel Data Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn pdta(&mut self) -> PDTA_W<9> {
        PDTA_W::new(self)
    }
    #[doc = "Bit 10 - Serial Data Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn sdta(&mut self) -> SDTA_W<10> {
        SDTA_W::new(self)
    }
    #[doc = "Bit 11 - Serial Padding Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn spdp(&mut self) -> SPDP_W<11> {
        SPDP_W::new(self)
    }
    #[doc = "Bit 12 - Serial WS Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn swsp(&mut self) -> SWSP_W<12> {
        SWSP_W::new(self)
    }
    #[doc = "Bit 13 - Serial Bit Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn sckp(&mut self) -> SCKP_W<13> {
        SCKP_W::new(self)
    }
    #[doc = "Bit 14 - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited."]
    #[inline(always)]
    #[must_use]
    pub fn swsd(&mut self) -> SWSD_W<14> {
        SWSD_W::new(self)
    }
    #[doc = "Bits 16:18 - System Word LengthSet the system word length to the bit clock frequency/2 fs."]
    #[inline(always)]
    #[must_use]
    pub fn swl(&mut self) -> SWL_W<16> {
        SWL_W::new(self)
    }
    #[doc = "Bits 19:21 - Data Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn dwl(&mut self) -> DWL_W<19> {
        DWL_W::new(self)
    }
    #[doc = "Bits 22:23 - Channels"]
    #[inline(always)]
    #[must_use]
    pub fn chnl(&mut self) -> CHNL_W<22> {
        CHNL_W::new(self)
    }
    #[doc = "Bit 25 - Idle Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iien(&mut self) -> IIEN_W<25> {
        IIEN_W::new(self)
    }
    #[doc = "Bit 26 - Receive Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roien(&mut self) -> ROIEN_W<26> {
        ROIEN_W::new(self)
    }
    #[doc = "Bit 27 - Receive Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ruien(&mut self) -> RUIEN_W<27> {
        RUIEN_W::new(self)
    }
    #[doc = "Bit 28 - Transmit Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toien(&mut self) -> TOIEN_W<28> {
        TOIEN_W::new(self)
    }
    #[doc = "Bit 29 - Transmit Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuien(&mut self) -> TUIEN_W<29> {
        TUIEN_W::new(self)
    }
    #[doc = "Bit 30 - Oversampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<30> {
        CKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssicr](index.html) module"]
pub struct SSICR_SPEC;
impl crate::RegisterSpec for SSICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssicr::R](R) reader structure"]
impl crate::Readable for SSICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssicr::W](W) writer structure"]
impl crate::Writable for SSICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSICR to value 0"]
impl crate::Resettable for SSICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
