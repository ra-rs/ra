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
