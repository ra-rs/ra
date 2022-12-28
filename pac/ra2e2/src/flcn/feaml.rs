#[doc = "Register `FEAML` reader"]
pub struct R(crate::R<FEAML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEAML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEAML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEAML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEAML` writer"]
pub struct W(crate::W<FEAML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEAML_SPEC>;
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
impl From<crate::W<FEAML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEAML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEAML` reader - Flash Error Address Monitor Register L"]
pub type FEAML_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FEAML` writer - Flash Error Address Monitor Register L"]
pub type FEAML_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FEAML_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Error Address Monitor Register L"]
    #[inline(always)]
    pub fn feaml(&self) -> FEAML_R {
        FEAML_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Error Address Monitor Register L"]
    #[inline(always)]
    #[must_use]
    pub fn feaml(&mut self) -> FEAML_W<0> {
        FEAML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Error Address Monitor Register L\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feaml](index.html) module"]
pub struct FEAML_SPEC;
impl crate::RegisterSpec for FEAML_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [feaml::R](R) reader structure"]
impl crate::Readable for FEAML_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [feaml::W](W) writer structure"]
impl crate::Writable for FEAML_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEAML to value 0"]
impl crate::Resettable for FEAML_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
