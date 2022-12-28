#[doc = "Register `FPSR` reader"]
pub struct R(crate::R<FPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERR` reader - Protect Error Flag"]
pub type PERR_R = crate::BitReader<PERR_A>;
#[doc = "Protect Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERR_A {
    #[doc = "0: No error"]
    _0 = 0,
    #[doc = "1: An error occurs"]
    _1 = 1,
}
impl From<PERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERR_A {
        match self.bits {
            false => PERR_A::_0,
            true => PERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PERR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Protect Error Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Protection Unlock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpsr](index.html) module"]
pub struct FPSR_SPEC;
impl crate::RegisterSpec for FPSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fpsr::R](R) reader structure"]
impl crate::Readable for FPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FPSR to value 0"]
impl crate::Resettable for FPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
