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
#[doc = "Channel 0 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT320.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR0` writer - Channel 0 GTCNT Count Clear"]
pub type CCLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR0_AW, O>;
impl<'a, const O: u8> CCLR0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR0_AW::_0)
    }
    #[doc = "GPT320.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR0_AW::_1)
    }
}
#[doc = "Channel 1 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT161.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR1` writer - Channel 1 GTCNT Count Clear"]
pub type CCLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR1_AW, O>;
impl<'a, const O: u8> CCLR1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR1_AW::_0)
    }
    #[doc = "GPT161.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR1_AW::_1)
    }
}
#[doc = "Channel 2 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT162.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR2_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR2` writer - Channel 2 GTCNT Count Clear"]
pub type CCLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR2_AW, O>;
impl<'a, const O: u8> CCLR2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR2_AW::_0)
    }
    #[doc = "GPT162.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR2_AW::_1)
    }
}
#[doc = "Channel 3 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT163.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR3_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR3` writer - Channel 3 GTCNT Count Clear"]
pub type CCLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR3_AW, O>;
impl<'a, const O: u8> CCLR3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR3_AW::_0)
    }
    #[doc = "GPT163.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR3_AW::_1)
    }
}
#[doc = "Channel 4 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT164.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR4_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR4` writer - Channel 4 GTCNT Count Clear"]
pub type CCLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR4_AW, O>;
impl<'a, const O: u8> CCLR4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR4_AW::_0)
    }
    #[doc = "GPT164.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR4_AW::_1)
    }
}
#[doc = "Channel 5 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT165.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR5_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR5` writer - Channel 5 GTCNT Count Clear"]
pub type CCLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR5_AW, O>;
impl<'a, const O: u8> CCLR5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR5_AW::_0)
    }
    #[doc = "GPT165.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR5_AW::_1)
    }
}
#[doc = "Channel 6 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT166.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR6_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR6` writer - Channel 6 GTCNT Count Clear"]
pub type CCLR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTCLR_SPEC, CCLR6_AW, O>;
impl<'a, const O: u8> CCLR6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCLR6_AW::_0)
    }
    #[doc = "GPT166.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCLR6_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr0(&mut self) -> CCLR0_W<0> {
        CCLR0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr1(&mut self) -> CCLR1_W<1> {
        CCLR1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr2(&mut self) -> CCLR2_W<2> {
        CCLR2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr3(&mut self) -> CCLR3_W<3> {
        CCLR3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr4(&mut self) -> CCLR4_W<4> {
        CCLR4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr5(&mut self) -> CCLR5_W<5> {
        CCLR5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr6(&mut self) -> CCLR6_W<6> {
        CCLR6_W::new(self)
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
