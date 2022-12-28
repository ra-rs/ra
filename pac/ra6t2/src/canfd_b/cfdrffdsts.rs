#[doc = "Register `CFDRFFDSTS%s` reader"]
pub struct R(crate::R<CFDRFFDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRFFDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRFFDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRFFDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFESI` reader - Error State Indicator bit"]
pub type RFESI_R = crate::BitReader<RFESI_A>;
#[doc = "Error State Indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFESI_A {
    #[doc = "0: CANFD frame received from error active node"]
    _0 = 0,
    #[doc = "1: CANFD frame received from error passive node"]
    _1 = 1,
}
impl From<RFESI_A> for bool {
    #[inline(always)]
    fn from(variant: RFESI_A) -> Self {
        variant as u8 != 0
    }
}
impl RFESI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFESI_A {
        match self.bits {
            false => RFESI_A::_0,
            true => RFESI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFESI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFESI_A::_1
    }
}
#[doc = "Field `RFBRS` reader - Bit Rate Switch bit"]
pub type RFBRS_R = crate::BitReader<RFBRS_A>;
#[doc = "Bit Rate Switch bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFBRS_A {
    #[doc = "0: CANFD frame received with no bit rate switch"]
    _0 = 0,
    #[doc = "1: CANFD frame received with bit rate switch"]
    _1 = 1,
}
impl From<RFBRS_A> for bool {
    #[inline(always)]
    fn from(variant: RFBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RFBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFBRS_A {
        match self.bits {
            false => RFBRS_A::_0,
            true => RFBRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFBRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFBRS_A::_1
    }
}
#[doc = "Field `RFFDF` reader - CAN FD Format bit"]
pub type RFFDF_R = crate::BitReader<RFFDF_A>;
#[doc = "CAN FD Format bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFFDF_A {
    #[doc = "0: Non CANFD frame received"]
    _0 = 0,
    #[doc = "1: CANFD frame received"]
    _1 = 1,
}
impl From<RFFDF_A> for bool {
    #[inline(always)]
    fn from(variant: RFFDF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFFDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFFDF_A {
        match self.bits {
            false => RFFDF_A::_0,
            true => RFFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFFDF_A::_1
    }
}
#[doc = "Field `RFIFL` reader - RX FIFO Buffer Information Label Field"]
pub type RFIFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFDRFPTR` reader - RX FIFO Buffer Pointer Field"]
pub type CFDRFPTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Error State Indicator bit"]
    #[inline(always)]
    pub fn rfesi(&self) -> RFESI_R {
        RFESI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit Rate Switch bit"]
    #[inline(always)]
    pub fn rfbrs(&self) -> RFBRS_R {
        RFBRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAN FD Format bit"]
    #[inline(always)]
    pub fn rffdf(&self) -> RFFDF_R {
        RFFDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RX FIFO Buffer Information Label Field"]
    #[inline(always)]
    pub fn rfifl(&self) -> RFIFL_R {
        RFIFL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - RX FIFO Buffer Pointer Field"]
    #[inline(always)]
    pub fn cfdrfptr(&self) -> CFDRFPTR_R {
        CFDRFPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "RX FIFO Access CANFD Status Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrffdsts](index.html) module"]
pub struct CFDRFFDSTS_SPEC;
impl crate::RegisterSpec for CFDRFFDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrffdsts::R](R) reader structure"]
impl crate::Readable for CFDRFFDSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRFFDSTS%s to value 0"]
impl crate::Resettable for CFDRFFDSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
