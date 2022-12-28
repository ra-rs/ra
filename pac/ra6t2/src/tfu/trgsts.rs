#[doc = "Register `TRGSTS` reader"]
pub struct R(crate::R<TRGSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRGSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRGSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRGSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BSYF` reader - Calculation in progress flag"]
pub type BSYF_R = crate::BitReader<BSYF_A>;
#[doc = "Calculation in progress flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYF_A {
    #[doc = "0: No calculating"]
    _0 = 0,
    #[doc = "1: Calculating"]
    _1 = 1,
}
impl From<BSYF_A> for bool {
    #[inline(always)]
    fn from(variant: BSYF_A) -> Self {
        variant as u8 != 0
    }
}
impl BSYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYF_A {
        match self.bits {
            false => BSYF_A::_0,
            true => BSYF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSYF_A::_1
    }
}
#[doc = "Field `ERRF` reader - Input error flag"]
pub type ERRF_R = crate::BitReader<ERRF_A>;
#[doc = "Input error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRF_A {
    #[doc = "0: No input error occurred"]
    _0 = 0,
    #[doc = "1: Input error occurred"]
    _1 = 1,
}
impl From<ERRF_A> for bool {
    #[inline(always)]
    fn from(variant: ERRF_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRF_A {
        match self.bits {
            false => ERRF_A::_0,
            true => ERRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Calculation in progress flag"]
    #[inline(always)]
    pub fn bsyf(&self) -> BSYF_R {
        BSYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input error flag"]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Trigonometric Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgsts](index.html) module"]
pub struct TRGSTS_SPEC;
impl crate::RegisterSpec for TRGSTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [trgsts::R](R) reader structure"]
impl crate::Readable for TRGSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRGSTS to value 0"]
impl crate::Resettable for TRGSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
