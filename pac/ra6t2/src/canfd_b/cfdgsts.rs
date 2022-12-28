#[doc = "Register `CFDGSTS` reader"]
pub struct R(crate::R<CFDGSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GRSTSTS` reader - Global Reset Status"]
pub type GRSTSTS_R = crate::BitReader<GRSTSTS_A>;
#[doc = "Global Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRSTSTS_A {
    #[doc = "0: Not in Reset mode"]
    _0 = 0,
    #[doc = "1: In Reset mode"]
    _1 = 1,
}
impl From<GRSTSTS_A> for bool {
    #[inline(always)]
    fn from(variant: GRSTSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl GRSTSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRSTSTS_A {
        match self.bits {
            false => GRSTSTS_A::_0,
            true => GRSTSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRSTSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRSTSTS_A::_1
    }
}
#[doc = "Field `GHLTSTS` reader - Global Halt Status"]
pub type GHLTSTS_R = crate::BitReader<GHLTSTS_A>;
#[doc = "Global Halt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GHLTSTS_A {
    #[doc = "0: Not in Halt mode"]
    _0 = 0,
    #[doc = "1: In Halt mode"]
    _1 = 1,
}
impl From<GHLTSTS_A> for bool {
    #[inline(always)]
    fn from(variant: GHLTSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl GHLTSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GHLTSTS_A {
        match self.bits {
            false => GHLTSTS_A::_0,
            true => GHLTSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GHLTSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GHLTSTS_A::_1
    }
}
#[doc = "Field `GSLPSTS` reader - Global Sleep Status"]
pub type GSLPSTS_R = crate::BitReader<GSLPSTS_A>;
#[doc = "Global Sleep Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSLPSTS_A {
    #[doc = "0: Not in Sleep mode"]
    _0 = 0,
    #[doc = "1: In Sleep mode"]
    _1 = 1,
}
impl From<GSLPSTS_A> for bool {
    #[inline(always)]
    fn from(variant: GSLPSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl GSLPSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSLPSTS_A {
        match self.bits {
            false => GSLPSTS_A::_0,
            true => GSLPSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GSLPSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GSLPSTS_A::_1
    }
}
#[doc = "Field `GRAMINIT` reader - Global RAM Initialization"]
pub type GRAMINIT_R = crate::BitReader<GRAMINIT_A>;
#[doc = "Global RAM Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRAMINIT_A {
    #[doc = "0: RAM initialization is complete"]
    _0 = 0,
    #[doc = "1: RAM initialization is ongoing"]
    _1 = 1,
}
impl From<GRAMINIT_A> for bool {
    #[inline(always)]
    fn from(variant: GRAMINIT_A) -> Self {
        variant as u8 != 0
    }
}
impl GRAMINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRAMINIT_A {
        match self.bits {
            false => GRAMINIT_A::_0,
            true => GRAMINIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRAMINIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRAMINIT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Global Reset Status"]
    #[inline(always)]
    pub fn grststs(&self) -> GRSTSTS_R {
        GRSTSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Halt Status"]
    #[inline(always)]
    pub fn ghltsts(&self) -> GHLTSTS_R {
        GHLTSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Sleep Status"]
    #[inline(always)]
    pub fn gslpsts(&self) -> GSLPSTS_R {
        GSLPSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global RAM Initialization"]
    #[inline(always)]
    pub fn graminit(&self) -> GRAMINIT_R {
        GRAMINIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Global Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgsts](index.html) module"]
pub struct CFDGSTS_SPEC;
impl crate::RegisterSpec for CFDGSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgsts::R](R) reader structure"]
impl crate::Readable for CFDGSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDGSTS to value 0x0d"]
impl crate::Resettable for CFDGSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
