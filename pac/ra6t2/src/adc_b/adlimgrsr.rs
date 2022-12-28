#[doc = "Register `ADLIMGRSR` reader"]
pub struct R(crate::R<ADLIMGRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADLIMGRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADLIMGRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADLIMGRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LIMGRF0` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF0_R = crate::BitReader<LIMGRF0_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF0_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF0_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF0_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF0_A {
        match self.bits {
            false => LIMGRF0_A::_0,
            true => LIMGRF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF0_A::_1
    }
}
#[doc = "Field `LIMGRF1` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF1_R = crate::BitReader<LIMGRF1_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF1_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF1_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF1_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF1_A {
        match self.bits {
            false => LIMGRF1_A::_0,
            true => LIMGRF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF1_A::_1
    }
}
#[doc = "Field `LIMGRF2` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF2_R = crate::BitReader<LIMGRF2_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF2_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF2_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF2_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF2_A {
        match self.bits {
            false => LIMGRF2_A::_0,
            true => LIMGRF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF2_A::_1
    }
}
#[doc = "Field `LIMGRF3` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF3_R = crate::BitReader<LIMGRF3_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF3_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF3_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF3_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF3_A {
        match self.bits {
            false => LIMGRF3_A::_0,
            true => LIMGRF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF3_A::_1
    }
}
#[doc = "Field `LIMGRF4` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF4_R = crate::BitReader<LIMGRF4_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF4_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF4_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF4_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF4_A {
        match self.bits {
            false => LIMGRF4_A::_0,
            true => LIMGRF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF4_A::_1
    }
}
#[doc = "Field `LIMGRF5` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF5_R = crate::BitReader<LIMGRF5_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF5_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF5_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF5_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF5_A {
        match self.bits {
            false => LIMGRF5_A::_0,
            true => LIMGRF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF5_A::_1
    }
}
#[doc = "Field `LIMGRF6` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF6_R = crate::BitReader<LIMGRF6_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF6_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF6_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF6_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF6_A {
        match self.bits {
            false => LIMGRF6_A::_0,
            true => LIMGRF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF6_A::_1
    }
}
#[doc = "Field `LIMGRF7` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF7_R = crate::BitReader<LIMGRF7_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF7_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF7_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF7_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF7_A {
        match self.bits {
            false => LIMGRF7_A::_0,
            true => LIMGRF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF7_A::_1
    }
}
#[doc = "Field `LIMGRF8` reader - Scan Group n Limiter Clip Flag"]
pub type LIMGRF8_R = crate::BitReader<LIMGRF8_A>;
#[doc = "Scan Group n Limiter Clip Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRF8_A {
    #[doc = "0: Limiter clip for scan group n is not detected"]
    _0 = 0,
    #[doc = "1: Limiter clip for scan group n is detected"]
    _1 = 1,
}
impl From<LIMGRF8_A> for bool {
    #[inline(always)]
    fn from(variant: LIMGRF8_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMGRF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMGRF8_A {
        match self.bits {
            false => LIMGRF8_A::_0,
            true => LIMGRF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LIMGRF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LIMGRF8_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf0(&self) -> LIMGRF0_R {
        LIMGRF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf1(&self) -> LIMGRF1_R {
        LIMGRF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf2(&self) -> LIMGRF2_R {
        LIMGRF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf3(&self) -> LIMGRF3_R {
        LIMGRF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf4(&self) -> LIMGRF4_R {
        LIMGRF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf5(&self) -> LIMGRF5_R {
        LIMGRF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf6(&self) -> LIMGRF6_R {
        LIMGRF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf7(&self) -> LIMGRF7_R {
        LIMGRF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n Limiter Clip Flag"]
    #[inline(always)]
    pub fn limgrf8(&self) -> LIMGRF8_R {
        LIMGRF8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Limiter Clip Scan Group Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimgrsr](index.html) module"]
pub struct ADLIMGRSR_SPEC;
impl crate::RegisterSpec for ADLIMGRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adlimgrsr::R](R) reader structure"]
impl crate::Readable for ADLIMGRSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADLIMGRSR to value 0"]
impl crate::Resettable for ADLIMGRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
