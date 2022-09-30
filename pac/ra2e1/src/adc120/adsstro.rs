#[doc = "Register `ADSSTRO` reader"]
pub struct R(crate::R<ADSSTRO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTRO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTRO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTRO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTRO` writer"]
pub struct W(crate::W<ADSSTRO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTRO_SPEC>;
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
impl From<crate::W<ADSSTRO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTRO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST` reader - Sampling Time Setting"]
pub type SST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SST` writer - Sampling Time Setting"]
pub type SST_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADSSTRO_SPEC, u8, u8, 8, O>;
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
#[doc = "A/D Sampling State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstro](index.html) module"]
pub struct ADSSTRO_SPEC;
impl crate::RegisterSpec for ADSSTRO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adsstro::R](R) reader structure"]
impl crate::Readable for ADSSTRO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstro::W](W) writer structure"]
impl crate::Writable for ADSSTRO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADSSTRO to value 0x0d"]
impl crate::Resettable for ADSSTRO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
