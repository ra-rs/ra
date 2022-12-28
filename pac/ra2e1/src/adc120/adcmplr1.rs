#[doc = "Register `ADCMPLR1` reader"]
pub struct R(crate::R<ADCMPLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPLR1` writer"]
pub struct W(crate::W<ADCMPLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPLR1_SPEC>;
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
impl From<crate::W<ADCMPLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPLCHAn` reader - Compare Window A Comparison Condition Select"]
pub type CMPLCHAN_R = crate::FieldReader<u16, CMPLCHAN_A>;
#[doc = "Compare Window A Comparison Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CMPLCHAN_A {
    #[doc = "0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    _0 = 0,
    #[doc = "1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    _1 = 1,
}
impl From<CMPLCHAN_A> for u16 {
    #[inline(always)]
    fn from(variant: CMPLCHAN_A) -> Self {
        variant as _
    }
}
impl CMPLCHAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPLCHAN_A> {
        match self.bits {
            0 => Some(CMPLCHAN_A::_0),
            1 => Some(CMPLCHAN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHAN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHAN_A::_1
    }
}
#[doc = "Field `CMPLCHAn` writer - Compare Window A Comparison Condition Select"]
pub type CMPLCHAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, ADCMPLR1_SPEC, u16, CMPLCHAN_A, 16, O>;
impl<'a, const O: u8> CMPLCHAN_W<'a, O> {
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHAN_A::_0)
    }
    #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHAN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplchan(&self) -> CMPLCHAN_R {
        CMPLCHAN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Window A Comparison Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmplchan(&mut self) -> CMPLCHAN_W<0> {
        CMPLCHAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmplr1](index.html) module"]
pub struct ADCMPLR1_SPEC;
impl crate::RegisterSpec for ADCMPLR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmplr1::R](R) reader structure"]
impl crate::Readable for ADCMPLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmplr1::W](W) writer structure"]
impl crate::Writable for ADCMPLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPLR1 to value 0"]
impl crate::Resettable for ADCMPLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
