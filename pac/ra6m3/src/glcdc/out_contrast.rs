#[doc = "Register `OUT_CONTRAST` reader"]
pub struct R(crate::R<OUT_CONTRAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONTRAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONTRAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONTRAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONTRAST` writer"]
pub struct W(crate::W<OUT_CONTRAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONTRAST_SPEC>;
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
impl From<crate::W<OUT_CONTRAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONTRAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTR` reader - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point"]
pub type CONTR_R = crate::FieldReader<u8, CONTR_A>;
#[doc = "Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CONTR_A(u8);
impl From<CONTR_A> for u8 {
    #[inline(always)]
    fn from(val: CONTR_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `CONTR` writer - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point"]
pub type CONTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT_CONTRAST_SPEC, u8, CONTR_A, 8, O>;
#[doc = "Field `CONTB` reader - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point"]
pub type CONTB_R = crate::FieldReader<u8, CONTB_A>;
#[doc = "Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CONTB_A(u8);
impl From<CONTB_A> for u8 {
    #[inline(always)]
    fn from(val: CONTB_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `CONTB` writer - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point"]
pub type CONTB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT_CONTRAST_SPEC, u8, CONTB_A, 8, O>;
#[doc = "Field `CONTG` reader - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point."]
pub type CONTG_R = crate::FieldReader<u8, CONTG_A>;
#[doc = "Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CONTG_A(u8);
impl From<CONTG_A> for u8 {
    #[inline(always)]
    fn from(val: CONTG_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `CONTG` writer - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point."]
pub type CONTG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT_CONTRAST_SPEC, u8, CONTG_A, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    pub fn contr(&self) -> CONTR_R {
        CONTR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    pub fn contb(&self) -> CONTB_R {
        CONTB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point."]
    #[inline(always)]
    pub fn contg(&self) -> CONTG_R {
        CONTG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn contr(&mut self) -> CONTR_W<0> {
        CONTR_W::new(self)
    }
    #[doc = "Bits 8:15 - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    #[must_use]
    pub fn contb(&mut self) -> CONTB_W<8> {
        CONTB_W::new(self)
    }
    #[doc = "Bits 16:23 - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point."]
    #[inline(always)]
    #[must_use]
    pub fn contg(&mut self) -> CONTG_W<16> {
        CONTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Control Block Contrast Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_contrast](index.html) module"]
pub struct OUT_CONTRAST_SPEC;
impl crate::RegisterSpec for OUT_CONTRAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_contrast::R](R) reader structure"]
impl crate::Readable for OUT_CONTRAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_contrast::W](W) writer structure"]
impl crate::Writable for OUT_CONTRAST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CONTRAST to value 0"]
impl crate::Resettable for OUT_CONTRAST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
