#[doc = "Register `FSTATR1` reader"]
pub struct R(crate::R<FSTATR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTATR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTATR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTATR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DRRDY` reader - Data Read Ready Flag"]
pub type DRRDY_R = crate::BitReader<DRRDY_A>;
#[doc = "Data Read Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRRDY_A {
    #[doc = "0: The read processing of the consecutive read command at each address is not terminated."]
    _0 = 0,
    #[doc = "1: The read processing of the consecutive read command at each address is terminated and read data is stored to the FRBH and FRBL registers."]
    _1 = 1,
}
impl From<DRRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DRRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl DRRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRRDY_A {
        match self.bits {
            false => DRRDY_A::_0,
            true => DRRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRRDY_A::_1
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Flag"]
pub type FRDY_R = crate::BitReader<FRDY_A>;
#[doc = "Flash Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDY_A {
    #[doc = "0: The software command of the FCR register is not terminated."]
    _0 = 0,
    #[doc = "1: The software command of the FCR register is terminated."]
    _1 = 1,
}
impl From<FRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl FRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDY_A {
        match self.bits {
            false => FRDY_A::_0,
            true => FRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDY_A::_1
    }
}
#[doc = "Field `EXRDY` reader - Extra Area Ready Flag"]
pub type EXRDY_R = crate::BitReader<EXRDY_A>;
#[doc = "Extra Area Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXRDY_A {
    #[doc = "0: The software command of the FEXCR register is not terminated."]
    _0 = 0,
    #[doc = "1: The software command of the FEXCR register is terminated."]
    _1 = 1,
}
impl From<EXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: EXRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl EXRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXRDY_A {
        match self.bits {
            false => EXRDY_A::_0,
            true => EXRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXRDY_A::_1
    }
}
impl R {
    #[doc = "Bit 1 - Data Read Ready Flag"]
    #[inline(always)]
    pub fn drrdy(&self) -> DRRDY_R {
        DRRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Flash Ready Flag"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Extra Area Ready Flag"]
    #[inline(always)]
    pub fn exrdy(&self) -> EXRDY_R {
        EXRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Flash Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstatr1](index.html) module"]
pub struct FSTATR1_SPEC;
impl crate::RegisterSpec for FSTATR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fstatr1::R](R) reader structure"]
impl crate::Readable for FSTATR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSTATR1 to value 0x04"]
impl crate::Resettable for FSTATR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
