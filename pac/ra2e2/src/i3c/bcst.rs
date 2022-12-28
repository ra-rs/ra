#[doc = "Register `BCST` reader"]
pub struct R(crate::R<BCST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BFREF` reader - Bus Free Detection Flag"]
pub type BFREF_R = crate::BitReader<BFREF_A>;
#[doc = "Bus Free Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFREF_A {
    #[doc = "0: Have not Detected Bus Free"]
    _0 = 0,
    #[doc = "1: Have Detected Bus Free"]
    _1 = 1,
}
impl From<BFREF_A> for bool {
    #[inline(always)]
    fn from(variant: BFREF_A) -> Self {
        variant as u8 != 0
    }
}
impl BFREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFREF_A {
        match self.bits {
            false => BFREF_A::_0,
            true => BFREF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFREF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFREF_A::_1
    }
}
#[doc = "Field `BAVLF` reader - Bus Available Detection Flag"]
pub type BAVLF_R = crate::BitReader<BAVLF_A>;
#[doc = "Bus Available Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BAVLF_A {
    #[doc = "0: Have not Detected Bus Available"]
    _0 = 0,
    #[doc = "1: Have Detected Bus Available"]
    _1 = 1,
}
impl From<BAVLF_A> for bool {
    #[inline(always)]
    fn from(variant: BAVLF_A) -> Self {
        variant as u8 != 0
    }
}
impl BAVLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BAVLF_A {
        match self.bits {
            false => BAVLF_A::_0,
            true => BAVLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BAVLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BAVLF_A::_1
    }
}
#[doc = "Field `BIDLF` reader - Bus Idle Detection Flag"]
pub type BIDLF_R = crate::BitReader<BIDLF_A>;
#[doc = "Bus Idle Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDLF_A {
    #[doc = "0: Have not Detected Bus Idle"]
    _0 = 0,
    #[doc = "1: Have Detected Bus Idle"]
    _1 = 1,
}
impl From<BIDLF_A> for bool {
    #[inline(always)]
    fn from(variant: BIDLF_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDLF_A {
        match self.bits {
            false => BIDLF_A::_0,
            true => BIDLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIDLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIDLF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Bus Free Detection Flag"]
    #[inline(always)]
    pub fn bfref(&self) -> BFREF_R {
        BFREF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus Available Detection Flag"]
    #[inline(always)]
    pub fn bavlf(&self) -> BAVLF_R {
        BAVLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Idle Detection Flag"]
    #[inline(always)]
    pub fn bidlf(&self) -> BIDLF_R {
        BIDLF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Bus Condition Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcst](index.html) module"]
pub struct BCST_SPEC;
impl crate::RegisterSpec for BCST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcst::R](R) reader structure"]
impl crate::Readable for BCST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCST to value 0"]
impl crate::Resettable for BCST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
