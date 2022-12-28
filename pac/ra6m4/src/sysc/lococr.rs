#[doc = "Register `LOCOCR` reader"]
pub struct R(crate::R<LOCOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCOCR` writer"]
pub struct W(crate::W<LOCOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCOCR_SPEC>;
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
impl From<crate::W<LOCOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCSTP` reader - LOCO Stop"]
pub type LCSTP_R = crate::BitReader<LCSTP_A>;
#[doc = "LOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCSTP_A {
    #[doc = "0: Operate the LOCO clock"]
    _0 = 0,
    #[doc = "1: Stop the LOCO clock"]
    _1 = 1,
}
impl From<LCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: LCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl LCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCSTP_A {
        match self.bits {
            false => LCSTP_A::_0,
            true => LCSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCSTP_A::_1
    }
}
#[doc = "Field `LCSTP` writer - LOCO Stop"]
pub type LCSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOCOCR_SPEC, LCSTP_A, O>;
impl<'a, const O: u8> LCSTP_W<'a, O> {
    #[doc = "Operate the LOCO clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCSTP_A::_0)
    }
    #[doc = "Stop the LOCO clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(&self) -> LCSTP_R {
        LCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCO Stop"]
    #[inline(always)]
    #[must_use]
    pub fn lcstp(&mut self) -> LCSTP_W<0> {
        LCSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low-Speed On-Chip Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lococr](index.html) module"]
pub struct LOCOCR_SPEC;
impl crate::RegisterSpec for LOCOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lococr::R](R) reader structure"]
impl crate::Readable for LOCOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lococr::W](W) writer structure"]
impl crate::Writable for LOCOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCOCR to value 0"]
impl crate::Resettable for LOCOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
