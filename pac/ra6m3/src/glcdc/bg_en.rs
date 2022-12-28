#[doc = "Register `BG_EN` reader"]
pub struct R(crate::R<BG_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BG_EN` writer"]
pub struct W(crate::W<BG_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_EN_SPEC>;
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
impl From<crate::W<BG_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Background plane generation module operation enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Background plane generation module operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "1: Enables operation."]
    _1 = 1,
    #[doc = "0: Disables operation."]
    _0 = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::_1,
            false => EN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
}
#[doc = "Field `EN` writer - Background plane generation module operation enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BG_EN_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Enables operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
    }
    #[doc = "Disables operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
}
#[doc = "Field `VEN` reader - Control of LCDC internal register value reflection to internal operations\n\nThe field is **modified** in some way after a read operation."]
pub type VEN_R = crate::BitReader<VEN_A>;
#[doc = "Control of LCDC internal register value reflection to internal operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEN_A {
    #[doc = "1: Enables"]
    _1 = 1,
    #[doc = "0: Disables(Cleared to 0 by an internal source)"]
    _0 = 0,
}
impl From<VEN_A> for bool {
    #[inline(always)]
    fn from(variant: VEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEN_A {
        match self.bits {
            true => VEN_A::_1,
            false => VEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VEN_A::_0
    }
}
#[doc = "Field `VEN` writer - Control of LCDC internal register value reflection to internal operations"]
pub type VEN_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, BG_EN_SPEC, VEN_A, O>;
impl<'a, const O: u8> VEN_W<'a, O> {
    #[doc = "Enables"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VEN_A::_1)
    }
    #[doc = "Disables(Cleared to 0 by an internal source)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VEN_A::_0)
    }
}
#[doc = "Field `SWRST` reader - Entire module SW reset control"]
pub type SWRST_R = crate::BitReader<SWRST_A>;
#[doc = "Entire module SW reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST_A {
    #[doc = "1: Releases the entire module from the SW reset state."]
    _1 = 1,
    #[doc = "0: Places the entire module in the SW reset state."]
    _0 = 0,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            true => SWRST_A::_1,
            false => SWRST_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRST_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRST_A::_0
    }
}
#[doc = "Field `SWRST` writer - Entire module SW reset control"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BG_EN_SPEC, SWRST_A, O>;
impl<'a, const O: u8> SWRST_W<'a, O> {
    #[doc = "Releases the entire module from the SW reset state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRST_A::_1)
    }
    #[doc = "Places the entire module in the SW reset state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRST_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Background plane generation module operation enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Control of LCDC internal register value reflection to internal operations"]
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Entire module SW reset control"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Background plane generation module operation enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 8 - Control of LCDC internal register value reflection to internal operations"]
    #[inline(always)]
    #[must_use]
    pub fn ven(&mut self) -> VEN_W<8> {
        VEN_W::new(self)
    }
    #[doc = "Bit 16 - Entire module SW reset control"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<16> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Plane Setting Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_en](index.html) module"]
pub struct BG_EN_SPEC;
impl crate::RegisterSpec for BG_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_en::R](R) reader structure"]
impl crate::Readable for BG_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_en::W](W) writer structure"]
impl crate::Writable for BG_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0100;
}
#[doc = "`reset()` method sets BG_EN to value 0"]
impl crate::Resettable for BG_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
