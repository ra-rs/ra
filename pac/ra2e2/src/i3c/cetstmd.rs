#[doc = "Register `CETSTMD` reader"]
pub struct R(crate::R<CETSTMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CETSTMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CETSTMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CETSTMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSTMD` reader - Test Mode"]
pub type TSTMD_R = crate::FieldReader<u8, TSTMD_A>;
#[doc = "Test Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTMD_A {
    #[doc = "0: Exit Test Mode This value removes all I3C devices from Test Mode."]
    _0X00 = 0,
    #[doc = "1: Vendor Test Mode This value indicates that I3C devices shall return a random 32bit value in the provisional ID during the Dynamic Address Assignment procedure."]
    _0X01 = 1,
}
impl From<TSTMD_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTMD_A) -> Self {
        variant as _
    }
}
impl TSTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTMD_A> {
        match self.bits {
            0 => Some(TSTMD_A::_0X00),
            1 => Some(TSTMD_A::_0X01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == TSTMD_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == TSTMD_A::_0X01
    }
}
impl R {
    #[doc = "Bits 0:7 - Test Mode"]
    #[inline(always)]
    pub fn tstmd(&self) -> TSTMD_R {
        TSTMD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CCC Enter Test Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cetstmd](index.html) module"]
pub struct CETSTMD_SPEC;
impl crate::RegisterSpec for CETSTMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cetstmd::R](R) reader structure"]
impl crate::Readable for CETSTMD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CETSTMD to value 0"]
impl crate::Resettable for CETSTMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
