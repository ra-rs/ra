#[doc = "Register `ADWINULB` reader"]
pub struct R(crate::R<ADWINULB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADWINULB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADWINULB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADWINULB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADWINULB` writer"]
pub struct W(crate::W<ADWINULB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADWINULB_SPEC>;
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
impl From<crate::W<ADWINULB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADWINULB_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adwinulb](index.html) module"]
pub struct ADWINULB_SPEC;
impl crate::RegisterSpec for ADWINULB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adwinulb::R](R) reader structure"]
impl crate::Readable for ADWINULB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adwinulb::W](W) writer structure"]
impl crate::Writable for ADWINULB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADWINULB to value 0"]
impl crate::Resettable for ADWINULB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
