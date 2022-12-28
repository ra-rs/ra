#[doc = "Register `PIDR` reader"]
pub struct R(crate::R<PIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR` reader - Pmn Input Data"]
pub type PIDR_R = crate::FieldReader<u16, PIDR_A>;
#[doc = "Pmn Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIDR_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input."]
    _1 = 1,
}
impl From<PIDR_A> for u16 {
    #[inline(always)]
    fn from(variant: PIDR_A) -> Self {
        variant as _
    }
}
impl PIDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIDR_A> {
        match self.bits {
            0 => Some(PIDR_A::_0),
            1 => Some(PIDR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR_A::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Input Data"]
    #[inline(always)]
    pub fn pidr(&self) -> PIDR_R {
        PIDR_R::new(self.bits)
    }
}
#[doc = "Input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr](index.html) module"]
pub struct PIDR_SPEC;
impl crate::RegisterSpec for PIDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pidr::R](R) reader structure"]
impl crate::Readable for PIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR to value 0"]
impl crate::Resettable for PIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
