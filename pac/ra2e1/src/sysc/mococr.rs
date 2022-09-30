#[doc = "Register `MOCOCR` reader"]
pub struct R(crate::R<MOCOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOCOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOCOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOCOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOCOCR` writer"]
pub struct W(crate::W<MOCOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOCOCR_SPEC>;
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
impl From<crate::W<MOCOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOCOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCSTP` reader - MOCO Stop"]
pub type MCSTP_R = crate::BitReader<MCSTP_A>;
#[doc = "MOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCSTP_A {
    #[doc = "0: MOCO clock is operating"]
    _0 = 0,
    #[doc = "1: MOCO clock is stopped"]
    _1 = 1,
}
impl From<MCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: MCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl MCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCSTP_A {
        match self.bits {
            false => MCSTP_A::_0,
            true => MCSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCSTP_A::_1
    }
}
#[doc = "Field `MCSTP` writer - MOCO Stop"]
pub type MCSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, MOCOCR_SPEC, MCSTP_A, O>;
impl<'a, const O: u8> MCSTP_W<'a, O> {
    #[doc = "MOCO clock is operating"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCSTP_A::_0)
    }
    #[doc = "MOCO clock is stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - MOCO Stop"]
    #[inline(always)]
    pub fn mcstp(&self) -> MCSTP_R {
        MCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MOCO Stop"]
    #[inline(always)]
    pub fn mcstp(&mut self) -> MCSTP_W<0> {
        MCSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Middle-Speed On-Chip Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mococr](index.html) module"]
pub struct MOCOCR_SPEC;
impl crate::RegisterSpec for MOCOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mococr::R](R) reader structure"]
impl crate::Readable for MOCOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mococr::W](W) writer structure"]
impl crate::Writable for MOCOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOCOCR to value 0"]
impl crate::Resettable for MOCOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
