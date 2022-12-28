#[doc = "Register `PDR` reader"]
pub struct R(crate::R<PDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDR` writer"]
pub struct W(crate::W<PDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDR_SPEC>;
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
impl From<crate::W<PDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDR` reader - Pmn Direction"]
pub type PDR_R = crate::FieldReader<u16, PDR_A>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PDR_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)."]
    _1 = 1,
}
impl From<PDR_A> for u16 {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        variant as _
    }
}
impl PDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDR_A> {
        match self.bits {
            0 => Some(PDR_A::_0),
            1 => Some(PDR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR_A::_1
    }
}
#[doc = "Field `PDR` writer - Pmn Direction"]
pub type PDR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDR_SPEC, u16, PDR_A, 16, O>;
impl<'a, const O: u8> PDR_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR_A::_0)
    }
    #[doc = "Output (functions as an output pin)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr(&mut self) -> PDR_W<0> {
        PDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data direction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr](index.html) module"]
pub struct PDR_SPEC;
impl crate::RegisterSpec for PDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdr::R](R) reader structure"]
impl crate::Readable for PDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdr::W](W) writer structure"]
impl crate::Writable for PDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDR to value 0"]
impl crate::Resettable for PDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
