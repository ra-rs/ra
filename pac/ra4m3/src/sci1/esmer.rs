#[doc = "Register `ESMER` reader"]
pub struct R(crate::R<ESMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESMER` writer"]
pub struct W(crate::W<ESMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESMER_SPEC>;
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
impl From<crate::W<ESMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESME` reader - Extended Serial Mode Enable"]
pub type ESME_R = crate::BitReader<ESME_A>;
#[doc = "Extended Serial Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESME_A {
    #[doc = "0: The extended serial mode is disabled."]
    _0 = 0,
    #[doc = "1: The extended serial mode is enabled."]
    _1 = 1,
}
impl From<ESME_A> for bool {
    #[inline(always)]
    fn from(variant: ESME_A) -> Self {
        variant as u8 != 0
    }
}
impl ESME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESME_A {
        match self.bits {
            false => ESME_A::_0,
            true => ESME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESME_A::_1
    }
}
#[doc = "Field `ESME` writer - Extended Serial Mode Enable"]
pub type ESME_W<'a, const O: u8> = crate::BitWriter<'a, u8, ESMER_SPEC, ESME_A, O>;
impl<'a, const O: u8> ESME_W<'a, O> {
    #[doc = "The extended serial mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESME_A::_0)
    }
    #[doc = "The extended serial mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESME_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Extended Serial Mode Enable"]
    #[inline(always)]
    pub fn esme(&self) -> ESME_R {
        ESME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Extended Serial Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn esme(&mut self) -> ESME_W<0> {
        ESME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Serial Module Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esmer](index.html) module"]
pub struct ESMER_SPEC;
impl crate::RegisterSpec for ESMER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [esmer::R](R) reader structure"]
impl crate::Readable for ESMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esmer::W](W) writer structure"]
impl crate::Writable for ESMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESMER to value 0"]
impl crate::Resettable for ESMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
