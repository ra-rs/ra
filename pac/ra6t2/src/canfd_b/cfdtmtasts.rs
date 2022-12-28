#[doc = "Register `CFDTMTASTS` reader"]
pub struct R(crate::R<CFDTMTASTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMTASTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMTASTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMTASTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFDTMTASTS` reader - TX Message Buffer Transmission Abort Status"]
pub type CFDTMTASTS_R = crate::FieldReader<u8, CFDTMTASTS_A>;
#[doc = "TX Message Buffer Transmission Abort Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDTMTASTS_A {
    #[doc = "0: Transmission not aborted for corresponding TX message buffer"]
    _0 = 0,
    #[doc = "1: Transmission aborted for corresponding TX message buffer"]
    _1 = 1,
}
impl From<CFDTMTASTS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDTMTASTS_A) -> Self {
        variant as _
    }
}
impl CFDTMTASTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFDTMTASTS_A> {
        match self.bits {
            0 => Some(CFDTMTASTS_A::_0),
            1 => Some(CFDTMTASTS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFDTMTASTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFDTMTASTS_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - TX Message Buffer Transmission Abort Status"]
    #[inline(always)]
    pub fn cfdtmtasts(&self) -> CFDTMTASTS_R {
        CFDTMTASTS_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "TX Message Buffer Transmission Abort Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmtasts](index.html) module"]
pub struct CFDTMTASTS_SPEC;
impl crate::RegisterSpec for CFDTMTASTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmtasts::R](R) reader structure"]
impl crate::Readable for CFDTMTASTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDTMTASTS to value 0"]
impl crate::Resettable for CFDTMTASTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
