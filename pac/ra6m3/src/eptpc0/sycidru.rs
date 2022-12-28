#[doc = "Register `SYCIDRU` reader"]
pub struct R(crate::R<SYCIDRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYCIDRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYCIDRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYCIDRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYCIDRU` writer"]
pub struct W(crate::W<SYCIDRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYCIDRU_SPEC>;
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
impl From<crate::W<SYCIDRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYCIDRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYCIDRU` reader - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port."]
pub type SYCIDRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYCIDRU` writer - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port."]
pub type SYCIDRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYCIDRU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port."]
    #[inline(always)]
    pub fn sycidru(&self) -> SYCIDRU_R {
        SYCIDRU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port."]
    #[inline(always)]
    #[must_use]
    pub fn sycidru(&mut self) -> SYCIDRU_W<0> {
        SYCIDRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Local Clock ID Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sycidru](index.html) module"]
pub struct SYCIDRU_SPEC;
impl crate::RegisterSpec for SYCIDRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sycidru::R](R) reader structure"]
impl crate::Readable for SYCIDRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sycidru::W](W) writer structure"]
impl crate::Writable for SYCIDRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYCIDRU to value 0"]
impl crate::Resettable for SYCIDRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
