#[doc = "Register `ELCSARC` reader"]
pub struct R(crate::R<ELCSARC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELCSARC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELCSARC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELCSARC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELCSARC` writer"]
pub struct W(crate::W<ELCSARC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELCSARC_SPEC>;
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
impl From<crate::W<ELCSARC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELCSARC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELSR` reader - Event Link Setting Register n Security Attribution (n = 16, 17)"]
pub type ELSR_R = crate::FieldReader<u8, ELSR_A>;
#[doc = "Event Link Setting Register n Security Attribution (n = 16, 17)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ELSR_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSR_A> for u8 {
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
#[doc = "Field `ELSR` writer - Event Link Setting Register n Security Attribution (n = 16, 17)"]
pub type ELSR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ELCSARC_SPEC, u8, ELSR_A, 2, O>;
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
    #[doc = "Bits 0:1 - Event Link Setting Register n Security Attribution (n = 16, 17)"]
    #[inline(always)]
    pub fn elsr(&self) -> ELSR_R {
        ELSR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event Link Setting Register n Security Attribution (n = 16, 17)"]
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
#[doc = "Event Link Controller Security Attribution Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elcsarc](index.html) module"]
pub struct ELCSARC_SPEC;
impl crate::RegisterSpec for ELCSARC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [elcsarc::R](R) reader structure"]
impl crate::Readable for ELCSARC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elcsarc::W](W) writer structure"]
impl crate::Writable for ELCSARC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELCSARC to value 0xffff"]
impl crate::Resettable for ELCSARC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
