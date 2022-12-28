#[doc = "Register `CLBSSR` reader"]
pub struct R(crate::R<CLBSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLBSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLBSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLBSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLBSS` reader - Calibration status"]
pub type CLBSS_R = crate::BitReader<CLBSS_A>;
#[doc = "Calibration status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLBSS_A {
    #[doc = "0: Calibration is not running"]
    _0 = 0,
    #[doc = "1: Calibration is running"]
    _1 = 1,
}
impl From<CLBSS_A> for bool {
    #[inline(always)]
    fn from(variant: CLBSS_A) -> Self {
        variant as u8 != 0
    }
}
impl CLBSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLBSS_A {
        match self.bits {
            false => CLBSS_A::_0,
            true => CLBSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLBSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLBSS_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Calibration status"]
    #[inline(always)]
    pub fn clbss(&self) -> CLBSS_R {
        CLBSS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clbssr](index.html) module"]
pub struct CLBSSR_SPEC;
impl crate::RegisterSpec for CLBSSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clbssr::R](R) reader structure"]
impl crate::Readable for CLBSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLBSSR to value 0"]
impl crate::Resettable for CLBSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
