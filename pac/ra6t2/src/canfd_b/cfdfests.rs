#[doc = "Register `CFDFESTS` reader"]
pub struct R(crate::R<CFDFESTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDFESTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDFESTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDFESTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFXEMP` reader - RX FIFO Empty Status"]
pub type RFXEMP_R = crate::FieldReader<u8, RFXEMP_A>;
#[doc = "RX FIFO Empty Status\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFXEMP_A {
    #[doc = "0: Corresponding FIFO not empty"]
    _0 = 0,
    #[doc = "1: Corresponding FIFO empty"]
    _1 = 1,
}
impl From<RFXEMP_A> for u8 {
    #[inline(always)]
    fn from(variant: RFXEMP_A) -> Self {
        variant as _
    }
}
impl RFXEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFXEMP_A> {
        match self.bits {
            0 => Some(RFXEMP_A::_0),
            1 => Some(RFXEMP_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFXEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFXEMP_A::_1
    }
}
#[doc = "Field `CFEMP` reader - Common FIFO Empty Status"]
pub type CFEMP_R = crate::BitReader<CFEMP_A>;
#[doc = "Common FIFO Empty Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEMP_A {
    #[doc = "0: Corresponding FIFO not empty"]
    _0 = 0,
    #[doc = "1: Corresponding FIFO empty"]
    _1 = 1,
}
impl From<CFEMP_A> for bool {
    #[inline(always)]
    fn from(variant: CFEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl CFEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEMP_A {
        match self.bits {
            false => CFEMP_A::_0,
            true => CFEMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFEMP_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - RX FIFO Empty Status"]
    #[inline(always)]
    pub fn rfxemp(&self) -> RFXEMP_R {
        RFXEMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Common FIFO Empty Status"]
    #[inline(always)]
    pub fn cfemp(&self) -> CFEMP_R {
        CFEMP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "FIFO Empty Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdfests](index.html) module"]
pub struct CFDFESTS_SPEC;
impl crate::RegisterSpec for CFDFESTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdfests::R](R) reader structure"]
impl crate::Readable for CFDFESTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDFESTS to value 0x0103"]
impl crate::Resettable for CFDFESTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0103;
}
