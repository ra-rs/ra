#[doc = "Register `CFDRMFDSTS%s` reader"]
pub struct R(crate::R<CFDRMFDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRMFDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRMFDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRMFDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RMESI` reader - Error State Indicator bit"]
pub type RMESI_R = crate::BitReader<RMESI_A>;
#[doc = "Error State Indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMESI_A {
    #[doc = "0: CANFD frame received from error active node"]
    _0 = 0,
    #[doc = "1: CANFD frame received from error passive node"]
    _1 = 1,
}
impl From<RMESI_A> for bool {
    #[inline(always)]
    fn from(variant: RMESI_A) -> Self {
        variant as u8 != 0
    }
}
impl RMESI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMESI_A {
        match self.bits {
            false => RMESI_A::_0,
            true => RMESI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMESI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMESI_A::_1
    }
}
#[doc = "Field `RMBRS` reader - Bit Rate Switch bit"]
pub type RMBRS_R = crate::BitReader<RMBRS_A>;
#[doc = "Bit Rate Switch bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMBRS_A {
    #[doc = "0: CANFD frame received with no bit rate switch"]
    _0 = 0,
    #[doc = "1: CANFD frame received with bit rate switch"]
    _1 = 1,
}
impl From<RMBRS_A> for bool {
    #[inline(always)]
    fn from(variant: RMBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RMBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMBRS_A {
        match self.bits {
            false => RMBRS_A::_0,
            true => RMBRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMBRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMBRS_A::_1
    }
}
#[doc = "Field `RMFDF` reader - CAN FD Format bit"]
pub type RMFDF_R = crate::BitReader<RMFDF_A>;
#[doc = "CAN FD Format bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMFDF_A {
    #[doc = "0: Non CANFD frame received"]
    _0 = 0,
    #[doc = "1: CANFD frame received"]
    _1 = 1,
}
impl From<RMFDF_A> for bool {
    #[inline(always)]
    fn from(variant: RMFDF_A) -> Self {
        variant as u8 != 0
    }
}
impl RMFDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMFDF_A {
        match self.bits {
            false => RMFDF_A::_0,
            true => RMFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMFDF_A::_1
    }
}
#[doc = "Field `RMIFL` reader - RX Message Buffer Information Label Field"]
pub type RMIFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMPTR` reader - RX Message Buffer Pointer Field"]
pub type RMPTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Error State Indicator bit"]
    #[inline(always)]
    pub fn rmesi(&self) -> RMESI_R {
        RMESI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit Rate Switch bit"]
    #[inline(always)]
    pub fn rmbrs(&self) -> RMBRS_R {
        RMBRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAN FD Format bit"]
    #[inline(always)]
    pub fn rmfdf(&self) -> RMFDF_R {
        RMFDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RX Message Buffer Information Label Field"]
    #[inline(always)]
    pub fn rmifl(&self) -> RMIFL_R {
        RMIFL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - RX Message Buffer Pointer Field"]
    #[inline(always)]
    pub fn rmptr(&self) -> RMPTR_R {
        RMPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "RX Message Buffer CANFD Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrmfdsts](index.html) module"]
pub struct CFDRMFDSTS_SPEC;
impl crate::RegisterSpec for CFDRMFDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrmfdsts::R](R) reader structure"]
impl crate::Readable for CFDRMFDSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRMFDSTS%s to value 0"]
impl crate::Resettable for CFDRMFDSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
