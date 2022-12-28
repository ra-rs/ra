#[doc = "Register `ADCMPLER` reader"]
pub struct R(crate::R<ADCMPLER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPLER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPLER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPLER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPLER` writer"]
pub struct W(crate::W<ADCMPLER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPLER_SPEC>;
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
impl From<crate::W<ADCMPLER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPLER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPLTSA` reader - Compare Window A Temperature Sensor Output Comparison Condition Select"]
pub type CMPLTSA_R = crate::BitReader<CMPLTSA_A>;
#[doc = "Compare Window A Temperature Sensor Output Comparison Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLTSA_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted valueCompare Window A Temperature Sensor Output Comparison Condition Select When window function is enabled (ADCMPCR.WCMPE = 1) : Compare Window A Temperature Sensor Output Comparison ConditionA/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1) : ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLTSA_A {
        match self.bits {
            false => CMPLTSA_A::_0,
            true => CMPLTSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLTSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLTSA_A::_1
    }
}
#[doc = "Field `CMPLTSA` writer - Compare Window A Temperature Sensor Output Comparison Condition Select"]
pub type CMPLTSA_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCMPLER_SPEC, CMPLTSA_A, O>;
impl<'a, const O: u8> CMPLTSA_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted valueCompare Window A Temperature Sensor Output Comparison Condition Select When window function is enabled (ADCMPCR.WCMPE = 1) : Compare Window A Temperature Sensor Output Comparison ConditionA/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLTSA_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1) : ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLTSA_A::_1)
    }
}
#[doc = "Field `CMPLOCA` reader - Compare Window A Internal Reference Voltage Comparison Condition Select"]
pub type CMPLOCA_R = crate::BitReader<CMPLOCA_A>;
#[doc = "Compare Window A Internal Reference Voltage Comparison Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLOCA_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLOCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLOCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLOCA_A {
        match self.bits {
            false => CMPLOCA_A::_0,
            true => CMPLOCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLOCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLOCA_A::_1
    }
}
#[doc = "Field `CMPLOCA` writer - Compare Window A Internal Reference Voltage Comparison Condition Select"]
pub type CMPLOCA_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCMPLER_SPEC, CMPLOCA_A, O>;
impl<'a, const O: u8> CMPLOCA_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLOCA_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLOCA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select"]
    #[inline(always)]
    pub fn cmpltsa(&self) -> CMPLTSA_R {
        CMPLTSA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Comparison Condition Select"]
    #[inline(always)]
    pub fn cmploca(&self) -> CMPLOCA_R {
        CMPLOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpltsa(&mut self) -> CMPLTSA_W<0> {
        CMPLTSA_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Comparison Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmploca(&mut self) -> CMPLOCA_W<1> {
        CMPLOCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpler](index.html) module"]
pub struct ADCMPLER_SPEC;
impl crate::RegisterSpec for ADCMPLER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcmpler::R](R) reader structure"]
impl crate::Readable for ADCMPLER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpler::W](W) writer structure"]
impl crate::Writable for ADCMPLER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPLER to value 0"]
impl crate::Resettable for ADCMPLER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
