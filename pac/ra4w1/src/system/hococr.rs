#[doc = "Register `HOCOCR` reader"]
pub struct R(crate::R<HOCOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOCOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOCOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOCOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOCOCR` writer"]
pub struct W(crate::W<HOCOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOCOCR_SPEC>;
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
impl From<crate::W<HOCOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOCOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCSTP` reader - HOCO Stop"]
pub type HCSTP_R = crate::BitReader<HCSTP_A>;
#[doc = "HOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCSTP_A {
    #[doc = "0: HOCO is operating."]
    _0 = 0,
    #[doc = "1: HOCO is stopped."]
    _1 = 1,
}
impl From<HCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: HCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl HCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCSTP_A {
        match self.bits {
            false => HCSTP_A::_0,
            true => HCSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCSTP_A::_1
    }
}
#[doc = "Field `HCSTP` writer - HOCO Stop"]
pub type HCSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, HOCOCR_SPEC, HCSTP_A, O>;
impl<'a, const O: u8> HCSTP_W<'a, O> {
    #[doc = "HOCO is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCSTP_A::_0)
    }
    #[doc = "HOCO is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(&self) -> HCSTP_R {
        HCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HOCO Stop"]
    #[inline(always)]
    #[must_use]
    pub fn hcstp(&mut self) -> HCSTP_W<0> {
        HCSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High-Speed On-Chip Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hococr](index.html) module"]
pub struct HOCOCR_SPEC;
impl crate::RegisterSpec for HOCOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hococr::R](R) reader structure"]
impl crate::Readable for HOCOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hococr::W](W) writer structure"]
impl crate::Writable for HOCOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOCOCR to value 0"]
impl crate::Resettable for HOCOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
