#[doc = "Register `IPGR` reader"]
pub struct R(crate::R<IPGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPGR` writer"]
pub struct W(crate::W<IPGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPGR_SPEC>;
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
impl From<crate::W<IPGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPG` reader - "]
pub type IPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPG` writer - "]
pub type IPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPGR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IPG_W<0> {
        IPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interpacket Gap Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgr](index.html) module"]
pub struct IPGR_SPEC;
impl crate::RegisterSpec for IPGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipgr::R](R) reader structure"]
impl crate::Readable for IPGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipgr::W](W) writer structure"]
impl crate::Writable for IPGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPGR to value 0x14"]
impl crate::Resettable for IPGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
