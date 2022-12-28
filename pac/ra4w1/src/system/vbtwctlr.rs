#[doc = "Register `VBTWCTLR` reader"]
pub struct R(crate::R<VBTWCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTWCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTWCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTWCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTWCTLR` writer"]
pub struct W(crate::W<VBTWCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTWCTLR_SPEC>;
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
impl From<crate::W<VBTWCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTWCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VWEN` reader - VBATT wakeup enable"]
pub type VWEN_R = crate::BitReader<VWEN_A>;
#[doc = "VBATT wakeup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VWEN_A {
    #[doc = "0: Disable Wakeup function"]
    _0 = 0,
    #[doc = "1: Enable Wakeup function"]
    _1 = 1,
}
impl From<VWEN_A> for bool {
    #[inline(always)]
    fn from(variant: VWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VWEN_A {
        match self.bits {
            false => VWEN_A::_0,
            true => VWEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VWEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VWEN_A::_1
    }
}
#[doc = "Field `VWEN` writer - VBATT wakeup enable"]
pub type VWEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTWCTLR_SPEC, VWEN_A, O>;
impl<'a, const O: u8> VWEN_W<'a, O> {
    #[doc = "Disable Wakeup function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VWEN_A::_0)
    }
    #[doc = "Enable Wakeup function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VWEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT wakeup enable"]
    #[inline(always)]
    pub fn vwen(&self) -> VWEN_R {
        VWEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn vwen(&mut self) -> VWEN_W<0> {
        VWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Wakeup function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtwctlr](index.html) module"]
pub struct VBTWCTLR_SPEC;
impl crate::RegisterSpec for VBTWCTLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtwctlr::R](R) reader structure"]
impl crate::Readable for VBTWCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtwctlr::W](W) writer structure"]
impl crate::Writable for VBTWCTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWCTLR to value 0"]
impl crate::Resettable for VBTWCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
