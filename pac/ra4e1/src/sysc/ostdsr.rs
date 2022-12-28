#[doc = "Register `OSTDSR` reader"]
pub struct R(crate::R<OSTDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSTDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSTDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSTDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSTDSR` writer"]
pub struct W(crate::W<OSTDSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSTDSR_SPEC>;
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
impl From<crate::W<OSTDSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSTDSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTDF` reader - Oscillation Stop Detection Flag"]
pub type OSTDF_R = crate::BitReader<OSTDF_A>;
#[doc = "Oscillation Stop Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDF_A {
    #[doc = "0: Main clock oscillation stop not detected"]
    _0 = 0,
    #[doc = "1: Main clock oscillation stop detected"]
    _1 = 1,
}
impl From<OSTDF_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDF_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTDF_A {
        match self.bits {
            false => OSTDF_A::_0,
            true => OSTDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDF_A::_1
    }
}
#[doc = "Field `OSTDF` writer - Oscillation Stop Detection Flag"]
pub type OSTDF_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSTDSR_SPEC, OSTDF_A, O>;
impl<'a, const O: u8> OSTDF_W<'a, O> {
    #[doc = "Main clock oscillation stop not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSTDF_A::_0)
    }
    #[doc = "Main clock oscillation stop detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSTDF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostdf(&self) -> OSTDF_R {
        OSTDF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ostdf(&mut self) -> OSTDF_W<0> {
        OSTDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillation Stop Detection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostdsr](index.html) module"]
pub struct OSTDSR_SPEC;
impl crate::RegisterSpec for OSTDSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ostdsr::R](R) reader structure"]
impl crate::Readable for OSTDSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ostdsr::W](W) writer structure"]
impl crate::Writable for OSTDSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSTDSR to value 0"]
impl crate::Resettable for OSTDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
