#[doc = "Register `SYNTDARL` reader"]
pub struct R(crate::R<SYNTDARL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNTDARL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNTDARL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNTDARL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNTDARL` writer"]
pub struct W(crate::W<SYNTDARL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNTDARL_SPEC>;
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
impl From<crate::W<SYNTDARL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNTDARL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNTDARL` reader - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization."]
pub type SYNTDARL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNTDARL` writer - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization."]
pub type SYNTDARL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNTDARL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization."]
    #[inline(always)]
    pub fn syntdarl(&self) -> SYNTDARL_R {
        SYNTDARL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn syntdarl(&mut self) -> SYNTDARL_W<0> {
        SYNTDARL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization Loss Detection Threshold Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syntdarl](index.html) module"]
pub struct SYNTDARL_SPEC;
impl crate::RegisterSpec for SYNTDARL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syntdarl::R](R) reader structure"]
impl crate::Readable for SYNTDARL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syntdarl::W](W) writer structure"]
impl crate::Writable for SYNTDARL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNTDARL to value 0"]
impl crate::Resettable for SYNTDARL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
