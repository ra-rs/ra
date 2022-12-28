#[doc = "Register `ADCMPCHSCR0` writer"]
pub struct W(crate::W<ADCMPCHSCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPCHSCR0_SPEC>;
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
impl From<crate::W<ADCMPCHSCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPCHSCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC0_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC0` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC0_AW, O>;
impl<'a, const O: u8> CMPCHC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC0_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC0_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC1_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC1` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC1_AW, O>;
impl<'a, const O: u8> CMPCHC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC1_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC1_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC2_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC2` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC2_AW, O>;
impl<'a, const O: u8> CMPCHC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC2_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC2_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC3_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC3` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC3_AW, O>;
impl<'a, const O: u8> CMPCHC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC3_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC3_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC4_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC4` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC4_AW, O>;
impl<'a, const O: u8> CMPCHC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC4_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC4_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC5_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC5` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC5_AW, O>;
impl<'a, const O: u8> CMPCHC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC5_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC5_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC6_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC6` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC6_AW, O>;
impl<'a, const O: u8> CMPCHC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC6_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC6_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC7_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC7` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC7_AW, O>;
impl<'a, const O: u8> CMPCHC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC7_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC7_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC8_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC8` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC8_AW, O>;
impl<'a, const O: u8> CMPCHC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC8_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC8_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC9_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC9_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC9` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC9_AW, O>;
impl<'a, const O: u8> CMPCHC9_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC9_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC9_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC10_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC10_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC10` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC10_AW, O>;
impl<'a, const O: u8> CMPCHC10_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC10_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC10_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC11_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC11_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC11` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC11_AW, O>;
impl<'a, const O: u8> CMPCHC11_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC11_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC11_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC12_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC12_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC12` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC12_AW, O>;
impl<'a, const O: u8> CMPCHC12_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC12_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC12_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC13_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC13_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC13` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC13_AW, O>;
impl<'a, const O: u8> CMPCHC13_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC13_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC13_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC14_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC14_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC14` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC14_AW, O>;
impl<'a, const O: u8> CMPCHC14_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC14_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC14_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC15_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC15_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC15` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC15_AW, O>;
impl<'a, const O: u8> CMPCHC15_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC15_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC15_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC16_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC16_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC16` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC16_AW, O>;
impl<'a, const O: u8> CMPCHC16_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC16_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC16_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC17_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC17_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC17` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC17_AW, O>;
impl<'a, const O: u8> CMPCHC17_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC17_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC17_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC18_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC18_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC18` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC18_AW, O>;
impl<'a, const O: u8> CMPCHC18_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC18_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC18_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC19_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC19_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC19` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC19_AW, O>;
impl<'a, const O: u8> CMPCHC19_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC19_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC19_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC20_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC20_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC20` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC20_AW, O>;
impl<'a, const O: u8> CMPCHC20_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC20_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC20_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC21_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC21_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC21` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC21_AW, O>;
impl<'a, const O: u8> CMPCHC21_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC21_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC21_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC22_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC22_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC22` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC22_AW, O>;
impl<'a, const O: u8> CMPCHC22_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC22_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC22_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC23_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC23_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC23` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC23_AW, O>;
impl<'a, const O: u8> CMPCHC23_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC23_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC23_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC24_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC24_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC24` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC24_AW, O>;
impl<'a, const O: u8> CMPCHC24_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC24_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC24_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC25_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC25_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC25` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC25_AW, O>;
impl<'a, const O: u8> CMPCHC25_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC25_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC25_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC26_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC26_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC26` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC26_AW, O>;
impl<'a, const O: u8> CMPCHC26_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC26_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC26_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC27_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC27_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC27` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC27_AW, O>;
impl<'a, const O: u8> CMPCHC27_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC27_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC27_AW::_1)
    }
}
#[doc = "Analog Channel n: Compare Match Flag Clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHC28_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPCHSR0.CMPCHFn is cleared"]
    _1 = 1,
}
impl From<CMPCHC28_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPCHC28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHC28` writer - Analog Channel n: Compare Match Flag Clear bit"]
pub type CMPCHC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPCHSCR0_SPEC, CMPCHC28_AW, O>;
impl<'a, const O: u8> CMPCHC28_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHC28_AW::_0)
    }
    #[doc = "ADCMPCHSR0.CMPCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHC28_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc0(&mut self) -> CMPCHC0_W<0> {
        CMPCHC0_W::new(self)
    }
    #[doc = "Bit 1 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc1(&mut self) -> CMPCHC1_W<1> {
        CMPCHC1_W::new(self)
    }
    #[doc = "Bit 2 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc2(&mut self) -> CMPCHC2_W<2> {
        CMPCHC2_W::new(self)
    }
    #[doc = "Bit 3 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc3(&mut self) -> CMPCHC3_W<3> {
        CMPCHC3_W::new(self)
    }
    #[doc = "Bit 4 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc4(&mut self) -> CMPCHC4_W<4> {
        CMPCHC4_W::new(self)
    }
    #[doc = "Bit 5 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc5(&mut self) -> CMPCHC5_W<5> {
        CMPCHC5_W::new(self)
    }
    #[doc = "Bit 6 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc6(&mut self) -> CMPCHC6_W<6> {
        CMPCHC6_W::new(self)
    }
    #[doc = "Bit 7 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc7(&mut self) -> CMPCHC7_W<7> {
        CMPCHC7_W::new(self)
    }
    #[doc = "Bit 8 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc8(&mut self) -> CMPCHC8_W<8> {
        CMPCHC8_W::new(self)
    }
    #[doc = "Bit 9 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc9(&mut self) -> CMPCHC9_W<9> {
        CMPCHC9_W::new(self)
    }
    #[doc = "Bit 10 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc10(&mut self) -> CMPCHC10_W<10> {
        CMPCHC10_W::new(self)
    }
    #[doc = "Bit 11 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc11(&mut self) -> CMPCHC11_W<11> {
        CMPCHC11_W::new(self)
    }
    #[doc = "Bit 12 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc12(&mut self) -> CMPCHC12_W<12> {
        CMPCHC12_W::new(self)
    }
    #[doc = "Bit 13 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc13(&mut self) -> CMPCHC13_W<13> {
        CMPCHC13_W::new(self)
    }
    #[doc = "Bit 14 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc14(&mut self) -> CMPCHC14_W<14> {
        CMPCHC14_W::new(self)
    }
    #[doc = "Bit 15 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc15(&mut self) -> CMPCHC15_W<15> {
        CMPCHC15_W::new(self)
    }
    #[doc = "Bit 16 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc16(&mut self) -> CMPCHC16_W<16> {
        CMPCHC16_W::new(self)
    }
    #[doc = "Bit 17 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc17(&mut self) -> CMPCHC17_W<17> {
        CMPCHC17_W::new(self)
    }
    #[doc = "Bit 18 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc18(&mut self) -> CMPCHC18_W<18> {
        CMPCHC18_W::new(self)
    }
    #[doc = "Bit 19 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc19(&mut self) -> CMPCHC19_W<19> {
        CMPCHC19_W::new(self)
    }
    #[doc = "Bit 20 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc20(&mut self) -> CMPCHC20_W<20> {
        CMPCHC20_W::new(self)
    }
    #[doc = "Bit 21 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc21(&mut self) -> CMPCHC21_W<21> {
        CMPCHC21_W::new(self)
    }
    #[doc = "Bit 22 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc22(&mut self) -> CMPCHC22_W<22> {
        CMPCHC22_W::new(self)
    }
    #[doc = "Bit 23 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc23(&mut self) -> CMPCHC23_W<23> {
        CMPCHC23_W::new(self)
    }
    #[doc = "Bit 24 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc24(&mut self) -> CMPCHC24_W<24> {
        CMPCHC24_W::new(self)
    }
    #[doc = "Bit 25 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc25(&mut self) -> CMPCHC25_W<25> {
        CMPCHC25_W::new(self)
    }
    #[doc = "Bit 26 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc26(&mut self) -> CMPCHC26_W<26> {
        CMPCHC26_W::new(self)
    }
    #[doc = "Bit 27 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc27(&mut self) -> CMPCHC27_W<27> {
        CMPCHC27_W::new(self)
    }
    #[doc = "Bit 28 - Analog Channel n: Compare Match Flag Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchc28(&mut self) -> CMPCHC28_W<28> {
        CMPCHC28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Channel Status Clear Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpchscr0](index.html) module"]
pub struct ADCMPCHSCR0_SPEC;
impl crate::RegisterSpec for ADCMPCHSCR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adcmpchscr0::W](W) writer structure"]
impl crate::Writable for ADCMPCHSCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPCHSCR0 to value 0"]
impl crate::Resettable for ADCMPCHSCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
