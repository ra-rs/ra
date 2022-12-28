#[doc = "Register `ADOVFEXSCR` writer"]
pub struct W(crate::W<ADOVFEXSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADOVFEXSCR_SPEC>;
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
impl From<crate::W<ADOVFEXSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADOVFEXSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Self-diagnosis Channel: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFEXSR.OVFEXF0 is cleared"]
    _1 = 1,
}
impl From<OVFEXC0_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFEXC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEXC0` writer - Self-diagnosis Channel: Overflow Flag Clear"]
pub type OVFEXC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFEXSCR_SPEC, OVFEXC0_AW, O>;
impl<'a, const O: u8> OVFEXC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFEXC0_AW::_0)
    }
    #[doc = "ADOVFEXSR.OVFEXF0 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFEXC0_AW::_1)
    }
}
#[doc = "Temperature Sensor Channel: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFEXSR.OVFEXF1 is cleared"]
    _1 = 1,
}
impl From<OVFEXC1_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFEXC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEXC1` writer - Temperature Sensor Channel: Overflow Flag Clear"]
pub type OVFEXC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFEXSCR_SPEC, OVFEXC1_AW, O>;
impl<'a, const O: u8> OVFEXC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFEXC1_AW::_0)
    }
    #[doc = "ADOVFEXSR.OVFEXF1 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFEXC1_AW::_1)
    }
}
#[doc = "Internal Reference Voltage Channel: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFEXSR.OVFEXF2 is cleared"]
    _1 = 1,
}
impl From<OVFEXC2_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFEXC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEXC2` writer - Internal Reference Voltage Channel: Overflow Flag Clear"]
pub type OVFEXC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFEXSCR_SPEC, OVFEXC2_AW, O>;
impl<'a, const O: u8> OVFEXC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFEXC2_AW::_0)
    }
    #[doc = "ADOVFEXSR.OVFEXF2 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFEXC2_AW::_1)
    }
}
#[doc = "D/A Converter 0 Channel: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFEXSR.OVFEXF5 is cleared"]
    _1 = 1,
}
impl From<OVFEXC5_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFEXC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEXC5` writer - D/A Converter 0 Channel: Overflow Flag Clear"]
pub type OVFEXC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFEXSCR_SPEC, OVFEXC5_AW, O>;
impl<'a, const O: u8> OVFEXC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFEXC5_AW::_0)
    }
    #[doc = "ADOVFEXSR.OVFEXF5 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFEXC5_AW::_1)
    }
}
#[doc = "D/A Converter 1 Channel: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFEXSR.OVFEXF6 is cleared"]
    _1 = 1,
}
impl From<OVFEXC6_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFEXC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEXC6` writer - D/A Converter 1 Channel: Overflow Flag Clear"]
pub type OVFEXC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFEXSCR_SPEC, OVFEXC6_AW, O>;
impl<'a, const O: u8> OVFEXC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFEXC6_AW::_0)
    }
    #[doc = "ADOVFEXSR.OVFEXF6 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFEXC6_AW::_1)
    }
}
#[doc = "D/A Converter 2 Channel: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFEXSR.OVFEXF7 is cleared"]
    _1 = 1,
}
impl From<OVFEXC7_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFEXC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEXC7` writer - D/A Converter 2 Channel: Overflow Flag Clear"]
pub type OVFEXC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFEXSCR_SPEC, OVFEXC7_AW, O>;
impl<'a, const O: u8> OVFEXC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFEXC7_AW::_0)
    }
    #[doc = "ADOVFEXSR.OVFEXF7 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFEXC7_AW::_1)
    }
}
#[doc = "D/A Converter 3 Channel: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFEXC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFEXSR.OVFEXF8 is cleared"]
    _1 = 1,
}
impl From<OVFEXC8_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFEXC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEXC8` writer - D/A Converter 3 Channel: Overflow Flag Clear"]
pub type OVFEXC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFEXSCR_SPEC, OVFEXC8_AW, O>;
impl<'a, const O: u8> OVFEXC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFEXC8_AW::_0)
    }
    #[doc = "ADOVFEXSR.OVFEXF8 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFEXC8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Self-diagnosis Channel: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfexc0(&mut self) -> OVFEXC0_W<0> {
        OVFEXC0_W::new(self)
    }
    #[doc = "Bit 1 - Temperature Sensor Channel: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfexc1(&mut self) -> OVFEXC1_W<1> {
        OVFEXC1_W::new(self)
    }
    #[doc = "Bit 2 - Internal Reference Voltage Channel: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfexc2(&mut self) -> OVFEXC2_W<2> {
        OVFEXC2_W::new(self)
    }
    #[doc = "Bit 5 - D/A Converter 0 Channel: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfexc5(&mut self) -> OVFEXC5_W<5> {
        OVFEXC5_W::new(self)
    }
    #[doc = "Bit 6 - D/A Converter 1 Channel: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfexc6(&mut self) -> OVFEXC6_W<6> {
        OVFEXC6_W::new(self)
    }
    #[doc = "Bit 7 - D/A Converter 2 Channel: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfexc7(&mut self) -> OVFEXC7_W<7> {
        OVFEXC7_W::new(self)
    }
    #[doc = "Bit 8 - D/A Converter 3 Channel: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfexc8(&mut self) -> OVFEXC8_W<8> {
        OVFEXC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Analog A/D Conversion Overflow Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adovfexscr](index.html) module"]
pub struct ADOVFEXSCR_SPEC;
impl crate::RegisterSpec for ADOVFEXSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adovfexscr::W](W) writer structure"]
impl crate::Writable for ADOVFEXSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADOVFEXSCR to value 0"]
impl crate::Resettable for ADOVFEXSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
