#[doc = "Register `MSSR` reader"]
pub struct R(crate::R<MSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MBNST` reader - Search Result Mailbox Number Status"]
pub type MBNST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEST` reader - Search Result Status"]
pub type SEST_R = crate::BitReader<SEST_A>;
#[doc = "Search Result Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEST_A {
    #[doc = "0: Search result found"]
    _0 = 0,
    #[doc = "1: No search result"]
    _1 = 1,
}
impl From<SEST_A> for bool {
    #[inline(always)]
    fn from(variant: SEST_A) -> Self {
        variant as u8 != 0
    }
}
impl SEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEST_A {
        match self.bits {
            false => SEST_A::_0,
            true => SEST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEST_A::_1
    }
}
impl R {
    #[doc = "Bits 0:4 - Search Result Mailbox Number Status"]
    #[inline(always)]
    pub fn mbnst(&self) -> MBNST_R {
        MBNST_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Search Result Status"]
    #[inline(always)]
    pub fn sest(&self) -> SEST_R {
        SEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Mailbox Search Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mssr](index.html) module"]
pub struct MSSR_SPEC;
impl crate::RegisterSpec for MSSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mssr::R](R) reader structure"]
impl crate::Readable for MSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSSR to value 0x80"]
impl crate::Resettable for MSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
