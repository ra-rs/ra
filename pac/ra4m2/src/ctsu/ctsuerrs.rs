#[doc = "Register `CTSUERRS` reader"]
pub struct R(crate::R<CTSUERRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUERRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUERRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUERRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUERRS` writer"]
pub struct W(crate::W<CTSUERRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUERRS_SPEC>;
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
impl From<crate::W<CTSUERRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUERRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUSPMD` reader - Calibration Mode"]
pub type CTSUSPMD_R = crate::FieldReader<u8, CTSUSPMD_A>;
#[doc = "Calibration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUSPMD_A {
    #[doc = "0: Capacitance measurement mode"]
    _00 = 0,
    #[doc = "2: Calibration mode"]
    _10 = 2,
}
impl From<CTSUSPMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUSPMD_A) -> Self {
        variant as _
    }
}
impl CTSUSPMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSUSPMD_A> {
        match self.bits {
            0 => Some(CTSUSPMD_A::_00),
            2 => Some(CTSUSPMD_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUSPMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUSPMD_A::_10
    }
}
#[doc = "Field `CTSUSPMD` writer - Calibration Mode"]
pub type CTSUSPMD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CTSUERRS_SPEC, u8, CTSUSPMD_A, 2, O>;
impl<'a, const O: u8> CTSUSPMD_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CTSUSPMD_A::_00)
    }
    #[doc = "Calibration mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CTSUSPMD_A::_10)
    }
}
#[doc = "Field `CTSUTSOD` reader - TS Pin Fixed Output"]
pub type CTSUTSOD_R = crate::BitReader<CTSUTSOD_A>;
#[doc = "TS Pin Fixed Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUTSOD_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: TS pins are forced to be high or low"]
    _1 = 1,
}
impl From<CTSUTSOD_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUTSOD_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUTSOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUTSOD_A {
        match self.bits {
            false => CTSUTSOD_A::_0,
            true => CTSUTSOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUTSOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUTSOD_A::_1
    }
}
#[doc = "Field `CTSUTSOD` writer - TS Pin Fixed Output"]
pub type CTSUTSOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTSUERRS_SPEC, CTSUTSOD_A, O>;
impl<'a, const O: u8> CTSUTSOD_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUTSOD_A::_0)
    }
    #[doc = "TS pins are forced to be high or low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUTSOD_A::_1)
    }
}
#[doc = "Field `CTSUDRV` reader - Calibration Setting 1"]
pub type CTSUDRV_R = crate::BitReader<CTSUDRV_A>;
#[doc = "Calibration Setting 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUDRV_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Calibration setting 1"]
    _1 = 1,
}
impl From<CTSUDRV_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUDRV_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUDRV_A {
        match self.bits {
            false => CTSUDRV_A::_0,
            true => CTSUDRV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUDRV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUDRV_A::_1
    }
}
#[doc = "Field `CTSUDRV` writer - Calibration Setting 1"]
pub type CTSUDRV_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTSUERRS_SPEC, CTSUDRV_A, O>;
impl<'a, const O: u8> CTSUDRV_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUDRV_A::_0)
    }
    #[doc = "Calibration setting 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUDRV_A::_1)
    }
}
#[doc = "Field `CTSUTSOC` reader - Calibration Setting 2"]
pub type CTSUTSOC_R = crate::BitReader<CTSUTSOC_A>;
#[doc = "Calibration Setting 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUTSOC_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Calibration setting 2"]
    _1 = 1,
}
impl From<CTSUTSOC_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUTSOC_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUTSOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUTSOC_A {
        match self.bits {
            false => CTSUTSOC_A::_0,
            true => CTSUTSOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUTSOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUTSOC_A::_1
    }
}
#[doc = "Field `CTSUTSOC` writer - Calibration Setting 2"]
pub type CTSUTSOC_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTSUERRS_SPEC, CTSUTSOC_A, O>;
impl<'a, const O: u8> CTSUTSOC_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUTSOC_A::_0)
    }
    #[doc = "Calibration setting 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUTSOC_A::_1)
    }
}
#[doc = "Field `CTSUICOMP` reader - TSCAP Voltage Error Monitor"]
pub type CTSUICOMP_R = crate::BitReader<CTSUICOMP_A>;
#[doc = "TSCAP Voltage Error Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUICOMP_A {
    #[doc = "0: Normal TSCAP voltage"]
    _0 = 0,
    #[doc = "1: Abnormal TSCAP voltage"]
    _1 = 1,
}
impl From<CTSUICOMP_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUICOMP_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUICOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUICOMP_A {
        match self.bits {
            false => CTSUICOMP_A::_0,
            true => CTSUICOMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUICOMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUICOMP_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Calibration Mode"]
    #[inline(always)]
    pub fn ctsuspmd(&self) -> CTSUSPMD_R {
        CTSUSPMD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TS Pin Fixed Output"]
    #[inline(always)]
    pub fn ctsutsod(&self) -> CTSUTSOD_R {
        CTSUTSOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Calibration Setting 1"]
    #[inline(always)]
    pub fn ctsudrv(&self) -> CTSUDRV_R {
        CTSUDRV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration Setting 2"]
    #[inline(always)]
    pub fn ctsutsoc(&self) -> CTSUTSOC_R {
        CTSUTSOC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - TSCAP Voltage Error Monitor"]
    #[inline(always)]
    pub fn ctsuicomp(&self) -> CTSUICOMP_R {
        CTSUICOMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Calibration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuspmd(&mut self) -> CTSUSPMD_W<0> {
        CTSUSPMD_W::new(self)
    }
    #[doc = "Bit 2 - TS Pin Fixed Output"]
    #[inline(always)]
    #[must_use]
    pub fn ctsutsod(&mut self) -> CTSUTSOD_W<2> {
        CTSUTSOD_W::new(self)
    }
    #[doc = "Bit 3 - Calibration Setting 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctsudrv(&mut self) -> CTSUDRV_W<3> {
        CTSUDRV_W::new(self)
    }
    #[doc = "Bit 7 - Calibration Setting 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctsutsoc(&mut self) -> CTSUTSOC_W<7> {
        CTSUTSOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuerrs](index.html) module"]
pub struct CTSUERRS_SPEC;
impl crate::RegisterSpec for CTSUERRS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctsuerrs::R](R) reader structure"]
impl crate::Readable for CTSUERRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuerrs::W](W) writer structure"]
impl crate::Writable for CTSUERRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUERRS to value 0"]
impl crate::Resettable for CTSUERRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
