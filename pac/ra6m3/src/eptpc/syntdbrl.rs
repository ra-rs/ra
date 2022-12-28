#[doc = "Register `SYNTDBRL` reader"]
pub struct R(crate::R<SYNTDBRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNTDBRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNTDBRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNTDBRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNTDBRL` writer"]
pub struct W(crate::W<SYNTDBRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNTDBRL_SPEC>;
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
impl From<crate::W<SYNTDBRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNTDBRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNTDBRL` reader - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization."]
pub type SYNTDBRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNTDBRL` writer - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization."]
pub type SYNTDBRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNTDBRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization."]
    #[inline(always)]
    pub fn syntdbrl(&self) -> SYNTDBRL_R {
        SYNTDBRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn syntdbrl(&mut self) -> SYNTDBRL_W<0> {
        SYNTDBRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization Detection Threshold Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syntdbrl](index.html) module"]
pub struct SYNTDBRL_SPEC;
impl crate::RegisterSpec for SYNTDBRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syntdbrl::R](R) reader structure"]
impl crate::Readable for SYNTDBRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syntdbrl::W](W) writer structure"]
impl crate::Writable for SYNTDBRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNTDBRL to value 0"]
impl crate::Resettable for SYNTDBRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
