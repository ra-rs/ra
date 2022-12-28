#[doc = "Register `AMPMON` reader"]
pub struct R(crate::R<AMPMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AMPMON0` reader - Operational amplifier status(UNIT0)"]
pub type AMPMON0_R = crate::BitReader<AMPMON0_A>;
#[doc = "Operational amplifier status(UNIT0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON0_A {
    #[doc = "0: Operational amplifier 0 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 0 is operating."]
    _1 = 1,
}
impl From<AMPMON0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON0_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMON0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMON0_A {
        match self.bits {
            false => AMPMON0_A::_0,
            true => AMPMON0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON0_A::_1
    }
}
#[doc = "Field `AMPMON1` reader - Operational amplifier status(UNIT1)"]
pub type AMPMON1_R = crate::BitReader<AMPMON1_A>;
#[doc = "Operational amplifier status(UNIT1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON1_A {
    #[doc = "0: Operational amplifier 1 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 1 is operating."]
    _1 = 1,
}
impl From<AMPMON1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON1_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMON1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMON1_A {
        match self.bits {
            false => AMPMON1_A::_0,
            true => AMPMON1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON1_A::_1
    }
}
#[doc = "Field `AMPMON2` reader - Operational amplifier status(UNIT2)"]
pub type AMPMON2_R = crate::BitReader<AMPMON2_A>;
#[doc = "Operational amplifier status(UNIT2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON2_A {
    #[doc = "0: Operational amplifier 2 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 2 is operating."]
    _1 = 1,
}
impl From<AMPMON2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON2_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMON2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMON2_A {
        match self.bits {
            false => AMPMON2_A::_0,
            true => AMPMON2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON2_A::_1
    }
}
#[doc = "Field `AMPMON3` reader - Operational amplifier status(UNIT3)"]
pub type AMPMON3_R = crate::BitReader<AMPMON3_A>;
#[doc = "Operational amplifier status(UNIT3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON3_A {
    #[doc = "0: Operational amplifier 3 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 3 is operating."]
    _1 = 1,
}
impl From<AMPMON3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON3_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPMON3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPMON3_A {
        match self.bits {
            false => AMPMON3_A::_0,
            true => AMPMON3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON3_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier status(UNIT0)"]
    #[inline(always)]
    pub fn ampmon0(&self) -> AMPMON0_R {
        AMPMON0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier status(UNIT1)"]
    #[inline(always)]
    pub fn ampmon1(&self) -> AMPMON1_R {
        AMPMON1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operational amplifier status(UNIT2)"]
    #[inline(always)]
    pub fn ampmon2(&self) -> AMPMON2_R {
        AMPMON2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operational amplifier status(UNIT3)"]
    #[inline(always)]
    pub fn ampmon3(&self) -> AMPMON3_R {
        AMPMON3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Operational amplifier monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampmon](index.html) module"]
pub struct AMPMON_SPEC;
impl crate::RegisterSpec for AMPMON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ampmon::R](R) reader structure"]
impl crate::Readable for AMPMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AMPMON to value 0"]
impl crate::Resettable for AMPMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
