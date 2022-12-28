#[doc = "Register `CFDTHLSTS` reader"]
pub struct R(crate::R<CFDTHLSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTHLSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTHLSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTHLSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTHLSTS` writer"]
pub struct W(crate::W<CFDTHLSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTHLSTS_SPEC>;
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
impl From<crate::W<CFDTHLSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTHLSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THLEMP` reader - TX History List Empty"]
pub type THLEMP_R = crate::BitReader<THLEMP_A>;
#[doc = "TX History List Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLEMP_A {
    #[doc = "0: TX History List not empty"]
    _0 = 0,
    #[doc = "1: TX History List empty"]
    _1 = 1,
}
impl From<THLEMP_A> for bool {
    #[inline(always)]
    fn from(variant: THLEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl THLEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLEMP_A {
        match self.bits {
            false => THLEMP_A::_0,
            true => THLEMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLEMP_A::_1
    }
}
#[doc = "Field `THLFLL` reader - TX History List Full"]
pub type THLFLL_R = crate::BitReader<THLFLL_A>;
#[doc = "TX History List Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLFLL_A {
    #[doc = "0: TX History List not full"]
    _0 = 0,
    #[doc = "1: TX History List full"]
    _1 = 1,
}
impl From<THLFLL_A> for bool {
    #[inline(always)]
    fn from(variant: THLFLL_A) -> Self {
        variant as u8 != 0
    }
}
impl THLFLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLFLL_A {
        match self.bits {
            false => THLFLL_A::_0,
            true => THLFLL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLFLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLFLL_A::_1
    }
}
#[doc = "Field `THLELT` reader - TX History List Entry Lost"]
pub type THLELT_R = crate::BitReader<THLELT_A>;
#[doc = "TX History List Entry Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLELT_A {
    #[doc = "0: No entry lost in TX History List"]
    _0 = 0,
    #[doc = "1: TX History List entry Lost"]
    _1 = 1,
}
impl From<THLELT_A> for bool {
    #[inline(always)]
    fn from(variant: THLELT_A) -> Self {
        variant as u8 != 0
    }
}
impl THLELT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLELT_A {
        match self.bits {
            false => THLELT_A::_0,
            true => THLELT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLELT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLELT_A::_1
    }
}
#[doc = "Field `THLELT` writer - TX History List Entry Lost"]
pub type THLELT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTHLSTS_SPEC, THLELT_A, O>;
impl<'a, const O: u8> THLELT_W<'a, O> {
    #[doc = "No entry lost in TX History List"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLELT_A::_0)
    }
    #[doc = "TX History List entry Lost"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLELT_A::_1)
    }
}
#[doc = "Field `THLIF` reader - TX History List Interrupt Flag"]
pub type THLIF_R = crate::BitReader<THLIF_A>;
#[doc = "TX History List Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLIF_A {
    #[doc = "0: TX History List interrupt condition not satisfied"]
    _0 = 0,
    #[doc = "1: TX History List interrupt condition satisfied"]
    _1 = 1,
}
impl From<THLIF_A> for bool {
    #[inline(always)]
    fn from(variant: THLIF_A) -> Self {
        variant as u8 != 0
    }
}
impl THLIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLIF_A {
        match self.bits {
            false => THLIF_A::_0,
            true => THLIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLIF_A::_1
    }
}
#[doc = "Field `THLIF` writer - TX History List Interrupt Flag"]
pub type THLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTHLSTS_SPEC, THLIF_A, O>;
impl<'a, const O: u8> THLIF_W<'a, O> {
    #[doc = "TX History List interrupt condition not satisfied"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLIF_A::_0)
    }
    #[doc = "TX History List interrupt condition satisfied"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLIF_A::_1)
    }
}
#[doc = "Field `THLMC` reader - TX History List Message Count"]
pub type THLMC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - TX History List Empty"]
    #[inline(always)]
    pub fn thlemp(&self) -> THLEMP_R {
        THLEMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX History List Full"]
    #[inline(always)]
    pub fn thlfll(&self) -> THLFLL_R {
        THLFLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX History List Entry Lost"]
    #[inline(always)]
    pub fn thlelt(&self) -> THLELT_R {
        THLELT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX History List Interrupt Flag"]
    #[inline(always)]
    pub fn thlif(&self) -> THLIF_R {
        THLIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - TX History List Message Count"]
    #[inline(always)]
    pub fn thlmc(&self) -> THLMC_R {
        THLMC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - TX History List Entry Lost"]
    #[inline(always)]
    #[must_use]
    pub fn thlelt(&mut self) -> THLELT_W<2> {
        THLELT_W::new(self)
    }
    #[doc = "Bit 3 - TX History List Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn thlif(&mut self) -> THLIF_W<3> {
        THLIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX History List Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdthlsts](index.html) module"]
pub struct CFDTHLSTS_SPEC;
impl crate::RegisterSpec for CFDTHLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdthlsts::R](R) reader structure"]
impl crate::Readable for CFDTHLSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdthlsts::W](W) writer structure"]
impl crate::Writable for CFDTHLSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTHLSTS to value 0x01"]
impl crate::Resettable for CFDTHLSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
