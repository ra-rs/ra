#[doc = "Register `ADSSTR7` reader"]
pub struct R(crate::R<ADSSTR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR7` writer"]
pub struct W(crate::W<ADSSTR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR7_SPEC>;
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
impl From<crate::W<ADSSTR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST14` reader - Sampling State Table 14"]
pub type SST14_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST14` writer - Sampling State Table 14"]
pub type SST14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR7_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST15` reader - Sampling State Table 15"]
pub type SST15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST15` writer - Sampling State Table 15"]
pub type SST15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR7_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 14"]
    #[inline(always)]
    pub fn sst14(&self) -> SST14_R {
        SST14_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 15"]
    #[inline(always)]
    pub fn sst15(&self) -> SST15_R {
        SST15_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 14"]
    #[inline(always)]
    #[must_use]
    pub fn sst14(&mut self) -> SST14_W<0> {
        SST14_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 15"]
    #[inline(always)]
    #[must_use]
    pub fn sst15(&mut self) -> SST15_W<16> {
        SST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr7](index.html) module"]
pub struct ADSSTR7_SPEC;
impl crate::RegisterSpec for ADSSTR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr7::R](R) reader structure"]
impl crate::Readable for ADSSTR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr7::W](W) writer structure"]
impl crate::Writable for ADSSTR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR7 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR7_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
