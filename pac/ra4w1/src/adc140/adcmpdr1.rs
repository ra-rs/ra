#[doc = "Register `ADCMPDR1` reader"]
pub struct R(crate::R<ADCMPDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPDR1` writer"]
pub struct W(crate::W<ADCMPDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPDR1_SPEC>;
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
impl From<crate::W<ADCMPDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCMPDR1` reader - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
pub type ADCMPDR1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADCMPDR1` writer - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
pub type ADCMPDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADCMPDR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
    #[inline(always)]
    pub fn adcmpdr1(&self) -> ADCMPDR1_R {
        ADCMPDR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
    #[inline(always)]
    #[must_use]
    pub fn adcmpdr1(&mut self) -> ADCMPDR1_W<0> {
        ADCMPDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Upper-Side Level Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpdr1](index.html) module"]
pub struct ADCMPDR1_SPEC;
impl crate::RegisterSpec for ADCMPDR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpdr1::R](R) reader structure"]
impl crate::Readable for ADCMPDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpdr1::W](W) writer structure"]
impl crate::Writable for ADCMPDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPDR1 to value 0"]
impl crate::Resettable for ADCMPDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
