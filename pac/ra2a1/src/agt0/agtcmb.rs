#[doc = "Register `AGTCMB` reader"]
pub struct R(crate::R<AGTCMB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTCMB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTCMB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTCMB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTCMB` writer"]
pub struct W(crate::W<AGTCMB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTCMB_SPEC>;
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
impl From<crate::W<AGTCMB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTCMB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGTCMB` reader - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
pub type AGTCMB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AGTCMB` writer - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
pub type AGTCMB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AGTCMB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcmb(&self) -> AGTCMB_R {
        AGTCMB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[inline(always)]
    #[must_use]
    pub fn agtcmb(&mut self) -> AGTCMB_W<0> {
        AGTCMB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Compare Match B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtcmb](index.html) module"]
pub struct AGTCMB_SPEC;
impl crate::RegisterSpec for AGTCMB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [agtcmb::R](R) reader structure"]
impl crate::Readable for AGTCMB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtcmb::W](W) writer structure"]
impl crate::Writable for AGTCMB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTCMB to value 0xffff"]
impl crate::Resettable for AGTCMB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
