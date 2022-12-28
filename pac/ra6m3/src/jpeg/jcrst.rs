#[doc = "Register `JCRST` reader"]
pub struct R(crate::R<JCRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RST` reader - Operating State"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Operating State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: State other than below"]
    _0 = 0,
    #[doc = "1: Suspended state caused by interrupt sources of JINTE0"]
    _1 = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::_0,
            true => RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Operating State"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 1) != 0)
    }
}
#[doc = "JPEG Code Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcrst](index.html) module"]
pub struct JCRST_SPEC;
impl crate::RegisterSpec for JCRST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcrst::R](R) reader structure"]
impl crate::Readable for JCRST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JCRST to value 0"]
impl crate::Resettable for JCRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
