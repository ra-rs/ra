#[doc = "Register `ADSCANENDSR` reader"]
pub struct R(crate::R<ADSCANENDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSCANENDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSCANENDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSCANENDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCENDF0` reader - Scan Group n Scan End Flag"]
pub type SCENDF0_R = crate::BitReader<SCENDF0_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF0_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF0_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF0_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF0_A {
        match self.bits {
            false => SCENDF0_A::_0,
            true => SCENDF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF0_A::_1
    }
}
#[doc = "Field `SCENDF1` reader - Scan Group n Scan End Flag"]
pub type SCENDF1_R = crate::BitReader<SCENDF1_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF1_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF1_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF1_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF1_A {
        match self.bits {
            false => SCENDF1_A::_0,
            true => SCENDF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF1_A::_1
    }
}
#[doc = "Field `SCENDF2` reader - Scan Group n Scan End Flag"]
pub type SCENDF2_R = crate::BitReader<SCENDF2_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF2_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF2_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF2_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF2_A {
        match self.bits {
            false => SCENDF2_A::_0,
            true => SCENDF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF2_A::_1
    }
}
#[doc = "Field `SCENDF3` reader - Scan Group n Scan End Flag"]
pub type SCENDF3_R = crate::BitReader<SCENDF3_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF3_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF3_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF3_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF3_A {
        match self.bits {
            false => SCENDF3_A::_0,
            true => SCENDF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF3_A::_1
    }
}
#[doc = "Field `SCENDF4` reader - Scan Group n Scan End Flag"]
pub type SCENDF4_R = crate::BitReader<SCENDF4_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF4_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF4_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF4_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF4_A {
        match self.bits {
            false => SCENDF4_A::_0,
            true => SCENDF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF4_A::_1
    }
}
#[doc = "Field `SCENDF5` reader - Scan Group n Scan End Flag"]
pub type SCENDF5_R = crate::BitReader<SCENDF5_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF5_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF5_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF5_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF5_A {
        match self.bits {
            false => SCENDF5_A::_0,
            true => SCENDF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF5_A::_1
    }
}
#[doc = "Field `SCENDF6` reader - Scan Group n Scan End Flag"]
pub type SCENDF6_R = crate::BitReader<SCENDF6_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF6_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF6_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF6_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF6_A {
        match self.bits {
            false => SCENDF6_A::_0,
            true => SCENDF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF6_A::_1
    }
}
#[doc = "Field `SCENDF7` reader - Scan Group n Scan End Flag"]
pub type SCENDF7_R = crate::BitReader<SCENDF7_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF7_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF7_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF7_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF7_A {
        match self.bits {
            false => SCENDF7_A::_0,
            true => SCENDF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF7_A::_1
    }
}
#[doc = "Field `SCENDF8` reader - Scan Group n Scan End Flag"]
pub type SCENDF8_R = crate::BitReader<SCENDF8_A>;
#[doc = "Scan Group n Scan End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDF8_A {
    #[doc = "0: Scan group n has not been scanned"]
    _0 = 0,
    #[doc = "1: End of scan for scan group n is detected"]
    _1 = 1,
}
impl From<SCENDF8_A> for bool {
    #[inline(always)]
    fn from(variant: SCENDF8_A) -> Self {
        variant as u8 != 0
    }
}
impl SCENDF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCENDF8_A {
        match self.bits {
            false => SCENDF8_A::_0,
            true => SCENDF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCENDF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCENDF8_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf0(&self) -> SCENDF0_R {
        SCENDF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf1(&self) -> SCENDF1_R {
        SCENDF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf2(&self) -> SCENDF2_R {
        SCENDF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf3(&self) -> SCENDF3_R {
        SCENDF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf4(&self) -> SCENDF4_R {
        SCENDF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf5(&self) -> SCENDF5_R {
        SCENDF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf6(&self) -> SCENDF6_R {
        SCENDF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf7(&self) -> SCENDF7_R {
        SCENDF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n Scan End Flag"]
    #[inline(always)]
    pub fn scendf8(&self) -> SCENDF8_R {
        SCENDF8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Scan End Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adscanendsr](index.html) module"]
pub struct ADSCANENDSR_SPEC;
impl crate::RegisterSpec for ADSCANENDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adscanendsr::R](R) reader structure"]
impl crate::Readable for ADSCANENDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADSCANENDSR to value 0"]
impl crate::Resettable for ADSCANENDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
