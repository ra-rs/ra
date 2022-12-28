#[doc = "Register `GMIDRU` reader"]
pub struct R(crate::R<GMIDRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMIDRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMIDRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMIDRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMIDRU` writer"]
pub struct W(crate::W<GMIDRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMIDRU_SPEC>;
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
impl From<crate::W<GMIDRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMIDRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GMIDRU` reader - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
pub type GMIDRU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GMIDRU` writer - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
pub type GMIDRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMIDRU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
    #[inline(always)]
    pub fn gmidru(&self) -> GMIDRU_R {
        GMIDRU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn gmidru(&mut self) -> GMIDRU_W<0> {
        GMIDRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "grandmasterIdentity Field Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmidru](index.html) module"]
pub struct GMIDRU_SPEC;
impl crate::RegisterSpec for GMIDRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmidru::R](R) reader structure"]
impl crate::Readable for GMIDRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmidru::W](W) writer structure"]
impl crate::Writable for GMIDRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GMIDRU to value 0"]
impl crate::Resettable for GMIDRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
