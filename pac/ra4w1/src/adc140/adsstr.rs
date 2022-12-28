#[doc = "Register `ADSSTR%s` reader"]
pub struct R(crate::R<ADSSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSSTR%s` writer"]
pub struct W(crate::W<ADSSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSSTR_SPEC>;
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
impl From<crate::W<ADSSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST` reader - Sampling time setting"]
pub type SST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SST` writer - Sampling time setting"]
pub type SST_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADSSTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sampling time setting"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sampling time setting"]
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
#[doc = "A/D Sampling State Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsstr](index.html) module"]
pub struct ADSSTR_SPEC;
impl crate::RegisterSpec for ADSSTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adsstr::R](R) reader structure"]
impl crate::Readable for ADSSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsstr::W](W) writer structure"]
impl crate::Writable for ADSSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTR%s to value 0x0d"]
impl crate::Resettable for ADSSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
