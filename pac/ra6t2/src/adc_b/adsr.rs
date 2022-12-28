#[doc = "Register `ADSR` reader"]
pub struct R(crate::R<ADSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADACT0` reader - A/D Converter Unit 0 (ADC0) A/D Conversion Status"]
pub type ADACT0_R = crate::BitReader<ADACT0_A>;
#[doc = "A/D Converter Unit 0 (ADC0) A/D Conversion Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADACT0_A {
    #[doc = "0: ADC0 is not in A/D conversion"]
    _0 = 0,
    #[doc = "1: ADC0 is in A/D conversion"]
    _1 = 1,
}
impl From<ADACT0_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADACT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT0_A {
        match self.bits {
            false => ADACT0_A::_0,
            true => ADACT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADACT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADACT0_A::_1
    }
}
#[doc = "Field `ADACT1` reader - A/D Converter Unit 1 (ADC1) A/D Conversion Status"]
pub type ADACT1_R = crate::BitReader<ADACT1_A>;
#[doc = "A/D Converter Unit 1 (ADC1) A/D Conversion Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADACT1_A {
    #[doc = "0: ADC1 is not in A/D conversion"]
    _0 = 0,
    #[doc = "1: ADC1 is in A/D conversion"]
    _1 = 1,
}
impl From<ADACT1_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADACT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT1_A {
        match self.bits {
            false => ADACT1_A::_0,
            true => ADACT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADACT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADACT1_A::_1
    }
}
#[doc = "Field `CALACT0` reader - A/D Converter Unit 0 (ADC0) : Calibration Status"]
pub type CALACT0_R = crate::BitReader<CALACT0_A>;
#[doc = "A/D Converter Unit 0 (ADC0) : Calibration Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALACT0_A {
    #[doc = "0: ADC0 is not in the calibration operation"]
    _0 = 0,
    #[doc = "1: ADC0 is in the calibration operation"]
    _1 = 1,
}
impl From<CALACT0_A> for bool {
    #[inline(always)]
    fn from(variant: CALACT0_A) -> Self {
        variant as u8 != 0
    }
}
impl CALACT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALACT0_A {
        match self.bits {
            false => CALACT0_A::_0,
            true => CALACT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALACT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALACT0_A::_1
    }
}
#[doc = "Field `CALACT1` reader - A/D Converter Unit 1 (ADC1) : Calibration Status"]
pub type CALACT1_R = crate::BitReader<CALACT1_A>;
#[doc = "A/D Converter Unit 1 (ADC1) : Calibration Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALACT1_A {
    #[doc = "0: ADC1 is not in the calibration operation"]
    _0 = 0,
    #[doc = "1: ADC1 is in the calibration operation"]
    _1 = 1,
}
impl From<CALACT1_A> for bool {
    #[inline(always)]
    fn from(variant: CALACT1_A) -> Self {
        variant as u8 != 0
    }
}
impl CALACT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALACT1_A {
        match self.bits {
            false => CALACT1_A::_0,
            true => CALACT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALACT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALACT1_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - A/D Converter Unit 0 (ADC0) A/D Conversion Status"]
    #[inline(always)]
    pub fn adact0(&self) -> ADACT0_R {
        ADACT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D Converter Unit 1 (ADC1) A/D Conversion Status"]
    #[inline(always)]
    pub fn adact1(&self) -> ADACT1_R {
        ADACT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - A/D Converter Unit 0 (ADC0) : Calibration Status"]
    #[inline(always)]
    pub fn calact0(&self) -> CALACT0_R {
        CALACT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - A/D Converter Unit 1 (ADC1) : Calibration Status"]
    #[inline(always)]
    pub fn calact1(&self) -> CALACT1_R {
        CALACT1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "A/D Conversion Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsr](index.html) module"]
pub struct ADSR_SPEC;
impl crate::RegisterSpec for ADSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsr::R](R) reader structure"]
impl crate::Readable for ADSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADSR to value 0"]
impl crate::Resettable for ADSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
