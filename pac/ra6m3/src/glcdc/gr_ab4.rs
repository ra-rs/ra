#[doc = "Register `GR%s_AB4` reader"]
pub struct R(crate::R<GR_AB4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB4` writer"]
pub struct W(crate::W<GR_AB4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB4_SPEC>;
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
impl From<crate::W<GR_AB4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARCVW` reader - Vertical width of rectangular area alpha blending image area."]
pub type ARCVW_R = crate::FieldReader<u16, ARCVW_A>;
#[doc = "Vertical width of rectangular area alpha blending image area.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ARCVW_A(u16);
impl From<ARCVW_A> for u16 {
    #[inline(always)]
    fn from(val: ARCVW_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `ARCVW` writer - Vertical width of rectangular area alpha blending image area."]
pub type ARCVW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB4_SPEC, u16, ARCVW_A, 11, O>;
#[doc = "Field `ARCVS` reader - Vertical start position of rectangular area alpha blending image area"]
pub type ARCVS_R = crate::FieldReader<u16, ARCVS_A>;
#[doc = "Vertical start position of rectangular area alpha blending image area\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ARCVS_A(u16);
impl From<ARCVS_A> for u16 {
    #[inline(always)]
    fn from(val: ARCVS_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `ARCVS` writer - Vertical start position of rectangular area alpha blending image area"]
pub type ARCVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB4_SPEC, u16, ARCVS_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Vertical width of rectangular area alpha blending image area."]
    #[inline(always)]
    pub fn arcvw(&self) -> ARCVW_R {
        ARCVW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Vertical start position of rectangular area alpha blending image area"]
    #[inline(always)]
    pub fn arcvs(&self) -> ARCVS_R {
        ARCVS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical width of rectangular area alpha blending image area."]
    #[inline(always)]
    #[must_use]
    pub fn arcvw(&mut self) -> ARCVW_W<0> {
        ARCVW_W::new(self)
    }
    #[doc = "Bits 16:26 - Vertical start position of rectangular area alpha blending image area"]
    #[inline(always)]
    #[must_use]
    pub fn arcvs(&mut self) -> ARCVS_W<16> {
        ARCVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab4](index.html) module"]
pub struct GR_AB4_SPEC;
impl crate::RegisterSpec for GR_AB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab4::R](R) reader structure"]
impl crate::Readable for GR_AB4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab4::W](W) writer structure"]
impl crate::Writable for GR_AB4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB4 to value 0x0006_0010"]
impl crate::Resettable for GR_AB4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_0010;
}
