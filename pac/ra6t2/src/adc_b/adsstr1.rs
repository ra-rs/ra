#[doc = "Register `ADSSTR1` reader"]
pub struct R(crate::R<ADSSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR1` writer"]
pub struct W(crate::W<ADSSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR1_SPEC>;
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
impl From<crate::W<ADSSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST2` reader - Sampling State Table 2"]
pub type SST2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST2` writer - Sampling State Table 2"]
pub type SST2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR1_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST3` reader - Sampling State Table 3"]
pub type SST3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST3` writer - Sampling State Table 3"]
pub type SST3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 2"]
    #[inline(always)]
    pub fn sst2(&self) -> SST2_R {
        SST2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 3"]
    #[inline(always)]
    pub fn sst3(&self) -> SST3_R {
        SST3_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 2"]
    #[inline(always)]
    #[must_use]
    pub fn sst2(&mut self) -> SST2_W<0> {
        SST2_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 3"]
    #[inline(always)]
    #[must_use]
    pub fn sst3(&mut self) -> SST3_W<16> {
        SST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr1](index.html) module"]
pub struct ADSSTR1_SPEC;
impl crate::RegisterSpec for ADSSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr1::R](R) reader structure"]
impl crate::Readable for ADSSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr1::W](W) writer structure"]
impl crate::Writable for ADSSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR1 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
