#[doc = "Register `XCR2` reader"]
pub struct R(crate::R<XCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XCR2` writer"]
pub struct W(crate::W<XCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XCR2_SPEC>;
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
impl From<crate::W<XCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF0D` reader - Control Field 0 compare data"]
pub type CF0D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CF0D` writer - Control Field 0 compare data"]
pub type CF0D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CF0CE` reader - Control Field 0 compare bit enable"]
pub type CF0CE_R = crate::FieldReader<u8, CF0CE_A>;
#[doc = "Control Field 0 compare bit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CF0CE_A {
    #[doc = "0: Control Field 0 bit N compare disabled"]
    _0 = 0,
    #[doc = "1: Control Field 0 bit N compare enabled"]
    _1 = 1,
}
impl From<CF0CE_A> for u8 {
    #[inline(always)]
    fn from(variant: CF0CE_A) -> Self {
        variant as _
    }
}
impl CF0CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CF0CE_A> {
        match self.bits {
            0 => Some(CF0CE_A::_0),
            1 => Some(CF0CE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE_A::_1
    }
}
#[doc = "Field `CF0CE` writer - Control Field 0 compare bit enable"]
pub type CF0CE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XCR2_SPEC, u8, CF0CE_A, 8, O>;
impl<'a, const O: u8> CF0CE_W<'a, O> {
    #[doc = "Control Field 0 bit N compare disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE_A::_0)
    }
    #[doc = "Control Field 0 bit N compare enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE_A::_1)
    }
}
#[doc = "Field `BFLW` reader - Break Field length setting"]
pub type BFLW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BFLW` writer - Break Field length setting"]
pub type BFLW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XCR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - Control Field 0 compare data"]
    #[inline(always)]
    pub fn cf0d(&self) -> CF0D_R {
        CF0D_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Control Field 0 compare bit enable"]
    #[inline(always)]
    pub fn cf0ce(&self) -> CF0CE_R {
        CF0CE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Break Field length setting"]
    #[inline(always)]
    pub fn bflw(&self) -> BFLW_R {
        BFLW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Control Field 0 compare data"]
    #[inline(always)]
    #[must_use]
    pub fn cf0d(&mut self) -> CF0D_W<0> {
        CF0D_W::new(self)
    }
    #[doc = "Bits 8:15 - Control Field 0 compare bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce(&mut self) -> CF0CE_W<8> {
        CF0CE_W::new(self)
    }
    #[doc = "Bits 16:31 - Break Field length setting"]
    #[inline(always)]
    #[must_use]
    pub fn bflw(&mut self) -> BFLW_W<16> {
        BFLW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Simple LIN Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xcr2](index.html) module"]
pub struct XCR2_SPEC;
impl crate::RegisterSpec for XCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xcr2::R](R) reader structure"]
impl crate::Readable for XCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xcr2::W](W) writer structure"]
impl crate::Writable for XCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XCR2 to value 0xfffe_0000"]
impl crate::Resettable for XCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfffe_0000;
}
