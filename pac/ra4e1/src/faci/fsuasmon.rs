#[doc = "Register `FSUASMON` reader"]
pub struct R(crate::R<FSUASMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSUASMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSUASMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSUASMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FSPR` reader - Protection Programming Flag to set Boot Flag and Startup Area Control"]
pub type FSPR_R = crate::BitReader<FSPR_A>;
#[doc = "Protection Programming Flag to set Boot Flag and Startup Area Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSPR_A {
    #[doc = "0: Protected state"]
    _0 = 0,
    #[doc = "1: Non-protected state."]
    _1 = 1,
}
impl From<FSPR_A> for bool {
    #[inline(always)]
    fn from(variant: FSPR_A) -> Self {
        variant as u8 != 0
    }
}
impl FSPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSPR_A {
        match self.bits {
            false => FSPR_A::_0,
            true => FSPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSPR_A::_1
    }
}
#[doc = "Field `BTFLG` reader - Flag of Startup Area Select for Boot Swap"]
pub type BTFLG_R = crate::BitReader<BTFLG_A>;
#[doc = "Flag of Startup Area Select for Boot Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTFLG_A {
    #[doc = "0: The startup area is the alternate block (block 1)"]
    _0 = 0,
    #[doc = "1: The startup area is the default block (block 0)."]
    _1 = 1,
}
impl From<BTFLG_A> for bool {
    #[inline(always)]
    fn from(variant: BTFLG_A) -> Self {
        variant as u8 != 0
    }
}
impl BTFLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTFLG_A {
        match self.bits {
            false => BTFLG_A::_0,
            true => BTFLG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BTFLG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BTFLG_A::_1
    }
}
impl R {
    #[doc = "Bit 15 - Protection Programming Flag to set Boot Flag and Startup Area Control"]
    #[inline(always)]
    pub fn fspr(&self) -> FSPR_R {
        FSPR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Flag of Startup Area Select for Boot Swap"]
    #[inline(always)]
    pub fn btflg(&self) -> BTFLG_R {
        BTFLG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Flash Startup Area Select Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsuasmon](index.html) module"]
pub struct FSUASMON_SPEC;
impl crate::RegisterSpec for FSUASMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsuasmon::R](R) reader structure"]
impl crate::Readable for FSUASMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSUASMON to value 0"]
impl crate::Resettable for FSUASMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
