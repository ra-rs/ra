#[doc = "Register `MTCIDL` reader"]
pub struct R(crate::R<MTCIDL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTCIDL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTCIDL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTCIDL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTCIDL` writer"]
pub struct W(crate::W<MTCIDL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTCIDL_SPEC>;
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
impl From<crate::W<MTCIDL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTCIDL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTCIDL` reader - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock."]
pub type MTCIDL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MTCIDL` writer - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock."]
pub type MTCIDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTCIDL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock."]
    #[inline(always)]
    pub fn mtcidl(&self) -> MTCIDL_R {
        MTCIDL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock."]
    #[inline(always)]
    #[must_use]
    pub fn mtcidl(&mut self) -> MTCIDL_W<0> {
        MTCIDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock ID Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtcidl](index.html) module"]
pub struct MTCIDL_SPEC;
impl crate::RegisterSpec for MTCIDL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtcidl::R](R) reader structure"]
impl crate::Readable for MTCIDL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtcidl::W](W) writer structure"]
impl crate::Writable for MTCIDL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTCIDL to value 0"]
impl crate::Resettable for MTCIDL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
