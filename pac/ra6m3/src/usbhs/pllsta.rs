#[doc = "Register `PLLSTA` reader"]
pub struct R(crate::R<PLLSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLLLOCK` reader - PLL Lock Flag"]
pub type PLLLOCK_R = crate::BitReader<PLLLOCK_A>;
#[doc = "PLL Lock Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLLOCK_A {
    #[doc = "0: PLL is not locked."]
    _0 = 0,
    #[doc = "1: PLL is locked."]
    _1 = 1,
}
impl From<PLLLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLOCK_A {
        match self.bits {
            false => PLLLOCK_A::_0,
            true => PLLLOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLLOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLLOCK_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - PLL Lock Flag"]
    #[inline(always)]
    pub fn plllock(&self) -> PLLLOCK_R {
        PLLLOCK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsta](index.html) module"]
pub struct PLLSTA_SPEC;
impl crate::RegisterSpec for PLLSTA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pllsta::R](R) reader structure"]
impl crate::Readable for PLLSTA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLLSTA to value 0"]
impl crate::Resettable for PLLSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
