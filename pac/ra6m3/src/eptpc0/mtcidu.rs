#[doc = "Register `MTCIDU` reader"]
pub struct R(crate::R<MTCIDU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTCIDU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTCIDU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTCIDU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTCIDU` writer"]
pub struct W(crate::W<MTCIDU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTCIDU_SPEC>;
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
impl From<crate::W<MTCIDU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTCIDU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTCIDU` reader - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock."]
pub type MTCIDU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MTCIDU` writer - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock."]
pub type MTCIDU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTCIDU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock."]
    #[inline(always)]
    pub fn mtcidu(&self) -> MTCIDU_R {
        MTCIDU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock."]
    #[inline(always)]
    #[must_use]
    pub fn mtcidu(&mut self) -> MTCIDU_W<0> {
        MTCIDU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock ID Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtcidu](index.html) module"]
pub struct MTCIDU_SPEC;
impl crate::RegisterSpec for MTCIDU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtcidu::R](R) reader structure"]
impl crate::Readable for MTCIDU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtcidu::W](W) writer structure"]
impl crate::Writable for MTCIDU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTCIDU to value 0"]
impl crate::Resettable for MTCIDU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
