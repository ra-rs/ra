#[doc = "Register `GR%s_FLM1` reader"]
pub struct R(crate::R<GR_FLM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_FLM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_FLM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_FLM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_FLM1` writer"]
pub struct W(crate::W<GR_FLM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_FLM1_SPEC>;
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
impl From<crate::W<GR_FLM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_FLM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSTMD` reader - Burst transfer control for graphics data (frame buffer data)access"]
pub type BSTMD_R = crate::FieldReader<u8, BSTMD_A>;
#[doc = "Burst transfer control for graphics data (frame buffer data)access\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BSTMD_A {
    #[doc = "3: 16-beat increment burst transfer (64-byte boundary)"]
    _11 = 3,
}
impl From<BSTMD_A> for u8 {
    #[inline(always)]
    fn from(variant: BSTMD_A) -> Self {
        variant as _
    }
}
impl BSTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BSTMD_A> {
        match self.bits {
            3 => Some(BSTMD_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BSTMD_A::_11
    }
}
#[doc = "Field `BSTMD` writer - Burst transfer control for graphics data (frame buffer data)access"]
pub type BSTMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_FLM1_SPEC, u8, BSTMD_A, 2, O>;
impl<'a, const O: u8> BSTMD_W<'a, O> {
    #[doc = "16-beat increment burst transfer (64-byte boundary)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BSTMD_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Burst transfer control for graphics data (frame buffer data)access"]
    #[inline(always)]
    pub fn bstmd(&self) -> BSTMD_R {
        BSTMD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Burst transfer control for graphics data (frame buffer data)access"]
    #[inline(always)]
    #[must_use]
    pub fn bstmd(&mut self) -> BSTMD_W<0> {
        BSTMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Frame Buffer Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_flm1](index.html) module"]
pub struct GR_FLM1_SPEC;
impl crate::RegisterSpec for GR_FLM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_flm1::R](R) reader structure"]
impl crate::Readable for GR_FLM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_flm1::W](W) writer structure"]
impl crate::Writable for GR_FLM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_FLM1 to value 0x03"]
impl crate::Resettable for GR_FLM1_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
