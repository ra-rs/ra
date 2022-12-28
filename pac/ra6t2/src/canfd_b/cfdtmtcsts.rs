#[doc = "Register `CFDTMTCSTS` reader"]
pub struct R(crate::R<CFDTMTCSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMTCSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMTCSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMTCSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFDTMTCSTS` reader - TX Message Buffer Transmission Completion Status"]
pub type CFDTMTCSTS_R = crate::FieldReader<u8, CFDTMTCSTS_A>;
#[doc = "TX Message Buffer Transmission Completion Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDTMTCSTS_A {
    #[doc = "0: Transmission not complete for corresponding TX message buffer"]
    _0 = 0,
    #[doc = "1: Transmission completed for corresponding TX message buffer"]
    _1 = 1,
}
impl From<CFDTMTCSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDTMTCSTS_A) -> Self {
        variant as _
    }
}
impl CFDTMTCSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFDTMTCSTS_A> {
        match self.bits {
            0 => Some(CFDTMTCSTS_A::_0),
            1 => Some(CFDTMTCSTS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFDTMTCSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFDTMTCSTS_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - TX Message Buffer Transmission Completion Status"]
    #[inline(always)]
    pub fn cfdtmtcsts(&self) -> CFDTMTCSTS_R {
        CFDTMTCSTS_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "TX Message Buffer Transmission Completion Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmtcsts](index.html) module"]
pub struct CFDTMTCSTS_SPEC;
impl crate::RegisterSpec for CFDTMTCSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmtcsts::R](R) reader structure"]
impl crate::Readable for CFDTMTCSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDTMTCSTS to value 0"]
impl crate::Resettable for CFDTMTCSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
