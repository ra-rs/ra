#[doc = "Register `GR%s_FLM6` reader"]
pub struct R(crate::R<GR_FLM6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_FLM6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_FLM6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_FLM6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_FLM6` writer"]
pub struct W(crate::W<GR_FLM6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_FLM6_SPEC>;
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
impl From<crate::W<GR_FLM6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_FLM6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - Data format for accessing graphics data (frame buffer data)."]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "Data format for accessing graphics data (frame buffer data).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "7: CLUT11bit/pix)"]
    _111 = 7,
    #[doc = "6: CLUT4 (4 bits/pix)"]
    _110 = 6,
    #[doc = "5: CLUT8 (8 bits/pix)"]
    _101 = 5,
    #[doc = "4: ARGB8888 (32 bits/pix)"]
    _100 = 4,
    #[doc = "3: ARGB4444 (16 bits/pix)"]
    _011 = 3,
    #[doc = "2: ARGB1555 (16 bits/pix, 1 bit of A is LUT data)"]
    _010 = 2,
    #[doc = "1: RGB888 (32 bits/pix, 8 bits on the MSB side are invalid)"]
    _001 = 1,
    #[doc = "0: RGB565 (16 bits/pix)"]
    _000 = 0,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            7 => Some(FORMAT_A::_111),
            6 => Some(FORMAT_A::_110),
            5 => Some(FORMAT_A::_101),
            4 => Some(FORMAT_A::_100),
            3 => Some(FORMAT_A::_011),
            2 => Some(FORMAT_A::_010),
            1 => Some(FORMAT_A::_001),
            0 => Some(FORMAT_A::_000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FORMAT_A::_111
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FORMAT_A::_110
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FORMAT_A::_101
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FORMAT_A::_100
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FORMAT_A::_011
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FORMAT_A::_010
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FORMAT_A::_001
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FORMAT_A::_000
    }
}
#[doc = "Field `FORMAT` writer - Data format for accessing graphics data (frame buffer data)."]
pub type FORMAT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GR_FLM6_SPEC, u8, FORMAT_A, 3, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "CLUT11bit/pix)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FORMAT_A::_111)
    }
    #[doc = "CLUT4 (4 bits/pix)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FORMAT_A::_110)
    }
    #[doc = "CLUT8 (8 bits/pix)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FORMAT_A::_101)
    }
    #[doc = "ARGB8888 (32 bits/pix)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FORMAT_A::_100)
    }
    #[doc = "ARGB4444 (16 bits/pix)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FORMAT_A::_011)
    }
    #[doc = "ARGB1555 (16 bits/pix, 1 bit of A is LUT data)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FORMAT_A::_010)
    }
    #[doc = "RGB888 (32 bits/pix, 8 bits on the MSB side are invalid)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FORMAT_A::_001)
    }
    #[doc = "RGB565 (16 bits/pix)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FORMAT_A::_000)
    }
}
impl R {
    #[doc = "Bits 28:30 - Data format for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Data format for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<28> {
        FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Frame Buffer Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_flm6](index.html) module"]
pub struct GR_FLM6_SPEC;
impl crate::RegisterSpec for GR_FLM6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_flm6::R](R) reader structure"]
impl crate::Readable for GR_FLM6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_flm6::W](W) writer structure"]
impl crate::Writable for GR_FLM6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_FLM6 to value 0"]
impl crate::Resettable for GR_FLM6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
