#[doc = "Register `GTCLR` writer"]
pub struct W(crate::W<GTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCLR_SPEC>;
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
impl From<crate::W<GTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR0_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR0` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR0_AW, O>;
impl<'a, const O: u8> CCLR0_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR0_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR0_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR1_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR1` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR1_AW, O>;
impl<'a, const O: u8> CCLR1_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR1_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR1_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR2_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR2_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR2` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR2_AW, O>;
impl<'a, const O: u8> CCLR2_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR2_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR2_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR3_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR3_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR3` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR3_AW, O>;
impl<'a, const O: u8> CCLR3_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR3_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR3_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR4_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR4_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR4` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR4_AW, O>;
impl<'a, const O: u8> CCLR4_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR4_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR4_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR5_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR5_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR5` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR5_AW, O>;
impl<'a, const O: u8> CCLR5_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR5_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR5_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR6_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR6_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR6` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR6_AW, O>;
impl<'a, const O: u8> CCLR6_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR6_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR6_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR7_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR7_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR7` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR7_AW, O>;
impl<'a, const O: u8> CCLR7_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR7_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR7_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR8_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR8_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR8` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR8_AW, O>;
impl<'a, const O: u8> CCLR8_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR8_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR8_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR9_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR9_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR9` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR9_AW, O>;
impl<'a, const O: u8> CCLR9_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR9_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR9_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR10_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR10_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR10` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR10_AW, O>;
impl<'a, const O: u8> CCLR10_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR10_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR10_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR11_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR11_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR11` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR11_AW, O>;
impl<'a, const O: u8> CCLR11_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR11_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR11_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR12_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR12_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR12` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR12_AW, O>;
impl<'a, const O: u8> CCLR12_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR12_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR12_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR13_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR13_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR13` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR13_AW, O>;
impl<'a, const O: u8> CCLR13_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR13_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR13_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR14_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR14_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR14` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR14_AW, O>;
impl<'a, const O: u8> CCLR14_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR14_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR14_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR15_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR15_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR15` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR15_AW, O>;
impl<'a, const O: u8> CCLR15_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR15_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR15_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR16_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR16_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR16` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR16_AW, O>;
impl<'a, const O: u8> CCLR16_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR16_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR16_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR17_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR17_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR17` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR17_AW, O>;
impl<'a, const O: u8> CCLR17_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR17_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR17_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR18_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR18_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR18` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR18_AW, O>;
impl<'a, const O: u8> CCLR18_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR18_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR18_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR19_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR19_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR19` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR19_AW, O>;
impl<'a, const O: u8> CCLR19_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR19_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR19_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR20_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR20_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR20` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR20_AW, O>;
impl<'a, const O: u8> CCLR20_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR20_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR20_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR21_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR21_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR21` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR21_AW, O>;
impl<'a, const O: u8> CCLR21_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR21_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR21_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR22_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR22_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR22` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR22_AW, O>;
impl<'a, const O: u8> CCLR22_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR22_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR22_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR23_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR23_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR23` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR23_AW, O>;
impl<'a, const O: u8> CCLR23_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR23_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR23_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR24_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR24_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR24` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR24_AW, O>;
impl<'a, const O: u8> CCLR24_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR24_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR24_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR25_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR25_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR25` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR25_AW, O>;
impl<'a, const O: u8> CCLR25_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR25_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR25_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR26_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR26_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR26` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR26_AW, O>;
impl<'a, const O: u8> CCLR26_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR26_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR26_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR27_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR27_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR27` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR27_AW, O>;
impl<'a, const O: u8> CCLR27_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR27_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR27_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR28_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR28_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR28` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR28_AW, O>;
impl<'a, const O: u8> CCLR28_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR28_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR28_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR29_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR29_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR29` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR29_AW, O>;
impl<'a, const O: u8> CCLR29_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR29_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR29_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR30_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR30_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR30` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR30_AW, O>;
impl<'a, const O: u8> CCLR30_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR30_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR30_AW::_1)
    }
}
#[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR31_AW {
    #[doc = "0: GTCNT counter is not cleared"]
    _0 = 0,
    #[doc = "1: GTCNT counter is cleared"]
    _1 = 1,
}
impl From<CCLR31_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR31` writer - Channel n GTCNT Count Clear (n : the same as bit position value)"]
pub type CCLR31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR31_AW, O>;
impl<'a, const O: u8> CCLR31_W<'a, O> {
    #[doc = "GTCNT counter is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR31_AW::_0)
    }
    #[doc = "GTCNT counter is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR31_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr0(&mut self) -> CCLR0_W<0> {
        CCLR0_W::new(self)
    }
    #[doc = "Bit 1 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr1(&mut self) -> CCLR1_W<1> {
        CCLR1_W::new(self)
    }
    #[doc = "Bit 2 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr2(&mut self) -> CCLR2_W<2> {
        CCLR2_W::new(self)
    }
    #[doc = "Bit 3 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr3(&mut self) -> CCLR3_W<3> {
        CCLR3_W::new(self)
    }
    #[doc = "Bit 4 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr4(&mut self) -> CCLR4_W<4> {
        CCLR4_W::new(self)
    }
    #[doc = "Bit 5 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr5(&mut self) -> CCLR5_W<5> {
        CCLR5_W::new(self)
    }
    #[doc = "Bit 6 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr6(&mut self) -> CCLR6_W<6> {
        CCLR6_W::new(self)
    }
    #[doc = "Bit 7 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr7(&mut self) -> CCLR7_W<7> {
        CCLR7_W::new(self)
    }
    #[doc = "Bit 8 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr8(&mut self) -> CCLR8_W<8> {
        CCLR8_W::new(self)
    }
    #[doc = "Bit 9 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr9(&mut self) -> CCLR9_W<9> {
        CCLR9_W::new(self)
    }
    #[doc = "Bit 10 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr10(&mut self) -> CCLR10_W<10> {
        CCLR10_W::new(self)
    }
    #[doc = "Bit 11 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr11(&mut self) -> CCLR11_W<11> {
        CCLR11_W::new(self)
    }
    #[doc = "Bit 12 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr12(&mut self) -> CCLR12_W<12> {
        CCLR12_W::new(self)
    }
    #[doc = "Bit 13 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr13(&mut self) -> CCLR13_W<13> {
        CCLR13_W::new(self)
    }
    #[doc = "Bit 14 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr14(&mut self) -> CCLR14_W<14> {
        CCLR14_W::new(self)
    }
    #[doc = "Bit 15 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr15(&mut self) -> CCLR15_W<15> {
        CCLR15_W::new(self)
    }
    #[doc = "Bit 16 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr16(&mut self) -> CCLR16_W<16> {
        CCLR16_W::new(self)
    }
    #[doc = "Bit 17 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr17(&mut self) -> CCLR17_W<17> {
        CCLR17_W::new(self)
    }
    #[doc = "Bit 18 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr18(&mut self) -> CCLR18_W<18> {
        CCLR18_W::new(self)
    }
    #[doc = "Bit 19 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr19(&mut self) -> CCLR19_W<19> {
        CCLR19_W::new(self)
    }
    #[doc = "Bit 20 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr20(&mut self) -> CCLR20_W<20> {
        CCLR20_W::new(self)
    }
    #[doc = "Bit 21 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr21(&mut self) -> CCLR21_W<21> {
        CCLR21_W::new(self)
    }
    #[doc = "Bit 22 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr22(&mut self) -> CCLR22_W<22> {
        CCLR22_W::new(self)
    }
    #[doc = "Bit 23 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr23(&mut self) -> CCLR23_W<23> {
        CCLR23_W::new(self)
    }
    #[doc = "Bit 24 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr24(&mut self) -> CCLR24_W<24> {
        CCLR24_W::new(self)
    }
    #[doc = "Bit 25 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr25(&mut self) -> CCLR25_W<25> {
        CCLR25_W::new(self)
    }
    #[doc = "Bit 26 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr26(&mut self) -> CCLR26_W<26> {
        CCLR26_W::new(self)
    }
    #[doc = "Bit 27 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr27(&mut self) -> CCLR27_W<27> {
        CCLR27_W::new(self)
    }
    #[doc = "Bit 28 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr28(&mut self) -> CCLR28_W<28> {
        CCLR28_W::new(self)
    }
    #[doc = "Bit 29 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr29(&mut self) -> CCLR29_W<29> {
        CCLR29_W::new(self)
    }
    #[doc = "Bit 30 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr30(&mut self) -> CCLR30_W<30> {
        CCLR30_W::new(self)
    }
    #[doc = "Bit 31 - Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    #[must_use]
    pub fn cclr31(&mut self) -> CCLR31_W<31> {
        CCLR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Software Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtclr](index.html) module"]
pub struct GTCLR_SPEC;
impl crate::RegisterSpec for GTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gtclr::W](W) writer structure"]
impl crate::Writable for GTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCLR to value 0"]
impl crate::Resettable for GTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
