#[doc = "Register `ELCSARB` reader"]
pub struct R(crate::R<ELCSARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELCSARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELCSARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELCSARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELCSARB` writer"]
pub struct W(crate::W<ELCSARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELCSARB_SPEC>;
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
impl From<crate::W<ELCSARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELCSARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELSR` reader - Event Link Setting Register n Security Attribution"]
pub type ELSR_R = crate::FieldReader<u16, ELSR_A>;
#[doc = "Event Link Setting Register n Security Attribution\n\nValue on reset: 65535"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ELSR_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR_A> for u16 {
    #[inline(always)]
    fn from(variant: ELSR_A) -> Self {
        variant as _
    }
}
impl ELSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ELSR_A> {
        match self.bits {
            0 => Some(ELSR_A::_0),
            1 => Some(ELSR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSR_A::_1
    }
}
#[doc = "Field `ELSR` writer - Event Link Setting Register n Security Attribution"]
pub type ELSR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ELCSARB_SPEC, u16, ELSR_A, 16, O>;
impl<'a, const O: u8> ELSR_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSR_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr(&self) -> ELSR_R {
        ELSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsr(&mut self) -> ELSR_W<0> {
        ELSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Link Controller Security Attribution Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elcsarb](index.html) module"]
pub struct ELCSARB_SPEC;
impl crate::RegisterSpec for ELCSARB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [elcsarb::R](R) reader structure"]
impl crate::Readable for ELCSARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elcsarb::W](W) writer structure"]
impl crate::Writable for ELCSARB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELCSARB to value 0xffff"]
impl crate::Resettable for ELCSARB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
