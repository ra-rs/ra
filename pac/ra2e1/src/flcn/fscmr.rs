#[doc = "Register `FSCMR` reader"]
pub struct R(crate::R<FSCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SASMF` reader - Startup Area Setting Monitor Flag"]
pub type SASMF_R = crate::BitReader<SASMF_A>;
#[doc = "Startup Area Setting Monitor Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SASMF_A {
    #[doc = "0: Setting to start up using the alternative area"]
    _0 = 0,
    #[doc = "1: Setting to start up using the default area"]
    _1 = 1,
}
impl From<SASMF_A> for bool {
    #[inline(always)]
    fn from(variant: SASMF_A) -> Self {
        variant as u8 != 0
    }
}
impl SASMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SASMF_A {
        match self.bits {
            false => SASMF_A::_0,
            true => SASMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SASMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SASMF_A::_1
    }
}
#[doc = "Field `FSPR` reader - Access Window Protection Flag"]
pub type FSPR_R = crate::BitReader<FSPR_A>;
#[doc = "Access Window Protection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPR_A {
    #[doc = "0: Access window setting disabled."]
    _0 = 0,
    #[doc = "1: Access window setting enabled."]
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
impl R {
    #[doc = "Bit 8 - Startup Area Setting Monitor Flag"]
    #[inline(always)]
    pub fn sasmf(&self) -> SASMF_R {
        SASMF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Access Window Protection Flag"]
    #[inline(always)]
    pub fn fspr(&self) -> FSPR_R {
        FSPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Flash Start-Up Setting Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscmr](index.html) module"]
pub struct FSCMR_SPEC;
impl crate::RegisterSpec for FSCMR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fscmr::R](R) reader structure"]
impl crate::Readable for FSCMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSCMR to value 0"]
impl crate::Resettable for FSCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
