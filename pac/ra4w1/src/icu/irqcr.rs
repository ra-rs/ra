#[doc = "Register `IRQCR%s` reader"]
pub struct R(crate::R<IRQCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQCR%s` writer"]
pub struct W(crate::W<IRQCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQCR_SPEC>;
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
impl From<crate::W<IRQCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQMD` reader - IRQ Detection Sense Select"]
pub type IRQMD_R = crate::FieldReader<u8, IRQMD_A>;
#[doc = "IRQ Detection Sense Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRQMD_A {
    #[doc = "0: Falling edge"]
    _00 = 0,
    #[doc = "1: Rising edge"]
    _01 = 1,
    #[doc = "2: Rising and falling edges"]
    _10 = 2,
    #[doc = "3: Low level"]
    _11 = 3,
}
impl From<IRQMD_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQMD_A) -> Self {
        variant as _
    }
}
impl IRQMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQMD_A {
        match self.bits {
            0 => IRQMD_A::_00,
            1 => IRQMD_A::_01,
            2 => IRQMD_A::_10,
            3 => IRQMD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IRQMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IRQMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IRQMD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IRQMD_A::_11
    }
}
#[doc = "Field `IRQMD` writer - IRQ Detection Sense Select"]
pub type IRQMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, IRQCR_SPEC, u8, IRQMD_A, 2, O>;
impl<'a, const O: u8> IRQMD_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IRQMD_A::_00)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IRQMD_A::_01)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IRQMD_A::_10)
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IRQMD_A::_11)
    }
}
#[doc = "Field `FCLKSEL` reader - IRQ Digital Filter Sampling Clock Select"]
pub type FCLKSEL_R = crate::FieldReader<u8, FCLKSEL_A>;
#[doc = "IRQ Digital Filter Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCLKSEL_A {
    #[doc = "0: PCLKB"]
    _00 = 0,
    #[doc = "1: PCLKB/8"]
    _01 = 1,
    #[doc = "2: PCLKB/32"]
    _10 = 2,
    #[doc = "3: PCLKB/64"]
    _11 = 3,
}
impl From<FCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FCLKSEL_A) -> Self {
        variant as _
    }
}
impl FCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCLKSEL_A {
        match self.bits {
            0 => FCLKSEL_A::_00,
            1 => FCLKSEL_A::_01,
            2 => FCLKSEL_A::_10,
            3 => FCLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FCLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FCLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FCLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FCLKSEL_A::_11
    }
}
#[doc = "Field `FCLKSEL` writer - IRQ Digital Filter Sampling Clock Select"]
pub type FCLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, IRQCR_SPEC, u8, FCLKSEL_A, 2, O>;
impl<'a, const O: u8> FCLKSEL_W<'a, O> {
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FCLKSEL_A::_00)
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FCLKSEL_A::_01)
    }
    #[doc = "PCLKB/32"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FCLKSEL_A::_10)
    }
    #[doc = "PCLKB/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FCLKSEL_A::_11)
    }
}
#[doc = "Field `FLTEN` reader - IRQ Digital Filter Enable"]
pub type FLTEN_R = crate::BitReader<FLTEN_A>;
#[doc = "IRQ Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLTEN_A {
    #[doc = "0: Digital filter disabled."]
    _0 = 0,
    #[doc = "1: Digital filter enabled."]
    _1 = 1,
}
impl From<FLTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTEN_A {
        match self.bits {
            false => FLTEN_A::_0,
            true => FLTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLTEN_A::_1
    }
}
#[doc = "Field `FLTEN` writer - IRQ Digital Filter Enable"]
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRQCR_SPEC, FLTEN_A, O>;
impl<'a, const O: u8> FLTEN_W<'a, O> {
    #[doc = "Digital filter disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLTEN_A::_0)
    }
    #[doc = "Digital filter enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLTEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - IRQ Detection Sense Select"]
    #[inline(always)]
    pub fn irqmd(&self) -> IRQMD_R {
        IRQMD_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - IRQ Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn fclksel(&self) -> FCLKSEL_R {
        FCLKSEL_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - IRQ Digital Filter Enable"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - IRQ Detection Sense Select"]
    #[inline(always)]
    #[must_use]
    pub fn irqmd(&mut self) -> IRQMD_W<0> {
        IRQMD_W::new(self)
    }
    #[doc = "Bits 4:5 - IRQ Digital Filter Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fclksel(&mut self) -> FCLKSEL_W<4> {
        FCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - IRQ Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<7> {
        FLTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ Control Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqcr](index.html) module"]
pub struct IRQCR_SPEC;
impl crate::RegisterSpec for IRQCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [irqcr::R](R) reader structure"]
impl crate::Readable for IRQCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqcr::W](W) writer structure"]
impl crate::Writable for IRQCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQCR%s to value 0"]
impl crate::Resettable for IRQCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
