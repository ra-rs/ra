#[doc = "Register `BUS3ERRSTAT` reader"]
pub struct R(crate::R<BUS3ERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS3ERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS3ERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS3ERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACCSTAT` reader - Error Access Status flag"]
pub type ACCSTAT_R = crate::BitReader<ACCSTAT_A>;
#[doc = "Error Access Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCSTAT_A {
    #[doc = "0: Read access"]
    _0 = 0,
    #[doc = "1: Write access"]
    _1 = 1,
}
impl From<ACCSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: ACCSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCSTAT_A {
        match self.bits {
            false => ACCSTAT_A::_0,
            true => ACCSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACCSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACCSTAT_A::_1
    }
}
#[doc = "Field `ERRSTAT` reader - Bus Error Status flag"]
pub type ERRSTAT_R = crate::BitReader<ERRSTAT_A>;
#[doc = "Bus Error Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRSTAT_A {
    #[doc = "0: No bus error occurred."]
    _0 = 0,
    #[doc = "1: Bus error occurred."]
    _1 = 1,
}
impl From<ERRSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRSTAT_A {
        match self.bits {
            false => ERRSTAT_A::_0,
            true => ERRSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRSTAT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Error Access Status flag"]
    #[inline(always)]
    pub fn accstat(&self) -> ACCSTAT_R {
        ACCSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Error Status flag"]
    #[inline(always)]
    pub fn errstat(&self) -> ERRSTAT_R {
        ERRSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "BUS Error Status Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus3errstat](index.html) module"]
pub struct BUS3ERRSTAT_SPEC;
impl crate::RegisterSpec for BUS3ERRSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bus3errstat::R](R) reader structure"]
impl crate::Readable for BUS3ERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS3ERRSTAT to value 0"]
impl crate::Resettable for BUS3ERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
