#[doc = "Register `CESR` reader"]
pub struct R(crate::R<CESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RIST` reader - RE Internal status"]
pub type RIST_R = crate::BitReader<RIST_A>;
#[doc = "RE Internal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIST_A {
    #[doc = "0: RE signal internal state value 0"]
    _0 = 0,
    #[doc = "1: RE signal internal state value 1"]
    _1 = 1,
}
impl From<RIST_A> for bool {
    #[inline(always)]
    fn from(variant: RIST_A) -> Self {
        variant as u8 != 0
    }
}
impl RIST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIST_A {
        match self.bits {
            false => RIST_A::_0,
            true => RIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIST_A::_1
    }
}
#[doc = "Field `TIST` reader - TE Internal status"]
pub type TIST_R = crate::BitReader<TIST_A>;
#[doc = "TE Internal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIST_A {
    #[doc = "0: TE signal internal state value 0"]
    _0 = 0,
    #[doc = "1: TE signal internal state value 1"]
    _1 = 1,
}
impl From<TIST_A> for bool {
    #[inline(always)]
    fn from(variant: TIST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIST_A {
        match self.bits {
            false => TIST_A::_0,
            true => TIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - RE Internal status"]
    #[inline(always)]
    pub fn rist(&self) -> RIST_R {
        RIST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TE Internal status"]
    #[inline(always)]
    pub fn tist(&self) -> TIST_R {
        TIST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Communication Enable Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cesr](index.html) module"]
pub struct CESR_SPEC;
impl crate::RegisterSpec for CESR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cesr::R](R) reader structure"]
impl crate::Readable for CESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CESR to value 0"]
impl crate::Resettable for CESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
