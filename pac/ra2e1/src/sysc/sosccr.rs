#[doc = "Register `SOSCCR` reader"]
pub struct R(crate::R<SOSCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOSCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOSCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOSCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOSCCR` writer"]
pub struct W(crate::W<SOSCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOSCCR_SPEC>;
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
impl From<crate::W<SOSCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOSCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOSTP` reader - Sub Clock Oscillator Stop"]
pub type SOSTP_R = crate::BitReader<SOSTP_A>;
#[doc = "Sub Clock Oscillator Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSTP_A {
    #[doc = "0: Operate the sub-clock oscillator"]
    _0 = 0,
    #[doc = "1: Stop the sub-clock oscillator"]
    _1 = 1,
}
impl From<SOSTP_A> for bool {
    #[inline(always)]
    fn from(variant: SOSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl SOSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSTP_A {
        match self.bits {
            false => SOSTP_A::_0,
            true => SOSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSTP_A::_1
    }
}
#[doc = "Field `SOSTP` writer - Sub Clock Oscillator Stop"]
pub type SOSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOSCCR_SPEC, SOSTP_A, O>;
impl<'a, const O: u8> SOSTP_W<'a, O> {
    #[doc = "Operate the sub-clock oscillator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSTP_A::_0)
    }
    #[doc = "Stop the sub-clock oscillator"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Sub Clock Oscillator Stop"]
    #[inline(always)]
    pub fn sostp(&self) -> SOSTP_R {
        SOSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub Clock Oscillator Stop"]
    #[inline(always)]
    pub fn sostp(&mut self) -> SOSTP_W<0> {
        SOSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Clock Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sosccr](index.html) module"]
pub struct SOSCCR_SPEC;
impl crate::RegisterSpec for SOSCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sosccr::R](R) reader structure"]
impl crate::Readable for SOSCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sosccr::W](W) writer structure"]
impl crate::Writable for SOSCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOSCCR to value 0x01"]
impl crate::Resettable for SOSCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
