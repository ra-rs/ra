#[doc = "Register `GR%s_MON` reader"]
pub struct R(crate::R<GR_MON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_MON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_MON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_MON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ARCST` reader - Status monitor for alpha blending in rectangular area"]
pub type ARCST_R = crate::BitReader<ARCST_A>;
#[doc = "Status monitor for alpha blending in rectangular area\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARCST_A {
    #[doc = "1: Fade-in/fade-out is in progress."]
    _1 = 1,
    #[doc = "0: Fade-in/fade-out is not in progress."]
    _0 = 0,
}
impl From<ARCST_A> for bool {
    #[inline(always)]
    fn from(variant: ARCST_A) -> Self {
        variant as u8 != 0
    }
}
impl ARCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARCST_A {
        match self.bits {
            true => ARCST_A::_1,
            false => ARCST_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARCST_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARCST_A::_0
    }
}
#[doc = "Field `UNDFLST` reader - Status monitor for underflow"]
pub type UNDFLST_R = crate::BitReader<UNDFLST_A>;
#[doc = "Status monitor for underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDFLST_A {
    #[doc = "1: An underflow occurs in internal operations."]
    _1 = 1,
    #[doc = "0: No underflow occurs in internal operations."]
    _0 = 0,
}
impl From<UNDFLST_A> for bool {
    #[inline(always)]
    fn from(variant: UNDFLST_A) -> Self {
        variant as u8 != 0
    }
}
impl UNDFLST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDFLST_A {
        match self.bits {
            true => UNDFLST_A::_1,
            false => UNDFLST_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNDFLST_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNDFLST_A::_0
    }
}
impl R {
    #[doc = "Bit 0 - Status monitor for alpha blending in rectangular area"]
    #[inline(always)]
    pub fn arcst(&self) -> ARCST_R {
        ARCST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Status monitor for underflow"]
    #[inline(always)]
    pub fn undflst(&self) -> UNDFLST_R {
        UNDFLST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Graphics %s Status Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_mon](index.html) module"]
pub struct GR_MON_SPEC;
impl crate::RegisterSpec for GR_MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_mon::R](R) reader structure"]
impl crate::Readable for GR_MON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GR%s_MON to value 0"]
impl crate::Resettable for GR_MON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
