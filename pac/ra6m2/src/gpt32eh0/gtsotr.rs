#[doc = "Register `GTSOTR` reader"]
pub struct R(crate::R<GTSOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTSOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTSOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTSOTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTSOTR` writer"]
pub struct W(crate::W<GTSOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTSOTR_SPEC>;
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
impl From<crate::W<GTSOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTSOTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOTR` reader - Output Protection Function Temporary Release"]
pub type SOTR_R = crate::BitReader<SOTR_A>;
#[doc = "Output Protection Function Temporary Release\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOTR_A {
    #[doc = "0: Do not release protected state"]
    _0 = 0,
    #[doc = "1: Release protected state"]
    _1 = 1,
}
impl From<SOTR_A> for bool {
    #[inline(always)]
    fn from(variant: SOTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SOTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOTR_A {
        match self.bits {
            false => SOTR_A::_0,
            true => SOTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOTR_A::_1
    }
}
#[doc = "Field `SOTR` writer - Output Protection Function Temporary Release"]
pub type SOTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSOTR_SPEC, SOTR_A, O>;
impl<'a, const O: u8> SOTR_W<'a, O> {
    #[doc = "Do not release protected state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOTR_A::_0)
    }
    #[doc = "Release protected state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOTR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Output Protection Function Temporary Release"]
    #[inline(always)]
    pub fn sotr(&self) -> SOTR_R {
        SOTR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Protection Function Temporary Release"]
    #[inline(always)]
    #[must_use]
    pub fn sotr(&mut self) -> SOTR_W<0> {
        SOTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Output Protection Function Temporary Release Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtsotr](index.html) module"]
pub struct GTSOTR_SPEC;
impl crate::RegisterSpec for GTSOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtsotr::R](R) reader structure"]
impl crate::Readable for GTSOTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtsotr::W](W) writer structure"]
impl crate::Writable for GTSOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSOTR to value 0"]
impl crate::Resettable for GTSOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
