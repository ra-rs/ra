#[doc = "Register `ADFIFOERSCR` writer"]
pub struct W(crate::W<ADFIFOERSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOERSCR_SPEC>;
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
impl From<crate::W<ADFIFOERSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOERSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC0` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC0_AW, O>;
impl<'a, const O: u8> FIFOOVFC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC0_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC0_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC1_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC1` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC1_AW, O>;
impl<'a, const O: u8> FIFOOVFC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC1_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC1_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC2_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC2` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC2_AW, O>;
impl<'a, const O: u8> FIFOOVFC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC2_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC2_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC3_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC3` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC3_AW, O>;
impl<'a, const O: u8> FIFOOVFC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC3_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC3_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC4_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC4` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC4_AW, O>;
impl<'a, const O: u8> FIFOOVFC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC4_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC4_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC5_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC5` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC5_AW, O>;
impl<'a, const O: u8> FIFOOVFC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC5_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC5_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC6_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC6` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC6_AW, O>;
impl<'a, const O: u8> FIFOOVFC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC6_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC6_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC7_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC7` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC7_AW, O>;
impl<'a, const O: u8> FIFOOVFC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC7_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC7_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOOVFC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOOVFn is cleared"]
    _1 = 1,
}
impl From<FIFOOVFC8_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVFC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVFC8` writer - Scan Group n FIFO Overflow Flag Clear"]
pub type FIFOOVFC8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOOVFC8_AW, O>;
impl<'a, const O: u8> FIFOOVFC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOOVFC8_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOOVFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOOVFC8_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC0_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC0` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC0_AW, O>;
impl<'a, const O: u8> FIFOFLC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC0_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC0_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC1_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC1` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC1_AW, O>;
impl<'a, const O: u8> FIFOFLC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC1_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC1_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC2_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC2` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC2_AW, O>;
impl<'a, const O: u8> FIFOFLC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC2_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC2_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC3_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC3` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC3_AW, O>;
impl<'a, const O: u8> FIFOFLC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC3_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC3_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC4_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC4` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC4_AW, O>;
impl<'a, const O: u8> FIFOFLC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC4_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC4_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC5_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC5` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC5_AW, O>;
impl<'a, const O: u8> FIFOFLC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC5_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC5_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC6_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC6` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC6_AW, O>;
impl<'a, const O: u8> FIFOFLC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC6_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC6_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC7_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC7` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC7_AW, O>;
impl<'a, const O: u8> FIFOFLC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC7_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC7_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Read Request Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOFLC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADFIFOERSR.FIFOFLFn is cleared"]
    _1 = 1,
}
impl From<FIFOFLC8_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOFLC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFLC8` writer - Scan Group n FIFO Data Read Request Flag Clear"]
pub type FIFOFLC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOERSCR_SPEC, FIFOFLC8_AW, O>;
impl<'a, const O: u8> FIFOFLC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOFLC8_AW::_0)
    }
    #[doc = "ADFIFOERSR.FIFOFLFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOFLC8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc0(&mut self) -> FIFOOVFC0_W<0> {
        FIFOOVFC0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc1(&mut self) -> FIFOOVFC1_W<1> {
        FIFOOVFC1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc2(&mut self) -> FIFOOVFC2_W<2> {
        FIFOOVFC2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc3(&mut self) -> FIFOOVFC3_W<3> {
        FIFOOVFC3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc4(&mut self) -> FIFOOVFC4_W<4> {
        FIFOOVFC4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc5(&mut self) -> FIFOOVFC5_W<5> {
        FIFOOVFC5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc6(&mut self) -> FIFOOVFC6_W<6> {
        FIFOOVFC6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc7(&mut self) -> FIFOOVFC7_W<7> {
        FIFOOVFC7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n FIFO Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovfc8(&mut self) -> FIFOOVFC8_W<8> {
        FIFOOVFC8_W::new(self)
    }
    #[doc = "Bit 16 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc0(&mut self) -> FIFOFLC0_W<16> {
        FIFOFLC0_W::new(self)
    }
    #[doc = "Bit 17 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc1(&mut self) -> FIFOFLC1_W<17> {
        FIFOFLC1_W::new(self)
    }
    #[doc = "Bit 18 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc2(&mut self) -> FIFOFLC2_W<18> {
        FIFOFLC2_W::new(self)
    }
    #[doc = "Bit 19 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc3(&mut self) -> FIFOFLC3_W<19> {
        FIFOFLC3_W::new(self)
    }
    #[doc = "Bit 20 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc4(&mut self) -> FIFOFLC4_W<20> {
        FIFOFLC4_W::new(self)
    }
    #[doc = "Bit 21 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc5(&mut self) -> FIFOFLC5_W<21> {
        FIFOFLC5_W::new(self)
    }
    #[doc = "Bit 22 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc6(&mut self) -> FIFOFLC6_W<22> {
        FIFOFLC6_W::new(self)
    }
    #[doc = "Bit 23 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc7(&mut self) -> FIFOFLC7_W<23> {
        FIFOFLC7_W::new(self)
    }
    #[doc = "Bit 24 - Scan Group n FIFO Data Read Request Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoflc8(&mut self) -> FIFOFLC8_W<24> {
        FIFOFLC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Error Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifoerscr](index.html) module"]
pub struct ADFIFOERSCR_SPEC;
impl crate::RegisterSpec for ADFIFOERSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adfifoerscr::W](W) writer structure"]
impl crate::Writable for ADFIFOERSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOERSCR to value 0"]
impl crate::Resettable for ADFIFOERSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
