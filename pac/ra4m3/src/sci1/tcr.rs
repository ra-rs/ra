#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCST` reader - Timer Count Start"]
pub type TCST_R = crate::BitReader<TCST_A>;
#[doc = "Timer Count Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCST_A {
    #[doc = "0: Stops the timer counting"]
    _0 = 0,
    #[doc = "1: Starts the timer counting"]
    _1 = 1,
}
impl From<TCST_A> for bool {
    #[inline(always)]
    fn from(variant: TCST_A) -> Self {
        variant as u8 != 0
    }
}
impl TCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCST_A {
        match self.bits {
            false => TCST_A::_0,
            true => TCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCST_A::_1
    }
}
#[doc = "Field `TCST` writer - Timer Count Start"]
pub type TCST_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCR_SPEC, TCST_A, O>;
impl<'a, const O: u8> TCST_W<'a, O> {
    #[doc = "Stops the timer counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCST_A::_0)
    }
    #[doc = "Starts the timer counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Count Start"]
    #[inline(always)]
    pub fn tcst(&self) -> TCST_R {
        TCST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Count Start"]
    #[inline(always)]
    #[must_use]
    pub fn tcst(&mut self) -> TCST_W<0> {
        TCST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
