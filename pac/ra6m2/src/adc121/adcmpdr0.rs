#[doc = "Register `ADCMPDR0` reader"]
pub struct R(crate::R<ADCMPDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPDR0` writer"]
pub struct W(crate::W<ADCMPDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPDR0_SPEC>;
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
impl From<crate::W<ADCMPDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCMPDR0` reader - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
pub type ADCMPDR0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADCMPDR0` writer - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
pub type ADCMPDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADCMPDR0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
    #[inline(always)]
    pub fn adcmpdr0(&self) -> ADCMPDR0_R {
        ADCMPDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
    #[inline(always)]
    #[must_use]
    pub fn adcmpdr0(&mut self) -> ADCMPDR0_W<0> {
        ADCMPDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Lower-Side Level Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpdr0](index.html) module"]
pub struct ADCMPDR0_SPEC;
impl crate::RegisterSpec for ADCMPDR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpdr0::R](R) reader structure"]
impl crate::Readable for ADCMPDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpdr0::W](W) writer structure"]
impl crate::Writable for ADCMPDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPDR0 to value 0"]
impl crate::Resettable for ADCMPDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
