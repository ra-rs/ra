#[doc = "Register `PFBER` reader"]
pub struct R(crate::R<PFBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFBER` writer"]
pub struct W(crate::W<PFBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFBER_SPEC>;
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
impl From<crate::W<PFBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFBE` reader - Prefetch Buffer Enable bit"]
pub type PFBE_R = crate::BitReader<PFBE_A>;
#[doc = "Prefetch Buffer Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFBE_A {
    #[doc = "0: Prefetch buffer is disabled"]
    _0 = 0,
    #[doc = "1: Prefetch buffer is enabled"]
    _1 = 1,
}
impl From<PFBE_A> for bool {
    #[inline(always)]
    fn from(variant: PFBE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFBE_A {
        match self.bits {
            false => PFBE_A::_0,
            true => PFBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFBE_A::_1
    }
}
#[doc = "Field `PFBE` writer - Prefetch Buffer Enable bit"]
pub type PFBE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PFBER_SPEC, PFBE_A, O>;
impl<'a, const O: u8> PFBE_W<'a, O> {
    #[doc = "Prefetch buffer is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFBE_A::_0)
    }
    #[doc = "Prefetch buffer is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFBE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Prefetch Buffer Enable bit"]
    #[inline(always)]
    pub fn pfbe(&self) -> PFBE_R {
        PFBE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Prefetch Buffer Enable bit"]
    #[inline(always)]
    pub fn pfbe(&mut self) -> PFBE_W<0> {
        PFBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prefetch Buffer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfber](index.html) module"]
pub struct PFBER_SPEC;
impl crate::RegisterSpec for PFBER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pfber::R](R) reader structure"]
impl crate::Readable for PFBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfber::W](W) writer structure"]
impl crate::Writable for PFBER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFBER to value 0"]
impl crate::Resettable for PFBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
