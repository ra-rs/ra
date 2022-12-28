#[doc = "Register `GR%s_CLUTINT` reader"]
pub struct R(crate::R<GR_CLUTINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_CLUTINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_CLUTINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_CLUTINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_CLUTINT` writer"]
pub struct W(crate::W<GR_CLUTINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_CLUTINT_SPEC>;
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
impl From<crate::W<GR_CLUTINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_CLUTINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE` reader - Number of detection lines"]
pub type LINE_R = crate::FieldReader<u16, LINE_A>;
#[doc = "Number of detection lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LINE_A(u16);
impl From<LINE_A> for u16 {
    #[inline(always)]
    fn from(val: LINE_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `LINE` writer - Number of detection lines"]
pub type LINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_CLUTINT_SPEC, u16, LINE_A, 11, O>;
#[doc = "Field `SEL` reader - CLUT table control"]
pub type SEL_R = crate::BitReader<SEL_A>;
#[doc = "CLUT table control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_A {
    #[doc = "1: Uses CLUT1 plane for internal operations."]
    _1 = 1,
    #[doc = "0: Uses CLUT0 plane for internal operations."]
    _0 = 0,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            true => Some(SEL_A::_1),
            false => Some(SEL_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEL_A::_0
    }
}
#[doc = "Field `SEL` writer - CLUT table control"]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GR_CLUTINT_SPEC, SEL_A, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Uses CLUT1 plane for internal operations."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEL_A::_1)
    }
    #[doc = "Uses CLUT0 plane for internal operations."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEL_A::_0)
    }
}
impl R {
    #[doc = "Bits 0:10 - Number of detection lines"]
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - CLUT table control"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Number of detection lines"]
    #[inline(always)]
    #[must_use]
    pub fn line(&mut self) -> LINE_W<0> {
        LINE_W::new(self)
    }
    #[doc = "Bit 16 - CLUT table control"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<16> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s CLUT Table Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_clutint](index.html) module"]
pub struct GR_CLUTINT_SPEC;
impl crate::RegisterSpec for GR_CLUTINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_clutint::R](R) reader structure"]
impl crate::Readable for GR_CLUTINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_clutint::W](W) writer structure"]
impl crate::Writable for GR_CLUTINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_CLUTINT to value 0"]
impl crate::Resettable for GR_CLUTINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
