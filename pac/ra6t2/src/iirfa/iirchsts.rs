#[doc = "Register `IIRCH%sSTS` reader"]
pub struct R(crate::R<IIRCHSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRCHSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRCHSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRCHSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPRCS` reader - Channel processing status flag"]
pub type CPRCS_R = crate::BitReader<CPRCS_A>;
#[doc = "Channel processing status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPRCS_A {
    #[doc = "0: The channel processing is not being performed."]
    _0 = 0,
    #[doc = "1: The channel processing is being performed."]
    _1 = 1,
}
impl From<CPRCS_A> for bool {
    #[inline(always)]
    fn from(variant: CPRCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CPRCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPRCS_A {
        match self.bits {
            false => CPRCS_A::_0,
            true => CPRCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPRCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPRCS_A::_1
    }
}
#[doc = "Field `CPRCFF` reader - Channel processing completion flag"]
pub type CPRCFF_R = crate::BitReader<CPRCFF_A>;
#[doc = "Channel processing completion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPRCFF_A {
    #[doc = "0: The channel processing is not completed."]
    _0 = 0,
    #[doc = "1: The channel processing is completed."]
    _1 = 1,
}
impl From<CPRCFF_A> for bool {
    #[inline(always)]
    fn from(variant: CPRCFF_A) -> Self {
        variant as u8 != 0
    }
}
impl CPRCFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPRCFF_A {
        match self.bits {
            false => CPRCFF_A::_0,
            true => CPRCFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPRCFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPRCFF_A::_1
    }
}
#[doc = "Field `ORDYF` reader - Output data preparation completion flag"]
pub type ORDYF_R = crate::BitReader<ORDYF_A>;
#[doc = "Output data preparation completion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORDYF_A {
    #[doc = "0: The output data preparation is not completed."]
    _0 = 0,
    #[doc = "1: The output data preparation is completed."]
    _1 = 1,
}
impl From<ORDYF_A> for bool {
    #[inline(always)]
    fn from(variant: ORDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl ORDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORDYF_A {
        match self.bits {
            false => ORDYF_A::_0,
            true => ORDYF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORDYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORDYF_A::_1
    }
}
#[doc = "Field `CERRF` reader - Operation error flag"]
pub type CERRF_R = crate::BitReader<CERRF_A>;
#[doc = "Operation error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERRF_A {
    #[doc = "0: No operation error has occurred."]
    _0 = 0,
    #[doc = "1: An operation error has occurred."]
    _1 = 1,
}
impl From<CERRF_A> for bool {
    #[inline(always)]
    fn from(variant: CERRF_A) -> Self {
        variant as u8 != 0
    }
}
impl CERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERRF_A {
        match self.bits {
            false => CERRF_A::_0,
            true => CERRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERRF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Channel processing status flag"]
    #[inline(always)]
    pub fn cprcs(&self) -> CPRCS_R {
        CPRCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel processing completion flag"]
    #[inline(always)]
    pub fn cprcff(&self) -> CPRCFF_R {
        CPRCFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output data preparation completion flag"]
    #[inline(always)]
    pub fn ordyf(&self) -> ORDYF_R {
        ORDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operation error flag"]
    #[inline(always)]
    pub fn cerrf(&self) -> CERRF_R {
        CERRF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Channel %s Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirchsts](index.html) module"]
pub struct IIRCHSTS_SPEC;
impl crate::RegisterSpec for IIRCHSTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iirchsts::R](R) reader structure"]
impl crate::Readable for IIRCHSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIRCH%sSTS to value 0"]
impl crate::Resettable for IIRCHSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
