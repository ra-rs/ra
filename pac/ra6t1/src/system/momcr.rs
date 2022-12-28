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
#[doc = "Field `MODRV0` reader - Main Clock Oscillator Drive Capability 0 Switching"]
pub type MODRV0_R = crate::FieldReader<u8, MODRV0_A>;
#[doc = "Main Clock Oscillator Drive Capability 0 Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODRV0_A {
    #[doc = "0: 20MHz to 24MHz"]
    _00 = 0,
    #[doc = "1: 16MHz to 20MHz"]
    _01 = 1,
    #[doc = "2: 8MHz to 16MHz"]
    _10 = 2,
    #[doc = "3: 8MHz"]
    _11 = 3,
}
impl From<MODRV0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODRV0_A) -> Self {
        variant as _
    }
}
impl MODRV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODRV0_A {
        match self.bits {
            0 => MODRV0_A::_00,
            1 => MODRV0_A::_01,
            2 => MODRV0_A::_10,
            3 => MODRV0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MODRV0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MODRV0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MODRV0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MODRV0_A::_11
    }
}
#[doc = "Field `MODRV0` writer - Main Clock Oscillator Drive Capability 0 Switching"]
pub type MODRV0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MOMCR_SPEC, u8, MODRV0_A, 2, O>;
impl<'a, const O: u8> MODRV0_W<'a, O> {
    #[doc = "20MHz to 24MHz"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODRV0_A::_00)
    }
    #[doc = "16MHz to 20MHz"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODRV0_A::_01)
    }
    #[doc = "8MHz to 16MHz"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODRV0_A::_10)
    }
    #[doc = "8MHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODRV0_A::_11)
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
#[doc = "Field `AUTODRVEN` reader - Main Clock Oscillator Drive Capability Auto Switching Enable"]
pub type AUTODRVEN_R = crate::BitReader<AUTODRVEN_A>;
#[doc = "Main Clock Oscillator Drive Capability Auto Switching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTODRVEN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable."]
    _1 = 1,
}
impl From<AUTODRVEN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTODRVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTODRVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTODRVEN_A {
        match self.bits {
            false => AUTODRVEN_A::_0,
            true => AUTODRVEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AUTODRVEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AUTODRVEN_A::_1
    }
}
#[doc = "Field `AUTODRVEN` writer - Main Clock Oscillator Drive Capability Auto Switching Enable"]
pub type AUTODRVEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MOMCR_SPEC, AUTODRVEN_A, O>;
impl<'a, const O: u8> AUTODRVEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTODRVEN_A::_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTODRVEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching"]
    #[inline(always)]
    pub fn modrv0(&self) -> MODRV0_R {
        MODRV0_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(&self) -> MOSEL_R {
        MOSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Main Clock Oscillator Drive Capability Auto Switching Enable"]
    #[inline(always)]
    pub fn autodrven(&self) -> AUTODRVEN_R {
        AUTODRVEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching"]
    #[inline(always)]
    #[must_use]
    pub fn modrv0(&mut self) -> MODRV0_W<4> {
        MODRV0_W::new(self)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    #[must_use]
    pub fn mosel(&mut self) -> MOSEL_W<6> {
        MOSEL_W::new(self)
    }
    #[doc = "Bit 7 - Main Clock Oscillator Drive Capability Auto Switching Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autodrven(&mut self) -> AUTODRVEN_W<7> {
        AUTODRVEN_W::new(self)
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
