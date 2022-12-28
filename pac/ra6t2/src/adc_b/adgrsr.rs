#[doc = "Register `ADGRSR` reader"]
pub struct R(crate::R<ADGRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADGRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADGRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADGRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTGR0` reader - Scan Group n Status"]
pub type ACTGR0_R = crate::BitReader<ACTGR0_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR0_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR0_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR0_A {
        match self.bits {
            false => ACTGR0_A::_0,
            true => ACTGR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR0_A::_1
    }
}
#[doc = "Field `ACTGR1` reader - Scan Group n Status"]
pub type ACTGR1_R = crate::BitReader<ACTGR1_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR1_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR1_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR1_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR1_A {
        match self.bits {
            false => ACTGR1_A::_0,
            true => ACTGR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR1_A::_1
    }
}
#[doc = "Field `ACTGR2` reader - Scan Group n Status"]
pub type ACTGR2_R = crate::BitReader<ACTGR2_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR2_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR2_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR2_A {
        match self.bits {
            false => ACTGR2_A::_0,
            true => ACTGR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR2_A::_1
    }
}
#[doc = "Field `ACTGR3` reader - Scan Group n Status"]
pub type ACTGR3_R = crate::BitReader<ACTGR3_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR3_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR3_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR3_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR3_A {
        match self.bits {
            false => ACTGR3_A::_0,
            true => ACTGR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR3_A::_1
    }
}
#[doc = "Field `ACTGR4` reader - Scan Group n Status"]
pub type ACTGR4_R = crate::BitReader<ACTGR4_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR4_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR4_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR4_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR4_A {
        match self.bits {
            false => ACTGR4_A::_0,
            true => ACTGR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR4_A::_1
    }
}
#[doc = "Field `ACTGR5` reader - Scan Group n Status"]
pub type ACTGR5_R = crate::BitReader<ACTGR5_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR5_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR5_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR5_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR5_A {
        match self.bits {
            false => ACTGR5_A::_0,
            true => ACTGR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR5_A::_1
    }
}
#[doc = "Field `ACTGR6` reader - Scan Group n Status"]
pub type ACTGR6_R = crate::BitReader<ACTGR6_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR6_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR6_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR6_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR6_A {
        match self.bits {
            false => ACTGR6_A::_0,
            true => ACTGR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR6_A::_1
    }
}
#[doc = "Field `ACTGR7` reader - Scan Group n Status"]
pub type ACTGR7_R = crate::BitReader<ACTGR7_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR7_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR7_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR7_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR7_A {
        match self.bits {
            false => ACTGR7_A::_0,
            true => ACTGR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR7_A::_1
    }
}
#[doc = "Field `ACTGR8` reader - Scan Group n Status"]
pub type ACTGR8_R = crate::BitReader<ACTGR8_A>;
#[doc = "Scan Group n Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTGR8_A {
    #[doc = "0: Scan group n is idle"]
    _0 = 0,
    #[doc = "1: Scan group n is in the scanning operation"]
    _1 = 1,
}
impl From<ACTGR8_A> for bool {
    #[inline(always)]
    fn from(variant: ACTGR8_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTGR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTGR8_A {
        match self.bits {
            false => ACTGR8_A::_0,
            true => ACTGR8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTGR8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTGR8_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr0(&self) -> ACTGR0_R {
        ACTGR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr1(&self) -> ACTGR1_R {
        ACTGR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr2(&self) -> ACTGR2_R {
        ACTGR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr3(&self) -> ACTGR3_R {
        ACTGR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr4(&self) -> ACTGR4_R {
        ACTGR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr5(&self) -> ACTGR5_R {
        ACTGR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr6(&self) -> ACTGR6_R {
        ACTGR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr7(&self) -> ACTGR7_R {
        ACTGR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n Status"]
    #[inline(always)]
    pub fn actgr8(&self) -> ACTGR8_R {
        ACTGR8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Scan Group Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adgrsr](index.html) module"]
pub struct ADGRSR_SPEC;
impl crate::RegisterSpec for ADGRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adgrsr::R](R) reader structure"]
impl crate::Readable for ADGRSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADGRSR to value 0"]
impl crate::Resettable for ADGRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
