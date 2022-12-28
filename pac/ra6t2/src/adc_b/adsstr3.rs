#[doc = "Register `ADSSTR3` reader"]
pub struct R(crate::R<ADSSTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR3` writer"]
pub struct W(crate::W<ADSSTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR3_SPEC>;
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
impl From<crate::W<ADSSTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST6` reader - Sampling State Table 6"]
pub type SST6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST6` writer - Sampling State Table 6"]
pub type SST6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR3_SPEC, u16, u16, 10, O>;
#[doc = "Field `SST7` reader - Sampling State Table 7"]
pub type SST7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SST7` writer - Sampling State Table 7"]
pub type SST7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSSTR3_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sampling State Table 6"]
    #[inline(always)]
    pub fn sst6(&self) -> SST6_R {
        SST6_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sampling State Table 7"]
    #[inline(always)]
    pub fn sst7(&self) -> SST7_R {
        SST7_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sampling State Table 6"]
    #[inline(always)]
    #[must_use]
    pub fn sst6(&mut self) -> SST6_W<0> {
        SST6_W::new(self)
    }
    #[doc = "Bits 16:25 - Sampling State Table 7"]
    #[inline(always)]
    #[must_use]
    pub fn sst7(&mut self) -> SST7_W<16> {
        SST7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling State Table Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr3](index.html) module"]
pub struct ADSSTR3_SPEC;
impl crate::RegisterSpec for ADSSTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsstr3::R](R) reader structure"]
impl crate::Readable for ADSSTR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr3::W](W) writer structure"]
impl crate::Writable for ADSSTR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR3 to value 0x0002_0002"]
impl crate::Resettable for ADSSTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
