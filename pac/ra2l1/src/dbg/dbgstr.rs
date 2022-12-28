#[doc = "Register `DBGSTR` reader"]
pub struct R(crate::R<DBGSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CDBGPWRUPREQ` reader - Debug power-up request"]
pub type CDBGPWRUPREQ_R = crate::BitReader<CDBGPWRUPREQ_A>;
#[doc = "Debug power-up request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDBGPWRUPREQ_A {
    #[doc = "0: OCD is not requesting debug power up"]
    _0 = 0,
    #[doc = "1: OCD is requesting debug power up"]
    _1 = 1,
}
impl From<CDBGPWRUPREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CDBGPWRUPREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CDBGPWRUPREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDBGPWRUPREQ_A {
        match self.bits {
            false => CDBGPWRUPREQ_A::_0,
            true => CDBGPWRUPREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDBGPWRUPREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDBGPWRUPREQ_A::_1
    }
}
#[doc = "Field `CDBGPWRUPACK` reader - Debug power-up acknowledge"]
pub type CDBGPWRUPACK_R = crate::BitReader<CDBGPWRUPACK_A>;
#[doc = "Debug power-up acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDBGPWRUPACK_A {
    #[doc = "0: Debug power-up request is not acknowledged"]
    _0 = 0,
    #[doc = "1: Debug power-up request is acknowledged"]
    _1 = 1,
}
impl From<CDBGPWRUPACK_A> for bool {
    #[inline(always)]
    fn from(variant: CDBGPWRUPACK_A) -> Self {
        variant as u8 != 0
    }
}
impl CDBGPWRUPACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDBGPWRUPACK_A {
        match self.bits {
            false => CDBGPWRUPACK_A::_0,
            true => CDBGPWRUPACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDBGPWRUPACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDBGPWRUPACK_A::_1
    }
}
impl R {
    #[doc = "Bit 28 - Debug power-up request"]
    #[inline(always)]
    pub fn cdbgpwrupreq(&self) -> CDBGPWRUPREQ_R {
        CDBGPWRUPREQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Debug power-up acknowledge"]
    #[inline(always)]
    pub fn cdbgpwrupack(&self) -> CDBGPWRUPACK_R {
        CDBGPWRUPACK_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Debug Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgstr](index.html) module"]
pub struct DBGSTR_SPEC;
impl crate::RegisterSpec for DBGSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgstr::R](R) reader structure"]
impl crate::Readable for DBGSTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBGSTR to value 0"]
impl crate::Resettable for DBGSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
