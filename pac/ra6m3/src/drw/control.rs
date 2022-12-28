#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable limiter 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM1ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM1ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM1ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM1ENABLE` writer - Enable limiter 1"]
pub type LIM1ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, LIM1ENABLE_AW, O>;
impl<'a, const O: u8> LIM1ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM1ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM1ENABLE_AW::_1)
    }
}
#[doc = "Enable limiter 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM2ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM2ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM2ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM2ENABLE` writer - Enable limiter 2"]
pub type LIM2ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, LIM2ENABLE_AW, O>;
impl<'a, const O: u8> LIM2ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM2ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM2ENABLE_AW::_1)
    }
}
#[doc = "Enable limiter 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM3ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM3ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM3ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM3ENABLE` writer - Enable limiter 3"]
pub type LIM3ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, LIM3ENABLE_AW, O>;
impl<'a, const O: u8> LIM3ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM3ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM3ENABLE_AW::_1)
    }
}
#[doc = "Enable limiter 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM4ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM4ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM4ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM4ENABLE` writer - Enable limiter 4"]
pub type LIM4ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, LIM4ENABLE_AW, O>;
impl<'a, const O: u8> LIM4ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM4ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM4ENABLE_AW::_1)
    }
}
#[doc = "Enable limiter 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM5ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM5ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM5ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM5ENABLE` writer - Enable limiter 5"]
pub type LIM5ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, LIM5ENABLE_AW, O>;
impl<'a, const O: u8> LIM5ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM5ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM5ENABLE_AW::_1)
    }
}
#[doc = "Enable limiter 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM6ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM6ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM6ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM6ENABLE` writer - Enable limiter 6"]
pub type LIM6ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, LIM6ENABLE_AW, O>;
impl<'a, const O: u8> LIM6ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM6ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM6ENABLE_AW::_1)
    }
}
#[doc = "Enable quadratic coupling of limiters 1 and 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUAD1ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<QUAD1ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: QUAD1ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUAD1ENABLE` writer - Enable quadratic coupling of limiters 1 and 2"]
pub type QUAD1ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, QUAD1ENABLE_AW, O>;
impl<'a, const O: u8> QUAD1ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUAD1ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUAD1ENABLE_AW::_1)
    }
}
#[doc = "Enable quadratic coupling of limiters 3 and 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUAD2ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<QUAD2ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: QUAD2ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUAD2ENABLE` writer - Enable quadratic coupling of limiters 3 and 4"]
pub type QUAD2ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, QUAD2ENABLE_AW, O>;
impl<'a, const O: u8> QUAD2ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUAD2ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUAD2ENABLE_AW::_1)
    }
}
#[doc = "Enable quadratic coupling of limiters 5 and 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUAD3ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<QUAD3ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: QUAD3ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUAD3ENABLE` writer - Enable quadratic coupling of limiters 5 and 6"]
pub type QUAD3ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, QUAD3ENABLE_AW, O>;
impl<'a, const O: u8> QUAD3ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUAD3ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUAD3ENABLE_AW::_1)
    }
}
#[doc = "Enable limiter 1 threshold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM1THRESHOLD_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM1THRESHOLD_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM1THRESHOLD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM1THRESHOLD` writer - Enable limiter 1 threshold mode"]
pub type LIM1THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, LIM1THRESHOLD_AW, O>;
impl<'a, const O: u8> LIM1THRESHOLD_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM1THRESHOLD_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM1THRESHOLD_AW::_1)
    }
}
#[doc = "Enable limiter 2 threshold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM2THRESHOLD_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM2THRESHOLD_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM2THRESHOLD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM2THRESHOLD` writer - Enable limiter 2 threshold mode"]
pub type LIM2THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, LIM2THRESHOLD_AW, O>;
impl<'a, const O: u8> LIM2THRESHOLD_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM2THRESHOLD_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM2THRESHOLD_AW::_1)
    }
}
#[doc = "Enable limiter 3 threshold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM3THRESHOLD_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM3THRESHOLD_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM3THRESHOLD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM3THRESHOLD` writer - Enable limiter 3 threshold mode"]
pub type LIM3THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, LIM3THRESHOLD_AW, O>;
impl<'a, const O: u8> LIM3THRESHOLD_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM3THRESHOLD_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM3THRESHOLD_AW::_1)
    }
}
#[doc = "Enable limiter 4 threshold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM4THRESHOLD_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM4THRESHOLD_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM4THRESHOLD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM4THRESHOLD` writer - Enable limiter 4 threshold mode"]
pub type LIM4THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, LIM4THRESHOLD_AW, O>;
impl<'a, const O: u8> LIM4THRESHOLD_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM4THRESHOLD_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM4THRESHOLD_AW::_1)
    }
}
#[doc = "Enable limiter 5 threshold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM5THRESHOLD_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM5THRESHOLD_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM5THRESHOLD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM5THRESHOLD` writer - Enable limiter 5 threshold mode"]
pub type LIM5THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, LIM5THRESHOLD_AW, O>;
impl<'a, const O: u8> LIM5THRESHOLD_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM5THRESHOLD_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM5THRESHOLD_AW::_1)
    }
}
#[doc = "Enable limiter 6 threshold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM6THRESHOLD_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<LIM6THRESHOLD_AW> for bool {
    #[inline(always)]
    fn from(variant: LIM6THRESHOLD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIM6THRESHOLD` writer - Enable limiter 6 threshold mode"]
pub type LIM6THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, LIM6THRESHOLD_AW, O>;
impl<'a, const O: u8> LIM6THRESHOLD_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIM6THRESHOLD_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIM6THRESHOLD_AW::_1)
    }
}
#[doc = "Enable band postprocess for limiter 1 (see L1BAND)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BAND1ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<BAND1ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: BAND1ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BAND1ENABLE` writer - Enable band postprocess for limiter 1 (see L1BAND)"]
pub type BAND1ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, BAND1ENABLE_AW, O>;
impl<'a, const O: u8> BAND1ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BAND1ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BAND1ENABLE_AW::_1)
    }
}
#[doc = "Enable band postprocess for limiter 1 (see L1BAND)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BAND2ENABLE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<BAND2ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: BAND2ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BAND2ENABLE` writer - Enable band postprocess for limiter 1 (see L1BAND)"]
pub type BAND2ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, BAND2ENABLE_AW, O>;
impl<'a, const O: u8> BAND2ENABLE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BAND2ENABLE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BAND2ENABLE_AW::_1)
    }
}
#[doc = "Combine limter 1 & 2 as union (output is called A)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNION12_AW {
    #[doc = "0: minimum/intersect"]
    _0 = 0,
    #[doc = "1: maximum/union"]
    _1 = 1,
}
impl From<UNION12_AW> for bool {
    #[inline(always)]
    fn from(variant: UNION12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNION12` writer - Combine limter 1 & 2 as union (output is called A)"]
pub type UNION12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, UNION12_AW, O>;
impl<'a, const O: u8> UNION12_W<'a, O> {
    #[doc = "minimum/intersect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNION12_AW::_0)
    }
    #[doc = "maximum/union"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNION12_AW::_1)
    }
}
#[doc = "Combine limter 3 & 4 as union (output is called B)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNION34_AW {
    #[doc = "0: minimum/intersect"]
    _0 = 0,
    #[doc = "1: maximum/union"]
    _1 = 1,
}
impl From<UNION34_AW> for bool {
    #[inline(always)]
    fn from(variant: UNION34_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNION34` writer - Combine limter 3 & 4 as union (output is called B)"]
pub type UNION34_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, UNION34_AW, O>;
impl<'a, const O: u8> UNION34_W<'a, O> {
    #[doc = "minimum/intersect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNION34_AW::_0)
    }
    #[doc = "maximum/union"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNION34_AW::_1)
    }
}
#[doc = "Combine limter 5 & 6 as union (output is called D)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNION56_AW {
    #[doc = "0: minimum/intersect"]
    _0 = 0,
    #[doc = "1: maximum/union"]
    _1 = 1,
}
impl From<UNION56_AW> for bool {
    #[inline(always)]
    fn from(variant: UNION56_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNION56` writer - Combine limter 5 & 6 as union (output is called D)"]
pub type UNION56_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, UNION56_AW, O>;
impl<'a, const O: u8> UNION56_W<'a, O> {
    #[doc = "minimum/intersect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNION56_AW::_0)
    }
    #[doc = "maximum/union"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNION56_AW::_1)
    }
}
#[doc = "Combine outputs A & B as union (output is called C)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNIONAB_AW {
    #[doc = "0: minimum/intersect"]
    _0 = 0,
    #[doc = "1: maximum/union"]
    _1 = 1,
}
impl From<UNIONAB_AW> for bool {
    #[inline(always)]
    fn from(variant: UNIONAB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNIONAB` writer - Combine outputs A & B as union (output is called C)"]
pub type UNIONAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, UNIONAB_AW, O>;
impl<'a, const O: u8> UNIONAB_W<'a, O> {
    #[doc = "minimum/intersect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNIONAB_AW::_0)
    }
    #[doc = "maximum/union"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNIONAB_AW::_1)
    }
}
#[doc = "Combine outputs C & D as union (output is final)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNIONCD_AW {
    #[doc = "0: minimum/intersect"]
    _0 = 0,
    #[doc = "1: maximum/union"]
    _1 = 1,
}
impl From<UNIONCD_AW> for bool {
    #[inline(always)]
    fn from(variant: UNIONCD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNIONCD` writer - Combine outputs C & D as union (output is final)"]
pub type UNIONCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, UNIONCD_AW, O>;
impl<'a, const O: u8> UNIONCD_W<'a, O> {
    #[doc = "minimum/intersect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNIONCD_AW::_0)
    }
    #[doc = "maximum/union"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNIONCD_AW::_1)
    }
}
#[doc = "Shape is horizontally convex, only a single span per scanline\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPANABORT_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<SPANABORT_AW> for bool {
    #[inline(always)]
    fn from(variant: SPANABORT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPANABORT` writer - Shape is horizontally convex, only a single span per scanline"]
pub type SPANABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, SPANABORT_AW, O>;
impl<'a, const O: u8> SPANABORT_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPANABORT_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPANABORT_AW::_1)
    }
}
#[doc = "Nextline span start is always equal or left to current-line span start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPANSTORE_AW {
    #[doc = "0: disabled"]
    _0 = 0,
    #[doc = "1: enabled"]
    _1 = 1,
}
impl From<SPANSTORE_AW> for bool {
    #[inline(always)]
    fn from(variant: SPANSTORE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPANSTORE` writer - Nextline span start is always equal or left to current-line span start"]
pub type SPANSTORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, SPANSTORE_AW, O>;
impl<'a, const O: u8> SPANSTORE_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPANSTORE_AW::_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPANSTORE_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Enable limiter 1"]
    #[inline(always)]
    #[must_use]
    pub fn lim1enable(&mut self) -> LIM1ENABLE_W<0> {
        LIM1ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable limiter 2"]
    #[inline(always)]
    #[must_use]
    pub fn lim2enable(&mut self) -> LIM2ENABLE_W<1> {
        LIM2ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Enable limiter 3"]
    #[inline(always)]
    #[must_use]
    pub fn lim3enable(&mut self) -> LIM3ENABLE_W<2> {
        LIM3ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Enable limiter 4"]
    #[inline(always)]
    #[must_use]
    pub fn lim4enable(&mut self) -> LIM4ENABLE_W<3> {
        LIM4ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Enable limiter 5"]
    #[inline(always)]
    #[must_use]
    pub fn lim5enable(&mut self) -> LIM5ENABLE_W<4> {
        LIM5ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Enable limiter 6"]
    #[inline(always)]
    #[must_use]
    pub fn lim6enable(&mut self) -> LIM6ENABLE_W<5> {
        LIM6ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Enable quadratic coupling of limiters 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn quad1enable(&mut self) -> QUAD1ENABLE_W<6> {
        QUAD1ENABLE_W::new(self)
    }
    #[doc = "Bit 7 - Enable quadratic coupling of limiters 3 and 4"]
    #[inline(always)]
    #[must_use]
    pub fn quad2enable(&mut self) -> QUAD2ENABLE_W<7> {
        QUAD2ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Enable quadratic coupling of limiters 5 and 6"]
    #[inline(always)]
    #[must_use]
    pub fn quad3enable(&mut self) -> QUAD3ENABLE_W<8> {
        QUAD3ENABLE_W::new(self)
    }
    #[doc = "Bit 9 - Enable limiter 1 threshold mode"]
    #[inline(always)]
    #[must_use]
    pub fn lim1threshold(&mut self) -> LIM1THRESHOLD_W<9> {
        LIM1THRESHOLD_W::new(self)
    }
    #[doc = "Bit 10 - Enable limiter 2 threshold mode"]
    #[inline(always)]
    #[must_use]
    pub fn lim2threshold(&mut self) -> LIM2THRESHOLD_W<10> {
        LIM2THRESHOLD_W::new(self)
    }
    #[doc = "Bit 11 - Enable limiter 3 threshold mode"]
    #[inline(always)]
    #[must_use]
    pub fn lim3threshold(&mut self) -> LIM3THRESHOLD_W<11> {
        LIM3THRESHOLD_W::new(self)
    }
    #[doc = "Bit 12 - Enable limiter 4 threshold mode"]
    #[inline(always)]
    #[must_use]
    pub fn lim4threshold(&mut self) -> LIM4THRESHOLD_W<12> {
        LIM4THRESHOLD_W::new(self)
    }
    #[doc = "Bit 13 - Enable limiter 5 threshold mode"]
    #[inline(always)]
    #[must_use]
    pub fn lim5threshold(&mut self) -> LIM5THRESHOLD_W<13> {
        LIM5THRESHOLD_W::new(self)
    }
    #[doc = "Bit 14 - Enable limiter 6 threshold mode"]
    #[inline(always)]
    #[must_use]
    pub fn lim6threshold(&mut self) -> LIM6THRESHOLD_W<14> {
        LIM6THRESHOLD_W::new(self)
    }
    #[doc = "Bit 15 - Enable band postprocess for limiter 1 (see L1BAND)"]
    #[inline(always)]
    #[must_use]
    pub fn band1enable(&mut self) -> BAND1ENABLE_W<15> {
        BAND1ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Enable band postprocess for limiter 1 (see L1BAND)"]
    #[inline(always)]
    #[must_use]
    pub fn band2enable(&mut self) -> BAND2ENABLE_W<16> {
        BAND2ENABLE_W::new(self)
    }
    #[doc = "Bit 17 - Combine limter 1 & 2 as union (output is called A)"]
    #[inline(always)]
    #[must_use]
    pub fn union12(&mut self) -> UNION12_W<17> {
        UNION12_W::new(self)
    }
    #[doc = "Bit 18 - Combine limter 3 & 4 as union (output is called B)"]
    #[inline(always)]
    #[must_use]
    pub fn union34(&mut self) -> UNION34_W<18> {
        UNION34_W::new(self)
    }
    #[doc = "Bit 19 - Combine limter 5 & 6 as union (output is called D)"]
    #[inline(always)]
    #[must_use]
    pub fn union56(&mut self) -> UNION56_W<19> {
        UNION56_W::new(self)
    }
    #[doc = "Bit 20 - Combine outputs A & B as union (output is called C)"]
    #[inline(always)]
    #[must_use]
    pub fn unionab(&mut self) -> UNIONAB_W<20> {
        UNIONAB_W::new(self)
    }
    #[doc = "Bit 21 - Combine outputs C & D as union (output is final)"]
    #[inline(always)]
    #[must_use]
    pub fn unioncd(&mut self) -> UNIONCD_W<21> {
        UNIONCD_W::new(self)
    }
    #[doc = "Bit 22 - Shape is horizontally convex, only a single span per scanline"]
    #[inline(always)]
    #[must_use]
    pub fn spanabort(&mut self) -> SPANABORT_W<22> {
        SPANABORT_W::new(self)
    }
    #[doc = "Bit 23 - Nextline span start is always equal or left to current-line span start"]
    #[inline(always)]
    #[must_use]
    pub fn spanstore(&mut self) -> SPANSTORE_W<23> {
        SPANSTORE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Geometry Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
