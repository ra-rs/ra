#[doc = "Register `ADWINMON` reader"]
pub struct R(crate::R<ADWINMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADWINMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADWINMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADWINMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MONCOMB` reader - Combination Result Monitor"]
pub type MONCOMB_R = crate::BitReader<MONCOMB_A>;
#[doc = "Combination Result Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONCOMB_A {
    #[doc = "0: Window A/B composite conditions are not met."]
    _0 = 0,
    #[doc = "1: Window A/B composite conditions are met."]
    _1 = 1,
}
impl From<MONCOMB_A> for bool {
    #[inline(always)]
    fn from(variant: MONCOMB_A) -> Self {
        variant as u8 != 0
    }
}
impl MONCOMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONCOMB_A {
        match self.bits {
            false => MONCOMB_A::_0,
            true => MONCOMB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONCOMB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONCOMB_A::_1
    }
}
#[doc = "Field `MONCMPA` reader - Comparison Result Monitor A"]
pub type MONCMPA_R = crate::BitReader<MONCMPA_A>;
#[doc = "Comparison Result Monitor A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONCMPA_A {
    #[doc = "0: Window A comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Window A comparison conditions are met."]
    _1 = 1,
}
impl From<MONCMPA_A> for bool {
    #[inline(always)]
    fn from(variant: MONCMPA_A) -> Self {
        variant as u8 != 0
    }
}
impl MONCMPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONCMPA_A {
        match self.bits {
            false => MONCMPA_A::_0,
            true => MONCMPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONCMPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONCMPA_A::_1
    }
}
#[doc = "Field `MONCMPB` reader - Comparison Result Monitor B"]
pub type MONCMPB_R = crate::BitReader<MONCMPB_A>;
#[doc = "Comparison Result Monitor B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONCMPB_A {
    #[doc = "0: Window B comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Window B comparison conditions are met."]
    _1 = 1,
}
impl From<MONCMPB_A> for bool {
    #[inline(always)]
    fn from(variant: MONCMPB_A) -> Self {
        variant as u8 != 0
    }
}
impl MONCMPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONCMPB_A {
        match self.bits {
            false => MONCMPB_A::_0,
            true => MONCMPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONCMPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONCMPB_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Combination Result Monitor"]
    #[inline(always)]
    pub fn moncomb(&self) -> MONCOMB_R {
        MONCOMB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison Result Monitor A"]
    #[inline(always)]
    pub fn moncmpa(&self) -> MONCMPA_R {
        MONCMPA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison Result Monitor B"]
    #[inline(always)]
    pub fn moncmpb(&self) -> MONCMPB_R {
        MONCMPB_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "A/D Compare Function Window A/B Status Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adwinmon](index.html) module"]
pub struct ADWINMON_SPEC;
impl crate::RegisterSpec for ADWINMON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adwinmon::R](R) reader structure"]
impl crate::Readable for ADWINMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADWINMON to value 0"]
impl crate::Resettable for ADWINMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
