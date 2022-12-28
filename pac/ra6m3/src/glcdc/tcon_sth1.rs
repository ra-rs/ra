#[doc = "Register `TCON_STH%s1` reader"]
pub struct R(crate::R<TCON_STH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCON_STH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCON_STH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCON_STH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCON_STH%s1` writer"]
pub struct W(crate::W<TCON_STH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCON_STH1_SPEC>;
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
impl From<crate::W<TCON_STH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCON_STH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HW` reader - STHx1 second change timing.Sets the signal assertion width."]
pub type HW_R = crate::FieldReader<u16, HW_A>;
#[doc = "STHx1 second change timing.Sets the signal assertion width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HW_A(u16);
impl From<HW_A> for u16 {
    #[inline(always)]
    fn from(val: HW_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `HW` writer - STHx1 second change timing.Sets the signal assertion width."]
pub type HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCON_STH1_SPEC, u16, HW_A, 11, O>;
#[doc = "Field `HS` reader - STHx1 first change timing"]
pub type HS_R = crate::FieldReader<u16, HS_A>;
#[doc = "STHx1 first change timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HS_A(u16);
impl From<HS_A> for u16 {
    #[inline(always)]
    fn from(val: HS_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `HS` writer - STHx1 first change timing"]
pub type HS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCON_STH1_SPEC, u16, HS_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - STHx1 second change timing.Sets the signal assertion width."]
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - STHx1 first change timing"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - STHx1 second change timing.Sets the signal assertion width."]
    #[inline(always)]
    #[must_use]
    pub fn hw(&mut self) -> HW_W<0> {
        HW_W::new(self)
    }
    #[doc = "Bits 16:26 - STHx1 first change timing"]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HS_W<16> {
        HS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Horizontal Timing Setting Register STH%s1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcon_sth1](index.html) module"]
pub struct TCON_STH1_SPEC;
impl crate::RegisterSpec for TCON_STH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcon_sth1::R](R) reader structure"]
impl crate::Readable for TCON_STH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcon_sth1::W](W) writer structure"]
impl crate::Writable for TCON_STH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCON_STH%s1 to value 0"]
impl crate::Resettable for TCON_STH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
