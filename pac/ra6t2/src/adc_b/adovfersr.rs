#[doc = "Register `ADOVFERSR` reader"]
pub struct R(crate::R<ADOVFERSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADOVFERSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADOVFERSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADOVFERSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADOVFEF0` reader - A/D Converter Unit 0 (ADC0) Overflow Error Flag"]
pub type ADOVFEF0_R = crate::BitReader<ADOVFEF0_A>;
#[doc = "A/D Converter Unit 0 (ADC0) Overflow Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADOVFEF0_A {
    #[doc = "0: ADC0 overflow error is not detected"]
    _0 = 0,
    #[doc = "1: ADC0 overflow error is detected"]
    _1 = 1,
}
impl From<ADOVFEF0_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVFEF0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADOVFEF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVFEF0_A {
        match self.bits {
            false => ADOVFEF0_A::_0,
            true => ADOVFEF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADOVFEF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADOVFEF0_A::_1
    }
}
#[doc = "Field `ADOVFEF1` reader - A/D Converter Unit 1 (ADC1) Overflow Error Flag"]
pub type ADOVFEF1_R = crate::BitReader<ADOVFEF1_A>;
#[doc = "A/D Converter Unit 1 (ADC1) Overflow Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADOVFEF1_A {
    #[doc = "0: ADC1 overflow error is not detected"]
    _0 = 0,
    #[doc = "1: ADC1 overflow error is detected"]
    _1 = 1,
}
impl From<ADOVFEF1_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVFEF1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADOVFEF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVFEF1_A {
        match self.bits {
            false => ADOVFEF1_A::_0,
            true => ADOVFEF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADOVFEF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADOVFEF1_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - A/D Converter Unit 0 (ADC0) Overflow Error Flag"]
    #[inline(always)]
    pub fn adovfef0(&self) -> ADOVFEF0_R {
        ADOVFEF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Converter Unit 1 (ADC1) Overflow Error Flag"]
    #[inline(always)]
    pub fn adovfef1(&self) -> ADOVFEF1_R {
        ADOVFEF1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "A/D Conversion Overflow Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adovfersr](index.html) module"]
pub struct ADOVFERSR_SPEC;
impl crate::RegisterSpec for ADOVFERSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adovfersr::R](R) reader structure"]
impl crate::Readable for ADOVFERSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADOVFERSR to value 0"]
impl crate::Resettable for ADOVFERSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
