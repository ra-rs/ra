#[doc = "Register `GMIDRL` reader"]
pub struct R(crate::R<GMIDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMIDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMIDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMIDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMIDRL` writer"]
pub struct W(crate::W<GMIDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMIDRL_SPEC>;
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
impl From<crate::W<GMIDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMIDRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GMIDRL` reader - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
pub type GMIDRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GMIDRL` writer - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
pub type GMIDRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMIDRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
    #[inline(always)]
    pub fn gmidrl(&self) -> GMIDRL_R {
        GMIDRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn gmidrl(&mut self) -> GMIDRL_W<0> {
        GMIDRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "grandmasterIdentity Field Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmidrl](index.html) module"]
pub struct GMIDRL_SPEC;
impl crate::RegisterSpec for GMIDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmidrl::R](R) reader structure"]
impl crate::Readable for GMIDRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmidrl::W](W) writer structure"]
impl crate::Writable for GMIDRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GMIDRL to value 0"]
impl crate::Resettable for GMIDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
