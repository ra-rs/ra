#[doc = "Register `PCCR0` reader"]
pub struct R(crate::R<PCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCR0` writer"]
pub struct W(crate::W<PCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCR0_SPEC>;
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
impl From<crate::W<PCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCKE` reader - Channel 0 GTCNT Count Clear"]
pub type PCKE_R = crate::BitReader<PCKE_A>;
#[doc = "Channel 0 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCKE_A {
    #[doc = "0: Operations for reception are stopped."]
    _0 = 0,
    #[doc = "1: Operations for reception are ongoing."]
    _1 = 1,
}
impl From<PCKE_A> for bool {
    #[inline(always)]
    fn from(variant: PCKE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKE_A {
        match self.bits {
            false => PCKE_A::_0,
            true => PCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCKE_A::_1
    }
}
#[doc = "Field `PCKE` writer - Channel 0 GTCNT Count Clear"]
pub type PCKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, PCKE_A, O>;
impl<'a, const O: u8> PCKE_W<'a, O> {
    #[doc = "Operations for reception are stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCKE_A::_0)
    }
    #[doc = "Operations for reception are ongoing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCKE_A::_1)
    }
}
#[doc = "Field `VPS` reader - VSYNC Signal Polarity Select"]
pub type VPS_R = crate::BitReader<VPS_A>;
#[doc = "VSYNC Signal Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPS_A {
    #[doc = "0: VSYNC signal is active high."]
    _0 = 0,
    #[doc = "1: VSYNC signal is active low."]
    _1 = 1,
}
impl From<VPS_A> for bool {
    #[inline(always)]
    fn from(variant: VPS_A) -> Self {
        variant as u8 != 0
    }
}
impl VPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPS_A {
        match self.bits {
            false => VPS_A::_0,
            true => VPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPS_A::_1
    }
}
#[doc = "Field `VPS` writer - VSYNC Signal Polarity Select"]
pub type VPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, VPS_A, O>;
impl<'a, const O: u8> VPS_W<'a, O> {
    #[doc = "VSYNC signal is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VPS_A::_0)
    }
    #[doc = "VSYNC signal is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VPS_A::_1)
    }
}
#[doc = "Field `HPS` reader - HSYNC Signal Polarity Select"]
pub type HPS_R = crate::BitReader<HPS_A>;
#[doc = "HSYNC Signal Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPS_A {
    #[doc = "0: HSYNC signal is active high."]
    _0 = 0,
    #[doc = "1: HSYNC signal is active low."]
    _1 = 1,
}
impl From<HPS_A> for bool {
    #[inline(always)]
    fn from(variant: HPS_A) -> Self {
        variant as u8 != 0
    }
}
impl HPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPS_A {
        match self.bits {
            false => HPS_A::_0,
            true => HPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HPS_A::_1
    }
}
#[doc = "Field `HPS` writer - HSYNC Signal Polarity Select"]
pub type HPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, HPS_A, O>;
impl<'a, const O: u8> HPS_W<'a, O> {
    #[doc = "HSYNC signal is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HPS_A::_0)
    }
    #[doc = "HSYNC signal is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HPS_A::_1)
    }
}
#[doc = "PDC Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRST_AW {
    #[doc = "0: PDC reset is not applied."]
    _0 = 0,
    #[doc = "1: PDC is reset."]
    _1 = 1,
}
impl From<PRST_AW> for bool {
    #[inline(always)]
    fn from(variant: PRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRST` writer - PDC Reset"]
pub type PRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, PRST_AW, O>;
impl<'a, const O: u8> PRST_W<'a, O> {
    #[doc = "PDC reset is not applied."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRST_AW::_0)
    }
    #[doc = "PDC is reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRST_AW::_1)
    }
}
#[doc = "Field `DFIE` reader - Receive Data Ready Interrupt Enable"]
pub type DFIE_R = crate::BitReader<DFIE_A>;
#[doc = "Receive Data Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFIE_A {
    #[doc = "0: Generation of receive data ready interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: Generation of receive data ready interrupt requests is enabled."]
    _1 = 1,
}
impl From<DFIE_A> for bool {
    #[inline(always)]
    fn from(variant: DFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFIE_A {
        match self.bits {
            false => DFIE_A::_0,
            true => DFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFIE_A::_1
    }
}
#[doc = "Field `DFIE` writer - Receive Data Ready Interrupt Enable"]
pub type DFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, DFIE_A, O>;
impl<'a, const O: u8> DFIE_W<'a, O> {
    #[doc = "Generation of receive data ready interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFIE_A::_0)
    }
    #[doc = "Generation of receive data ready interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFIE_A::_1)
    }
}
#[doc = "Field `FEIE` reader - Frame End Interrupt Enable"]
pub type FEIE_R = crate::BitReader<FEIE_A>;
#[doc = "Frame End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE_A {
    #[doc = "0: Generation of frame end interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: Generation of frame end interrupt requests is enabled."]
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEIE_A::_1
    }
}
#[doc = "Field `FEIE` writer - Frame End Interrupt Enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, FEIE_A, O>;
impl<'a, const O: u8> FEIE_W<'a, O> {
    #[doc = "Generation of frame end interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "Generation of frame end interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
    }
}
#[doc = "Field `OVIE` reader - Overrun Interrupt Enable"]
pub type OVIE_R = crate::BitReader<OVIE_A>;
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVIE_A {
    #[doc = "0: Generation of overrun interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: Generation of overrun interrupt requests is enabled."]
    _1 = 1,
}
impl From<OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVIE_A {
        match self.bits {
            false => OVIE_A::_0,
            true => OVIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVIE_A::_1
    }
}
#[doc = "Field `OVIE` writer - Overrun Interrupt Enable"]
pub type OVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, OVIE_A, O>;
impl<'a, const O: u8> OVIE_W<'a, O> {
    #[doc = "Generation of overrun interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVIE_A::_0)
    }
    #[doc = "Generation of overrun interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVIE_A::_1)
    }
}
#[doc = "Field `UDRIE` reader - Underrun Interrupt Enable"]
pub type UDRIE_R = crate::BitReader<UDRIE_A>;
#[doc = "Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRIE_A {
    #[doc = "0: Generation of underrun interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: Generation of underrun interrupt requests is enabled."]
    _1 = 1,
}
impl From<UDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: UDRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDRIE_A {
        match self.bits {
            false => UDRIE_A::_0,
            true => UDRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDRIE_A::_1
    }
}
#[doc = "Field `UDRIE` writer - Underrun Interrupt Enable"]
pub type UDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, UDRIE_A, O>;
impl<'a, const O: u8> UDRIE_W<'a, O> {
    #[doc = "Generation of underrun interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UDRIE_A::_0)
    }
    #[doc = "Generation of underrun interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UDRIE_A::_1)
    }
}
#[doc = "Field `VERIE` reader - Vertical Line Number Setting Error Interrupt Enable"]
pub type VERIE_R = crate::BitReader<VERIE_A>;
#[doc = "Vertical Line Number Setting Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VERIE_A {
    #[doc = "0: Generation of vertical line number setting error interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: Generation of vertical line number setting error interrupt requests is enabled."]
    _1 = 1,
}
impl From<VERIE_A> for bool {
    #[inline(always)]
    fn from(variant: VERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl VERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VERIE_A {
        match self.bits {
            false => VERIE_A::_0,
            true => VERIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VERIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VERIE_A::_1
    }
}
#[doc = "Field `VERIE` writer - Vertical Line Number Setting Error Interrupt Enable"]
pub type VERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, VERIE_A, O>;
impl<'a, const O: u8> VERIE_W<'a, O> {
    #[doc = "Generation of vertical line number setting error interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VERIE_A::_0)
    }
    #[doc = "Generation of vertical line number setting error interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VERIE_A::_1)
    }
}
#[doc = "Field `HERIE` reader - Horizontal Byte Number Setting Error Interrupt Enable"]
pub type HERIE_R = crate::BitReader<HERIE_A>;
#[doc = "Horizontal Byte Number Setting Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HERIE_A {
    #[doc = "0: Generation of horizontal byte number setting error interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: Generation of horizontal byte number setting error interrupt requests is enabled."]
    _1 = 1,
}
impl From<HERIE_A> for bool {
    #[inline(always)]
    fn from(variant: HERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HERIE_A {
        match self.bits {
            false => HERIE_A::_0,
            true => HERIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HERIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HERIE_A::_1
    }
}
#[doc = "Field `HERIE` writer - Horizontal Byte Number Setting Error Interrupt Enable"]
pub type HERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, HERIE_A, O>;
impl<'a, const O: u8> HERIE_W<'a, O> {
    #[doc = "Generation of horizontal byte number setting error interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HERIE_A::_0)
    }
    #[doc = "Generation of horizontal byte number setting error interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HERIE_A::_1)
    }
}
#[doc = "Field `PCKOE` reader - PCKO Output Enable"]
pub type PCKOE_R = crate::BitReader<PCKOE_A>;
#[doc = "PCKO Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCKOE_A {
    #[doc = "0: PCKO output is disabled (fixed to the high level)"]
    _0 = 0,
    #[doc = "1: PCKO output is enabled."]
    _1 = 1,
}
impl From<PCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: PCKOE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKOE_A {
        match self.bits {
            false => PCKOE_A::_0,
            true => PCKOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCKOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCKOE_A::_1
    }
}
#[doc = "Field `PCKOE` writer - PCKO Output Enable"]
pub type PCKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, PCKOE_A, O>;
impl<'a, const O: u8> PCKOE_W<'a, O> {
    #[doc = "PCKO output is disabled (fixed to the high level)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCKOE_A::_0)
    }
    #[doc = "PCKO output is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCKOE_A::_1)
    }
}
#[doc = "Field `PCKDIV` reader - PCKO Frequency Division Ratio Select"]
pub type PCKDIV_R = crate::FieldReader<u8, PCKDIV_A>;
#[doc = "PCKO Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKDIV_A {
    #[doc = "0: PCKO/2"]
    _000 = 0,
    #[doc = "1: PCKO/4"]
    _001 = 1,
    #[doc = "2: PCKO/6"]
    _010 = 2,
    #[doc = "3: PCKO/8"]
    _011 = 3,
    #[doc = "4: PCKO/10"]
    _100 = 4,
    #[doc = "5: PCKO/12"]
    _101 = 5,
    #[doc = "6: PCKO/14"]
    _110 = 6,
    #[doc = "7: PCKO/16"]
    _111 = 7,
}
impl From<PCKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKDIV_A) -> Self {
        variant as _
    }
}
impl PCKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKDIV_A {
        match self.bits {
            0 => PCKDIV_A::_000,
            1 => PCKDIV_A::_001,
            2 => PCKDIV_A::_010,
            3 => PCKDIV_A::_011,
            4 => PCKDIV_A::_100,
            5 => PCKDIV_A::_101,
            6 => PCKDIV_A::_110,
            7 => PCKDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PCKDIV_A::_111
    }
}
#[doc = "Field `PCKDIV` writer - PCKO Frequency Division Ratio Select"]
pub type PCKDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PCCR0_SPEC, u8, PCKDIV_A, 3, O>;
impl<'a, const O: u8> PCKDIV_W<'a, O> {
    #[doc = "PCKO/2"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCKDIV_A::_000)
    }
    #[doc = "PCKO/4"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCKDIV_A::_001)
    }
    #[doc = "PCKO/6"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCKDIV_A::_010)
    }
    #[doc = "PCKO/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCKDIV_A::_011)
    }
    #[doc = "PCKO/10"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCKDIV_A::_100)
    }
    #[doc = "PCKO/12"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCKDIV_A::_101)
    }
    #[doc = "PCKO/14"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCKDIV_A::_110)
    }
    #[doc = "PCKO/16"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PCKDIV_A::_111)
    }
}
#[doc = "Field `EDS` reader - Endian Select"]
pub type EDS_R = crate::BitReader<EDS_A>;
#[doc = "Endian Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDS_A {
    #[doc = "0: Little endian"]
    _0 = 0,
    #[doc = "1: Big endian"]
    _1 = 1,
}
impl From<EDS_A> for bool {
    #[inline(always)]
    fn from(variant: EDS_A) -> Self {
        variant as u8 != 0
    }
}
impl EDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDS_A {
        match self.bits {
            false => EDS_A::_0,
            true => EDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDS_A::_1
    }
}
#[doc = "Field `EDS` writer - Endian Select"]
pub type EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCCR0_SPEC, EDS_A, O>;
impl<'a, const O: u8> EDS_W<'a, O> {
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDS_A::_0)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    pub fn pcke(&self) -> PCKE_R {
        PCKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSYNC Signal Polarity Select"]
    #[inline(always)]
    pub fn vps(&self) -> VPS_R {
        VPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSYNC Signal Polarity Select"]
    #[inline(always)]
    pub fn hps(&self) -> HPS_R {
        HPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfie(&self) -> DFIE_R {
        DFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame End Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Vertical Line Number Setting Error Interrupt Enable"]
    #[inline(always)]
    pub fn verie(&self) -> VERIE_R {
        VERIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Horizontal Byte Number Setting Error Interrupt Enable"]
    #[inline(always)]
    pub fn herie(&self) -> HERIE_R {
        HERIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCKO Output Enable"]
    #[inline(always)]
    pub fn pckoe(&self) -> PCKOE_R {
        PCKOE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - PCKO Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn pckdiv(&self) -> PCKDIV_R {
        PCKDIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Endian Select"]
    #[inline(always)]
    pub fn eds(&self) -> EDS_R {
        EDS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pcke(&mut self) -> PCKE_W<0> {
        PCKE_W::new(self)
    }
    #[doc = "Bit 1 - VSYNC Signal Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn vps(&mut self) -> VPS_W<1> {
        VPS_W::new(self)
    }
    #[doc = "Bit 2 - HSYNC Signal Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn hps(&mut self) -> HPS_W<2> {
        HPS_W::new(self)
    }
    #[doc = "Bit 3 - PDC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PRST_W<3> {
        PRST_W::new(self)
    }
    #[doc = "Bit 4 - Receive Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfie(&mut self) -> DFIE_W<4> {
        DFIE_W::new(self)
    }
    #[doc = "Bit 5 - Frame End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<5> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 6 - Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovie(&mut self) -> OVIE_W<6> {
        OVIE_W::new(self)
    }
    #[doc = "Bit 7 - Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<7> {
        UDRIE_W::new(self)
    }
    #[doc = "Bit 8 - Vertical Line Number Setting Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn verie(&mut self) -> VERIE_W<8> {
        VERIE_W::new(self)
    }
    #[doc = "Bit 9 - Horizontal Byte Number Setting Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn herie(&mut self) -> HERIE_W<9> {
        HERIE_W::new(self)
    }
    #[doc = "Bit 10 - PCKO Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckoe(&mut self) -> PCKOE_W<10> {
        PCKOE_W::new(self)
    }
    #[doc = "Bits 11:13 - PCKO Frequency Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn pckdiv(&mut self) -> PCKDIV_W<11> {
        PCKDIV_W::new(self)
    }
    #[doc = "Bit 14 - Endian Select"]
    #[inline(always)]
    #[must_use]
    pub fn eds(&mut self) -> EDS_W<14> {
        EDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDC Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccr0](index.html) module"]
pub struct PCCR0_SPEC;
impl crate::RegisterSpec for PCCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pccr0::R](R) reader structure"]
impl crate::Readable for PCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pccr0::W](W) writer structure"]
impl crate::Writable for PCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCCR0 to value 0"]
impl crate::Resettable for PCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
