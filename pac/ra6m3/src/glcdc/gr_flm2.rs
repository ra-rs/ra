#[doc = "Register `GR%s_FLM2` reader"]
pub struct R(crate::R<GR_FLM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_FLM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_FLM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_FLM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_FLM2` writer"]
pub struct W(crate::W<GR_FLM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_FLM2_SPEC>;
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
impl From<crate::W<GR_FLM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_FLM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE` reader - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\\[5:0\\]
should be fixed to 0 during 64-byte burst transfer."]
pub type BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASE` writer - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\\[5:0\\]
should be fixed to 0 during 64-byte burst transfer."]
pub type BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_FLM2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\\[5:0\\]
should be fixed to 0 during 64-byte burst transfer."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\\[5:0\\]
should be fixed to 0 during 64-byte burst transfer."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<0> {
        BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Frame Buffer Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_flm2](index.html) module"]
pub struct GR_FLM2_SPEC;
impl crate::RegisterSpec for GR_FLM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_flm2::R](R) reader structure"]
impl crate::Readable for GR_FLM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_flm2::W](W) writer structure"]
impl crate::Writable for GR_FLM2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_FLM2 to value 0"]
impl crate::Resettable for GR_FLM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
