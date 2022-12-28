#[doc = "Register `CFDFFSTS` reader"]
pub struct R(crate::R<CFDFFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDFFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDFFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDFFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFXFLL` reader - RX FIF0 Full Status"]
pub type RFXFLL_R = crate::FieldReader<u8, RFXFLL_A>;
#[doc = "RX FIF0 Full Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFXFLL_A {
    #[doc = "0: Corresponding FIFO not full"]
    _0 = 0,
    #[doc = "1: Corresponding FIFO full"]
    _1 = 1,
}
impl From<RFXFLL_A> for u8 {
    #[inline(always)]
    fn from(variant: RFXFLL_A) -> Self {
        variant as _
    }
}
impl RFXFLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFXFLL_A> {
        match self.bits {
            0 => Some(RFXFLL_A::_0),
            1 => Some(RFXFLL_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFXFLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFXFLL_A::_1
    }
}
#[doc = "Field `CFFLL` reader - Common FIF0 Full Status"]
pub type CFFLL_R = crate::BitReader<CFFLL_A>;
#[doc = "Common FIF0 Full Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFFLL_A {
    #[doc = "0: Corresponding FIFO not full"]
    _0 = 0,
    #[doc = "1: Corresponding FIFO full"]
    _1 = 1,
}
impl From<CFFLL_A> for bool {
    #[inline(always)]
    fn from(variant: CFFLL_A) -> Self {
        variant as u8 != 0
    }
}
impl CFFLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFFLL_A {
        match self.bits {
            false => CFFLL_A::_0,
            true => CFFLL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFFLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFFLL_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - RX FIF0 Full Status"]
    #[inline(always)]
    pub fn rfxfll(&self) -> RFXFLL_R {
        RFXFLL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Common FIF0 Full Status"]
    #[inline(always)]
    pub fn cffll(&self) -> CFFLL_R {
        CFFLL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "FIFO Full Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdffsts](index.html) module"]
pub struct CFDFFSTS_SPEC;
impl crate::RegisterSpec for CFDFFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdffsts::R](R) reader structure"]
impl crate::Readable for CFDFFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDFFSTS to value 0"]
impl crate::Resettable for CFDFFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
