#[doc = "Register `PCNTR1` reader"]
pub struct R(crate::R<PCNTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNTR1` writer"]
pub struct W(crate::W<PCNTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNTR1_SPEC>;
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
impl From<crate::W<PCNTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNTR1_SPEC>) -> Self {
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
pub type PDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNTR1_SPEC, u16, PDR_A, 16, O>;
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
#[doc = "Field `PODR` reader - Pmn Output Data"]
pub type PODR_R = crate::FieldReader<u16, PODR_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PODR_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<PODR_A> for u16 {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as _
    }
}
impl PODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PODR_A> {
        match self.bits {
            0 => Some(PODR_A::_0),
            1 => Some(PODR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
#[doc = "Field `PODR` writer - Pmn Output Data"]
pub type PODR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNTR1_SPEC, u16, PODR_A, 16, O>;
impl<'a, const O: u8> PODR_W<'a, O> {
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR_A::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr(&mut self) -> PDR_W<0> {
        PDR_W::new(self)
    }
    #[doc = "Bits 16:31 - Pmn Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn podr(&mut self) -> PODR_W<16> {
        PODR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntr1](index.html) module"]
pub struct PCNTR1_SPEC;
impl crate::RegisterSpec for PCNTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcntr1::R](R) reader structure"]
impl crate::Readable for PCNTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcntr1::W](W) writer structure"]
impl crate::Writable for PCNTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNTR1 to value 0"]
impl crate::Resettable for PCNTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
