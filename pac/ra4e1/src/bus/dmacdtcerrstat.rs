#[doc = "Register `DMACDTCERRSTAT` reader"]
pub struct R(crate::R<DMACDTCERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACDTCERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACDTCERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACDTCERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MTERRSTAT` reader - Master TrustZone Filter Error Status"]
pub type MTERRSTAT_R = crate::BitReader<MTERRSTAT_A>;
#[doc = "Master TrustZone Filter Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTERRSTAT_A {
    #[doc = "0: No error occurred"]
    _0 = 0,
    #[doc = "1: Error occurred"]
    _1 = 1,
}
impl From<MTERRSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: MTERRSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl MTERRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTERRSTAT_A {
        match self.bits {
            false => MTERRSTAT_A::_0,
            true => MTERRSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTERRSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTERRSTAT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Master TrustZone Filter Error Status"]
    #[inline(always)]
    pub fn mterrstat(&self) -> MTERRSTAT_R {
        MTERRSTAT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DMAC/DTC Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacdtcerrstat](index.html) module"]
pub struct DMACDTCERRSTAT_SPEC;
impl crate::RegisterSpec for DMACDTCERRSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmacdtcerrstat::R](R) reader structure"]
impl crate::Readable for DMACDTCERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACDTCERRSTAT to value 0"]
impl crate::Resettable for DMACDTCERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
