#[doc = "Register `MOSCCR` reader"]
pub struct R(crate::R<MOSCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSCCR` writer"]
pub struct W(crate::W<MOSCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSCCR_SPEC>;
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
impl From<crate::W<MOSCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOSTP` reader - Main Clock Oscillator Stop"]
pub type MOSTP_R = crate::BitReader<MOSTP_A>;
#[doc = "Main Clock Oscillator Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSTP_A {
    #[doc = "0: Operate the main clock oscillator"]
    _0 = 0,
    #[doc = "1: Stop the main clock oscillator"]
    _1 = 1,
}
impl From<MOSTP_A> for bool {
    #[inline(always)]
    fn from(variant: MOSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl MOSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOSTP_A {
        match self.bits {
            false => MOSTP_A::_0,
            true => MOSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSTP_A::_1
    }
}
#[doc = "Field `MOSTP` writer - Main Clock Oscillator Stop"]
pub type MOSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, MOSCCR_SPEC, MOSTP_A, O>;
impl<'a, const O: u8> MOSTP_W<'a, O> {
    #[doc = "Operate the main clock oscillator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOSTP_A::_0)
    }
    #[doc = "Stop the main clock oscillator"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Main Clock Oscillator Stop"]
    #[inline(always)]
    pub fn mostp(&self) -> MOSTP_R {
        MOSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Clock Oscillator Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mostp(&mut self) -> MOSTP_W<0> {
        MOSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosccr](index.html) module"]
pub struct MOSCCR_SPEC;
impl crate::RegisterSpec for MOSCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mosccr::R](R) reader structure"]
impl crate::Readable for MOSCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mosccr::W](W) writer structure"]
impl crate::Writable for MOSCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOSCCR to value 0x01"]
impl crate::Resettable for MOSCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
