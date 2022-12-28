#[doc = "Register `GR1_CLUT1[%s]` reader"]
pub struct R(crate::R<GR1_CLUT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR1_CLUT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR1_CLUT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR1_CLUT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR1_CLUT1[%s]` writer"]
pub struct W(crate::W<GR1_CLUT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR1_CLUT1_SPEC>;
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
impl From<crate::W<GR1_CLUT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR1_CLUT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B` reader - B Value of Color Palette n Plane for Graphics m Plane"]
pub type B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B` writer - B Value of Color Palette n Plane for Graphics m Plane"]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR1_CLUT1_SPEC, u8, u8, 8, O>;
#[doc = "Field `G` reader - G Value of Color Palette n Plane for Graphics m Plane"]
pub type G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `G` writer - G Value of Color Palette n Plane for Graphics m Plane"]
pub type G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR1_CLUT1_SPEC, u8, u8, 8, O>;
#[doc = "Field `R` reader - R Value of Color Palette n Plane for Graphics m Plane"]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R` writer - R Value of Color Palette n Plane for Graphics m Plane"]
pub type R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR1_CLUT1_SPEC, u8, u8, 8, O>;
#[doc = "Field `A` reader - Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
pub type A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `A` writer - Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
pub type A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR1_CLUT1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<0> {
        B_W::new(self)
    }
    #[doc = "Bits 8:15 - G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    #[must_use]
    pub fn g(&mut self) -> G_W<8> {
        G_W::new(self)
    }
    #[doc = "Bits 16:23 - R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<16> {
        R_W::new(self)
    }
    #[doc = "Bits 24:31 - Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<24> {
        A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color Palette 1 Plane for Graphics 1 Plane\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr1_clut1](index.html) module"]
pub struct GR1_CLUT1_SPEC;
impl crate::RegisterSpec for GR1_CLUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr1_clut1::R](R) reader structure"]
impl crate::Readable for GR1_CLUT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr1_clut1::W](W) writer structure"]
impl crate::Writable for GR1_CLUT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR1_CLUT1[%s]
to value 0"]
impl crate::Resettable for GR1_CLUT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
