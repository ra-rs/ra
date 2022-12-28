#[doc = "Register `ADLIMCHSCR0` writer"]
pub struct W(crate::W<ADLIMCHSCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADLIMCHSCR0_SPEC>;
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
impl From<crate::W<ADLIMCHSCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADLIMCHSCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC0_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC0` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC0_AW, O>;
impl<'a, const O: u8> LIMCHC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC0_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC0_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC1_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC1` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC1_AW, O>;
impl<'a, const O: u8> LIMCHC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC1_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC1_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC2_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC2` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC2_AW, O>;
impl<'a, const O: u8> LIMCHC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC2_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC2_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC3_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC3` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC3_AW, O>;
impl<'a, const O: u8> LIMCHC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC3_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC3_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC4_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC4` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC4_AW, O>;
impl<'a, const O: u8> LIMCHC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC4_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC4_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC5_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC5` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC5_AW, O>;
impl<'a, const O: u8> LIMCHC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC5_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC5_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC6_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC6` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC6_AW, O>;
impl<'a, const O: u8> LIMCHC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC6_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC6_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC7_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC7` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC7_AW, O>;
impl<'a, const O: u8> LIMCHC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC7_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC7_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC8_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC8` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC8_AW, O>;
impl<'a, const O: u8> LIMCHC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC8_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC8_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC9_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC9_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC9` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC9_AW, O>;
impl<'a, const O: u8> LIMCHC9_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC9_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC9_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC10_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC10_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC10` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC10_AW, O>;
impl<'a, const O: u8> LIMCHC10_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC10_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC10_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC11_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC11_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC11` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC11_AW, O>;
impl<'a, const O: u8> LIMCHC11_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC11_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC11_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC12_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC12_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC12` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC12_AW, O>;
impl<'a, const O: u8> LIMCHC12_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC12_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC12_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC13_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC13_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC13` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC13_AW, O>;
impl<'a, const O: u8> LIMCHC13_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC13_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC13_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC14_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC14_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC14` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC14_AW, O>;
impl<'a, const O: u8> LIMCHC14_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC14_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC14_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC15_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC15_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC15` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC15_AW, O>;
impl<'a, const O: u8> LIMCHC15_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC15_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC15_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC16_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC16_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC16` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC16_AW, O>;
impl<'a, const O: u8> LIMCHC16_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC16_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC16_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC17_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC17_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC17` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC17_AW, O>;
impl<'a, const O: u8> LIMCHC17_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC17_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC17_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC18_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC18_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC18` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC18_AW, O>;
impl<'a, const O: u8> LIMCHC18_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC18_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC18_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC19_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC19_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC19` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC19_AW, O>;
impl<'a, const O: u8> LIMCHC19_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC19_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC19_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC20_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC20_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC20` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC20_AW, O>;
impl<'a, const O: u8> LIMCHC20_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC20_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC20_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC21_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC21_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC21` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC21_AW, O>;
impl<'a, const O: u8> LIMCHC21_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC21_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC21_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC22_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC22_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC22` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC22_AW, O>;
impl<'a, const O: u8> LIMCHC22_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC22_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC22_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC23_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC23_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC23` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC23_AW, O>;
impl<'a, const O: u8> LIMCHC23_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC23_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC23_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC24_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC24_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC24` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC24_AW, O>;
impl<'a, const O: u8> LIMCHC24_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC24_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC24_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC25_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC25_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC25` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC25_AW, O>;
impl<'a, const O: u8> LIMCHC25_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC25_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC25_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC26_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC26_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC26` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC26_AW, O>;
impl<'a, const O: u8> LIMCHC26_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC26_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC26_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC27_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC27_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC27` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC27_AW, O>;
impl<'a, const O: u8> LIMCHC27_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC27_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC27_AW::_1)
    }
}
#[doc = "Analog Channel n Limiter Clip Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMCHC28_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMCHSR0.LIMCHFn is cleared"]
    _1 = 1,
}
impl From<LIMCHC28_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMCHC28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMCHC28` writer - Analog Channel n Limiter Clip Flag Clear bit"]
pub type LIMCHC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMCHSCR0_SPEC, LIMCHC28_AW, O>;
impl<'a, const O: u8> LIMCHC28_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMCHC28_AW::_0)
    }
    #[doc = "ADLIMCHSR0.LIMCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMCHC28_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc0(&mut self) -> LIMCHC0_W<0> {
        LIMCHC0_W::new(self)
    }
    #[doc = "Bit 1 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc1(&mut self) -> LIMCHC1_W<1> {
        LIMCHC1_W::new(self)
    }
    #[doc = "Bit 2 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc2(&mut self) -> LIMCHC2_W<2> {
        LIMCHC2_W::new(self)
    }
    #[doc = "Bit 3 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc3(&mut self) -> LIMCHC3_W<3> {
        LIMCHC3_W::new(self)
    }
    #[doc = "Bit 4 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc4(&mut self) -> LIMCHC4_W<4> {
        LIMCHC4_W::new(self)
    }
    #[doc = "Bit 5 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc5(&mut self) -> LIMCHC5_W<5> {
        LIMCHC5_W::new(self)
    }
    #[doc = "Bit 6 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc6(&mut self) -> LIMCHC6_W<6> {
        LIMCHC6_W::new(self)
    }
    #[doc = "Bit 7 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc7(&mut self) -> LIMCHC7_W<7> {
        LIMCHC7_W::new(self)
    }
    #[doc = "Bit 8 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc8(&mut self) -> LIMCHC8_W<8> {
        LIMCHC8_W::new(self)
    }
    #[doc = "Bit 9 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc9(&mut self) -> LIMCHC9_W<9> {
        LIMCHC9_W::new(self)
    }
    #[doc = "Bit 10 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc10(&mut self) -> LIMCHC10_W<10> {
        LIMCHC10_W::new(self)
    }
    #[doc = "Bit 11 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc11(&mut self) -> LIMCHC11_W<11> {
        LIMCHC11_W::new(self)
    }
    #[doc = "Bit 12 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc12(&mut self) -> LIMCHC12_W<12> {
        LIMCHC12_W::new(self)
    }
    #[doc = "Bit 13 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc13(&mut self) -> LIMCHC13_W<13> {
        LIMCHC13_W::new(self)
    }
    #[doc = "Bit 14 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc14(&mut self) -> LIMCHC14_W<14> {
        LIMCHC14_W::new(self)
    }
    #[doc = "Bit 15 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc15(&mut self) -> LIMCHC15_W<15> {
        LIMCHC15_W::new(self)
    }
    #[doc = "Bit 16 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc16(&mut self) -> LIMCHC16_W<16> {
        LIMCHC16_W::new(self)
    }
    #[doc = "Bit 17 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc17(&mut self) -> LIMCHC17_W<17> {
        LIMCHC17_W::new(self)
    }
    #[doc = "Bit 18 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc18(&mut self) -> LIMCHC18_W<18> {
        LIMCHC18_W::new(self)
    }
    #[doc = "Bit 19 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc19(&mut self) -> LIMCHC19_W<19> {
        LIMCHC19_W::new(self)
    }
    #[doc = "Bit 20 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc20(&mut self) -> LIMCHC20_W<20> {
        LIMCHC20_W::new(self)
    }
    #[doc = "Bit 21 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc21(&mut self) -> LIMCHC21_W<21> {
        LIMCHC21_W::new(self)
    }
    #[doc = "Bit 22 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc22(&mut self) -> LIMCHC22_W<22> {
        LIMCHC22_W::new(self)
    }
    #[doc = "Bit 23 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc23(&mut self) -> LIMCHC23_W<23> {
        LIMCHC23_W::new(self)
    }
    #[doc = "Bit 24 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc24(&mut self) -> LIMCHC24_W<24> {
        LIMCHC24_W::new(self)
    }
    #[doc = "Bit 25 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc25(&mut self) -> LIMCHC25_W<25> {
        LIMCHC25_W::new(self)
    }
    #[doc = "Bit 26 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc26(&mut self) -> LIMCHC26_W<26> {
        LIMCHC26_W::new(self)
    }
    #[doc = "Bit 27 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc27(&mut self) -> LIMCHC27_W<27> {
        LIMCHC27_W::new(self)
    }
    #[doc = "Bit 28 - Analog Channel n Limiter Clip Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn limchc28(&mut self) -> LIMCHC28_W<28> {
        LIMCHC28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limiter Clip Channel Status Clear Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimchscr0](index.html) module"]
pub struct ADLIMCHSCR0_SPEC;
impl crate::RegisterSpec for ADLIMCHSCR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adlimchscr0::W](W) writer structure"]
impl crate::Writable for ADLIMCHSCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADLIMCHSCR0 to value 0"]
impl crate::Resettable for ADLIMCHSCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
