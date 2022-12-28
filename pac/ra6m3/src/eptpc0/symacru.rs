#[doc = "Register `SYMACRU` reader"]
pub struct R(crate::R<SYMACRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYMACRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYMACRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYMACRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYMACRU` writer"]
pub struct W(crate::W<SYMACRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYMACRU_SPEC>;
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
impl From<crate::W<SYMACRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYMACRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYMACRU` reader - These bits hold the setting for the higher-order 24 bits of the local MAC address."]
pub type SYMACRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYMACRU` writer - These bits hold the setting for the higher-order 24 bits of the local MAC address."]
pub type SYMACRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYMACRU_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the local MAC address."]
    #[inline(always)]
    pub fn symacru(&self) -> SYMACRU_R {
        SYMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the local MAC address."]
    #[inline(always)]
    #[must_use]
    pub fn symacru(&mut self) -> SYMACRU_W<0> {
        SYMACRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP MAC Address Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [symacru](index.html) module"]
pub struct SYMACRU_SPEC;
impl crate::RegisterSpec for SYMACRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [symacru::R](R) reader structure"]
impl crate::Readable for SYMACRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [symacru::W](W) writer structure"]
impl crate::Writable for SYMACRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYMACRU to value 0"]
impl crate::Resettable for SYMACRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
