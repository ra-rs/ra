#[doc = "Register `GR%s_AB2` reader"]
pub struct R(crate::R<GR_AB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB2` writer"]
pub struct W(crate::W<GR_AB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB2_SPEC>;
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
impl From<crate::W<GR_AB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GRCVW` reader - Vertical width of graphics image area."]
pub type GRCVW_R = crate::FieldReader<u16, GRCVW_A>;
#[doc = "Vertical width of graphics image area.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GRCVW_A(u16);
impl From<GRCVW_A> for u16 {
    #[inline(always)]
    fn from(val: GRCVW_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GRCVW` writer - Vertical width of graphics image area."]
pub type GRCVW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB2_SPEC, u16, GRCVW_A, 11, O>;
#[doc = "Field `GRCVS` reader - Vertical start position of graphics image area."]
pub type GRCVS_R = crate::FieldReader<u16, GRCVS_A>;
#[doc = "Vertical start position of graphics image area.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GRCVS_A(u16);
impl From<GRCVS_A> for u16 {
    #[inline(always)]
    fn from(val: GRCVS_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `GRCVS` writer - Vertical start position of graphics image area."]
pub type GRCVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB2_SPEC, u16, GRCVS_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Vertical width of graphics image area."]
    #[inline(always)]
    pub fn grcvw(&self) -> GRCVW_R {
        GRCVW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Vertical start position of graphics image area."]
    #[inline(always)]
    pub fn grcvs(&self) -> GRCVS_R {
        GRCVS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical width of graphics image area."]
    #[inline(always)]
    #[must_use]
    pub fn grcvw(&mut self) -> GRCVW_W<0> {
        GRCVW_W::new(self)
    }
    #[doc = "Bits 16:26 - Vertical start position of graphics image area."]
    #[inline(always)]
    #[must_use]
    pub fn grcvs(&mut self) -> GRCVS_W<16> {
        GRCVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab2](index.html) module"]
pub struct GR_AB2_SPEC;
impl crate::RegisterSpec for GR_AB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab2::R](R) reader structure"]
impl crate::Readable for GR_AB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab2::W](W) writer structure"]
impl crate::Writable for GR_AB2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB2 to value 0x0006_0010"]
impl crate::Resettable for GR_AB2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_0010;
}
