#[doc = "Register `GR%s_AB7` reader"]
pub struct R(crate::R<GR_AB7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB7` writer"]
pub struct W(crate::W<GR_AB7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB7_SPEC>;
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
impl From<crate::W<GR_AB7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKON` reader - RGB-index chroma-key processing control."]
pub type CKON_R = crate::BitReader<CKON_A>;
#[doc = "RGB-index chroma-key processing control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKON_A {
    #[doc = "1: Enables chroma-key processing"]
    _1 = 1,
    #[doc = "0: Disables chroma-key processing"]
    _0 = 0,
}
impl From<CKON_A> for bool {
    #[inline(always)]
    fn from(variant: CKON_A) -> Self {
        variant as u8 != 0
    }
}
impl CKON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKON_A {
        match self.bits {
            true => CKON_A::_1,
            false => CKON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKON_A::_0
    }
}
#[doc = "Field `CKON` writer - RGB-index chroma-key processing control."]
pub type CKON_W<'a, const O: u8> = crate::BitWriter<'a, u32, GR_AB7_SPEC, CKON_A, O>;
impl<'a, const O: u8> CKON_W<'a, O> {
    #[doc = "Enables chroma-key processing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CKON_A::_1)
    }
    #[doc = "Disables chroma-key processing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CKON_A::_0)
    }
}
#[doc = "Field `ARCDEF` reader - Initial alpha value for alpha blending in rectangular area."]
pub type ARCDEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARCDEF` writer - Initial alpha value for alpha blending in rectangular area."]
pub type ARCDEF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_AB7_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - RGB-index chroma-key processing control."]
    #[inline(always)]
    pub fn ckon(&self) -> CKON_R {
        CKON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Initial alpha value for alpha blending in rectangular area."]
    #[inline(always)]
    pub fn arcdef(&self) -> ARCDEF_R {
        ARCDEF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RGB-index chroma-key processing control."]
    #[inline(always)]
    #[must_use]
    pub fn ckon(&mut self) -> CKON_W<0> {
        CKON_W::new(self)
    }
    #[doc = "Bits 16:23 - Initial alpha value for alpha blending in rectangular area."]
    #[inline(always)]
    #[must_use]
    pub fn arcdef(&mut self) -> ARCDEF_W<16> {
        ARCDEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab7](index.html) module"]
pub struct GR_AB7_SPEC;
impl crate::RegisterSpec for GR_AB7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab7::R](R) reader structure"]
impl crate::Readable for GR_AB7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab7::W](W) writer structure"]
impl crate::Writable for GR_AB7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB7 to value 0"]
impl crate::Resettable for GR_AB7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
