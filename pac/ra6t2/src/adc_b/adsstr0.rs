#[doc = "Register `ADSSTR0` reader"]
pub struct R(crate::R<ADSSTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR0` writer"]
pub struct W(crate::W<ADSSTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR0_SPEC>;
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
impl From<crate::W<ADSSTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST0` reader - Sampling State Table 0"]
pub type SST0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST0` writer - Sampling State Table 0"]
pub type SST0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR0_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST1` reader - Sampling State Table 1"]
pub type SST1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST1` writer - Sampling State Table 1"]
pub type SST1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR0_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 0"]
    #[inline(always)]
    pub fn sst0(&self) -> SST0_R {
        SST0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 1"]
    #[inline(always)]
    pub fn sst1(&self) -> SST1_R {
        SST1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 0"]
    #[inline(always)]
    #[must_use]
    pub fn sst0(&mut self) -> SST0_W<0> {
        SST0_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 1"]
    #[inline(always)]
    #[must_use]
    pub fn sst1(&mut self) -> SST1_W<16> {
        SST1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr0](index.html) module"]
pub struct ADSSTR0_SPEC;
impl crate::RegisterSpec for ADSSTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr0::R](R) reader structure"]
impl crate::Readable for ADSSTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr0::W](W) writer structure"]
impl crate::Writable for ADSSTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR0 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
