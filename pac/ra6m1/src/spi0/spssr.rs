#[doc = "Register `SPSSR` reader"]
pub struct R(crate::R<SPSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPCP` reader - RSPI Command Pointer"]
pub type SPCP_R = crate::FieldReader<u8, SPCP_A>;
#[doc = "RSPI Command Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPCP_A {
    #[doc = "0: SPCMD0"]
    _000 = 0,
    #[doc = "1: SPCMD1"]
    _001 = 1,
    #[doc = "2: SPCMD2"]
    _010 = 2,
    #[doc = "3: SPCMD3"]
    _011 = 3,
    #[doc = "4: SPCMD4"]
    _100 = 4,
    #[doc = "5: SPCMD5"]
    _101 = 5,
    #[doc = "6: SPCMD6"]
    _110 = 6,
    #[doc = "7: SPCMD7"]
    _111 = 7,
}
impl From<SPCP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPCP_A) -> Self {
        variant as _
    }
}
impl SPCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCP_A {
        match self.bits {
            0 => SPCP_A::_000,
            1 => SPCP_A::_001,
            2 => SPCP_A::_010,
            3 => SPCP_A::_011,
            4 => SPCP_A::_100,
            5 => SPCP_A::_101,
            6 => SPCP_A::_110,
            7 => SPCP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPCP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPCP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPCP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPCP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPCP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPCP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPCP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPCP_A::_111
    }
}
#[doc = "Field `SPECM` reader - RSPI Error Command"]
pub type SPECM_R = crate::FieldReader<u8, SPECM_A>;
#[doc = "RSPI Error Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPECM_A {
    #[doc = "0: SPCMD0"]
    _000 = 0,
    #[doc = "1: SPCMD1"]
    _001 = 1,
    #[doc = "2: SPCMD2"]
    _010 = 2,
    #[doc = "3: SPCMD3"]
    _011 = 3,
    #[doc = "4: SPCMD4"]
    _100 = 4,
    #[doc = "5: SPCMD5"]
    _101 = 5,
    #[doc = "6: SPCMD6"]
    _110 = 6,
    #[doc = "7: SPCMD7"]
    _111 = 7,
}
impl From<SPECM_A> for u8 {
    #[inline(always)]
    fn from(variant: SPECM_A) -> Self {
        variant as _
    }
}
impl SPECM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPECM_A {
        match self.bits {
            0 => SPECM_A::_000,
            1 => SPECM_A::_001,
            2 => SPECM_A::_010,
            3 => SPECM_A::_011,
            4 => SPECM_A::_100,
            5 => SPECM_A::_101,
            6 => SPECM_A::_110,
            7 => SPECM_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPECM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPECM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPECM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPECM_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPECM_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPECM_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPECM_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPECM_A::_111
    }
}
impl R {
    #[doc = "Bits 0:2 - RSPI Command Pointer"]
    #[inline(always)]
    pub fn spcp(&self) -> SPCP_R {
        SPCP_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - RSPI Error Command"]
    #[inline(always)]
    pub fn specm(&self) -> SPECM_R {
        SPECM_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "SPI Sequence Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spssr](index.html) module"]
pub struct SPSSR_SPEC;
impl crate::RegisterSpec for SPSSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spssr::R](R) reader structure"]
impl crate::Readable for SPSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPSSR to value 0"]
impl crate::Resettable for SPSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
