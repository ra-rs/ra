#[doc = "Register `SYIPADDRR` reader"]
pub struct R(crate::R<SYIPADDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYIPADDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYIPADDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYIPADDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYIPADDRR` writer"]
pub struct W(crate::W<SYIPADDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYIPADDRR_SPEC>;
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
impl From<crate::W<SYIPADDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYIPADDRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYIPADDRR` reader - These bits hold the setting for the local IP address."]
pub type SYIPADDRR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYIPADDRR` writer - These bits hold the setting for the local IP address."]
pub type SYIPADDRR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYIPADDRR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the local IP address."]
    #[inline(always)]
    pub fn syipaddrr(&self) -> SYIPADDRR_R {
        SYIPADDRR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the local IP address."]
    #[inline(always)]
    #[must_use]
    pub fn syipaddrr(&mut self) -> SYIPADDRR_W<0> {
        SYIPADDRR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Local IP Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syipaddrr](index.html) module"]
pub struct SYIPADDRR_SPEC;
impl crate::RegisterSpec for SYIPADDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syipaddrr::R](R) reader structure"]
impl crate::Readable for SYIPADDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syipaddrr::W](W) writer structure"]
impl crate::Writable for SYIPADDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYIPADDRR to value 0"]
impl crate::Resettable for SYIPADDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
