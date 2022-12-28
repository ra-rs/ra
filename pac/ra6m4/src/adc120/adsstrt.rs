#[doc = "Register `ADSSTRT` reader"]
pub struct R(crate::R<ADSSTRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTRT` writer"]
pub struct W(crate::W<ADSSTRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTRT_SPEC>;
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
impl From<crate::W<ADSSTRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST` reader - Sampling Time Setting"]
pub type SST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SST` writer - Sampling Time Setting"]
pub type SST_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADSSTRT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sampling Time Setting"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sampling Time Setting"]
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<0> {
        SST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Sampling State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstrt](index.html) module"]
pub struct ADSSTRT_SPEC;
impl crate::RegisterSpec for ADSSTRT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adsstrt::R](R) reader structure"]
impl crate::Readable for ADSSTRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstrt::W](W) writer structure"]
impl crate::Writable for ADSSTRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTRT to value 0x0b"]
impl crate::Resettable for ADSSTRT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
