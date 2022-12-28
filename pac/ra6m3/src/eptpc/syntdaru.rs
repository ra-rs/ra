#[doc = "Register `SYNTDARU` reader"]
pub struct R(crate::R<SYNTDARU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNTDARU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNTDARU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNTDARU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNTDARU` writer"]
pub struct W(crate::W<SYNTDARU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNTDARU_SPEC>;
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
impl From<crate::W<SYNTDARU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNTDARU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNTDARU` reader - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization."]
pub type SYNTDARU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNTDARU` writer - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization."]
pub type SYNTDARU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNTDARU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization."]
    #[inline(always)]
    pub fn syntdaru(&self) -> SYNTDARU_R {
        SYNTDARU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn syntdaru(&mut self) -> SYNTDARU_W<0> {
        SYNTDARU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization Loss Detection Threshold Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syntdaru](index.html) module"]
pub struct SYNTDARU_SPEC;
impl crate::RegisterSpec for SYNTDARU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syntdaru::R](R) reader structure"]
impl crate::Readable for SYNTDARU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syntdaru::W](W) writer structure"]
impl crate::Writable for SYNTDARU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNTDARU to value 0"]
impl crate::Resettable for SYNTDARU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
