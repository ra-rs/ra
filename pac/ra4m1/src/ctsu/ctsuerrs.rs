#[doc = "Register `CTSUERRS` reader"]
pub struct R(crate::R<CTSUERRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUERRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUERRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUERRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTSUICOMP` reader - TSCAP Voltage Error Monitor"]
pub type CTSUICOMP_R = crate::BitReader<CTSUICOMP_A>;
#[doc = "TSCAP Voltage Error Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUICOMP_A {
    #[doc = "0: Normal TSCAP voltage"]
    _0 = 0,
    #[doc = "1: Abnormal TSCAP voltage"]
    _1 = 1,
}
impl From<CTSUICOMP_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUICOMP_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUICOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUICOMP_A {
        match self.bits {
            false => CTSUICOMP_A::_0,
            true => CTSUICOMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUICOMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUICOMP_A::_1
    }
}
impl R {
    #[doc = "Bit 15 - TSCAP Voltage Error Monitor"]
    #[inline(always)]
    pub fn ctsuicomp(&self) -> CTSUICOMP_R {
        CTSUICOMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "CTSU Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuerrs](index.html) module"]
pub struct CTSUERRS_SPEC;
impl crate::RegisterSpec for CTSUERRS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsuerrs::R](R) reader structure"]
impl crate::Readable for CTSUERRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTSUERRS to value 0"]
impl crate::Resettable for CTSUERRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
