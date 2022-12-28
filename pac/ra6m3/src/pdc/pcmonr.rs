#[doc = "Register `PCMONR` reader"]
pub struct R(crate::R<PCMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VSYNC` reader - VSYNC Signal Status Flag"]
pub type VSYNC_R = crate::BitReader<VSYNC_A>;
#[doc = "VSYNC Signal Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_A {
    #[doc = "0: VSYNC signal is at the low level."]
    _0 = 0,
    #[doc = "1: VSYNC signal is at the high level."]
    _1 = 1,
}
impl From<VSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl VSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNC_A {
        match self.bits {
            false => VSYNC_A::_0,
            true => VSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VSYNC_A::_1
    }
}
#[doc = "Field `HSYNC` reader - HSYNC Signal Status Flag"]
pub type HSYNC_R = crate::BitReader<HSYNC_A>;
#[doc = "HSYNC Signal Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSYNC_A {
    #[doc = "0: HSYNC signal is at the low level."]
    _0 = 0,
    #[doc = "1: HSYNC signal is at the high level."]
    _1 = 1,
}
impl From<HSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl HSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSYNC_A {
        match self.bits {
            false => HSYNC_A::_0,
            true => HSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSYNC_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - VSYNC Signal Status Flag"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSYNC Signal Status Flag"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PDC Pin Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmonr](index.html) module"]
pub struct PCMONR_SPEC;
impl crate::RegisterSpec for PCMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcmonr::R](R) reader structure"]
impl crate::Readable for PCMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCMONR to value 0"]
impl crate::Resettable for PCMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
