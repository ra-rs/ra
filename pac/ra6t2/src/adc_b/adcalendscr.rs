#[doc = "Register `ADCALENDSCR` writer"]
pub struct W(crate::W<ADCALENDSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCALENDSCR_SPEC>;
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
impl From<crate::W<ADCALENDSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCALENDSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Converter Unit 0 Calibration End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALENDC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCALENDSR.CALENDF0 is cleared"]
    _1 = 1,
}
impl From<CALENDC0_AW> for bool {
    #[inline(always)]
    fn from(variant: CALENDC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALENDC0` writer - A/D Converter Unit 0 Calibration End Flag Clear"]
pub type CALENDC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCALENDSCR_SPEC, CALENDC0_AW, O>;
impl<'a, const O: u8> CALENDC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALENDC0_AW::_0)
    }
    #[doc = "ADCALENDSR.CALENDF0 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALENDC0_AW::_1)
    }
}
#[doc = "A/D Converter Unit 1 Calibration End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALENDC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCALENDSR.CALENDF1 is cleared"]
    _1 = 1,
}
impl From<CALENDC1_AW> for bool {
    #[inline(always)]
    fn from(variant: CALENDC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALENDC1` writer - A/D Converter Unit 1 Calibration End Flag Clear"]
pub type CALENDC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCALENDSCR_SPEC, CALENDC1_AW, O>;
impl<'a, const O: u8> CALENDC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALENDC1_AW::_0)
    }
    #[doc = "ADCALENDSR.CALENDF1 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALENDC1_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter Unit 0 Calibration End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn calendc0(&mut self) -> CALENDC0_W<0> {
        CALENDC0_W::new(self)
    }
    #[doc = "Bit 1 - A/D Converter Unit 1 Calibration End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn calendc1(&mut self) -> CALENDC1_W<1> {
        CALENDC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Calibration End Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalendscr](index.html) module"]
pub struct ADCALENDSCR_SPEC;
impl crate::RegisterSpec for ADCALENDSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adcalendscr::W](W) writer structure"]
impl crate::Writable for ADCALENDSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCALENDSCR to value 0"]
impl crate::Resettable for ADCALENDSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
