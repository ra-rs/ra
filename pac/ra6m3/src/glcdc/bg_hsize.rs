#[doc = "Register `BG_HSIZE` reader"]
pub struct R(crate::R<BG_HSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_HSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_HSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_HSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BG_HSIZE` writer"]
pub struct W(crate::W<BG_HSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_HSIZE_SPEC>;
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
impl From<crate::W<BG_HSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_HSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HW` reader - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field."]
pub type HW_R = crate::FieldReader<u16, HW_A>;
#[doc = "Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HW_A(u16);
impl From<HW_A> for u16 {
    #[inline(always)]
    fn from(val: HW_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `HW` writer - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field."]
pub type HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_HSIZE_SPEC, u16, HW_A, 11, O>;
#[doc = "Field `HP` reader - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK)."]
pub type HP_R = crate::FieldReader<u16, HP_A>;
#[doc = "Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK).\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HP_A(u16);
impl From<HP_A> for u16 {
    #[inline(always)]
    fn from(val: HP_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `HP` writer - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK)."]
pub type HP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_HSIZE_SPEC, u16, HP_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field."]
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn hp(&self) -> HP_R {
        HP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field."]
    #[inline(always)]
    #[must_use]
    pub fn hw(&mut self) -> HW_W<0> {
        HW_W::new(self)
    }
    #[doc = "Bits 16:26 - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    #[must_use]
    pub fn hp(&mut self) -> HP_W<16> {
        HP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Plane Setting Full Image Horizontal Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_hsize](index.html) module"]
pub struct BG_HSIZE_SPEC;
impl crate::RegisterSpec for BG_HSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_hsize::R](R) reader structure"]
impl crate::Readable for BG_HSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_hsize::W](W) writer structure"]
impl crate::Writable for BG_HSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BG_HSIZE to value 0x0006_0010"]
impl crate::Resettable for BG_HSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_0010;
}
