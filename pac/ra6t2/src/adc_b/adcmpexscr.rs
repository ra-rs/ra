#[doc = "Register `ADCMPEXSCR` writer"]
pub struct W(crate::W<ADCMPEXSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPEXSCR_SPEC>;
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
impl From<crate::W<ADCMPEXSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPEXSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Self-diagnosis Channel: Compare Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPEXSR.CMPCHF0 is cleared"]
    _1 = 1,
}
impl From<CMPEXC0_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPEXC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEXC0` writer - Self-diagnosis Channel: Compare Match Flag Clear"]
pub type CMPEXC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPEXSCR_SPEC, CMPEXC0_AW, O>;
impl<'a, const O: u8> CMPEXC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEXC0_AW::_0)
    }
    #[doc = "ADCMPEXSR.CMPCHF0 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEXC0_AW::_1)
    }
}
#[doc = "Temperature Sensor Channel: Compare Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPEXSR.CMPCHF1 is cleared"]
    _1 = 1,
}
impl From<CMPEXC1_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPEXC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEXC1` writer - Temperature Sensor Channel: Compare Match Flag Clear"]
pub type CMPEXC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPEXSCR_SPEC, CMPEXC1_AW, O>;
impl<'a, const O: u8> CMPEXC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEXC1_AW::_0)
    }
    #[doc = "ADCMPEXSR.CMPCHF1 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEXC1_AW::_1)
    }
}
#[doc = "Internal Reference Voltage Channel: Compare Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPEXSR.CMPCHF2 is cleared"]
    _1 = 1,
}
impl From<CMPEXC2_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPEXC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEXC2` writer - Internal Reference Voltage Channel: Compare Match Flag Clear"]
pub type CMPEXC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPEXSCR_SPEC, CMPEXC2_AW, O>;
impl<'a, const O: u8> CMPEXC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEXC2_AW::_0)
    }
    #[doc = "ADCMPEXSR.CMPCHF2 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEXC2_AW::_1)
    }
}
#[doc = "D/A Converter 0 Channel: Compare Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPEXSR.CMPCHF5 is cleared"]
    _1 = 1,
}
impl From<CMPEXC5_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPEXC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEXC5` writer - D/A Converter 0 Channel: Compare Match Flag Clear"]
pub type CMPEXC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPEXSCR_SPEC, CMPEXC5_AW, O>;
impl<'a, const O: u8> CMPEXC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEXC5_AW::_0)
    }
    #[doc = "ADCMPEXSR.CMPCHF5 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEXC5_AW::_1)
    }
}
#[doc = "D/A Converter 1 Channel: Compare Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPEXSR.CMPCHF6 is cleared"]
    _1 = 1,
}
impl From<CMPEXC6_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPEXC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEXC6` writer - D/A Converter 1 Channel: Compare Match Flag Clear"]
pub type CMPEXC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPEXSCR_SPEC, CMPEXC6_AW, O>;
impl<'a, const O: u8> CMPEXC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEXC6_AW::_0)
    }
    #[doc = "ADCMPEXSR.CMPCHF6 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEXC6_AW::_1)
    }
}
#[doc = "D/A Converter 2 Channel: Compare Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPEXSR.CMPCHF7 is cleared"]
    _1 = 1,
}
impl From<CMPEXC7_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPEXC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEXC7` writer - D/A Converter 2 Channel: Compare Match Flag Clear"]
pub type CMPEXC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPEXSCR_SPEC, CMPEXC7_AW, O>;
impl<'a, const O: u8> CMPEXC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEXC7_AW::_0)
    }
    #[doc = "ADCMPEXSR.CMPCHF7 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEXC7_AW::_1)
    }
}
#[doc = "D/A Converter 3 Channel: Compare Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPEXC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPEXSR.CMPCHF8 is cleared"]
    _1 = 1,
}
impl From<CMPEXC8_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPEXC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEXC8` writer - D/A Converter 3 Channel: Compare Match Flag Clear"]
pub type CMPEXC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPEXSCR_SPEC, CMPEXC8_AW, O>;
impl<'a, const O: u8> CMPEXC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEXC8_AW::_0)
    }
    #[doc = "ADCMPEXSR.CMPCHF8 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEXC8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Self-diagnosis Channel: Compare Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmpexc0(&mut self) -> CMPEXC0_W<0> {
        CMPEXC0_W::new(self)
    }
    #[doc = "Bit 1 - Temperature Sensor Channel: Compare Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmpexc1(&mut self) -> CMPEXC1_W<1> {
        CMPEXC1_W::new(self)
    }
    #[doc = "Bit 2 - Internal Reference Voltage Channel: Compare Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmpexc2(&mut self) -> CMPEXC2_W<2> {
        CMPEXC2_W::new(self)
    }
    #[doc = "Bit 5 - D/A Converter 0 Channel: Compare Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmpexc5(&mut self) -> CMPEXC5_W<5> {
        CMPEXC5_W::new(self)
    }
    #[doc = "Bit 6 - D/A Converter 1 Channel: Compare Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmpexc6(&mut self) -> CMPEXC6_W<6> {
        CMPEXC6_W::new(self)
    }
    #[doc = "Bit 7 - D/A Converter 2 Channel: Compare Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmpexc7(&mut self) -> CMPEXC7_W<7> {
        CMPEXC7_W::new(self)
    }
    #[doc = "Bit 8 - D/A Converter 3 Channel: Compare Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmpexc8(&mut self) -> CMPEXC8_W<8> {
        CMPEXC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Analog Compare Match Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpexscr](index.html) module"]
pub struct ADCMPEXSCR_SPEC;
impl crate::RegisterSpec for ADCMPEXSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adcmpexscr::W](W) writer structure"]
impl crate::Writable for ADCMPEXSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPEXSCR to value 0"]
impl crate::Resettable for ADCMPEXSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
