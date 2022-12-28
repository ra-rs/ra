#[doc = "Register `SYMACRL` reader"]
pub struct R(crate::R<SYMACRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYMACRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYMACRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYMACRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYMACRL` writer"]
pub struct W(crate::W<SYMACRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYMACRL_SPEC>;
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
impl From<crate::W<SYMACRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYMACRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYMACRL` reader - These bits hold the setting for the lower-order 24 bits of the local MAC address."]
pub type SYMACRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYMACRL` writer - These bits hold the setting for the lower-order 24 bits of the local MAC address."]
pub type SYMACRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYMACRL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the local MAC address."]
    #[inline(always)]
    pub fn symacrl(&self) -> SYMACRL_R {
        SYMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the local MAC address."]
    #[inline(always)]
    #[must_use]
    pub fn symacrl(&mut self) -> SYMACRL_W<0> {
        SYMACRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP MAC Address Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [symacrl](index.html) module"]
pub struct SYMACRL_SPEC;
impl crate::RegisterSpec for SYMACRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [symacrl::R](R) reader structure"]
impl crate::Readable for SYMACRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [symacrl::W](W) writer structure"]
impl crate::Writable for SYMACRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYMACRL to value 0"]
impl crate::Resettable for SYMACRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
