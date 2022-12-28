#[doc = "Register `FLLCR1` reader"]
pub struct R(crate::R<FLLCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLLCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLLCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLLCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLLCR1` writer"]
pub struct W(crate::W<FLLCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLLCR1_SPEC>;
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
impl From<crate::W<FLLCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLLCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLLEN` reader - FLL Enable"]
pub type FLLEN_R = crate::BitReader<FLLEN_A>;
#[doc = "FLL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLLEN_A {
    #[doc = "0: FLL function is disabled"]
    _0 = 0,
    #[doc = "1: FLL function is enabled."]
    _1 = 1,
}
impl From<FLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLEN_A {
        match self.bits {
            false => FLLEN_A::_0,
            true => FLLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLLEN_A::_1
    }
}
#[doc = "Field `FLLEN` writer - FLL Enable"]
pub type FLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLLCR1_SPEC, FLLEN_A, O>;
impl<'a, const O: u8> FLLEN_W<'a, O> {
    #[doc = "FLL function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLLEN_A::_0)
    }
    #[doc = "FLL function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLLEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FLL Enable"]
    #[inline(always)]
    pub fn fllen(&self) -> FLLEN_R {
        FLLEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fllen(&mut self) -> FLLEN_W<0> {
        FLLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLL Control Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fllcr1](index.html) module"]
pub struct FLLCR1_SPEC;
impl crate::RegisterSpec for FLLCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fllcr1::R](R) reader structure"]
impl crate::Readable for FLLCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fllcr1::W](W) writer structure"]
impl crate::Writable for FLLCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLLCR1 to value 0"]
impl crate::Resettable for FLLCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
