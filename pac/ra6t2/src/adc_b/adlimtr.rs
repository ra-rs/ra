#[doc = "Register `ADLIMTR%s` reader"]
pub struct R(crate::R<ADLIMTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADLIMTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADLIMTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADLIMTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADLIMTR%s` writer"]
pub struct W(crate::W<ADLIMTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADLIMTR_SPEC>;
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
impl From<crate::W<ADLIMTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADLIMTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIML` reader - Limiter clip table n : Lower-side limit value"]
pub type LIML_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LIML` writer - Limiter clip table n : Lower-side limit value"]
pub type LIML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADLIMTR_SPEC, u16, u16, 16, O>;
#[doc = "Field `LIMU` reader - Limiter clip table n : Upper-side limit value"]
pub type LIMU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LIMU` writer - Limiter clip table n : Upper-side limit value"]
pub type LIMU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADLIMTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Limiter clip table n : Lower-side limit value"]
    #[inline(always)]
    pub fn liml(&self) -> LIML_R {
        LIML_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Limiter clip table n : Upper-side limit value"]
    #[inline(always)]
    pub fn limu(&self) -> LIMU_R {
        LIMU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Limiter clip table n : Lower-side limit value"]
    #[inline(always)]
    #[must_use]
    pub fn liml(&mut self) -> LIML_W<0> {
        LIML_W::new(self)
    }
    #[doc = "Bits 16:31 - Limiter clip table n : Upper-side limit value"]
    #[inline(always)]
    #[must_use]
    pub fn limu(&mut self) -> LIMU_W<16> {
        LIMU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limiter Clip Table Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimtr](index.html) module"]
pub struct ADLIMTR_SPEC;
impl crate::RegisterSpec for ADLIMTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adlimtr::R](R) reader structure"]
impl crate::Readable for ADLIMTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adlimtr::W](W) writer structure"]
impl crate::Writable for ADLIMTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADLIMTR%s to value 0"]
impl crate::Resettable for ADLIMTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
