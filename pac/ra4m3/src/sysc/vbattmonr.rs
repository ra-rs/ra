#[doc = "Register `VBATTMONR` reader"]
pub struct R(crate::R<VBATTMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBATTMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBATTMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBATTMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VBATTMON` reader - VBATT Voltage Monitor Bit"]
pub type VBATTMON_R = crate::BitReader<VBATTMON_A>;
#[doc = "VBATT Voltage Monitor Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATTMON_A {
    #[doc = "0: VBATT ≥ Vbattldet"]
    _0 = 0,
    #[doc = "1: VBATT < Vbattldet"]
    _1 = 1,
}
impl From<VBATTMON_A> for bool {
    #[inline(always)]
    fn from(variant: VBATTMON_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATTMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATTMON_A {
        match self.bits {
            false => VBATTMON_A::_0,
            true => VBATTMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBATTMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBATTMON_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Voltage Monitor Bit"]
    #[inline(always)]
    pub fn vbattmon(&self) -> VBATTMON_R {
        VBATTMON_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Battery Backup Voltage Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbattmonr](index.html) module"]
pub struct VBATTMONR_SPEC;
impl crate::RegisterSpec for VBATTMONR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbattmonr::R](R) reader structure"]
impl crate::Readable for VBATTMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VBATTMONR to value 0"]
impl crate::Resettable for VBATTMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
