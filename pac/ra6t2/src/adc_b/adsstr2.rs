#[doc = "Register `ADSSTR2` reader"]
pub struct R(crate::R<ADSSTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR2` writer"]
pub struct W(crate::W<ADSSTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR2_SPEC>;
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
impl From<crate::W<ADSSTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST4` reader - Sampling State Table 4"]
pub type SST4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST4` writer - Sampling State Table 4"]
pub type SST4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR2_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST5` reader - Sampling State Table 5"]
pub type SST5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST5` writer - Sampling State Table 5"]
pub type SST5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 4"]
    #[inline(always)]
    pub fn sst4(&self) -> SST4_R {
        SST4_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 5"]
    #[inline(always)]
    pub fn sst5(&self) -> SST5_R {
        SST5_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 4"]
    #[inline(always)]
    #[must_use]
    pub fn sst4(&mut self) -> SST4_W<0> {
        SST4_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 5"]
    #[inline(always)]
    #[must_use]
    pub fn sst5(&mut self) -> SST5_W<16> {
        SST5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr2](index.html) module"]
pub struct ADSSTR2_SPEC;
impl crate::RegisterSpec for ADSSTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr2::R](R) reader structure"]
impl crate::Readable for ADSSTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr2::W](W) writer structure"]
impl crate::Writable for ADSSTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR2 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
