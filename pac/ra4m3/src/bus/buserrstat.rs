#[doc = "Register `BUS%sERRSTAT` reader"]
pub struct R(crate::R<BUSERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLERRSTAT` reader - Slave bus Error Status"]
pub type SLERRSTAT_R = crate::BitReader<SLERRSTAT_A>;
#[doc = "Slave bus Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLERRSTAT_A {
    #[doc = "0: No error occurred"]
    _0 = 0,
    #[doc = "1: Error occurred"]
    _1 = 1,
}
impl From<SLERRSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: SLERRSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLERRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLERRSTAT_A {
        match self.bits {
            false => SLERRSTAT_A::_0,
            true => SLERRSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLERRSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLERRSTAT_A::_1
    }
}
#[doc = "Field `STERRSTAT` reader - Slave TrustZone filter Error Status"]
pub type STERRSTAT_R = crate::BitReader<STERRSTAT_A>;
#[doc = "Slave TrustZone filter Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STERRSTAT_A {
    #[doc = "0: No error occurred"]
    _0 = 0,
    #[doc = "1: Error occurred"]
    _1 = 1,
}
impl From<STERRSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: STERRSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl STERRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STERRSTAT_A {
        match self.bits {
            false => STERRSTAT_A::_0,
            true => STERRSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STERRSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STERRSTAT_A::_1
    }
}
#[doc = "Field `MMERRSTAT` reader - Master MPU Error Status"]
pub type MMERRSTAT_R = crate::BitReader<MMERRSTAT_A>;
#[doc = "Master MPU Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMERRSTAT_A {
    #[doc = "0: No error occurred"]
    _0 = 0,
    #[doc = "1: Error occurred"]
    _1 = 1,
}
impl From<MMERRSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: MMERRSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl MMERRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMERRSTAT_A {
        match self.bits {
            false => MMERRSTAT_A::_0,
            true => MMERRSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MMERRSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MMERRSTAT_A::_1
    }
}
#[doc = "Field `ILERRSTAT` reader - Illegal address access Error Status"]
pub type ILERRSTAT_R = crate::BitReader<ILERRSTAT_A>;
#[doc = "Illegal address access Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILERRSTAT_A {
    #[doc = "0: No error occurred"]
    _0 = 0,
    #[doc = "1: Error occurred"]
    _1 = 1,
}
impl From<ILERRSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: ILERRSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl ILERRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILERRSTAT_A {
        match self.bits {
            false => ILERRSTAT_A::_0,
            true => ILERRSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILERRSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILERRSTAT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Slave bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(&self) -> SLERRSTAT_R {
        SLERRSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave TrustZone filter Error Status"]
    #[inline(always)]
    pub fn sterrstat(&self) -> STERRSTAT_R {
        STERRSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Master MPU Error Status"]
    #[inline(always)]
    pub fn mmerrstat(&self) -> MMERRSTAT_R {
        MMERRSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Illegal address access Error Status"]
    #[inline(always)]
    pub fn ilerrstat(&self) -> ILERRSTAT_R {
        ILERRSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "BUS Error Status Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buserrstat](index.html) module"]
pub struct BUSERRSTAT_SPEC;
impl crate::RegisterSpec for BUSERRSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [buserrstat::R](R) reader structure"]
impl crate::Readable for BUSERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS%sERRSTAT to value 0"]
impl crate::Resettable for BUSERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
