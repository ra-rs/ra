#[doc = "Register `SYSCNT_STMON` reader"]
pub struct R(crate::R<SYSCNT_STMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCNT_STMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCNT_STMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCNT_STMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VPOS` reader - Graphics 2 specified line detection flag"]
pub type VPOS_R = crate::BitReader<VPOS_A>;
#[doc = "Graphics 2 specified line detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOS_A {
    #[doc = "1: A specified line notification has been detected in graphics 2."]
    _1 = 1,
    #[doc = "0: No specified line notification has been detected in graphics 2."]
    _0 = 0,
}
impl From<VPOS_A> for bool {
    #[inline(always)]
    fn from(variant: VPOS_A) -> Self {
        variant as u8 != 0
    }
}
impl VPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPOS_A {
        match self.bits {
            true => VPOS_A::_1,
            false => VPOS_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOS_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOS_A::_0
    }
}
#[doc = "Field `L1UNDF` reader - Graphics 1 underflow detection flag"]
pub type L1UNDF_R = crate::BitReader<L1UNDF_A>;
#[doc = "Graphics 1 underflow detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDF_A {
    #[doc = "1: An underflow has been detected in graphics 1."]
    _1 = 1,
    #[doc = "0: No underflow has been detected in graphics 1."]
    _0 = 0,
}
impl From<L1UNDF_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDF_A) -> Self {
        variant as u8 != 0
    }
}
impl L1UNDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1UNDF_A {
        match self.bits {
            true => L1UNDF_A::_1,
            false => L1UNDF_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDF_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDF_A::_0
    }
}
#[doc = "Field `L2UNDF` reader - Graphics 2 underflow detection flag"]
pub type L2UNDF_R = crate::BitReader<L2UNDF_A>;
#[doc = "Graphics 2 underflow detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDF_A {
    #[doc = "1: An underflow has been detected in graphics 2."]
    _1 = 1,
    #[doc = "0: No underflow has been detected in graphics 2."]
    _0 = 0,
}
impl From<L2UNDF_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDF_A) -> Self {
        variant as u8 != 0
    }
}
impl L2UNDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L2UNDF_A {
        match self.bits {
            true => L2UNDF_A::_1,
            false => L2UNDF_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDF_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDF_A::_0
    }
}
impl R {
    #[doc = "Bit 0 - Graphics 2 specified line detection flag"]
    #[inline(always)]
    pub fn vpos(&self) -> VPOS_R {
        VPOS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Graphics 1 underflow detection flag"]
    #[inline(always)]
    pub fn l1undf(&self) -> L1UNDF_R {
        L1UNDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Graphics 2 underflow detection flag"]
    #[inline(always)]
    pub fn l2undf(&self) -> L2UNDF_R {
        L2UNDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "System Control Block Status Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscnt_stmon](index.html) module"]
pub struct SYSCNT_STMON_SPEC;
impl crate::RegisterSpec for SYSCNT_STMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscnt_stmon::R](R) reader structure"]
impl crate::Readable for SYSCNT_STMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCNT_STMON to value 0"]
impl crate::Resettable for SYSCNT_STMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
