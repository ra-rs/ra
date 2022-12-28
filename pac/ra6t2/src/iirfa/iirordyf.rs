#[doc = "Register `IIRORDYF` reader"]
pub struct R(crate::R<IIRORDYF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRORDYF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRORDYF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRORDYF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ORDYF` reader - Output data preparation completion flag"]
pub type ORDYF_R = crate::FieldReader<u16, ORDYF_A>;
#[doc = "Output data preparation completion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ORDYF_A {
    #[doc = "0: The output data preparation of the corresponding channel is not completed."]
    _0 = 0,
    #[doc = "1: The output data preparation of the corresponding channel is completed."]
    _1 = 1,
}
impl From<ORDYF_A> for u16 {
    #[inline(always)]
    fn from(variant: ORDYF_A) -> Self {
        variant as _
    }
}
impl ORDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ORDYF_A> {
        match self.bits {
            0 => Some(ORDYF_A::_0),
            1 => Some(ORDYF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORDYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORDYF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Output data preparation completion flag"]
    #[inline(always)]
    pub fn ordyf(&self) -> ORDYF_R {
        ORDYF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Output Data Preparation Completion Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirordyf](index.html) module"]
pub struct IIRORDYF_SPEC;
impl crate::RegisterSpec for IIRORDYF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iirordyf::R](R) reader structure"]
impl crate::Readable for IIRORDYF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRORDYF to value 0"]
impl crate::Resettable for IIRORDYF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
