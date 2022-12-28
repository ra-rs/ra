#[doc = "Register `FBCSTAT` reader"]
pub struct R(crate::R<FBCSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBCSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBCSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBCSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBCSTAT` writer"]
pub struct W(crate::W<FBCSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBCSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FBCSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBCSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCST` reader - Blank Check Status Flag"]
pub type BCST_R = crate::BitReader<BCST_A>;
#[doc = "Blank Check Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCST_A {
    #[doc = "0: The target area is in the non-programmed state, that is, the area has been erased but has not yet been reprogrammed"]
    _0 = 0,
    #[doc = "1: The target area has been programmed with 0s or 1s."]
    _1 = 1,
}
impl From<BCST_A> for bool {
    #[inline(always)]
    fn from(variant: BCST_A) -> Self {
        variant as u8 != 0
    }
}
impl BCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCST_A {
        match self.bits {
            false => BCST_A::_0,
            true => BCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Blank Check Status Flag"]
    #[inline(always)]
    pub fn bcst(&self) -> BCST_R {
        BCST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Blank Check Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbcstat](index.html) module"]
pub struct FBCSTAT_SPEC;
impl crate::RegisterSpec for FBCSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fbcstat::R](R) reader structure"]
impl crate::Readable for FBCSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbcstat::W](W) writer structure"]
impl crate::Writable for FBCSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBCSTAT to value 0"]
impl crate::Resettable for FBCSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
