#[doc = "Register `SPSR` reader"]
pub struct R(crate::R<SPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPSR` writer"]
pub struct W(crate::W<SPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPSR_SPEC>;
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
impl From<crate::W<SPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVRF` reader - Overrun Error Flag"]
pub type OVRF_R = crate::BitReader<OVRF_A>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRF_A {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: Overrun error occurred"]
    _1 = 1,
}
impl From<OVRF_A> for bool {
    #[inline(always)]
    fn from(variant: OVRF_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRF_A {
        match self.bits {
            false => OVRF_A::_0,
            true => OVRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRF_A::_1
    }
}
#[doc = "Field `OVRF` writer - Overrun Error Flag"]
pub type OVRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR_SPEC, OVRF_A, O>;
impl<'a, const O: u8> OVRF_W<'a, O> {
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRF_A::_0)
    }
    #[doc = "Overrun error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRF_A::_1)
    }
}
#[doc = "Field `IDLNF` reader - SPI Idle Flag"]
pub type IDLNF_R = crate::BitReader<IDLNF_A>;
#[doc = "SPI Idle Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLNF_A {
    #[doc = "0: SPI is in the idle state"]
    _0 = 0,
    #[doc = "1: SPI is in the transfer state"]
    _1 = 1,
}
impl From<IDLNF_A> for bool {
    #[inline(always)]
    fn from(variant: IDLNF_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLNF_A {
        match self.bits {
            false => IDLNF_A::_0,
            true => IDLNF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDLNF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDLNF_A::_1
    }
}
#[doc = "Field `MODF` reader - Mode Fault Error Flag"]
pub type MODF_R = crate::BitReader<MODF_A>;
#[doc = "Mode Fault Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF_A {
    #[doc = "0: No mode fault or underrun error occurred"]
    _0 = 0,
    #[doc = "1: Mode fault error or underrun error occurred"]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODF_A::_1
    }
}
#[doc = "Field `MODF` writer - Mode Fault Error Flag"]
pub type MODF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR_SPEC, MODF_A, O>;
impl<'a, const O: u8> MODF_W<'a, O> {
    #[doc = "No mode fault or underrun error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODF_A::_0)
    }
    #[doc = "Mode fault error or underrun error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODF_A::_1)
    }
}
#[doc = "Field `PERF` reader - Parity Error Flag"]
pub type PERF_R = crate::BitReader<PERF_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERF_A {
    #[doc = "0: No parity error occurred"]
    _0 = 0,
    #[doc = "1: Parity error occurred"]
    _1 = 1,
}
impl From<PERF_A> for bool {
    #[inline(always)]
    fn from(variant: PERF_A) -> Self {
        variant as u8 != 0
    }
}
impl PERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERF_A {
        match self.bits {
            false => PERF_A::_0,
            true => PERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PERF_A::_1
    }
}
#[doc = "Field `PERF` writer - Parity Error Flag"]
pub type PERF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR_SPEC, PERF_A, O>;
impl<'a, const O: u8> PERF_W<'a, O> {
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PERF_A::_0)
    }
    #[doc = "Parity error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PERF_A::_1)
    }
}
#[doc = "Field `UDRF` reader - Underrun Error Flag"]
pub type UDRF_R = crate::BitReader<UDRF_A>;
#[doc = "Underrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRF_A {
    #[doc = "0: Mode fault error occurred (MODF = 1)"]
    _0 = 0,
    #[doc = "1: Underrun error occurred (MODF = 1)"]
    _1 = 1,
}
impl From<UDRF_A> for bool {
    #[inline(always)]
    fn from(variant: UDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl UDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDRF_A {
        match self.bits {
            false => UDRF_A::_0,
            true => UDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDRF_A::_1
    }
}
#[doc = "Field `UDRF` writer - Underrun Error Flag"]
pub type UDRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR_SPEC, UDRF_A, O>;
impl<'a, const O: u8> UDRF_W<'a, O> {
    #[doc = "Mode fault error occurred (MODF = 1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UDRF_A::_0)
    }
    #[doc = "Underrun error occurred (MODF = 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UDRF_A::_1)
    }
}
#[doc = "Field `SPTEF` reader - SPI Transmit Buffer Empty Flag"]
pub type SPTEF_R = crate::BitReader<SPTEF_A>;
#[doc = "SPI Transmit Buffer Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPTEF_A {
    #[doc = "0: Data is in the transmit buffer"]
    _0 = 0,
    #[doc = "1: No data is in the transmit buffer"]
    _1 = 1,
}
impl From<SPTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SPTEF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPTEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTEF_A {
        match self.bits {
            false => SPTEF_A::_0,
            true => SPTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTEF_A::_1
    }
}
#[doc = "Field `SPTEF` writer - SPI Transmit Buffer Empty Flag"]
pub type SPTEF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR_SPEC, SPTEF_A, O>;
impl<'a, const O: u8> SPTEF_W<'a, O> {
    #[doc = "Data is in the transmit buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPTEF_A::_0)
    }
    #[doc = "No data is in the transmit buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPTEF_A::_1)
    }
}
#[doc = "Field `SPRF` reader - SPI Receive Buffer Full Flag"]
pub type SPRF_R = crate::BitReader<SPRF_A>;
#[doc = "SPI Receive Buffer Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRF_A {
    #[doc = "0: No valid data is in SPDR/SPDR_HA"]
    _0 = 0,
    #[doc = "1: Valid data is in SPDR/SPDR_HA"]
    _1 = 1,
}
impl From<SPRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPRF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRF_A {
        match self.bits {
            false => SPRF_A::_0,
            true => SPRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRF_A::_1
    }
}
#[doc = "Field `SPRF` writer - SPI Receive Buffer Full Flag"]
pub type SPRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR_SPEC, SPRF_A, O>;
impl<'a, const O: u8> SPRF_W<'a, O> {
    #[doc = "No valid data is in SPDR/SPDR_HA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRF_A::_0)
    }
    #[doc = "Valid data is in SPDR/SPDR_HA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun Error Flag"]
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Idle Flag"]
    #[inline(always)]
    pub fn idlnf(&self) -> IDLNF_R {
        IDLNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Flag"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Underrun Error Flag"]
    #[inline(always)]
    pub fn udrf(&self) -> UDRF_R {
        UDRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    pub fn sptef(&self) -> SPTEF_R {
        SPTEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn sprf(&self) -> SPRF_R {
        SPRF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovrf(&mut self) -> OVRF_W<0> {
        OVRF_W::new(self)
    }
    #[doc = "Bit 2 - Mode Fault Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> MODF_W<2> {
        MODF_W::new(self)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perf(&mut self) -> PERF_W<3> {
        PERF_W::new(self)
    }
    #[doc = "Bit 4 - Underrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn udrf(&mut self) -> UDRF_W<4> {
        UDRF_W::new(self)
    }
    #[doc = "Bit 5 - SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sptef(&mut self) -> SPTEF_W<5> {
        SPTEF_W::new(self)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sprf(&mut self) -> SPRF_W<7> {
        SPRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spsr](index.html) module"]
pub struct SPSR_SPEC;
impl crate::RegisterSpec for SPSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spsr::R](R) reader structure"]
impl crate::Readable for SPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spsr::W](W) writer structure"]
impl crate::Writable for SPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSR to value 0x20"]
impl crate::Resettable for SPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
