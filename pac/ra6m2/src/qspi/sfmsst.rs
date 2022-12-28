#[doc = "Register `SFMSST` reader"]
pub struct R(crate::R<SFMSST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PFCNT` reader - Number of bytes of prefetched dataRange: 00000 - 10010 (No combination other than the above is available.)"]
pub type PFCNT_R = crate::FieldReader<u8, PFCNT_A>;
#[doc = "Number of bytes of prefetched dataRange: 00000 - 10010 (No combination other than the above is available.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PFCNT_A {
    #[doc = "0: Nodata has been prefetched."]
    _00000 = 0,
}
impl From<PFCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: PFCNT_A) -> Self {
        variant as _
    }
}
impl PFCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PFCNT_A> {
        match self.bits {
            0 => Some(PFCNT_A::_00000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == PFCNT_A::_00000
    }
}
#[doc = "Field `PFFUL` reader - Prefetch buffer state"]
pub type PFFUL_R = crate::BitReader<PFFUL_A>;
#[doc = "Prefetch buffer state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFFUL_A {
    #[doc = "0: The prefetch buffer has a free space."]
    _0 = 0,
    #[doc = "1: The prefetch buffer is full."]
    _1 = 1,
}
impl From<PFFUL_A> for bool {
    #[inline(always)]
    fn from(variant: PFFUL_A) -> Self {
        variant as u8 != 0
    }
}
impl PFFUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFFUL_A {
        match self.bits {
            false => PFFUL_A::_0,
            true => PFFUL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFFUL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFFUL_A::_1
    }
}
#[doc = "Field `PFOFF` reader - Prefetch function operation state"]
pub type PFOFF_R = crate::BitReader<PFOFF_A>;
#[doc = "Prefetch function operation state\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFOFF_A {
    #[doc = "0: The prefetch function is operating."]
    _0 = 0,
    #[doc = "1: The prefetch function is not enabled or is not operating."]
    _1 = 1,
}
impl From<PFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: PFOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl PFOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFOFF_A {
        match self.bits {
            false => PFOFF_A::_0,
            true => PFOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFOFF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of bytes of prefetched dataRange: 00000 - 10010 (No combination other than the above is available.)"]
    #[inline(always)]
    pub fn pfcnt(&self) -> PFCNT_R {
        PFCNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Prefetch buffer state"]
    #[inline(always)]
    pub fn pfful(&self) -> PFFUL_R {
        PFFUL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Prefetch function operation state"]
    #[inline(always)]
    pub fn pfoff(&self) -> PFOFF_R {
        PFOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmsst](index.html) module"]
pub struct SFMSST_SPEC;
impl crate::RegisterSpec for SFMSST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmsst::R](R) reader structure"]
impl crate::Readable for SFMSST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SFMSST to value 0x80"]
impl crate::Resettable for SFMSST_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
