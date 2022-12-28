#[doc = "Register `FMAC%sRU` reader"]
pub struct R(crate::R<FMACRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMACRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMACRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMACRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMAC%sRU` writer"]
pub struct W(crate::W<FMACRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMACRU_SPEC>;
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
impl From<crate::W<FMACRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMACRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMACRU` reader - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames."]
pub type FMACRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FMACRU` writer - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames."]
pub type FMACRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMACRU_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames."]
    #[inline(always)]
    pub fn fmacru(&self) -> FMACRU_R {
        FMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames."]
    #[inline(always)]
    #[must_use]
    pub fn fmacru(&mut self) -> FMACRU_W<0> {
        FMACRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Reception Filter MAC Address %s Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmacru](index.html) module"]
pub struct FMACRU_SPEC;
impl crate::RegisterSpec for FMACRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmacru::R](R) reader structure"]
impl crate::Readable for FMACRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmacru::W](W) writer structure"]
impl crate::Writable for FMACRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMAC%sRU to value 0"]
impl crate::Resettable for FMACRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
