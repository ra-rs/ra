#[doc = "Register `GR%s_BASE` reader"]
pub struct R(crate::R<GR_BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_BASE` writer"]
pub struct W(crate::W<GR_BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_BASE_SPEC>;
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
impl From<crate::W<GR_BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_BASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R` reader - Background color R valueUnsigned; 8 bits"]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R` writer - Background color R valueUnsigned; 8 bits"]
pub type R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_BASE_SPEC, u8, u8, 8, O>;
#[doc = "Field `B` reader - Background color B valueUnsigned; 8 bits"]
pub type B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B` writer - Background color B valueUnsigned; 8 bits"]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_BASE_SPEC, u8, u8, 8, O>;
#[doc = "Field `G` reader - Background color G valueUnsigned; 8 bits"]
pub type G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `G` writer - Background color G valueUnsigned; 8 bits"]
pub type G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_BASE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Background color R valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background color B valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background color G valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background color R valueUnsigned; 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<0> {
        R_W::new(self)
    }
    #[doc = "Bits 8:15 - Background color B valueUnsigned; 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<8> {
        B_W::new(self)
    }
    #[doc = "Bits 16:23 - Background color G valueUnsigned; 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn g(&mut self) -> G_W<16> {
        G_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Background Color Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_base](index.html) module"]
pub struct GR_BASE_SPEC;
impl crate::RegisterSpec for GR_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_base::R](R) reader structure"]
impl crate::Readable for GR_BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_base::W](W) writer structure"]
impl crate::Writable for GR_BASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_BASE to value 0"]
impl crate::Resettable for GR_BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
