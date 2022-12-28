#[doc = "Register `ADERSR` reader"]
pub struct R(crate::R<ADERSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADERSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADERSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADERSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADERF0` reader - A/D Converter Unit 0 (ADC0) Error Flag"]
pub type ADERF0_R = crate::BitReader<ADERF0_A>;
#[doc = "A/D Converter Unit 0 (ADC0) Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADERF0_A {
    #[doc = "0: Error is not detected"]
    _0 = 0,
    #[doc = "1: Error is detected"]
    _1 = 1,
}
impl From<ADERF0_A> for bool {
    #[inline(always)]
    fn from(variant: ADERF0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADERF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADERF0_A {
        match self.bits {
            false => ADERF0_A::_0,
            true => ADERF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADERF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADERF0_A::_1
    }
}
#[doc = "Field `ADERF1` reader - A/D Converter Unit 1 (ADC1) Error Flag"]
pub type ADERF1_R = crate::BitReader<ADERF1_A>;
#[doc = "A/D Converter Unit 1 (ADC1) Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADERF1_A {
    #[doc = "0: Error is not detected"]
    _0 = 0,
    #[doc = "1: Error is detected"]
    _1 = 1,
}
impl From<ADERF1_A> for bool {
    #[inline(always)]
    fn from(variant: ADERF1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADERF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADERF1_A {
        match self.bits {
            false => ADERF1_A::_0,
            true => ADERF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADERF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADERF1_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - A/D Converter Unit 0 (ADC0) Error Flag"]
    #[inline(always)]
    pub fn aderf0(&self) -> ADERF0_R {
        ADERF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Converter Unit 1 (ADC1) Error Flag"]
    #[inline(always)]
    pub fn aderf1(&self) -> ADERF1_R {
        ADERF1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "A/D Conversion Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adersr](index.html) module"]
pub struct ADERSR_SPEC;
impl crate::RegisterSpec for ADERSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adersr::R](R) reader structure"]
impl crate::Readable for ADERSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADERSR to value 0"]
impl crate::Resettable for ADERSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
