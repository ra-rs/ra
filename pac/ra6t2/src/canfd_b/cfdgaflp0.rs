#[doc = "Register `CFDGAFLP0%s` reader"]
pub struct R(crate::R<CFDGAFLP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLP0%s` writer"]
pub struct W(crate::W<CFDGAFLP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLP0_SPEC>;
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
impl From<crate::W<CFDGAFLP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAFLDLC` reader - Global Acceptance Filter List DLC Field"]
pub type GAFLDLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAFLDLC` writer - Global Acceptance Filter List DLC Field"]
pub type GAFLDLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `GAFLIFL0` reader - Global Acceptance Filter List Information Label 0"]
pub type GAFLIFL0_R = crate::BitReader<bool>;
#[doc = "Field `GAFLIFL0` writer - Global Acceptance Filter List Information Label 0"]
pub type GAFLIFL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLP0_SPEC, bool, O>;
#[doc = "Field `GAFLRMDP` reader - Global Acceptance Filter List RX Message Buffer Direction Pointer"]
pub type GAFLRMDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAFLRMDP` writer - Global Acceptance Filter List RX Message Buffer Direction Pointer"]
pub type GAFLRMDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLP0_SPEC, u8, u8, 5, O>;
#[doc = "Field `GAFLRMV` reader - Global Acceptance Filter List RX Message Buffer Valid"]
pub type GAFLRMV_R = crate::BitReader<GAFLRMV_A>;
#[doc = "Global Acceptance Filter List RX Message Buffer Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLRMV_A {
    #[doc = "0: Single message buffer direction pointer is invalid"]
    _0 = 0,
    #[doc = "1: Single message buffer direction pointer is valid"]
    _1 = 1,
}
impl From<GAFLRMV_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLRMV_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLRMV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLRMV_A {
        match self.bits {
            false => GAFLRMV_A::_0,
            true => GAFLRMV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLRMV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLRMV_A::_1
    }
}
#[doc = "Field `GAFLRMV` writer - Global Acceptance Filter List RX Message Buffer Valid"]
pub type GAFLRMV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLP0_SPEC, GAFLRMV_A, O>;
impl<'a, const O: u8> GAFLRMV_W<'a, O> {
    #[doc = "Single message buffer direction pointer is invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLRMV_A::_0)
    }
    #[doc = "Single message buffer direction pointer is valid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLRMV_A::_1)
    }
}
#[doc = "Field `GAFLPTR` reader - Global Acceptance Filter List Pointer"]
pub type GAFLPTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GAFLPTR` writer - Global Acceptance Filter List Pointer"]
pub type GAFLPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLP0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - Global Acceptance Filter List DLC Field"]
    #[inline(always)]
    pub fn gafldlc(&self) -> GAFLDLC_R {
        GAFLDLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Global Acceptance Filter List Information Label 0"]
    #[inline(always)]
    pub fn gaflifl0(&self) -> GAFLIFL0_R {
        GAFLIFL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Global Acceptance Filter List RX Message Buffer Direction Pointer"]
    #[inline(always)]
    pub fn gaflrmdp(&self) -> GAFLRMDP_R {
        GAFLRMDP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Global Acceptance Filter List RX Message Buffer Valid"]
    #[inline(always)]
    pub fn gaflrmv(&self) -> GAFLRMV_R {
        GAFLRMV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Global Acceptance Filter List Pointer"]
    #[inline(always)]
    pub fn gaflptr(&self) -> GAFLPTR_R {
        GAFLPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Global Acceptance Filter List DLC Field"]
    #[inline(always)]
    #[must_use]
    pub fn gafldlc(&mut self) -> GAFLDLC_W<0> {
        GAFLDLC_W::new(self)
    }
    #[doc = "Bit 7 - Global Acceptance Filter List Information Label 0"]
    #[inline(always)]
    #[must_use]
    pub fn gaflifl0(&mut self) -> GAFLIFL0_W<7> {
        GAFLIFL0_W::new(self)
    }
    #[doc = "Bits 8:12 - Global Acceptance Filter List RX Message Buffer Direction Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn gaflrmdp(&mut self) -> GAFLRMDP_W<8> {
        GAFLRMDP_W::new(self)
    }
    #[doc = "Bit 15 - Global Acceptance Filter List RX Message Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn gaflrmv(&mut self) -> GAFLRMV_W<15> {
        GAFLRMV_W::new(self)
    }
    #[doc = "Bits 16:31 - Global Acceptance Filter List Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn gaflptr(&mut self) -> GAFLPTR_W<16> {
        GAFLPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Acceptance Filter List Pointer 0 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflp0](index.html) module"]
pub struct CFDGAFLP0_SPEC;
impl crate::RegisterSpec for CFDGAFLP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflp0::R](R) reader structure"]
impl crate::Readable for CFDGAFLP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflp0::W](W) writer structure"]
impl crate::Writable for CFDGAFLP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLP0%s to value 0"]
impl crate::Resettable for CFDGAFLP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
