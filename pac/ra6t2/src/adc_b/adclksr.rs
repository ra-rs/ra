#[doc = "Register `ADCLKSR` reader"]
pub struct R(crate::R<ADCLKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCLKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCLKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCLKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLKSR` reader - ADCLK status"]
pub type CLKSR_R = crate::BitReader<CLKSR_A>;
#[doc = "ADCLK status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSR_A {
    #[doc = "0: ADCLK is stopped"]
    _0 = 0,
    #[doc = "1: ADCLK is in supply"]
    _1 = 1,
}
impl From<CLKSR_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSR_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSR_A {
        match self.bits {
            false => CLKSR_A::_0,
            true => CLKSR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - ADCLK status"]
    #[inline(always)]
    pub fn clksr(&self) -> CLKSR_R {
        CLKSR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "A/D Conversion Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adclksr](index.html) module"]
pub struct ADCLKSR_SPEC;
impl crate::RegisterSpec for ADCLKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adclksr::R](R) reader structure"]
impl crate::Readable for ADCLKSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCLKSR to value 0"]
impl crate::Resettable for ADCLKSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
