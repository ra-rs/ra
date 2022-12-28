#[doc = "Register `ADOVFERSCR` writer"]
pub struct W(crate::W<ADOVFERSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADOVFERSCR_SPEC>;
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
impl From<crate::W<ADOVFERSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADOVFERSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Converter Unit 0 (ADC0) Overflow Error Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADOVFEC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFERSR.ADOVFEF0 is cleared"]
    _1 = 1,
}
impl From<ADOVFEC0_AW> for bool {
    #[inline(always)]
    fn from(variant: ADOVFEC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOVFEC0` writer - A/D Converter Unit 0 (ADC0) Overflow Error Flag Clear"]
pub type ADOVFEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFERSCR_SPEC, ADOVFEC0_AW, O>;
impl<'a, const O: u8> ADOVFEC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVFEC0_AW::_0)
    }
    #[doc = "ADOVFERSR.ADOVFEF0 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVFEC0_AW::_1)
    }
}
#[doc = "A/D Converter Unit 1 (ADC1) Overflow Error Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADOVFEC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFERSR.ADOVFEF1 is cleared"]
    _1 = 1,
}
impl From<ADOVFEC1_AW> for bool {
    #[inline(always)]
    fn from(variant: ADOVFEC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOVFEC1` writer - A/D Converter Unit 1 (ADC1) Overflow Error Flag Clear"]
pub type ADOVFEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFERSCR_SPEC, ADOVFEC1_AW, O>;
impl<'a, const O: u8> ADOVFEC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVFEC1_AW::_0)
    }
    #[doc = "ADOVFERSR.ADOVFEF1 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVFEC1_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter Unit 0 (ADC0) Overflow Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adovfec0(&mut self) -> ADOVFEC0_W<0> {
        ADOVFEC0_W::new(self)
    }
    #[doc = "Bit 1 - A/D Converter Unit 1 (ADC1) Overflow Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adovfec1(&mut self) -> ADOVFEC1_W<1> {
        ADOVFEC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Overflow Error Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adovferscr](index.html) module"]
pub struct ADOVFERSCR_SPEC;
impl crate::RegisterSpec for ADOVFERSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adovferscr::W](W) writer structure"]
impl crate::Writable for ADOVFERSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADOVFERSCR to value 0"]
impl crate::Resettable for ADOVFERSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
