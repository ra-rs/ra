#[doc = "Register `CFDTMTRSTS` reader"]
pub struct R(crate::R<CFDTMTRSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMTRSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMTRSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMTRSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFDTMTRSTS` reader - TX Message Buffer Transmission Request Status"]
pub type CFDTMTRSTS_R = crate::FieldReader<u8, CFDTMTRSTS_A>;
#[doc = "TX Message Buffer Transmission Request Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDTMTRSTS_A {
    #[doc = "0: Transmission not requested for corresponding TX message buffer"]
    _0 = 0,
    #[doc = "1: Transmission requested for corresponding TX message buffer"]
    _1 = 1,
}
impl From<CFDTMTRSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDTMTRSTS_A) -> Self {
        variant as _
    }
}
impl CFDTMTRSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFDTMTRSTS_A> {
        match self.bits {
            0 => Some(CFDTMTRSTS_A::_0),
            1 => Some(CFDTMTRSTS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFDTMTRSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFDTMTRSTS_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - TX Message Buffer Transmission Request Status"]
    #[inline(always)]
    pub fn cfdtmtrsts(&self) -> CFDTMTRSTS_R {
        CFDTMTRSTS_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "TX Message Buffer Transmission Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmtrsts](index.html) module"]
pub struct CFDTMTRSTS_SPEC;
impl crate::RegisterSpec for CFDTMTRSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmtrsts::R](R) reader structure"]
impl crate::Readable for CFDTMTRSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDTMTRSTS to value 0"]
impl crate::Resettable for CFDTMTRSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
