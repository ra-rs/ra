#[doc = "Register `TCON_STV%s1` reader"]
pub struct R(crate::R<TCON_STV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCON_STV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCON_STV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCON_STV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCON_STV%s1` writer"]
pub struct W(crate::W<TCON_STV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCON_STV1_SPEC>;
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
impl From<crate::W<TCON_STV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCON_STV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VW` reader - STVx1 second change timingSets the signal assertion width."]
pub type VW_R = crate::FieldReader<u16, VW_A>;
#[doc = "STVx1 second change timingSets the signal assertion width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VW_A(u16);
impl From<VW_A> for u16 {
    #[inline(always)]
    fn from(val: VW_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `VW` writer - STVx1 second change timingSets the signal assertion width."]
pub type VW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TCON_STV1_SPEC, u16, VW_A, 11, O>;
#[doc = "Field `VS` reader - STVx1 first change timing"]
pub type VS_R = crate::FieldReader<u16, VS_A>;
#[doc = "STVx1 first change timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VS_A(u16);
impl From<VS_A> for u16 {
    #[inline(always)]
    fn from(val: VS_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `VS` writer - STVx1 first change timing"]
pub type VS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TCON_STV1_SPEC, u16, VS_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - STVx1 second change timingSets the signal assertion width."]
    #[inline(always)]
    pub fn vw(&self) -> VW_R {
        VW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - STVx1 first change timing"]
    #[inline(always)]
    pub fn vs(&self) -> VS_R {
        VS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - STVx1 second change timingSets the signal assertion width."]
    #[inline(always)]
    #[must_use]
    pub fn vw(&mut self) -> VW_W<0> {
        VW_W::new(self)
    }
    #[doc = "Bits 16:26 - STVx1 first change timing"]
    #[inline(always)]
    #[must_use]
    pub fn vs(&mut self) -> VS_W<16> {
        VS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Vertical Timing Setting Register %s1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcon_stv1](index.html) module"]
pub struct TCON_STV1_SPEC;
impl crate::RegisterSpec for TCON_STV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcon_stv1::R](R) reader structure"]
impl crate::Readable for TCON_STV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcon_stv1::W](W) writer structure"]
impl crate::Writable for TCON_STV1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCON_STV%s1 to value 0"]
impl crate::Resettable for TCON_STV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
