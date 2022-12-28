#[doc = "Register `DLMMON` reader"]
pub struct R(crate::R<DLMMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLMMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLMMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLMMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLMMON` reader - Device Lifecycle Management State Monitor"]
pub type DLMMON_R = crate::FieldReader<u8, DLMMON_A>;
#[doc = "Device Lifecycle Management State Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLMMON_A {
    #[doc = "1: CM"]
    _0X1 = 1,
    #[doc = "2: SSD"]
    _0X2 = 2,
    #[doc = "3: NSECSD"]
    _0X3 = 3,
    #[doc = "4: DPL"]
    _0X4 = 4,
    #[doc = "5: LCK_DBG"]
    _0X5 = 5,
    #[doc = "6: LCK_BOOT"]
    _0X6 = 6,
    #[doc = "7: RMA_REQ"]
    _0X7 = 7,
    #[doc = "8: RMA_ACK"]
    _0X8 = 8,
}
impl From<DLMMON_A> for u8 {
    #[inline(always)]
    fn from(variant: DLMMON_A) -> Self {
        variant as _
    }
}
impl DLMMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DLMMON_A> {
        match self.bits {
            1 => Some(DLMMON_A::_0X1),
            2 => Some(DLMMON_A::_0X2),
            3 => Some(DLMMON_A::_0X3),
            4 => Some(DLMMON_A::_0X4),
            5 => Some(DLMMON_A::_0X5),
            6 => Some(DLMMON_A::_0X6),
            7 => Some(DLMMON_A::_0X7),
            8 => Some(DLMMON_A::_0X8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == DLMMON_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == DLMMON_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == DLMMON_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == DLMMON_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == DLMMON_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == DLMMON_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == DLMMON_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == DLMMON_A::_0X8
    }
}
impl R {
    #[doc = "Bits 0:3 - Device Lifecycle Management State Monitor"]
    #[inline(always)]
    pub fn dlmmon(&self) -> DLMMON_R {
        DLMMON_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Device Lifecycle Management State Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlmmon](index.html) module"]
pub struct DLMMON_SPEC;
impl crate::RegisterSpec for DLMMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlmmon::R](R) reader structure"]
impl crate::Readable for DLMMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLMMON to value 0"]
impl crate::Resettable for DLMMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
