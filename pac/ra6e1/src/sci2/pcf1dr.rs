#[doc = "Register `PCF1DR` reader"]
pub struct R(crate::R<PCF1DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCF1DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCF1DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCF1DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCF1DR` writer"]
pub struct W(crate::W<PCF1DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCF1DR_SPEC>;
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
impl From<crate::W<PCF1DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCF1DR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Primary Control Field 1 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf1dr](index.html) module"]
pub struct PCF1DR_SPEC;
impl crate::RegisterSpec for PCF1DR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcf1dr::R](R) reader structure"]
impl crate::Readable for PCF1DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcf1dr::W](W) writer structure"]
impl crate::Writable for PCF1DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCF1DR to value 0"]
impl crate::Resettable for PCF1DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
