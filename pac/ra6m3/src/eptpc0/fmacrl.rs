#[doc = "Register `FMAC%sRL` reader"]
pub struct R(crate::R<FMACRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMACRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMACRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMACRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMAC%sRL` writer"]
pub struct W(crate::W<FMACRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMACRL_SPEC>;
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
impl From<crate::W<FMACRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMACRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMACRL` reader - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames."]
pub type FMACRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FMACRL` writer - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames."]
pub type FMACRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMACRL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames."]
    #[inline(always)]
    pub fn fmacrl(&self) -> FMACRL_R {
        FMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames."]
    #[inline(always)]
    #[must_use]
    pub fn fmacrl(&mut self) -> FMACRL_W<0> {
        FMACRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Reception Filter MAC Address %s Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmacrl](index.html) module"]
pub struct FMACRL_SPEC;
impl crate::RegisterSpec for FMACRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmacrl::R](R) reader structure"]
impl crate::Readable for FMACRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmacrl::W](W) writer structure"]
impl crate::Writable for FMACRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMAC%sRL to value 0"]
impl crate::Resettable for FMACRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
