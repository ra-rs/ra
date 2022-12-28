#[doc = "Register `ADLIMEXSCR` writer"]
pub struct W(crate::W<ADLIMEXSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADLIMEXSCR_SPEC>;
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
impl From<crate::W<ADLIMEXSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADLIMEXSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Self-diagnosis Channel: Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMEXSR.LIMEXF0 is cleared"]
    _1 = 1,
}
impl From<LIMEXF0_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMEXF0` writer - Self-diagnosis Channel: Limiter Clip Flag Clear"]
pub type LIMEXF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMEXSCR_SPEC, LIMEXF0_AW, O>;
impl<'a, const O: u8> LIMEXF0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMEXF0_AW::_0)
    }
    #[doc = "ADLIMEXSR.LIMEXF0 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMEXF0_AW::_1)
    }
}
#[doc = "Temperature Sensor Channel: Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMEXSR.LIMEXF1 is cleared"]
    _1 = 1,
}
impl From<LIMEXF1_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMEXF1` writer - Temperature Sensor Channel: Limiter Clip Flag Clear"]
pub type LIMEXF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMEXSCR_SPEC, LIMEXF1_AW, O>;
impl<'a, const O: u8> LIMEXF1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMEXF1_AW::_0)
    }
    #[doc = "ADLIMEXSR.LIMEXF1 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMEXF1_AW::_1)
    }
}
#[doc = "Internal Reference Voltage Channel: Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMEXSR.LIMEXF2 is cleared"]
    _1 = 1,
}
impl From<LIMEXF2_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMEXF2` writer - Internal Reference Voltage Channel: Limiter Clip Flag Clear"]
pub type LIMEXF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMEXSCR_SPEC, LIMEXF2_AW, O>;
impl<'a, const O: u8> LIMEXF2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMEXF2_AW::_0)
    }
    #[doc = "ADLIMEXSR.LIMEXF2 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMEXF2_AW::_1)
    }
}
#[doc = "D/A Converter 0 Channel: Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMEXSR.LIMEXF5 is cleared"]
    _1 = 1,
}
impl From<LIMEXF5_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMEXF5` writer - D/A Converter 0 Channel: Limiter Clip Flag Clear"]
pub type LIMEXF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMEXSCR_SPEC, LIMEXF5_AW, O>;
impl<'a, const O: u8> LIMEXF5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMEXF5_AW::_0)
    }
    #[doc = "ADLIMEXSR.LIMEXF5 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMEXF5_AW::_1)
    }
}
#[doc = "D/A Converter 1 Channel: Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMEXSR.LIMEXF6 is cleared"]
    _1 = 1,
}
impl From<LIMEXF6_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMEXF6` writer - D/A Converter 1 Channel: Limiter Clip Flag Clear"]
pub type LIMEXF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMEXSCR_SPEC, LIMEXF6_AW, O>;
impl<'a, const O: u8> LIMEXF6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMEXF6_AW::_0)
    }
    #[doc = "ADLIMEXSR.LIMEXF6 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMEXF6_AW::_1)
    }
}
#[doc = "D/A Converter 2 Channel: Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMEXSR.LIMEXF7 is cleared"]
    _1 = 1,
}
impl From<LIMEXF7_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMEXF7` writer - D/A Converter 2 Channel: Limiter Clip Flag Clear"]
pub type LIMEXF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMEXSCR_SPEC, LIMEXF7_AW, O>;
impl<'a, const O: u8> LIMEXF7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMEXF7_AW::_0)
    }
    #[doc = "ADLIMEXSR.LIMEXF7 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMEXF7_AW::_1)
    }
}
#[doc = "D/A Converter 3 Channel: Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMEXF8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMEXSR.LIMEXF8 is cleared"]
    _1 = 1,
}
impl From<LIMEXF8_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMEXF8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMEXF8` writer - D/A Converter 3 Channel: Limiter Clip Flag Clear"]
pub type LIMEXF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMEXSCR_SPEC, LIMEXF8_AW, O>;
impl<'a, const O: u8> LIMEXF8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMEXF8_AW::_0)
    }
    #[doc = "ADLIMEXSR.LIMEXF8 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMEXF8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Self-diagnosis Channel: Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limexf0(&mut self) -> LIMEXF0_W<0> {
        LIMEXF0_W::new(self)
    }
    #[doc = "Bit 1 - Temperature Sensor Channel: Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limexf1(&mut self) -> LIMEXF1_W<1> {
        LIMEXF1_W::new(self)
    }
    #[doc = "Bit 2 - Internal Reference Voltage Channel: Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limexf2(&mut self) -> LIMEXF2_W<2> {
        LIMEXF2_W::new(self)
    }
    #[doc = "Bit 5 - D/A Converter 0 Channel: Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limexf5(&mut self) -> LIMEXF5_W<5> {
        LIMEXF5_W::new(self)
    }
    #[doc = "Bit 6 - D/A Converter 1 Channel: Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limexf6(&mut self) -> LIMEXF6_W<6> {
        LIMEXF6_W::new(self)
    }
    #[doc = "Bit 7 - D/A Converter 2 Channel: Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limexf7(&mut self) -> LIMEXF7_W<7> {
        LIMEXF7_W::new(self)
    }
    #[doc = "Bit 8 - D/A Converter 3 Channel: Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limexf8(&mut self) -> LIMEXF8_W<8> {
        LIMEXF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Analog Limiter Clip Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimexscr](index.html) module"]
pub struct ADLIMEXSCR_SPEC;
impl crate::RegisterSpec for ADLIMEXSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adlimexscr::W](W) writer structure"]
impl crate::Writable for ADLIMEXSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADLIMEXSCR to value 0"]
impl crate::Resettable for ADLIMEXSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
