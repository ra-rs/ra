#[doc = "Register `IIRCPRCS` reader"]
pub struct R(crate::R<IIRCPRCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRCPRCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRCPRCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRCPRCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPRCS` reader - Channel processing status bit"]
pub type CPRCS_R = crate::FieldReader<u16, CPRCS_A>;
#[doc = "Channel processing status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CPRCS_A {
    #[doc = "0: The channel processing of the corresponding channel is not being performed."]
    _0 = 0,
    #[doc = "1: The channel processing of the corresponding channel is being performed."]
    _1 = 1,
}
impl From<CPRCS_A> for u16 {
    #[inline(always)]
    fn from(variant: CPRCS_A) -> Self {
        variant as _
    }
}
impl CPRCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPRCS_A> {
        match self.bits {
            0 => Some(CPRCS_A::_0),
            1 => Some(CPRCS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPRCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPRCS_A::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel processing status bit"]
    #[inline(always)]
    pub fn cprcs(&self) -> CPRCS_R {
        CPRCS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel Processing Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iircprcs](index.html) module"]
pub struct IIRCPRCS_SPEC;
impl crate::RegisterSpec for IIRCPRCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iircprcs::R](R) reader structure"]
impl crate::Readable for IIRCPRCS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRCPRCS to value 0"]
impl crate::Resettable for IIRCPRCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
