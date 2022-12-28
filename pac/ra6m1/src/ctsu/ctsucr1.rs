#[doc = "Register `CTSUCR1` reader"]
pub struct R(crate::R<CTSUCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCR1` writer"]
pub struct W(crate::W<CTSUCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCR1_SPEC>;
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
impl From<crate::W<CTSUCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUPON` reader - CTSU Power Supply Enable"]
pub type CTSUPON_R = crate::BitReader<CTSUPON_A>;
#[doc = "CTSU Power Supply Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUPON_A {
    #[doc = "0: Powered off the CTSU"]
    _0 = 0,
    #[doc = "1: Powered on the CTSU"]
    _1 = 1,
}
impl From<CTSUPON_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUPON_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUPON_A {
        match self.bits {
            false => CTSUPON_A::_0,
            true => CTSUPON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUPON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUPON_A::_1
    }
}
#[doc = "Field `CTSUPON` writer - CTSU Power Supply Enable"]
pub type CTSUPON_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR1_SPEC, CTSUPON_A, O>;
impl<'a, const O: u8> CTSUPON_W<'a, O> {
    #[doc = "Powered off the CTSU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUPON_A::_0)
    }
    #[doc = "Powered on the CTSU"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUPON_A::_1)
    }
}
#[doc = "Field `CTSUCSW` reader - CTSU LPF Capacitance Charging Control"]
pub type CTSUCSW_R = crate::BitReader<CTSUCSW_A>;
#[doc = "CTSU LPF Capacitance Charging Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUCSW_A {
    #[doc = "0: Turned off capacitance switch"]
    _0 = 0,
    #[doc = "1: Turned on capacitance switch"]
    _1 = 1,
}
impl From<CTSUCSW_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUCSW_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUCSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUCSW_A {
        match self.bits {
            false => CTSUCSW_A::_0,
            true => CTSUCSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCSW_A::_1
    }
}
#[doc = "Field `CTSUCSW` writer - CTSU LPF Capacitance Charging Control"]
pub type CTSUCSW_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR1_SPEC, CTSUCSW_A, O>;
impl<'a, const O: u8> CTSUCSW_W<'a, O> {
    #[doc = "Turned off capacitance switch"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUCSW_A::_0)
    }
    #[doc = "Turned on capacitance switch"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUCSW_A::_1)
    }
}
#[doc = "Field `CTSUATUNE1` reader - CTSU Power Supply Capacity Adjustment"]
pub type CTSUATUNE1_R = crate::BitReader<CTSUATUNE1_A>;
#[doc = "CTSU Power Supply Capacity Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUATUNE1_A {
    #[doc = "0: Normal output"]
    _0 = 0,
    #[doc = "1: High-current output"]
    _1 = 1,
}
impl From<CTSUATUNE1_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUATUNE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUATUNE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUATUNE1_A {
        match self.bits {
            false => CTSUATUNE1_A::_0,
            true => CTSUATUNE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUATUNE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUATUNE1_A::_1
    }
}
#[doc = "Field `CTSUATUNE1` writer - CTSU Power Supply Capacity Adjustment"]
pub type CTSUATUNE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR1_SPEC, CTSUATUNE1_A, O>;
impl<'a, const O: u8> CTSUATUNE1_W<'a, O> {
    #[doc = "Normal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUATUNE1_A::_0)
    }
    #[doc = "High-current output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUATUNE1_A::_1)
    }
}
#[doc = "Field `CTSUCLK` reader - CTSU Operating Clock Select"]
pub type CTSUCLK_R = crate::FieldReader<u8, CTSUCLK_A>;
#[doc = "CTSU Operating Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCLK_A {
    #[doc = "0: PCLK"]
    _00 = 0,
    #[doc = "1: PCLK/2 (PCLK divided by 2)"]
    _01 = 1,
    #[doc = "2: PCLK/2 (PCLK divided by 4)"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<CTSUCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCLK_A) -> Self {
        variant as _
    }
}
impl CTSUCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUCLK_A {
        match self.bits {
            0 => CTSUCLK_A::_00,
            1 => CTSUCLK_A::_01,
            2 => CTSUCLK_A::_10,
            3 => CTSUCLK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUCLK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUCLK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUCLK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUCLK_A::_11
    }
}
#[doc = "Field `CTSUCLK` writer - CTSU Operating Clock Select"]
pub type CTSUCLK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTSUCR1_SPEC, u8, CTSUCLK_A, 2, O>;
impl<'a, const O: u8> CTSUCLK_W<'a, O> {
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CTSUCLK_A::_00)
    }
    #[doc = "PCLK/2 (PCLK divided by 2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CTSUCLK_A::_01)
    }
    #[doc = "PCLK/2 (PCLK divided by 4)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CTSUCLK_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CTSUCLK_A::_11)
    }
}
#[doc = "Field `CTSUMD` reader - CTSU Measurement Mode Select"]
pub type CTSUMD_R = crate::FieldReader<u8, CTSUMD_A>;
#[doc = "CTSU Measurement Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUMD_A {
    #[doc = "0: Self-capacitance single scan mode"]
    _00 = 0,
    #[doc = "1: Self-capacitance multi-scan mode"]
    _01 = 1,
    #[doc = "2: Mutual capacitance simple scan mode"]
    _10 = 2,
    #[doc = "3: Mutual capacitance full scan mode"]
    _11 = 3,
}
impl From<CTSUMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUMD_A) -> Self {
        variant as _
    }
}
impl CTSUMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUMD_A {
        match self.bits {
            0 => CTSUMD_A::_00,
            1 => CTSUMD_A::_01,
            2 => CTSUMD_A::_10,
            3 => CTSUMD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUMD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUMD_A::_11
    }
}
#[doc = "Field `CTSUMD` writer - CTSU Measurement Mode Select"]
pub type CTSUMD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTSUCR1_SPEC, u8, CTSUMD_A, 2, O>;
impl<'a, const O: u8> CTSUMD_W<'a, O> {
    #[doc = "Self-capacitance single scan mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CTSUMD_A::_00)
    }
    #[doc = "Self-capacitance multi-scan mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CTSUMD_A::_01)
    }
    #[doc = "Mutual capacitance simple scan mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CTSUMD_A::_10)
    }
    #[doc = "Mutual capacitance full scan mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CTSUMD_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Power Supply Enable"]
    #[inline(always)]
    pub fn ctsupon(&self) -> CTSUPON_R {
        CTSUPON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU LPF Capacitance Charging Control"]
    #[inline(always)]
    pub fn ctsucsw(&self) -> CTSUCSW_R {
        CTSUCSW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - CTSU Power Supply Capacity Adjustment"]
    #[inline(always)]
    pub fn ctsuatune1(&self) -> CTSUATUNE1_R {
        CTSUATUNE1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - CTSU Operating Clock Select"]
    #[inline(always)]
    pub fn ctsuclk(&self) -> CTSUCLK_R {
        CTSUCLK_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - CTSU Measurement Mode Select"]
    #[inline(always)]
    pub fn ctsumd(&self) -> CTSUMD_R {
        CTSUMD_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Power Supply Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsupon(&mut self) -> CTSUPON_W<0> {
        CTSUPON_W::new(self)
    }
    #[doc = "Bit 1 - CTSU LPF Capacitance Charging Control"]
    #[inline(always)]
    #[must_use]
    pub fn ctsucsw(&mut self) -> CTSUCSW_W<1> {
        CTSUCSW_W::new(self)
    }
    #[doc = "Bit 3 - CTSU Power Supply Capacity Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuatune1(&mut self) -> CTSUATUNE1_W<3> {
        CTSUATUNE1_W::new(self)
    }
    #[doc = "Bits 4:5 - CTSU Operating Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuclk(&mut self) -> CTSUCLK_W<4> {
        CTSUCLK_W::new(self)
    }
    #[doc = "Bits 6:7 - CTSU Measurement Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ctsumd(&mut self) -> CTSUMD_W<6> {
        CTSUMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucr1](index.html) module"]
pub struct CTSUCR1_SPEC;
impl crate::RegisterSpec for CTSUCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsucr1::R](R) reader structure"]
impl crate::Readable for CTSUCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsucr1::W](W) writer structure"]
impl crate::Writable for CTSUCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCR1 to value 0"]
impl crate::Resettable for CTSUCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
