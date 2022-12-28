#[doc = "Register `SYCIDRL` reader"]
pub struct R(crate::R<SYCIDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYCIDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYCIDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYCIDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYCIDRL` writer"]
pub struct W(crate::W<SYCIDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYCIDRL_SPEC>;
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
impl From<crate::W<SYCIDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYCIDRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYCIDRL` reader - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port."]
pub type SYCIDRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYCIDRL` writer - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port."]
pub type SYCIDRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYCIDRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port."]
    #[inline(always)]
    pub fn sycidrl(&self) -> SYCIDRL_R {
        SYCIDRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port."]
    #[inline(always)]
    #[must_use]
    pub fn sycidrl(&mut self) -> SYCIDRL_W<0> {
        SYCIDRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Local Clock ID Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sycidrl](index.html) module"]
pub struct SYCIDRL_SPEC;
impl crate::RegisterSpec for SYCIDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sycidrl::R](R) reader structure"]
impl crate::Readable for SYCIDRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sycidrl::W](W) writer structure"]
impl crate::Writable for SYCIDRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYCIDRL to value 0"]
impl crate::Resettable for SYCIDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
