#[doc = "Register `ADCALENDSR` reader"]
pub struct R(crate::R<ADCALENDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCALENDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCALENDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCALENDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALENDF0` reader - A/D Converter Unit 0 Calibration End flag"]
pub type CALENDF0_R = crate::BitReader<CALENDF0_A>;
#[doc = "A/D Converter Unit 0 Calibration End flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALENDF0_A {
    #[doc = "0: End of the calibration is not detected"]
    _0 = 0,
    #[doc = "1: End of the calibration is detected"]
    _1 = 1,
}
impl From<CALENDF0_A> for bool {
    #[inline(always)]
    fn from(variant: CALENDF0_A) -> Self {
        variant as u8 != 0
    }
}
impl CALENDF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALENDF0_A {
        match self.bits {
            false => CALENDF0_A::_0,
            true => CALENDF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALENDF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALENDF0_A::_1
    }
}
#[doc = "Field `CALENDF1` reader - A/D Converter Unit 1 Calibration End flag"]
pub type CALENDF1_R = crate::BitReader<CALENDF1_A>;
#[doc = "A/D Converter Unit 1 Calibration End flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALENDF1_A {
    #[doc = "0: End of the calibration is not detected"]
    _0 = 0,
    #[doc = "1: End of the calibration is detected"]
    _1 = 1,
}
impl From<CALENDF1_A> for bool {
    #[inline(always)]
    fn from(variant: CALENDF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CALENDF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALENDF1_A {
        match self.bits {
            false => CALENDF1_A::_0,
            true => CALENDF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALENDF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALENDF1_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - A/D Converter Unit 0 Calibration End flag"]
    #[inline(always)]
    pub fn calendf0(&self) -> CALENDF0_R {
        CALENDF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Converter Unit 1 Calibration End flag"]
    #[inline(always)]
    pub fn calendf1(&self) -> CALENDF1_R {
        CALENDF1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "A/D Converter Calibration End Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalendsr](index.html) module"]
pub struct ADCALENDSR_SPEC;
impl crate::RegisterSpec for ADCALENDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcalendsr::R](R) reader structure"]
impl crate::Readable for ADCALENDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCALENDSR to value 0"]
impl crate::Resettable for ADCALENDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
