#[doc = "Register `CFDRMID%s` reader"]
pub struct R(crate::R<CFDRMID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRMID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRMID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRMID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RMID` reader - RX Message Buffer ID Field"]
pub type RMID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RMRTR` reader - RX Message Buffer RTR Bit"]
pub type RMRTR_R = crate::BitReader<RMRTR_A>;
#[doc = "RX Message Buffer RTR Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMRTR_A {
    #[doc = "0: Data frame"]
    _0 = 0,
    #[doc = "1: Remote frame"]
    _1 = 1,
}
impl From<RMRTR_A> for bool {
    #[inline(always)]
    fn from(variant: RMRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RMRTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMRTR_A {
        match self.bits {
            false => RMRTR_A::_0,
            true => RMRTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMRTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMRTR_A::_1
    }
}
#[doc = "Field `RMIDE` reader - RX Message Buffer IDE Bit"]
pub type RMIDE_R = crate::BitReader<RMIDE_A>;
#[doc = "RX Message Buffer IDE Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMIDE_A {
    #[doc = "0: STD-ID is stored"]
    _0 = 0,
    #[doc = "1: EXT-ID is stored"]
    _1 = 1,
}
impl From<RMIDE_A> for bool {
    #[inline(always)]
    fn from(variant: RMIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl RMIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMIDE_A {
        match self.bits {
            false => RMIDE_A::_0,
            true => RMIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMIDE_A::_1
    }
}
impl R {
    #[doc = "Bits 0:28 - RX Message Buffer ID Field"]
    #[inline(always)]
    pub fn rmid(&self) -> RMID_R {
        RMID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - RX Message Buffer RTR Bit"]
    #[inline(always)]
    pub fn rmrtr(&self) -> RMRTR_R {
        RMRTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RX Message Buffer IDE Bit"]
    #[inline(always)]
    pub fn rmide(&self) -> RMIDE_R {
        RMIDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "RX Message Buffer ID Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrmid](index.html) module"]
pub struct CFDRMID_SPEC;
impl crate::RegisterSpec for CFDRMID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrmid::R](R) reader structure"]
impl crate::Readable for CFDRMID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRMID%s to value 0"]
impl crate::Resettable for CFDRMID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
