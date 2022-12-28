#[doc = "Register `SDSR` reader"]
pub struct R(crate::R<SDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MRSST` reader - Mode Register Setting Status"]
pub type MRSST_R = crate::BitReader<MRSST_A>;
#[doc = "Mode Register Setting Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRSST_A {
    #[doc = "0: Mode register setting not in progress"]
    _0 = 0,
    #[doc = "1: Mode register setting in progress"]
    _1 = 1,
}
impl From<MRSST_A> for bool {
    #[inline(always)]
    fn from(variant: MRSST_A) -> Self {
        variant as u8 != 0
    }
}
impl MRSST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRSST_A {
        match self.bits {
            false => MRSST_A::_0,
            true => MRSST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MRSST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MRSST_A::_1
    }
}
#[doc = "Field `INIST` reader - Initialization Status"]
pub type INIST_R = crate::BitReader<INIST_A>;
#[doc = "Initialization Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIST_A {
    #[doc = "0: Initialization sequence not in progress"]
    _0 = 0,
    #[doc = "1: Initialization sequence in progress"]
    _1 = 1,
}
impl From<INIST_A> for bool {
    #[inline(always)]
    fn from(variant: INIST_A) -> Self {
        variant as u8 != 0
    }
}
impl INIST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIST_A {
        match self.bits {
            false => INIST_A::_0,
            true => INIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INIST_A::_1
    }
}
#[doc = "Field `SRFST` reader - Self-Refresh Transition/Recovery Status"]
pub type SRFST_R = crate::BitReader<SRFST_A>;
#[doc = "Self-Refresh Transition/Recovery Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRFST_A {
    #[doc = "0: Transition/recovery not in progress"]
    _0 = 0,
    #[doc = "1: Transition/recovery in progress"]
    _1 = 1,
}
impl From<SRFST_A> for bool {
    #[inline(always)]
    fn from(variant: SRFST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRFST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRFST_A {
        match self.bits {
            false => SRFST_A::_0,
            true => SRFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRFST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Mode Register Setting Status"]
    #[inline(always)]
    pub fn mrsst(&self) -> MRSST_R {
        MRSST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Initialization Status"]
    #[inline(always)]
    pub fn inist(&self) -> INIST_R {
        INIST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Self-Refresh Transition/Recovery Status"]
    #[inline(always)]
    pub fn srfst(&self) -> SRFST_R {
        SRFST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "SDRAM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdsr](index.html) module"]
pub struct SDSR_SPEC;
impl crate::RegisterSpec for SDSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdsr::R](R) reader structure"]
impl crate::Readable for SDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDSR to value 0"]
impl crate::Resettable for SDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
