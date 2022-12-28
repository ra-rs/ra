#[doc = "Register `CFDTMTARSTS` reader"]
pub struct R(crate::R<CFDTMTARSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMTARSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMTARSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMTARSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFDTMTARSTS` reader - TX Message Buffer Transmission Abort Request Status"]
pub type CFDTMTARSTS_R = crate::FieldReader<u8, CFDTMTARSTS_A>;
#[doc = "TX Message Buffer Transmission Abort Request Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDTMTARSTS_A {
    #[doc = "0: Transmission abort not requested for corresponding TX message buffer"]
    _0 = 0,
    #[doc = "1: Transmission abort requested for corresponding TX message buffer"]
    _1 = 1,
}
impl From<CFDTMTARSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDTMTARSTS_A) -> Self {
        variant as _
    }
}
impl CFDTMTARSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFDTMTARSTS_A> {
        match self.bits {
            0 => Some(CFDTMTARSTS_A::_0),
            1 => Some(CFDTMTARSTS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFDTMTARSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFDTMTARSTS_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - TX Message Buffer Transmission Abort Request Status"]
    #[inline(always)]
    pub fn cfdtmtarsts(&self) -> CFDTMTARSTS_R {
        CFDTMTARSTS_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "TX Message Buffer Transmission Abort Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmtarsts](index.html) module"]
pub struct CFDTMTARSTS_SPEC;
impl crate::RegisterSpec for CFDTMTARSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmtarsts::R](R) reader structure"]
impl crate::Readable for CFDTMTARSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDTMTARSTS to value 0"]
impl crate::Resettable for CFDTMTARSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
