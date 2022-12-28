#[doc = "Register `ADSSTRL` reader"]
pub struct R(crate::R<ADSSTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTRL` writer"]
pub struct W(crate::W<ADSSTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTRL_SPEC>;
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
impl From<crate::W<ADSSTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST` reader - Sampling Time Setting (AN016-AN018 and AN020)"]
pub type SST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SST` writer - Sampling Time Setting (AN016-AN018 and AN020)"]
pub type SST_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADSSTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sampling Time Setting (AN016-AN018 and AN020)"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sampling Time Setting (AN016-AN018 and AN020)"]
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
#[doc = "A/D Sampling State Register L\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstrl](index.html) module"]
pub struct ADSSTRL_SPEC;
impl crate::RegisterSpec for ADSSTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adsstrl::R](R) reader structure"]
impl crate::Readable for ADSSTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstrl::W](W) writer structure"]
impl crate::Writable for ADSSTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTRL to value 0x0b"]
impl crate::Resettable for ADSSTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
