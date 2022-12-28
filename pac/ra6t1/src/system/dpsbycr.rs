#[doc = "Register `DPSBYCR` reader"]
pub struct R(crate::R<DPSBYCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSBYCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSBYCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSBYCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSBYCR` writer"]
pub struct W(crate::W<DPSBYCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSBYCR_SPEC>;
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
impl From<crate::W<DPSBYCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSBYCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEEPCUT` reader - Power-Supply Control"]
pub type DEEPCUT_R = crate::FieldReader<u8, DEEPCUT_A>;
#[doc = "Power-Supply Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEEPCUT_A {
    #[doc = "0: Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode."]
    _00 = 0,
    #[doc = "1: Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is not supplied in deep software standby mode."]
    _01 = 1,
    #[doc = "2: Setting prohibited."]
    _10 = 2,
    #[doc = "3: Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled."]
    _11 = 3,
}
impl From<DEEPCUT_A> for u8 {
    #[inline(always)]
    fn from(variant: DEEPCUT_A) -> Self {
        variant as _
    }
}
impl DEEPCUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPCUT_A {
        match self.bits {
            0 => DEEPCUT_A::_00,
            1 => DEEPCUT_A::_01,
            2 => DEEPCUT_A::_10,
            3 => DEEPCUT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DEEPCUT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DEEPCUT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DEEPCUT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DEEPCUT_A::_11
    }
}
#[doc = "Field `DEEPCUT` writer - Power-Supply Control"]
pub type DEEPCUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DPSBYCR_SPEC, u8, DEEPCUT_A, 2, O>;
impl<'a, const O: u8> DEEPCUT_W<'a, O> {
    #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DEEPCUT_A::_00)
    }
    #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is not supplied in deep software standby mode."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DEEPCUT_A::_01)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DEEPCUT_A::_10)
    }
    #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DEEPCUT_A::_11)
    }
}
#[doc = "Field `IOKEEP` reader - I/O Port Retention"]
pub type IOKEEP_R = crate::BitReader<IOKEEP_A>;
#[doc = "I/O Port Retention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOKEEP_A {
    #[doc = "0: When the Deep Software Standby mode is canceled, the I/O ports are in the reset state."]
    _0 = 0,
    #[doc = "1: When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode."]
    _1 = 1,
}
impl From<IOKEEP_A> for bool {
    #[inline(always)]
    fn from(variant: IOKEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl IOKEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOKEEP_A {
        match self.bits {
            false => IOKEEP_A::_0,
            true => IOKEEP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOKEEP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOKEEP_A::_1
    }
}
#[doc = "Field `IOKEEP` writer - I/O Port Retention"]
pub type IOKEEP_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSBYCR_SPEC, IOKEEP_A, O>;
impl<'a, const O: u8> IOKEEP_W<'a, O> {
    #[doc = "When the Deep Software Standby mode is canceled, the I/O ports are in the reset state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOKEEP_A::_0)
    }
    #[doc = "When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOKEEP_A::_1)
    }
}
#[doc = "Field `DPSBY` reader - Deep Software Standby"]
pub type DPSBY_R = crate::BitReader<DPSBY_A>;
#[doc = "Deep Software Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSBY_A {
    #[doc = "0: Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)"]
    _0 = 0,
    #[doc = "1: Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)"]
    _1 = 1,
}
impl From<DPSBY_A> for bool {
    #[inline(always)]
    fn from(variant: DPSBY_A) -> Self {
        variant as u8 != 0
    }
}
impl DPSBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSBY_A {
        match self.bits {
            false => DPSBY_A::_0,
            true => DPSBY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSBY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSBY_A::_1
    }
}
#[doc = "Field `DPSBY` writer - Deep Software Standby"]
pub type DPSBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSBYCR_SPEC, DPSBY_A, O>;
impl<'a, const O: u8> DPSBY_W<'a, O> {
    #[doc = "Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPSBY_A::_0)
    }
    #[doc = "Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPSBY_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power-Supply Control"]
    #[inline(always)]
    pub fn deepcut(&self) -> DEEPCUT_R {
        DEEPCUT_R::new(self.bits & 3)
    }
    #[doc = "Bit 6 - I/O Port Retention"]
    #[inline(always)]
    pub fn iokeep(&self) -> IOKEEP_R {
        IOKEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Deep Software Standby"]
    #[inline(always)]
    pub fn dpsby(&self) -> DPSBY_R {
        DPSBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power-Supply Control"]
    #[inline(always)]
    #[must_use]
    pub fn deepcut(&mut self) -> DEEPCUT_W<0> {
        DEEPCUT_W::new(self)
    }
    #[doc = "Bit 6 - I/O Port Retention"]
    #[inline(always)]
    #[must_use]
    pub fn iokeep(&mut self) -> IOKEEP_W<6> {
        IOKEEP_W::new(self)
    }
    #[doc = "Bit 7 - Deep Software Standby"]
    #[inline(always)]
    #[must_use]
    pub fn dpsby(&mut self) -> DPSBY_W<7> {
        DPSBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsbycr](index.html) module"]
pub struct DPSBYCR_SPEC;
impl crate::RegisterSpec for DPSBYCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsbycr::R](R) reader structure"]
impl crate::Readable for DPSBYCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsbycr::W](W) writer structure"]
impl crate::Writable for DPSBYCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSBYCR to value 0x01"]
impl crate::Resettable for DPSBYCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
