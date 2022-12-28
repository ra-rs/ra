#[doc = "Register `EIDR` reader"]
pub struct R(crate::R<EIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EIDR` reader - Pmn Event Input Data"]
pub type EIDR_R = crate::FieldReader<u16, EIDR_A>;
#[doc = "Pmn Event Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EIDR_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input."]
    _1 = 1,
}
impl From<EIDR_A> for u16 {
    #[inline(always)]
    fn from(variant: EIDR_A) -> Self {
        variant as _
    }
}
impl EIDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EIDR_A> {
        match self.bits {
            0 => Some(EIDR_A::_0),
            1 => Some(EIDR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR_A::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Event Input Data"]
    #[inline(always)]
    pub fn eidr(&self) -> EIDR_R {
        EIDR_R::new(self.bits)
    }
}
#[doc = "Event input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eidr](index.html) module"]
pub struct EIDR_SPEC;
impl crate::RegisterSpec for EIDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eidr::R](R) reader structure"]
impl crate::Readable for EIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EIDR to value 0"]
impl crate::Resettable for EIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
