#[doc = "Register `FRDYIE` reader"]
pub struct R(crate::R<FRDYIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRDYIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRDYIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRDYIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRDYIE` writer"]
pub struct W(crate::W<FRDYIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRDYIE_SPEC>;
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
impl From<crate::W<FRDYIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRDYIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDYIE` reader - Flash Ready Interrupt Enable"]
pub type FRDYIE_R = crate::BitReader<FRDYIE_A>;
#[doc = "Flash Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDYIE_A {
    #[doc = "0: Generation of an FRDY interrupt request is disabled"]
    _0 = 0,
    #[doc = "1: Generation of an FRDY interrupt request is enabled."]
    _1 = 1,
}
impl From<FRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: FRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDYIE_A {
        match self.bits {
            false => FRDYIE_A::_0,
            true => FRDYIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDYIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDYIE_A::_1
    }
}
#[doc = "Field `FRDYIE` writer - Flash Ready Interrupt Enable"]
pub type FRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FRDYIE_SPEC, FRDYIE_A, O>;
impl<'a, const O: u8> FRDYIE_W<'a, O> {
    #[doc = "Generation of an FRDY interrupt request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDYIE_A::_0)
    }
    #[doc = "Generation of an FRDY interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDYIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdyie(&self) -> FRDYIE_R {
        FRDYIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frdyie(&mut self) -> FRDYIE_W<0> {
        FRDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Ready Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frdyie](index.html) module"]
pub struct FRDYIE_SPEC;
impl crate::RegisterSpec for FRDYIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [frdyie::R](R) reader structure"]
impl crate::Readable for FRDYIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frdyie::W](W) writer structure"]
impl crate::Writable for FRDYIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRDYIE to value 0"]
impl crate::Resettable for FRDYIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
