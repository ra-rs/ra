#[doc = "Register `DPSIFR1` reader"]
pub struct R(crate::R<DPSIFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIFR1` writer"]
pub struct W(crate::W<DPSIFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIFR1_SPEC>;
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
impl From<crate::W<DPSIFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRQ8F` reader - IRQ8-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ8F_R = crate::BitReader<DIRQ8F_A>;
#[doc = "IRQ8-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ8F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ8F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ8F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ8F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ8F_A {
        match self.bits {
            false => DIRQ8F_A::_0,
            true => DIRQ8F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ8F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ8F_A::_1
    }
}
#[doc = "Field `DIRQ8F` writer - IRQ8-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ8F_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR1_SPEC, DIRQ8F_A, O>;
impl<'a, const O: u8> DIRQ8F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ8F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ8F_A::_1)
    }
}
#[doc = "Field `DIRQ9F` reader - IRQ9-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ9F_R = crate::BitReader<DIRQ9F_A>;
#[doc = "IRQ9-DS Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ9F_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DIRQ9F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ9F_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ9F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ9F_A {
        match self.bits {
            false => DIRQ9F_A::_0,
            true => DIRQ9F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ9F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ9F_A::_1
    }
}
#[doc = "Field `DIRQ9F` writer - IRQ9-DS Pin Deep Standby Cancel Flag"]
pub type DIRQ9F_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR1_SPEC, DIRQ9F_A, O>;
impl<'a, const O: u8> DIRQ9F_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ9F_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ9F_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ8-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq8f(&self) -> DIRQ8F_R {
        DIRQ8F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq9f(&self) -> DIRQ9F_R {
        DIRQ9F_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ8-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq8f(&mut self) -> DIRQ8F_W<0> {
        DIRQ8F_W::new(self)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq9f(&mut self) -> DIRQ9F_W<1> {
        DIRQ9F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Flag Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsifr1](index.html) module"]
pub struct DPSIFR1_SPEC;
impl crate::RegisterSpec for DPSIFR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsifr1::R](R) reader structure"]
impl crate::Readable for DPSIFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsifr1::W](W) writer structure"]
impl crate::Writable for DPSIFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIFR1 to value 0"]
impl crate::Resettable for DPSIFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
