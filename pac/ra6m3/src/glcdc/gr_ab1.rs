#[doc = "Register `GR%s_AB1` reader"]
pub struct R(crate::R<GR_AB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_AB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_AB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_AB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_AB1` writer"]
pub struct W(crate::W<GR_AB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_AB1_SPEC>;
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
impl From<crate::W<GR_AB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_AB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISPSEL` reader - Graphics display plane control."]
pub type DISPSEL_R = crate::FieldReader<u8, DISPSEL_A>;
#[doc = "Graphics display plane control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISPSEL_A {
    #[doc = "3: Blended display of lower-layer graphics (input image from the previous stage) and current graphics (graphics data read from the AHB bus)"]
    _11 = 3,
    #[doc = "2: Current graphics display"]
    _10 = 2,
    #[doc = "1: Lower-layer graphics display"]
    _01 = 1,
    #[doc = "0: Background color display (value set by the GRn_BASE register)."]
    _00 = 0,
}
impl From<DISPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DISPSEL_A) -> Self {
        variant as _
    }
}
impl DISPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISPSEL_A {
        match self.bits {
            3 => DISPSEL_A::_11,
            2 => DISPSEL_A::_10,
            1 => DISPSEL_A::_01,
            0 => DISPSEL_A::_00,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DISPSEL_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DISPSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DISPSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DISPSEL_A::_00
    }
}
#[doc = "Field `DISPSEL` writer - Graphics display plane control."]
pub type DISPSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GR_AB1_SPEC, u8, DISPSEL_A, 2, O>;
impl<'a, const O: u8> DISPSEL_W<'a, O> {
    #[doc = "Blended display of lower-layer graphics (input image from the previous stage) and current graphics (graphics data read from the AHB bus)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DISPSEL_A::_11)
    }
    #[doc = "Current graphics display"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DISPSEL_A::_10)
    }
    #[doc = "Lower-layer graphics display"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DISPSEL_A::_01)
    }
    #[doc = "Background color display (value set by the GRn_BASE register)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DISPSEL_A::_00)
    }
}
#[doc = "Field `GRCDISPON` reader - Graphics image area border display control."]
pub type GRCDISPON_R = crate::BitReader<GRCDISPON_A>;
#[doc = "Graphics image area border display control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRCDISPON_A {
    #[doc = "1: Display on"]
    _1 = 1,
    #[doc = "0: Display off"]
    _0 = 0,
}
impl From<GRCDISPON_A> for bool {
    #[inline(always)]
    fn from(variant: GRCDISPON_A) -> Self {
        variant as u8 != 0
    }
}
impl GRCDISPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRCDISPON_A {
        match self.bits {
            true => GRCDISPON_A::_1,
            false => GRCDISPON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRCDISPON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRCDISPON_A::_0
    }
}
#[doc = "Field `GRCDISPON` writer - Graphics image area border display control."]
pub type GRCDISPON_W<'a, const O: u8> = crate::BitWriter<'a, u32, GR_AB1_SPEC, GRCDISPON_A, O>;
impl<'a, const O: u8> GRCDISPON_W<'a, O> {
    #[doc = "Display on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRCDISPON_A::_1)
    }
    #[doc = "Display off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRCDISPON_A::_0)
    }
}
#[doc = "Field `ARCDISPON` reader - Image area border display control for rectangular area alpha blending."]
pub type ARCDISPON_R = crate::BitReader<ARCDISPON_A>;
#[doc = "Image area border display control for rectangular area alpha blending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARCDISPON_A {
    #[doc = "1: Display on"]
    _1 = 1,
    #[doc = "0: Display off"]
    _0 = 0,
}
impl From<ARCDISPON_A> for bool {
    #[inline(always)]
    fn from(variant: ARCDISPON_A) -> Self {
        variant as u8 != 0
    }
}
impl ARCDISPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARCDISPON_A {
        match self.bits {
            true => ARCDISPON_A::_1,
            false => ARCDISPON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARCDISPON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARCDISPON_A::_0
    }
}
#[doc = "Field `ARCDISPON` writer - Image area border display control for rectangular area alpha blending."]
pub type ARCDISPON_W<'a, const O: u8> = crate::BitWriter<'a, u32, GR_AB1_SPEC, ARCDISPON_A, O>;
impl<'a, const O: u8> ARCDISPON_W<'a, O> {
    #[doc = "Display on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARCDISPON_A::_1)
    }
    #[doc = "Display off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARCDISPON_A::_0)
    }
}
#[doc = "Field `ARCON` reader - Rectangular area alpha blending control."]
pub type ARCON_R = crate::BitReader<ARCON_A>;
#[doc = "Rectangular area alpha blending control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARCON_A {
    #[doc = "1: On"]
    _1 = 1,
    #[doc = "0: Off"]
    _0 = 0,
}
impl From<ARCON_A> for bool {
    #[inline(always)]
    fn from(variant: ARCON_A) -> Self {
        variant as u8 != 0
    }
}
impl ARCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARCON_A {
        match self.bits {
            true => ARCON_A::_1,
            false => ARCON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARCON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARCON_A::_0
    }
}
#[doc = "Field `ARCON` writer - Rectangular area alpha blending control."]
pub type ARCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, GR_AB1_SPEC, ARCON_A, O>;
impl<'a, const O: u8> ARCON_W<'a, O> {
    #[doc = "On"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARCON_A::_1)
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARCON_A::_0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Graphics display plane control."]
    #[inline(always)]
    pub fn dispsel(&self) -> DISPSEL_R {
        DISPSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Graphics image area border display control."]
    #[inline(always)]
    pub fn grcdispon(&self) -> GRCDISPON_R {
        GRCDISPON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Image area border display control for rectangular area alpha blending."]
    #[inline(always)]
    pub fn arcdispon(&self) -> ARCDISPON_R {
        ARCDISPON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Rectangular area alpha blending control."]
    #[inline(always)]
    pub fn arcon(&self) -> ARCON_R {
        ARCON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Graphics display plane control."]
    #[inline(always)]
    #[must_use]
    pub fn dispsel(&mut self) -> DISPSEL_W<0> {
        DISPSEL_W::new(self)
    }
    #[doc = "Bit 4 - Graphics image area border display control."]
    #[inline(always)]
    #[must_use]
    pub fn grcdispon(&mut self) -> GRCDISPON_W<4> {
        GRCDISPON_W::new(self)
    }
    #[doc = "Bit 8 - Image area border display control for rectangular area alpha blending."]
    #[inline(always)]
    #[must_use]
    pub fn arcdispon(&mut self) -> ARCDISPON_W<8> {
        ARCDISPON_W::new(self)
    }
    #[doc = "Bit 12 - Rectangular area alpha blending control."]
    #[inline(always)]
    #[must_use]
    pub fn arcon(&mut self) -> ARCON_W<12> {
        ARCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Alpha Blending Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ab1](index.html) module"]
pub struct GR_AB1_SPEC;
impl crate::RegisterSpec for GR_AB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ab1::R](R) reader structure"]
impl crate::Readable for GR_AB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ab1::W](W) writer structure"]
impl crate::Writable for GR_AB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_AB1 to value 0"]
impl crate::Resettable for GR_AB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
