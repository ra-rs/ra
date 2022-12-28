#[doc = "Register `BG_PERI` reader"]
pub struct R(crate::R<BG_PERI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_PERI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_PERI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_PERI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BG_PERI` writer"]
pub struct W(crate::W<BG_PERI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_PERI_SPEC>;
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
impl From<crate::W<BG_PERI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_PERI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FH` reader - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK)."]
pub type FH_R = crate::FieldReader<u16, FH_A>;
#[doc = "Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK).\n\nValue on reset: 23"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FH_A(u16);
impl From<FH_A> for u16 {
    #[inline(always)]
    fn from(val: FH_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `FH` writer - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK)."]
pub type FH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_PERI_SPEC, u16, FH_A, 11, O>;
#[doc = "Field `FV` reader - Background plane vertical synchronization signal period on the basis of line."]
pub type FV_R = crate::FieldReader<u16, FV_A>;
#[doc = "Background plane vertical synchronization signal period on the basis of line.\n\nValue on reset: 23"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FV_A(u16);
impl From<FV_A> for u16 {
    #[inline(always)]
    fn from(val: FV_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `FV` writer - Background plane vertical synchronization signal period on the basis of line."]
pub type FV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_PERI_SPEC, u16, FV_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn fh(&self) -> FH_R {
        FH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Background plane vertical synchronization signal period on the basis of line."]
    #[inline(always)]
    pub fn fv(&self) -> FV_R {
        FV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    #[must_use]
    pub fn fh(&mut self) -> FH_W<0> {
        FH_W::new(self)
    }
    #[doc = "Bits 16:26 - Background plane vertical synchronization signal period on the basis of line."]
    #[inline(always)]
    #[must_use]
    pub fn fv(&mut self) -> FV_W<16> {
        FV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Plane Setting Free-Running Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_peri](index.html) module"]
pub struct BG_PERI_SPEC;
impl crate::RegisterSpec for BG_PERI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_peri::R](R) reader structure"]
impl crate::Readable for BG_PERI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_peri::W](W) writer structure"]
impl crate::Writable for BG_PERI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BG_PERI to value 0x0017_0017"]
impl crate::Resettable for BG_PERI_SPEC {
    const RESET_VALUE: Self::Ux = 0x0017_0017;
}
