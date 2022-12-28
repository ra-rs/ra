#[doc = "Register `FLWT` reader"]
pub struct R(crate::R<FLWT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLWT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLWT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLWT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLWT` writer"]
pub struct W(crate::W<FLWT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLWT_SPEC>;
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
impl From<crate::W<FLWT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLWT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLWT` reader - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
pub type FLWT_R = crate::FieldReader<u8, FLWT_A>;
#[doc = "These bits represent the ratio of the CPU clock period to the Flash memory access time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLWT_A {
    #[doc = "0: zero wait"]
    _000 = 0,
}
impl From<FLWT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLWT_A) -> Self {
        variant as _
    }
}
impl FLWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLWT_A> {
        match self.bits {
            0 => Some(FLWT_A::_000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FLWT_A::_000
    }
}
#[doc = "Field `FLWT` writer - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
pub type FLWT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FLWT_SPEC, u8, FLWT_A, 3, O>;
impl<'a, const O: u8> FLWT_W<'a, O> {
    #[doc = "zero wait"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FLWT_A::_000)
    }
}
impl R {
    #[doc = "Bits 0:2 - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[inline(always)]
    pub fn flwt(&self) -> FLWT_R {
        FLWT_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[inline(always)]
    #[must_use]
    pub fn flwt(&mut self) -> FLWT_W<0> {
        FLWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Wait Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flwt](index.html) module"]
pub struct FLWT_SPEC;
impl crate::RegisterSpec for FLWT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flwt::R](R) reader structure"]
impl crate::Readable for FLWT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flwt::W](W) writer structure"]
impl crate::Writable for FLWT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLWT to value 0"]
impl crate::Resettable for FLWT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
