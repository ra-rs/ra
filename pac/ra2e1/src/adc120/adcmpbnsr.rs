#[doc = "Register `ADCMPBNSR` reader"]
pub struct R(crate::R<ADCMPBNSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPBNSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPBNSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPBNSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPBNSR` writer"]
pub struct W(crate::W<ADCMPBNSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPBNSR_SPEC>;
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
impl From<crate::W<ADCMPBNSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPBNSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPCHB` reader - Compare Window B Channel Select"]
pub type CMPCHB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPCHB` writer - Compare Window B Channel Select"]
pub type CMPCHB_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADCMPBNSR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMPLB` reader - Compare Window B Comparison Condition Setting"]
pub type CMPLB_R = crate::BitReader<CMPLB_A>;
#[doc = "Compare Window B Comparison Condition Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPLB_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADWINLLB value, or ADWINULB value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADWINLLB value < A/D-converted value < ADWINULB value"]
    _1 = 1,
}
impl From<CMPLB_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLB_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLB_A {
        match self.bits {
            false => CMPLB_A::_0,
            true => CMPLB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLB_A::_1
    }
}
#[doc = "Field `CMPLB` writer - Compare Window B Comparison Condition Setting"]
pub type CMPLB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCMPBNSR_SPEC, CMPLB_A, O>;
impl<'a, const O: u8> CMPLB_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADWINLLB value, or ADWINULB value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLB_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADWINLLB value < A/D-converted value < ADWINULB value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - Compare Window B Channel Select"]
    #[inline(always)]
    pub fn cmpchb(&self) -> CMPCHB_R {
        CMPCHB_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Compare Window B Comparison Condition Setting"]
    #[inline(always)]
    pub fn cmplb(&self) -> CMPLB_R {
        CMPLB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Compare Window B Channel Select"]
    #[inline(always)]
    pub fn cmpchb(&mut self) -> CMPCHB_W<0> {
        CMPCHB_W::new(self)
    }
    #[doc = "Bit 7 - Compare Window B Comparison Condition Setting"]
    #[inline(always)]
    pub fn cmplb(&mut self) -> CMPLB_W<7> {
        CMPLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window B Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpbnsr](index.html) module"]
pub struct ADCMPBNSR_SPEC;
impl crate::RegisterSpec for ADCMPBNSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcmpbnsr::R](R) reader structure"]
impl crate::Readable for ADCMPBNSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpbnsr::W](W) writer structure"]
impl crate::Writable for ADCMPBNSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCMPBNSR to value 0"]
impl crate::Resettable for ADCMPBNSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
