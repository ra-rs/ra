#[doc = "Register `SPPSR` reader"]
pub struct R(crate::R<SPPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPEPS` reader - SPI Polling Status"]
pub type SPEPS_R = crate::BitReader<SPEPS_A>;
#[doc = "SPI Polling Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEPS_A {
    #[doc = "0: SPCR.SPE is 0"]
    _0 = 0,
    #[doc = "1: SPCR.SPE is 1"]
    _1 = 1,
}
impl From<SPEPS_A> for bool {
    #[inline(always)]
    fn from(variant: SPEPS_A) -> Self {
        variant as u8 != 0
    }
}
impl SPEPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEPS_A {
        match self.bits {
            false => SPEPS_A::_0,
            true => SPEPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEPS_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - SPI Polling Status"]
    #[inline(always)]
    pub fn speps(&self) -> SPEPS_R {
        SPEPS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SPI Polling Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sppsr](index.html) module"]
pub struct SPPSR_SPEC;
impl crate::RegisterSpec for SPPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sppsr::R](R) reader structure"]
impl crate::Readable for SPPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPPSR to value 0"]
impl crate::Resettable for SPPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
