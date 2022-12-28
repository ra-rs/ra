#[doc = "Register `AMP1OTN` reader"]
pub struct R(crate::R<AMP1OTN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP1OTN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP1OTN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP1OTN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP1OTN` writer"]
pub struct W(crate::W<AMP1OTN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP1OTN_SPEC>;
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
impl From<crate::W<AMP1OTN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP1OTN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRMN` reader - AMP1 input offset trimming Nch side"]
pub type TRMN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRMN` writer - AMP1 input offset trimming Nch side"]
pub type TRMN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AMP1OTN_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - AMP1 input offset trimming Nch side"]
    #[inline(always)]
    pub fn trmn(&self) -> TRMN_R {
        TRMN_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - AMP1 input offset trimming Nch side"]
    #[inline(always)]
    #[must_use]
    pub fn trmn(&mut self) -> TRMN_W<0> {
        TRMN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier 1 Offset Trimming Nch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp1otn](index.html) module"]
pub struct AMP1OTN_SPEC;
impl crate::RegisterSpec for AMP1OTN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp1otn::R](R) reader structure"]
impl crate::Readable for AMP1OTN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp1otn::W](W) writer structure"]
impl crate::Writable for AMP1OTN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP1OTN to value 0"]
impl crate::Resettable for AMP1OTN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
