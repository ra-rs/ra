#[doc = "Register `VBTCR1` reader"]
pub struct R(crate::R<VBTCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTCR1` writer"]
pub struct W(crate::W<VBTCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTCR1_SPEC>;
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
impl From<crate::W<VBTCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPWSWSTP` reader - Battery Power supply Switch Stop"]
pub type BPWSWSTP_R = crate::BitReader<BPWSWSTP_A>;
#[doc = "Battery Power supply Switch Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPWSWSTP_A {
    #[doc = "0: Battery Power supply Switch Enable"]
    _0 = 0,
    #[doc = "1: Battery Power supply Switch stop"]
    _1 = 1,
}
impl From<BPWSWSTP_A> for bool {
    #[inline(always)]
    fn from(variant: BPWSWSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl BPWSWSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPWSWSTP_A {
        match self.bits {
            false => BPWSWSTP_A::_0,
            true => BPWSWSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPWSWSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPWSWSTP_A::_1
    }
}
#[doc = "Field `BPWSWSTP` writer - Battery Power supply Switch Stop"]
pub type BPWSWSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTCR1_SPEC, BPWSWSTP_A, O>;
impl<'a, const O: u8> BPWSWSTP_W<'a, O> {
    #[doc = "Battery Power supply Switch Enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPWSWSTP_A::_0)
    }
    #[doc = "Battery Power supply Switch stop"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPWSWSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Battery Power supply Switch Stop"]
    #[inline(always)]
    pub fn bpwswstp(&self) -> BPWSWSTP_R {
        BPWSWSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery Power supply Switch Stop"]
    #[inline(always)]
    #[must_use]
    pub fn bpwswstp(&mut self) -> BPWSWSTP_W<0> {
        BPWSWSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Control Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtcr1](index.html) module"]
pub struct VBTCR1_SPEC;
impl crate::RegisterSpec for VBTCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtcr1::R](R) reader structure"]
impl crate::Readable for VBTCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtcr1::W](W) writer structure"]
impl crate::Writable for VBTCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTCR1 to value 0"]
impl crate::Resettable for VBTCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
