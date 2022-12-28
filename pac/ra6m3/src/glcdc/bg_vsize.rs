#[doc = "Register `BG_VSIZE` reader"]
pub struct R(crate::R<BG_VSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_VSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_VSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_VSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BG_VSIZE` writer"]
pub struct W(crate::W<BG_VSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_VSIZE_SPEC>;
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
impl From<crate::W<BG_VSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_VSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VW` reader - Background plane vertical valid pixel width on the basis of line"]
pub type VW_R = crate::FieldReader<u16, VW_A>;
#[doc = "Background plane vertical valid pixel width on the basis of line\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VW_A(u16);
impl From<VW_A> for u16 {
    #[inline(always)]
    fn from(val: VW_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `VW` writer - Background plane vertical valid pixel width on the basis of line"]
pub type VW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_VSIZE_SPEC, u16, VW_A, 11, O>;
#[doc = "Field `VP` reader - Background plane vertical valid pixel start position on the basis of line"]
pub type VP_R = crate::FieldReader<u16, VP_A>;
#[doc = "Background plane vertical valid pixel start position on the basis of line\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VP_A(u16);
impl From<VP_A> for u16 {
    #[inline(always)]
    fn from(val: VP_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `VP` writer - Background plane vertical valid pixel start position on the basis of line"]
pub type VP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_VSIZE_SPEC, u16, VP_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Background plane vertical valid pixel width on the basis of line"]
    #[inline(always)]
    pub fn vw(&self) -> VW_R {
        VW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Background plane vertical valid pixel start position on the basis of line"]
    #[inline(always)]
    pub fn vp(&self) -> VP_R {
        VP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Background plane vertical valid pixel width on the basis of line"]
    #[inline(always)]
    #[must_use]
    pub fn vw(&mut self) -> VW_W<0> {
        VW_W::new(self)
    }
    #[doc = "Bits 16:26 - Background plane vertical valid pixel start position on the basis of line"]
    #[inline(always)]
    #[must_use]
    pub fn vp(&mut self) -> VP_W<16> {
        VP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Plane Setting Full Image Vertical Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_vsize](index.html) module"]
pub struct BG_VSIZE_SPEC;
impl crate::RegisterSpec for BG_VSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_vsize::R](R) reader structure"]
impl crate::Readable for BG_VSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_vsize::W](W) writer structure"]
impl crate::Writable for BG_VSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BG_VSIZE to value 0x0007_0010"]
impl crate::Resettable for BG_VSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0010;
}
