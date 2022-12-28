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
#[doc = "Field `REN` reader - Reception Enable"]
pub type REN_R = crate::BitReader<REN_A>;
#[doc = "Reception Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REN_A {
    #[doc = "0: Disables reception"]
    _0 = 0,
    #[doc = "1: Enables reception (starts reception)"]
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
#[doc = "Field `REN` writer - Reception Enable"]
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, REN_A, O>;
impl<'a, const O: u8> REN_W<'a, O> {
    #[doc = "Disables reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REN_A::_0)
    }
    #[doc = "Enables reception (starts reception)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REN_A::_1)
    }
}
#[doc = "Field `TEN` reader - Transmission Enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Transmission Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: Disables transmission"]
    _0 = 0,
    #[doc = "1: Enables transmission (starts transmission)"]
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
#[doc = "Field `TEN` writer - Transmission Enable"]
pub type TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, TEN_A, O>;
impl<'a, const O: u8> TEN_W<'a, O> {
    #[doc = "Disables transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEN_A::_0)
    }
    #[doc = "Enables transmission (starts transmission)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEN_A::_1)
    }
}
#[doc = "Field `MUEN` reader - Mute Enable"]
pub type MUEN_R = crate::BitReader<MUEN_A>;
#[doc = "Mute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUEN_A {
    #[doc = "0: Disables muting on the next frame boundary"]
    _0 = 0,
    #[doc = "1: Enables muting on the next frame boundary"]
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
#[doc = "Field `MUEN` writer - Mute Enable"]
pub type MUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, MUEN_A, O>;
impl<'a, const O: u8> MUEN_W<'a, O> {
    #[doc = "Disables muting on the next frame boundary"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUEN_A::_0)
    }
    #[doc = "Enables muting on the next frame boundary"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUEN_A::_1)
    }
}
#[doc = "Field `CKDV` reader - Selects Bit Clock Division Ratio"]
pub type CKDV_R = crate::FieldReader<u8, CKDV_A>;
#[doc = "Selects Bit Clock Division Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKDV_A {
    #[doc = "0: AUDIO_MCK"]
    _0X0 = 0,
    #[doc = "1: AUDIO_MCK/2"]
    _0X1 = 1,
    #[doc = "2: AUDIO_MCK/4"]
    _0X2 = 2,
    #[doc = "3: AUDIO_MCK/8"]
    _0X3 = 3,
    #[doc = "4: AUDIO_MCK/16"]
    _0X4 = 4,
    #[doc = "5: AUDIO_MCK/32"]
    _0X5 = 5,
    #[doc = "6: AUDIO_MCK/64"]
    _0X6 = 6,
    #[doc = "7: AUDIO_MCK/128"]
    _0X7 = 7,
    #[doc = "8: AUDIO_MCK/6"]
    _0X8 = 8,
    #[doc = "9: AUDIO_MCK/12"]
    _0X9 = 9,
    #[doc = "10: AUDIO_MCK/24"]
    _0X_A = 10,
    #[doc = "11: AUDIO_MCK/48"]
    _0X_B = 11,
    #[doc = "12: AUDIO_MCK/96"]
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
#[doc = "Field `CKDV` writer - Selects Bit Clock Division Ratio"]
pub type CKDV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICR_SPEC, u8, CKDV_A, 4, O>;
impl<'a, const O: u8> CKDV_W<'a, O> {
    #[doc = "AUDIO_MCK"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CKDV_A::_0X0)
    }
    #[doc = "AUDIO_MCK/2"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(CKDV_A::_0X1)
    }
    #[doc = "AUDIO_MCK/4"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(CKDV_A::_0X2)
    }
    #[doc = "AUDIO_MCK/8"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(CKDV_A::_0X3)
    }
    #[doc = "AUDIO_MCK/16"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(CKDV_A::_0X4)
    }
    #[doc = "AUDIO_MCK/32"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(CKDV_A::_0X5)
    }
    #[doc = "AUDIO_MCK/64"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(CKDV_A::_0X6)
    }
    #[doc = "AUDIO_MCK/128"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(CKDV_A::_0X7)
    }
    #[doc = "AUDIO_MCK/6"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(CKDV_A::_0X8)
    }
    #[doc = "AUDIO_MCK/12"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(CKDV_A::_0X9)
    }
    #[doc = "AUDIO_MCK/24"]
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(CKDV_A::_0X_A)
    }
    #[doc = "AUDIO_MCK/48"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(CKDV_A::_0X_B)
    }
    #[doc = "AUDIO_MCK/96"]
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut W {
        self.variant(CKDV_A::_0X_C)
    }
}
#[doc = "Field `DEL` reader - Selects Serial Data Delay"]
pub type DEL_R = crate::BitReader<DEL_A>;
#[doc = "Selects Serial Data Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEL_A {
    #[doc = "0: Delay of 1 cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0"]
    _0 = 0,
    #[doc = "1: No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0"]
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
#[doc = "Field `DEL` writer - Selects Serial Data Delay"]
pub type DEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, DEL_A, O>;
impl<'a, const O: u8> DEL_W<'a, O> {
    #[doc = "Delay of 1 cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEL_A::_0)
    }
    #[doc = "No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEL_A::_1)
    }
}
#[doc = "Field `PDTA` reader - Selects Placement Data Alignment"]
pub type PDTA_R = crate::BitReader<PDTA_A>;
#[doc = "Selects Placement Data Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDTA_A {
    #[doc = "0: Left-justifies placement data (SSIFTDR, SSIFRDR)"]
    _0 = 0,
    #[doc = "1: Right-justifies placement data (SSIFTDR, SSIFRDR)"]
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
#[doc = "Field `PDTA` writer - Selects Placement Data Alignment"]
pub type PDTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, PDTA_A, O>;
impl<'a, const O: u8> PDTA_W<'a, O> {
    #[doc = "Left-justifies placement data (SSIFTDR, SSIFRDR)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDTA_A::_0)
    }
    #[doc = "Right-justifies placement data (SSIFTDR, SSIFRDR)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDTA_A::_1)
    }
}
#[doc = "Field `SDTA` reader - Selects Serial Data Alignment"]
pub type SDTA_R = crate::BitReader<SDTA_A>;
#[doc = "Selects Serial Data Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTA_A {
    #[doc = "0: Transmits and receives serial data first and then padding bits"]
    _0 = 0,
    #[doc = "1: Transmit and receives padding bits first and then serial data"]
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
#[doc = "Field `SDTA` writer - Selects Serial Data Alignment"]
pub type SDTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, SDTA_A, O>;
impl<'a, const O: u8> SDTA_W<'a, O> {
    #[doc = "Transmits and receives serial data first and then padding bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDTA_A::_0)
    }
    #[doc = "Transmit and receives padding bits first and then serial data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDTA_A::_1)
    }
}
#[doc = "Field `SPDP` reader - Selects Serial Padding Polarity"]
pub type SPDP_R = crate::BitReader<SPDP_A>;
#[doc = "Selects Serial Padding Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDP_A {
    #[doc = "0: Padding data is at a low level"]
    _0 = 0,
    #[doc = "1: Padding data is at a high level"]
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
#[doc = "Field `SPDP` writer - Selects Serial Padding Polarity"]
pub type SPDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, SPDP_A, O>;
impl<'a, const O: u8> SPDP_W<'a, O> {
    #[doc = "Padding data is at a low level"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDP_A::_0)
    }
    #[doc = "Padding data is at a high level"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDP_A::_1)
    }
}
#[doc = "Field `LRCKP` reader - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
pub type LRCKP_R = crate::BitReader<LRCKP_A>;
#[doc = "Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRCKP_A {
    #[doc = "0: The initial value is at a high level. The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS."]
    _0 = 0,
    #[doc = "1: The initial value is at a low level. The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS."]
    _1 = 1,
}
impl From<LRCKP_A> for bool {
    #[inline(always)]
    fn from(variant: LRCKP_A) -> Self {
        variant as u8 != 0
    }
}
impl LRCKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRCKP_A {
        match self.bits {
            false => LRCKP_A::_0,
            true => LRCKP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRCKP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRCKP_A::_1
    }
}
#[doc = "Field `LRCKP` writer - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
pub type LRCKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, LRCKP_A, O>;
impl<'a, const O: u8> LRCKP_W<'a, O> {
    #[doc = "The initial value is at a high level. The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRCKP_A::_0)
    }
    #[doc = "The initial value is at a low level. The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRCKP_A::_1)
    }
}
#[doc = "Field `BCKP` reader - Selects Bit Clock Polarity"]
pub type BCKP_R = crate::BitReader<BCKP_A>;
#[doc = "Selects Bit Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCKP_A {
    #[doc = "0: SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0/SSIDATA0 are sampled at a rising edge of SSIBCK)."]
    _0 = 0,
    #[doc = "1: SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0/SSIDATA0 are sampled at a falling edge of SSIBCK)."]
    _1 = 1,
}
impl From<BCKP_A> for bool {
    #[inline(always)]
    fn from(variant: BCKP_A) -> Self {
        variant as u8 != 0
    }
}
impl BCKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCKP_A {
        match self.bits {
            false => BCKP_A::_0,
            true => BCKP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCKP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCKP_A::_1
    }
}
#[doc = "Field `BCKP` writer - Selects Bit Clock Polarity"]
pub type BCKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, BCKP_A, O>;
impl<'a, const O: u8> BCKP_W<'a, O> {
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0/SSIDATA0 are sampled at a rising edge of SSIBCK)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCKP_A::_0)
    }
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0/SSIDATA0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0/SSIDATA0 are sampled at a falling edge of SSIBCK)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCKP_A::_1)
    }
}
#[doc = "Field `MST` reader - Master Enable"]
pub type MST_R = crate::BitReader<MST_A>;
#[doc = "Master Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_A {
    #[doc = "0: Slave-mode communication"]
    _0 = 0,
    #[doc = "1: Master-mode communication"]
    _1 = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::_0,
            true => MST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MST_A::_1
    }
}
#[doc = "Field `MST` writer - Master Enable"]
pub type MST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, MST_A, O>;
impl<'a, const O: u8> MST_W<'a, O> {
    #[doc = "Slave-mode communication"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MST_A::_0)
    }
    #[doc = "Master-mode communication"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MST_A::_1)
    }
}
#[doc = "Field `SWL` reader - Selects System Word Length"]
pub type SWL_R = crate::FieldReader<u8, SWL_A>;
#[doc = "Selects System Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWL_A {
    #[doc = "0: 8 bits"]
    _000 = 0,
    #[doc = "1: 16 bits"]
    _001 = 1,
    #[doc = "2: 24 bits"]
    _010 = 2,
    #[doc = "3: 32 bits"]
    _011 = 3,
    #[doc = "4: 48 bits"]
    _100 = 4,
    #[doc = "5: 64 bits"]
    _101 = 5,
    #[doc = "6: 128 bits"]
    _110 = 6,
    #[doc = "7: 256 bits"]
    _111 = 7,
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
    pub fn variant(&self) -> SWL_A {
        match self.bits {
            0 => SWL_A::_000,
            1 => SWL_A::_001,
            2 => SWL_A::_010,
            3 => SWL_A::_011,
            4 => SWL_A::_100,
            5 => SWL_A::_101,
            6 => SWL_A::_110,
            7 => SWL_A::_111,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SWL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SWL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SWL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SWL_A::_111
    }
}
#[doc = "Field `SWL` writer - Selects System Word Length"]
pub type SWL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSICR_SPEC, u8, SWL_A, 3, O>;
impl<'a, const O: u8> SWL_W<'a, O> {
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SWL_A::_000)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SWL_A::_001)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SWL_A::_010)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SWL_A::_011)
    }
    #[doc = "48 bits"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SWL_A::_100)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SWL_A::_101)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SWL_A::_110)
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SWL_A::_111)
    }
}
#[doc = "Field `DWL` reader - Selects Data Word Length"]
pub type DWL_R = crate::FieldReader<u8, DWL_A>;
#[doc = "Selects Data Word Length\n\nValue on reset: 0"]
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
    #[doc = "6: 32 bits"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
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
    pub fn variant(&self) -> DWL_A {
        match self.bits {
            0 => DWL_A::_000,
            1 => DWL_A::_001,
            2 => DWL_A::_010,
            3 => DWL_A::_011,
            4 => DWL_A::_100,
            5 => DWL_A::_101,
            6 => DWL_A::_110,
            7 => DWL_A::_111,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DWL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DWL_A::_111
    }
}
#[doc = "Field `DWL` writer - Selects Data Word Length"]
pub type DWL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSICR_SPEC, u8, DWL_A, 3, O>;
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
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DWL_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DWL_A::_111)
    }
}
#[doc = "Field `FRM` reader - Selects Frame Word Number"]
pub type FRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRM` writer - Selects Frame Word Number"]
pub type FRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IIEN` reader - Idle Mode Interrupt Output Enable"]
pub type IIEN_R = crate::BitReader<IIEN_A>;
#[doc = "Idle Mode Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIEN_A {
    #[doc = "0: Disables idle mode interrupt output"]
    _0 = 0,
    #[doc = "1: Enables idle mode interrupt output"]
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
#[doc = "Field `IIEN` writer - Idle Mode Interrupt Output Enable"]
pub type IIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, IIEN_A, O>;
impl<'a, const O: u8> IIEN_W<'a, O> {
    #[doc = "Disables idle mode interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IIEN_A::_0)
    }
    #[doc = "Enables idle mode interrupt output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IIEN_A::_1)
    }
}
#[doc = "Field `ROIEN` reader - Receive Overflow Interrupt Output Enable"]
pub type ROIEN_R = crate::BitReader<ROIEN_A>;
#[doc = "Receive Overflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIEN_A {
    #[doc = "0: Disables receive overflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables receive overflow interrupt output"]
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
#[doc = "Field `ROIEN` writer - Receive Overflow Interrupt Output Enable"]
pub type ROIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, ROIEN_A, O>;
impl<'a, const O: u8> ROIEN_W<'a, O> {
    #[doc = "Disables receive overflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROIEN_A::_0)
    }
    #[doc = "Enables receive overflow interrupt output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROIEN_A::_1)
    }
}
#[doc = "Field `RUIEN` reader - Receive Underflow Interrupt Output Enable"]
pub type RUIEN_R = crate::BitReader<RUIEN_A>;
#[doc = "Receive Underflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIEN_A {
    #[doc = "0: Disables receive underflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables receive underflow interrupt output"]
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
#[doc = "Field `RUIEN` writer - Receive Underflow Interrupt Output Enable"]
pub type RUIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, RUIEN_A, O>;
impl<'a, const O: u8> RUIEN_W<'a, O> {
    #[doc = "Disables receive underflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RUIEN_A::_0)
    }
    #[doc = "Enables receive underflow interrupt output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RUIEN_A::_1)
    }
}
#[doc = "Field `TOIEN` reader - Transmit Overflow Interrupt Output Enable"]
pub type TOIEN_R = crate::BitReader<TOIEN_A>;
#[doc = "Transmit Overflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIEN_A {
    #[doc = "0: Disables transmit overflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables transmit overflow interrupt output"]
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
#[doc = "Field `TOIEN` writer - Transmit Overflow Interrupt Output Enable"]
pub type TOIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, TOIEN_A, O>;
impl<'a, const O: u8> TOIEN_W<'a, O> {
    #[doc = "Disables transmit overflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIEN_A::_0)
    }
    #[doc = "Enables transmit overflow interrupt output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIEN_A::_1)
    }
}
#[doc = "Field `TUIEN` reader - Transmit Underflow Interrupt Output Enable"]
pub type TUIEN_R = crate::BitReader<TUIEN_A>;
#[doc = "Transmit Underflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUIEN_A {
    #[doc = "0: Disables transmit underflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables transmit underflow interrupt output"]
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
#[doc = "Field `TUIEN` writer - Transmit Underflow Interrupt Output Enable"]
pub type TUIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, TUIEN_A, O>;
impl<'a, const O: u8> TUIEN_W<'a, O> {
    #[doc = "Disables transmit underflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TUIEN_A::_0)
    }
    #[doc = "Enables transmit underflow interrupt output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TUIEN_A::_1)
    }
}
#[doc = "Field `CKS` reader - Selects an Audio Clock for Master-mode Communication"]
pub type CKS_R = crate::BitReader<CKS_A>;
#[doc = "Selects an Audio Clock for Master-mode Communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKS_A {
    #[doc = "0: Selects the AUDIO_CLK input"]
    _0 = 0,
    #[doc = "1: Selects the GTIOC2A (GPT output)"]
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
#[doc = "Field `CKS` writer - Selects an Audio Clock for Master-mode Communication"]
pub type CKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSICR_SPEC, CKS_A, O>;
impl<'a, const O: u8> CKS_W<'a, O> {
    #[doc = "Selects the AUDIO_CLK input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CKS_A::_0)
    }
    #[doc = "Selects the GTIOC2A (GPT output)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CKS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Reception Enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mute Enable"]
    #[inline(always)]
    pub fn muen(&self) -> MUEN_R {
        MUEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Selects Bit Clock Division Ratio"]
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selects Serial Data Delay"]
    #[inline(always)]
    pub fn del(&self) -> DEL_R {
        DEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects Placement Data Alignment"]
    #[inline(always)]
    pub fn pdta(&self) -> PDTA_R {
        PDTA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects Serial Data Alignment"]
    #[inline(always)]
    pub fn sdta(&self) -> SDTA_R {
        SDTA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects Serial Padding Polarity"]
    #[inline(always)]
    pub fn spdp(&self) -> SPDP_R {
        SPDP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[inline(always)]
    pub fn lrckp(&self) -> LRCKP_R {
        LRCKP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects Bit Clock Polarity"]
    #[inline(always)]
    pub fn bckp(&self) -> BCKP_R {
        BCKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Master Enable"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Selects System Word Length"]
    #[inline(always)]
    pub fn swl(&self) -> SWL_R {
        SWL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Selects Data Word Length"]
    #[inline(always)]
    pub fn dwl(&self) -> DWL_R {
        DWL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Selects Frame Word Number"]
    #[inline(always)]
    pub fn frm(&self) -> FRM_R {
        FRM_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 25 - Idle Mode Interrupt Output Enable"]
    #[inline(always)]
    pub fn iien(&self) -> IIEN_R {
        IIEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receive Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn roien(&self) -> ROIEN_R {
        ROIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn ruien(&self) -> RUIEN_R {
        RUIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn toien(&self) -> TOIEN_R {
        TOIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn tuien(&self) -> TUIEN_R {
        TUIEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Selects an Audio Clock for Master-mode Communication"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<0> {
        REN_W::new(self)
    }
    #[doc = "Bit 1 - Transmission Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<1> {
        TEN_W::new(self)
    }
    #[doc = "Bit 3 - Mute Enable"]
    #[inline(always)]
    #[must_use]
    pub fn muen(&mut self) -> MUEN_W<3> {
        MUEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Selects Bit Clock Division Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ckdv(&mut self) -> CKDV_W<4> {
        CKDV_W::new(self)
    }
    #[doc = "Bit 8 - Selects Serial Data Delay"]
    #[inline(always)]
    #[must_use]
    pub fn del(&mut self) -> DEL_W<8> {
        DEL_W::new(self)
    }
    #[doc = "Bit 9 - Selects Placement Data Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn pdta(&mut self) -> PDTA_W<9> {
        PDTA_W::new(self)
    }
    #[doc = "Bit 10 - Selects Serial Data Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn sdta(&mut self) -> SDTA_W<10> {
        SDTA_W::new(self)
    }
    #[doc = "Bit 11 - Selects Serial Padding Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn spdp(&mut self) -> SPDP_W<11> {
        SPDP_W::new(self)
    }
    #[doc = "Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[inline(always)]
    #[must_use]
    pub fn lrckp(&mut self) -> LRCKP_W<12> {
        LRCKP_W::new(self)
    }
    #[doc = "Bit 13 - Selects Bit Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bckp(&mut self) -> BCKP_W<13> {
        BCKP_W::new(self)
    }
    #[doc = "Bit 14 - Master Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mst(&mut self) -> MST_W<14> {
        MST_W::new(self)
    }
    #[doc = "Bits 16:18 - Selects System Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn swl(&mut self) -> SWL_W<16> {
        SWL_W::new(self)
    }
    #[doc = "Bits 19:21 - Selects Data Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn dwl(&mut self) -> DWL_W<19> {
        DWL_W::new(self)
    }
    #[doc = "Bits 22:23 - Selects Frame Word Number"]
    #[inline(always)]
    #[must_use]
    pub fn frm(&mut self) -> FRM_W<22> {
        FRM_W::new(self)
    }
    #[doc = "Bit 25 - Idle Mode Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iien(&mut self) -> IIEN_W<25> {
        IIEN_W::new(self)
    }
    #[doc = "Bit 26 - Receive Overflow Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roien(&mut self) -> ROIEN_W<26> {
        ROIEN_W::new(self)
    }
    #[doc = "Bit 27 - Receive Underflow Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ruien(&mut self) -> RUIEN_W<27> {
        RUIEN_W::new(self)
    }
    #[doc = "Bit 28 - Transmit Overflow Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toien(&mut self) -> TOIEN_W<28> {
        TOIEN_W::new(self)
    }
    #[doc = "Bit 29 - Transmit Underflow Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuien(&mut self) -> TUIEN_W<29> {
        TUIEN_W::new(self)
    }
    #[doc = "Bit 30 - Selects an Audio Clock for Master-mode Communication"]
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
