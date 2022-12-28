#[doc = "Register `DPSIEGR1` reader"]
pub struct R(crate::R<DPSIEGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIEGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIEGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIEGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIEGR1` writer"]
pub struct W(crate::W<DPSIEGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIEGR1_SPEC>;
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
impl From<crate::W<DPSIEGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIEGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRQ8EG` reader - IRQ8-DS Pin Edge Select"]
pub type DIRQ8EG_R = crate::BitReader<DIRQ8EG_A>;
#[doc = "IRQ8-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ8EG_A {
    #[doc = "0: A cancel request is generated at a falling edge."]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge."]
    _1 = 1,
}
impl From<DIRQ8EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ8EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ8EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ8EG_A {
        match self.bits {
            false => DIRQ8EG_A::_0,
            true => DIRQ8EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ8EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ8EG_A::_1
    }
}
#[doc = "Field `DIRQ8EG` writer - IRQ8-DS Pin Edge Select"]
pub type DIRQ8EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR1_SPEC, DIRQ8EG_A, O>;
impl<'a, const O: u8> DIRQ8EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ8EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ8EG_A::_1)
    }
}
#[doc = "Field `DIRQ9EG` reader - IRQ9-DS Pin Edge Select"]
pub type DIRQ9EG_R = crate::BitReader<DIRQ9EG_A>;
#[doc = "IRQ9-DS Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ9EG_A {
    #[doc = "0: A cancel request is generated at a falling edge."]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge."]
    _1 = 1,
}
impl From<DIRQ9EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ9EG_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ9EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ9EG_A {
        match self.bits {
            false => DIRQ9EG_A::_0,
            true => DIRQ9EG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ9EG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ9EG_A::_1
    }
}
#[doc = "Field `DIRQ9EG` writer - IRQ9-DS Pin Edge Select"]
pub type DIRQ9EG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR1_SPEC, DIRQ9EG_A, O>;
impl<'a, const O: u8> DIRQ9EG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ9EG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ9EG_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ8-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq8eg(&self) -> DIRQ8EG_R {
        DIRQ8EG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq9eg(&self) -> DIRQ9EG_R {
        DIRQ9EG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ8-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq8eg(&mut self) -> DIRQ8EG_W<0> {
        DIRQ8EG_W::new(self)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dirq9eg(&mut self) -> DIRQ9EG_W<1> {
        DIRQ9EG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Edge Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsiegr1](index.html) module"]
pub struct DPSIEGR1_SPEC;
impl crate::RegisterSpec for DPSIEGR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsiegr1::R](R) reader structure"]
impl crate::Readable for DPSIEGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsiegr1::W](W) writer structure"]
impl crate::Writable for DPSIEGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIEGR1 to value 0"]
impl crate::Resettable for DPSIEGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
