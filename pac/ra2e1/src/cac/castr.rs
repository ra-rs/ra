#[doc = "Register `CASTR` reader"]
pub struct R(crate::R<CASTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CASTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CASTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CASTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FERRF` reader - Frequency Error Flag"]
pub type FERRF_R = crate::BitReader<FERRF_A>;
#[doc = "Frequency Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FERRF_A {
    #[doc = "0: Clock frequency is within the allowable range"]
    _0 = 0,
    #[doc = "1: Clock frequency has deviated beyond the allowable range (frequency error)."]
    _1 = 1,
}
impl From<FERRF_A> for bool {
    #[inline(always)]
    fn from(variant: FERRF_A) -> Self {
        variant as u8 != 0
    }
}
impl FERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FERRF_A {
        match self.bits {
            false => FERRF_A::_0,
            true => FERRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FERRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FERRF_A::_1
    }
}
#[doc = "Field `MENDF` reader - Measurement End Flag"]
pub type MENDF_R = crate::BitReader<MENDF_A>;
#[doc = "Measurement End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MENDF_A {
    #[doc = "0: Measurement is in progress"]
    _0 = 0,
    #[doc = "1: Measurement ended"]
    _1 = 1,
}
impl From<MENDF_A> for bool {
    #[inline(always)]
    fn from(variant: MENDF_A) -> Self {
        variant as u8 != 0
    }
}
impl MENDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MENDF_A {
        match self.bits {
            false => MENDF_A::_0,
            true => MENDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MENDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MENDF_A::_1
    }
}
#[doc = "Field `OVFF` reader - Overflow Flag"]
pub type OVFF_R = crate::BitReader<OVFF_A>;
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVFF_A {
    #[doc = "0: Counter has not overflowed"]
    _0 = 0,
    #[doc = "1: Counter overflowed"]
    _1 = 1,
}
impl From<OVFF_A> for bool {
    #[inline(always)]
    fn from(variant: OVFF_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFF_A {
        match self.bits {
            false => OVFF_A::_0,
            true => OVFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Frequency Error Flag"]
    #[inline(always)]
    pub fn ferrf(&self) -> FERRF_R {
        FERRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Measurement End Flag"]
    #[inline(always)]
    pub fn mendf(&self) -> MENDF_R {
        MENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "CAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [castr](index.html) module"]
pub struct CASTR_SPEC;
impl crate::RegisterSpec for CASTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [castr::R](R) reader structure"]
impl crate::Readable for CASTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CASTR to value 0"]
impl crate::Resettable for CASTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
