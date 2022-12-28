#[doc = "Register `SYNTDBRU` reader"]
pub struct R(crate::R<SYNTDBRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNTDBRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNTDBRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNTDBRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNTDBRU` writer"]
pub struct W(crate::W<SYNTDBRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNTDBRU_SPEC>;
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
impl From<crate::W<SYNTDBRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNTDBRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNTDBRU` reader - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization."]
pub type SYNTDBRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNTDBRU` writer - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization."]
pub type SYNTDBRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNTDBRU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization."]
    #[inline(always)]
    pub fn syntdbru(&self) -> SYNTDBRU_R {
        SYNTDBRU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn syntdbru(&mut self) -> SYNTDBRU_W<0> {
        SYNTDBRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization Detection Threshold Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syntdbru](index.html) module"]
pub struct SYNTDBRU_SPEC;
impl crate::RegisterSpec for SYNTDBRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syntdbru::R](R) reader structure"]
impl crate::Readable for SYNTDBRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syntdbru::W](W) writer structure"]
impl crate::Writable for SYNTDBRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNTDBRU to value 0"]
impl crate::Resettable for SYNTDBRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
