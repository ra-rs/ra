#[doc = "Register `GR%s_AB3` reader"]
pub struct R(crate::R<GR_AB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB3` writer"]
pub struct W(crate::W<GR_AB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB3_SPEC>;
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
impl From<crate::W<GR_AB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GRCHW` reader - Horizontal width of graphics image area."]
pub type GRCHW_R = crate::FieldReader<u16, GRCHW_A>;
#[doc = "Horizontal width of graphics image area.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GRCHW_A(u16);
impl From<GRCHW_A> for u16 {
    #[inline(always)]
    fn from(val: GRCHW_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GRCHW` writer - Horizontal width of graphics image area."]
pub type GRCHW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB3_SPEC, u16, GRCHW_A, 11, O>;
#[doc = "Field `GRCHS` reader - Horizontal start position of graphics image area."]
pub type GRCHS_R = crate::FieldReader<u16, GRCHS_A>;
#[doc = "Horizontal start position of graphics image area.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GRCHS_A(u16);
impl From<GRCHS_A> for u16 {
    #[inline(always)]
    fn from(val: GRCHS_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GRCHS` writer - Horizontal start position of graphics image area."]
pub type GRCHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB3_SPEC, u16, GRCHS_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Horizontal width of graphics image area."]
    #[inline(always)]
    pub fn grchw(&self) -> GRCHW_R {
        GRCHW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Horizontal start position of graphics image area."]
    #[inline(always)]
    pub fn grchs(&self) -> GRCHS_R {
        GRCHS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Horizontal width of graphics image area."]
    #[inline(always)]
    #[must_use]
    pub fn grchw(&mut self) -> GRCHW_W<0> {
        GRCHW_W::new(self)
    }
    #[doc = "Bits 16:26 - Horizontal start position of graphics image area."]
    #[inline(always)]
    #[must_use]
    pub fn grchs(&mut self) -> GRCHS_W<16> {
        GRCHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab3](index.html) module"]
pub struct GR_AB3_SPEC;
impl crate::RegisterSpec for GR_AB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab3::R](R) reader structure"]
impl crate::Readable for GR_AB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab3::W](W) writer structure"]
impl crate::Writable for GR_AB3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB3 to value 0x0005_0010"]
impl crate::Resettable for GR_AB3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_0010;
}
