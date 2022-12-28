#[doc = "Register `FSTATR2` reader"]
pub struct R(crate::R<FSTATR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTATR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTATR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTATR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERERR` reader - Erase Error Flag"]
pub type ERERR_R = crate::BitReader<ERERR_A>;
#[doc = "Erase Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERERR_A {
    #[doc = "0: Erasure terminates normally"]
    _0 = 0,
    #[doc = "1: An error occurs during erasure"]
    _1 = 1,
}
impl From<ERERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERERR_A {
        match self.bits {
            false => ERERR_A::_0,
            true => ERERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERERR_A::_1
    }
}
#[doc = "Field `PRGERR` reader - Program Error Flag"]
pub type PRGERR_R = crate::BitReader<PRGERR_A>;
#[doc = "Program Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGERR_A {
    #[doc = "0: Programming terminates normally"]
    _0 = 0,
    #[doc = "1: An error occurs during programming."]
    _1 = 1,
}
impl From<PRGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PRGERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGERR_A {
        match self.bits {
            false => PRGERR_A::_0,
            true => PRGERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRGERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRGERR_A::_1
    }
}
#[doc = "Field `PRGERR01` reader - Program Error Flag 01"]
pub type PRGERR01_R = crate::BitReader<PRGERR01_A>;
#[doc = "Program Error Flag 01\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGERR01_A {
    #[doc = "0: Programming by the FEXCR register terminates normally"]
    _0 = 0,
    #[doc = "1: An error occurs during programming."]
    _1 = 1,
}
impl From<PRGERR01_A> for bool {
    #[inline(always)]
    fn from(variant: PRGERR01_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGERR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGERR01_A {
        match self.bits {
            false => PRGERR01_A::_0,
            true => PRGERR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRGERR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRGERR01_A::_1
    }
}
#[doc = "Field `BCERR` reader - Blank Check Error Flag"]
pub type BCERR_R = crate::BitReader<BCERR_A>;
#[doc = "Blank Check Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCERR_A {
    #[doc = "0: Blank checking terminates normally"]
    _0 = 0,
    #[doc = "1: An error occurs during blank checking."]
    _1 = 1,
}
impl From<BCERR_A> for bool {
    #[inline(always)]
    fn from(variant: BCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCERR_A {
        match self.bits {
            false => BCERR_A::_0,
            true => BCERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCERR_A::_1
    }
}
#[doc = "Field `ILGLERR` reader - Illegal Command Error Flag"]
pub type ILGLERR_R = crate::BitReader<ILGLERR_A>;
#[doc = "Illegal Command Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILGLERR_A {
    #[doc = "0: No illegal software command or illegal access is detected"]
    _0 = 0,
    #[doc = "1: An illegal command or illegal access is detected."]
    _1 = 1,
}
impl From<ILGLERR_A> for bool {
    #[inline(always)]
    fn from(variant: ILGLERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ILGLERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILGLERR_A {
        match self.bits {
            false => ILGLERR_A::_0,
            true => ILGLERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILGLERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILGLERR_A::_1
    }
}
#[doc = "Field `EILGLERR` reader - Extra Area Illegal Command Error Flag"]
pub type EILGLERR_R = crate::BitReader<EILGLERR_A>;
#[doc = "Extra Area Illegal Command Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EILGLERR_A {
    #[doc = "0: No illegal command or illegal access to the extra area is detected"]
    _0 = 0,
    #[doc = "1: An illegal command or illegal access to the extra area is detected."]
    _1 = 1,
}
impl From<EILGLERR_A> for bool {
    #[inline(always)]
    fn from(variant: EILGLERR_A) -> Self {
        variant as u8 != 0
    }
}
impl EILGLERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EILGLERR_A {
        match self.bits {
            false => EILGLERR_A::_0,
            true => EILGLERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EILGLERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EILGLERR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Erase Error Flag"]
    #[inline(always)]
    pub fn ererr(&self) -> ERERR_R {
        ERERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Program Error Flag"]
    #[inline(always)]
    pub fn prgerr(&self) -> PRGERR_R {
        PRGERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Program Error Flag 01"]
    #[inline(always)]
    pub fn prgerr01(&self) -> PRGERR01_R {
        PRGERR01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Blank Check Error Flag"]
    #[inline(always)]
    pub fn bcerr(&self) -> BCERR_R {
        BCERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Illegal Command Error Flag"]
    #[inline(always)]
    pub fn ilglerr(&self) -> ILGLERR_R {
        ILGLERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Extra Area Illegal Command Error Flag"]
    #[inline(always)]
    pub fn eilglerr(&self) -> EILGLERR_R {
        EILGLERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Flash Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstatr2](index.html) module"]
pub struct FSTATR2_SPEC;
impl crate::RegisterSpec for FSTATR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fstatr2::R](R) reader structure"]
impl crate::Readable for FSTATR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSTATR2 to value 0"]
impl crate::Resettable for FSTATR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
