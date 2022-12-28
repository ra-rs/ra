#[doc = "Register `MOMCR` reader"]
pub struct R(crate::R<MOMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOMCR` writer"]
pub struct W(crate::W<MOMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOMCR_SPEC>;
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
impl From<crate::W<MOMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODRV` reader - Main Clock Oscillator Drive Capability 0 Switching"]
pub type MODRV_R = crate::FieldReader<u8, MODRV_A>;
#[doc = "Main Clock Oscillator Drive Capability 0 Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODRV_A {
    #[doc = "0: 20 MHz to 24 MHz"]
    _00 = 0,
    #[doc = "1: 16 MHz to 20 MHz"]
    _01 = 1,
    #[doc = "2: 8 MHz to 16 MHz"]
    _10 = 2,
    #[doc = "3: 8 MHz"]
    _11 = 3,
}
impl From<MODRV_A> for u8 {
    #[inline(always)]
    fn from(variant: MODRV_A) -> Self {
        variant as _
    }
}
impl MODRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODRV_A {
        match self.bits {
            0 => MODRV_A::_00,
            1 => MODRV_A::_01,
            2 => MODRV_A::_10,
            3 => MODRV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MODRV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MODRV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MODRV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MODRV_A::_11
    }
}
#[doc = "Field `MODRV` writer - Main Clock Oscillator Drive Capability 0 Switching"]
pub type MODRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MOMCR_SPEC, u8, MODRV_A, 2, O>;
impl<'a, const O: u8> MODRV_W<'a, O> {
    #[doc = "20 MHz to 24 MHz"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODRV_A::_00)
    }
    #[doc = "16 MHz to 20 MHz"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODRV_A::_01)
    }
    #[doc = "8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODRV_A::_10)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODRV_A::_11)
    }
}
#[doc = "Field `MOSEL` reader - Main Clock Oscillator Switching"]
pub type MOSEL_R = crate::BitReader<MOSEL_A>;
#[doc = "Main Clock Oscillator Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSEL_A {
    #[doc = "0: Resonator"]
    _0 = 0,
    #[doc = "1: External clock input"]
    _1 = 1,
}
impl From<MOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOSEL_A {
        match self.bits {
            false => MOSEL_A::_0,
            true => MOSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSEL_A::_1
    }
}
#[doc = "Field `MOSEL` writer - Main Clock Oscillator Switching"]
pub type MOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MOMCR_SPEC, MOSEL_A, O>;
impl<'a, const O: u8> MOSEL_W<'a, O> {
    #[doc = "Resonator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOSEL_A::_0)
    }
    #[doc = "External clock input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOSEL_A::_1)
    }
}
impl R {
    #[doc = "Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching"]
    #[inline(always)]
    pub fn modrv(&self) -> MODRV_R {
        MODRV_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(&self) -> MOSEL_R {
        MOSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching"]
    #[inline(always)]
    #[must_use]
    pub fn modrv(&mut self) -> MODRV_W<4> {
        MODRV_W::new(self)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    #[must_use]
    pub fn mosel(&mut self) -> MOSEL_W<6> {
        MOSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Oscillator Mode Oscillation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [momcr](index.html) module"]
pub struct MOMCR_SPEC;
impl crate::RegisterSpec for MOMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [momcr::R](R) reader structure"]
impl crate::Readable for MOMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [momcr::W](W) writer structure"]
impl crate::Writable for MOMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOMCR to value 0"]
impl crate::Resettable for MOMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
