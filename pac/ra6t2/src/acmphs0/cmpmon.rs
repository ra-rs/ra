#[doc = "Register `CMPMON` reader"]
pub struct R(crate::R<CMPMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMPMON` reader - Comparator Output Monitor"]
pub type CMPMON_R = crate::BitReader<CMPMON_A>;
#[doc = "Comparator Output Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMON_A {
    #[doc = "0: Comparator output is low"]
    _0 = 0,
    #[doc = "1: Comparator output is high"]
    _1 = 1,
}
impl From<CMPMON_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMON_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMON_A {
        match self.bits {
            false => CMPMON_A::_0,
            true => CMPMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPMON_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Comparator Output Monitor"]
    #[inline(always)]
    pub fn cmpmon(&self) -> CMPMON_R {
        CMPMON_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Comparator Output Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmon](index.html) module"]
pub struct CMPMON_SPEC;
impl crate::RegisterSpec for CMPMON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmpmon::R](R) reader structure"]
impl crate::Readable for CMPMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMPMON to value 0"]
impl crate::Resettable for CMPMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
