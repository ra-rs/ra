#[doc = "Register `ADSSTR6` reader"]
pub struct R(crate::R<ADSSTR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR6` writer"]
pub struct W(crate::W<ADSSTR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR6_SPEC>;
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
impl From<crate::W<ADSSTR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST12` reader - Sampling State Table 12"]
pub type SST12_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST12` writer - Sampling State Table 12"]
pub type SST12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR6_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST13` reader - Sampling State Table 13"]
pub type SST13_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST13` writer - Sampling State Table 13"]
pub type SST13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR6_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 12"]
    #[inline(always)]
    pub fn sst12(&self) -> SST12_R {
        SST12_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 13"]
    #[inline(always)]
    pub fn sst13(&self) -> SST13_R {
        SST13_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 12"]
    #[inline(always)]
    #[must_use]
    pub fn sst12(&mut self) -> SST12_W<0> {
        SST12_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 13"]
    #[inline(always)]
    #[must_use]
    pub fn sst13(&mut self) -> SST13_W<16> {
        SST13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr6](index.html) module"]
pub struct ADSSTR6_SPEC;
impl crate::RegisterSpec for ADSSTR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr6::R](R) reader structure"]
impl crate::Readable for ADSSTR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr6::W](W) writer structure"]
impl crate::Writable for ADSSTR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR6 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
