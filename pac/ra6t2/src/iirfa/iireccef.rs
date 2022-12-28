#[doc = "Register `IIRECCEF` reader"]
pub struct R(crate::R<IIRECCEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRECCEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRECCEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRECCEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ESEF` reader - ECC 1-bit error flag"]
pub type ESEF_R = crate::BitReader<ESEF_A>;
#[doc = "ECC 1-bit error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESEF_A {
    #[doc = "0: No 1-bit ECC error is detected."]
    _0 = 0,
    #[doc = "1: 1-bit ECC error is detected."]
    _1 = 1,
}
impl From<ESEF_A> for bool {
    #[inline(always)]
    fn from(variant: ESEF_A) -> Self {
        variant as u8 != 0
    }
}
impl ESEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESEF_A {
        match self.bits {
            false => ESEF_A::_0,
            true => ESEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESEF_A::_1
    }
}
#[doc = "Field `EDEF` reader - ECC 2-bit error flag"]
pub type EDEF_R = crate::BitReader<EDEF_A>;
#[doc = "ECC 2-bit error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDEF_A {
    #[doc = "0: No 2-bit ECC error is detected."]
    _0 = 0,
    #[doc = "1: 2-bit ECC error is detected."]
    _1 = 1,
}
impl From<EDEF_A> for bool {
    #[inline(always)]
    fn from(variant: EDEF_A) -> Self {
        variant as u8 != 0
    }
}
impl EDEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDEF_A {
        match self.bits {
            false => EDEF_A::_0,
            true => EDEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDEF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-bit error flag"]
    #[inline(always)]
    pub fn esef(&self) -> ESEF_R {
        ESEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC 2-bit error flag"]
    #[inline(always)]
    pub fn edef(&self) -> EDEF_R {
        EDEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "ECC Error Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iireccef](index.html) module"]
pub struct IIRECCEF_SPEC;
impl crate::RegisterSpec for IIRECCEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iireccef::R](R) reader structure"]
impl crate::Readable for IIRECCEF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRECCEF to value 0"]
impl crate::Resettable for IIRECCEF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
