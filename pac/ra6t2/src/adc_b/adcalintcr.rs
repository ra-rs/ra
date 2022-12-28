#[doc = "Register `ADCALINTCR` reader"]
pub struct R(crate::R<ADCALINTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCALINTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCALINTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCALINTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCALINTCR` writer"]
pub struct W(crate::W<ADCALINTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCALINTCR_SPEC>;
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
impl From<crate::W<ADCALINTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCALINTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALENDIE0` reader - ADC0 Calibration End Interrupt Enable"]
pub type CALENDIE0_R = crate::BitReader<CALENDIE0_A>;
#[doc = "ADC0 Calibration End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALENDIE0_A {
    #[doc = "0: Disable Calibration End Interrupt for ADC0"]
    _0 = 0,
    #[doc = "1: Enable Calibration End Interrupt for ADC0"]
    _1 = 1,
}
impl From<CALENDIE0_A> for bool {
    #[inline(always)]
    fn from(variant: CALENDIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl CALENDIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALENDIE0_A {
        match self.bits {
            false => CALENDIE0_A::_0,
            true => CALENDIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALENDIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALENDIE0_A::_1
    }
}
#[doc = "Field `CALENDIE0` writer - ADC0 Calibration End Interrupt Enable"]
pub type CALENDIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCALINTCR_SPEC, CALENDIE0_A, O>;
impl<'a, const O: u8> CALENDIE0_W<'a, O> {
    #[doc = "Disable Calibration End Interrupt for ADC0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALENDIE0_A::_0)
    }
    #[doc = "Enable Calibration End Interrupt for ADC0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALENDIE0_A::_1)
    }
}
#[doc = "Field `CALENDIE1` reader - ADC1 Calibration End Interrupt Enable"]
pub type CALENDIE1_R = crate::BitReader<CALENDIE1_A>;
#[doc = "ADC1 Calibration End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALENDIE1_A {
    #[doc = "0: Disable Calibration End Interrupt for ADC1"]
    _0 = 0,
    #[doc = "1: Enable Calibration End Interrupt for ADC1"]
    _1 = 1,
}
impl From<CALENDIE1_A> for bool {
    #[inline(always)]
    fn from(variant: CALENDIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CALENDIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALENDIE1_A {
        match self.bits {
            false => CALENDIE1_A::_0,
            true => CALENDIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALENDIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALENDIE1_A::_1
    }
}
#[doc = "Field `CALENDIE1` writer - ADC1 Calibration End Interrupt Enable"]
pub type CALENDIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCALINTCR_SPEC, CALENDIE1_A, O>;
impl<'a, const O: u8> CALENDIE1_W<'a, O> {
    #[doc = "Disable Calibration End Interrupt for ADC1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALENDIE1_A::_0)
    }
    #[doc = "Enable Calibration End Interrupt for ADC1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALENDIE1_A::_1)
    }
}
impl R {
    #[doc = "Bit 16 - ADC0 Calibration End Interrupt Enable"]
    #[inline(always)]
    pub fn calendie0(&self) -> CALENDIE0_R {
        CALENDIE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 Calibration End Interrupt Enable"]
    #[inline(always)]
    pub fn calendie1(&self) -> CALENDIE1_R {
        CALENDIE1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ADC0 Calibration End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calendie0(&mut self) -> CALENDIE0_W<16> {
        CALENDIE0_W::new(self)
    }
    #[doc = "Bit 17 - ADC1 Calibration End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calendie1(&mut self) -> CALENDIE1_W<17> {
        CALENDIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalintcr](index.html) module"]
pub struct ADCALINTCR_SPEC;
impl crate::RegisterSpec for ADCALINTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcalintcr::R](R) reader structure"]
impl crate::Readable for ADCALINTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcalintcr::W](W) writer structure"]
impl crate::Writable for ADCALINTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCALINTCR to value 0"]
impl crate::Resettable for ADCALINTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
