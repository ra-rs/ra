#[doc = "Register `ADSSTR5` reader"]
pub struct R(crate::R<ADSSTR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR5` writer"]
pub struct W(crate::W<ADSSTR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR5_SPEC>;
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
impl From<crate::W<ADSSTR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST10` reader - Sampling State Table 10"]
pub type SST10_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST10` writer - Sampling State Table 10"]
pub type SST10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR5_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST11` reader - Sampling State Table 11"]
pub type SST11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST11` writer - Sampling State Table 11"]
pub type SST11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR5_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 10"]
    #[inline(always)]
    pub fn sst10(&self) -> SST10_R {
        SST10_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 11"]
    #[inline(always)]
    pub fn sst11(&self) -> SST11_R {
        SST11_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 10"]
    #[inline(always)]
    #[must_use]
    pub fn sst10(&mut self) -> SST10_W<0> {
        SST10_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 11"]
    #[inline(always)]
    #[must_use]
    pub fn sst11(&mut self) -> SST11_W<16> {
        SST11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr5](index.html) module"]
pub struct ADSSTR5_SPEC;
impl crate::RegisterSpec for ADSSTR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr5::R](R) reader structure"]
impl crate::Readable for ADSSTR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr5::W](W) writer structure"]
impl crate::Writable for ADSSTR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR5 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
