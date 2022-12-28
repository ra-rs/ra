#[doc = "Register `ADAR` reader"]
pub struct R(crate::R<ADAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDADMVD` reader - The 24-bit A/D average value"]
pub type SDADMVD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SDADMVS` reader - Status of an A/D conversion result"]
pub type SDADMVS_R = crate::BitReader<SDADMVS_A>;
#[doc = "Status of an A/D conversion result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADMVS_A {
    #[doc = "0: Normal status (within the range)"]
    _0 = 0,
    #[doc = "1: Overflow occurred"]
    _1 = 1,
}
impl From<SDADMVS_A> for bool {
    #[inline(always)]
    fn from(variant: SDADMVS_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADMVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADMVS_A {
        match self.bits {
            false => SDADMVS_A::_0,
            true => SDADMVS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADMVS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADMVS_A::_1
    }
}
#[doc = "Field `SDADMVC` reader - Channel number for an A/D conversion result"]
pub type SDADMVC_R = crate::FieldReader<u8, SDADMVC_A>;
#[doc = "Channel number for an A/D conversion result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDADMVC_A {
    #[doc = "0: Reset value (Conversion result is invalid)"]
    _000 = 0,
    #[doc = "1: Input multiplexer 0 (ANSD0P / ANSD0N)"]
    _001 = 1,
    #[doc = "2: Input multiplexer 1 (ANSD1P / ANSD1N)"]
    _010 = 2,
    #[doc = "3: Input multiplexer 2 (ANSD2P / ANSD2N)"]
    _011 = 3,
    #[doc = "4: Input multiplexer 3 (ANSD3P / ANSD3N)"]
    _100 = 4,
    #[doc = "5: Input multiplexer 4 (AMP0O / AMP1O)."]
    _101 = 5,
}
impl From<SDADMVC_A> for u8 {
    #[inline(always)]
    fn from(variant: SDADMVC_A) -> Self {
        variant as _
    }
}
impl SDADMVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDADMVC_A> {
        match self.bits {
            0 => Some(SDADMVC_A::_000),
            1 => Some(SDADMVC_A::_001),
            2 => Some(SDADMVC_A::_010),
            3 => Some(SDADMVC_A::_011),
            4 => Some(SDADMVC_A::_100),
            5 => Some(SDADMVC_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SDADMVC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SDADMVC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SDADMVC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SDADMVC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SDADMVC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SDADMVC_A::_101
    }
}
impl R {
    #[doc = "Bits 0:23 - The 24-bit A/D average value"]
    #[inline(always)]
    pub fn sdadmvd(&self) -> SDADMVD_R {
        SDADMVD_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Status of an A/D conversion result"]
    #[inline(always)]
    pub fn sdadmvs(&self) -> SDADMVS_R {
        SDADMVS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Channel number for an A/D conversion result"]
    #[inline(always)]
    pub fn sdadmvc(&self) -> SDADMVC_R {
        SDADMVC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[doc = "Sigma-delta A/D Converter Average Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adar](index.html) module"]
pub struct ADAR_SPEC;
impl crate::RegisterSpec for ADAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adar::R](R) reader structure"]
impl crate::Readable for ADAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADAR to value 0"]
impl crate::Resettable for ADAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
