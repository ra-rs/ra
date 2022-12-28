#[doc = "Register `ADSSTR4` reader"]
pub struct R(crate::R<ADSSTR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR4` writer"]
pub struct W(crate::W<ADSSTR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR4_SPEC>;
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
impl From<crate::W<ADSSTR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST8` reader - Sampling State Table 8"]
pub type SST8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST8` writer - Sampling State Table 8"]
pub type SST8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR4_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST9` reader - Sampling State Table 9"]
pub type SST9_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST9` writer - Sampling State Table 9"]
pub type SST9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR4_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 8"]
    #[inline(always)]
    pub fn sst8(&self) -> SST8_R {
        SST8_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 9"]
    #[inline(always)]
    pub fn sst9(&self) -> SST9_R {
        SST9_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 8"]
    #[inline(always)]
    #[must_use]
    pub fn sst8(&mut self) -> SST8_W<0> {
        SST8_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 9"]
    #[inline(always)]
    #[must_use]
    pub fn sst9(&mut self) -> SST9_W<16> {
        SST9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr4](index.html) module"]
pub struct ADSSTR4_SPEC;
impl crate::RegisterSpec for ADSSTR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr4::R](R) reader structure"]
impl crate::Readable for ADSSTR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr4::W](W) writer structure"]
impl crate::Writable for ADSSTR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR4 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
