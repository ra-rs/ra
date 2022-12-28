#[doc = "Register `AMP1OTP` reader"]
pub struct R(crate::R<AMP1OTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMP1OTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMP1OTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMP1OTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMP1OTP` writer"]
pub struct W(crate::W<AMP1OTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMP1OTP_SPEC>;
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
impl From<crate::W<AMP1OTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMP1OTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRMP` reader - AMP1 input offset trimming Pch side"]
pub type TRMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRMP` writer - AMP1 input offset trimming Pch side"]
pub type TRMP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AMP1OTP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - AMP1 input offset trimming Pch side"]
    #[inline(always)]
    pub fn trmp(&self) -> TRMP_R {
        TRMP_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - AMP1 input offset trimming Pch side"]
    #[inline(always)]
    #[must_use]
    pub fn trmp(&mut self) -> TRMP_W<0> {
        TRMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier 1 Offset Trimming Pch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amp1otp](index.html) module"]
pub struct AMP1OTP_SPEC;
impl crate::RegisterSpec for AMP1OTP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amp1otp::R](R) reader structure"]
impl crate::Readable for AMP1OTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amp1otp::W](W) writer structure"]
impl crate::Writable for AMP1OTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMP1OTP to value 0"]
impl crate::Resettable for AMP1OTP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
