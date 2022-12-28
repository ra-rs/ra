#[doc = "Register `INIE` reader"]
pub struct R(crate::R<INIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIE` writer"]
pub struct W(crate::W<INIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIE_SPEC>;
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
impl From<crate::W<INIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEIE` reader - Internal Error Interrupt Enable"]
pub type INEIE_R = crate::BitReader<INEIE_A>;
#[doc = "Internal Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INEIE_A {
    #[doc = "0: Disables Non-recoverable Internal Error Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Non-recoverable Internal Error Interrupt Signal."]
    _1 = 1,
}
impl From<INEIE_A> for bool {
    #[inline(always)]
    fn from(variant: INEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl INEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEIE_A {
        match self.bits {
            false => INEIE_A::_0,
            true => INEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INEIE_A::_1
    }
}
#[doc = "Field `INEIE` writer - Internal Error Interrupt Enable"]
pub type INEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INIE_SPEC, INEIE_A, O>;
impl<'a, const O: u8> INEIE_W<'a, O> {
    #[doc = "Disables Non-recoverable Internal Error Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INEIE_A::_0)
    }
    #[doc = "Enables Non-recoverable Internal Error Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INEIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 10 - Internal Error Interrupt Enable"]
    #[inline(always)]
    pub fn ineie(&self) -> INEIE_R {
        INEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Internal Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ineie(&mut self) -> INEIE_W<10> {
        INEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inie](index.html) module"]
pub struct INIE_SPEC;
impl crate::RegisterSpec for INIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inie::R](R) reader structure"]
impl crate::Readable for INIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inie::W](W) writer structure"]
impl crate::Writable for INIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INIE to value 0"]
impl crate::Resettable for INIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
