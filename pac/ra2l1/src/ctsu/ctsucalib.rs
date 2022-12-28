#[doc = "Register `CTSUCALIB` reader"]
pub struct R(crate::R<CTSUCALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCALIB` writer"]
pub struct W(crate::W<CTSUCALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCALIB_SPEC>;
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
impl From<crate::W<CTSUCALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSOD` reader - TS Pin Fixed Output"]
pub type TSOD_R = crate::BitReader<TSOD_A>;
#[doc = "TS Pin Fixed Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOD_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Output high or low from TS terminals (controlling by the IOC bit)"]
    _1 = 1,
}
impl From<TSOD_A> for bool {
    #[inline(always)]
    fn from(variant: TSOD_A) -> Self {
        variant as u8 != 0
    }
}
impl TSOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSOD_A {
        match self.bits {
            false => TSOD_A::_0,
            true => TSOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSOD_A::_1
    }
}
#[doc = "Field `TSOD` writer - TS Pin Fixed Output"]
pub type TSOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, TSOD_A, O>;
impl<'a, const O: u8> TSOD_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSOD_A::_0)
    }
    #[doc = "Output high or low from TS terminals (controlling by the IOC bit)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSOD_A::_1)
    }
}
#[doc = "Field `DRV` reader - Power Supply Calibration Select"]
pub type DRV_R = crate::BitReader<DRV_A>;
#[doc = "Power Supply Calibration Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRV_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Power supply calibration mode"]
    _1 = 1,
}
impl From<DRV_A> for bool {
    #[inline(always)]
    fn from(variant: DRV_A) -> Self {
        variant as u8 != 0
    }
}
impl DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRV_A {
        match self.bits {
            false => DRV_A::_0,
            true => DRV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRV_A::_1
    }
}
#[doc = "Field `DRV` writer - Power Supply Calibration Select"]
pub type DRV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, DRV_A, O>;
impl<'a, const O: u8> DRV_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRV_A::_0)
    }
    #[doc = "Power supply calibration mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRV_A::_1)
    }
}
#[doc = "Field `CLKSEL` reader - Observation Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Observation Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Not selected (L fixed output)"]
    _00 = 0,
    #[doc = "1: Measurement clock (divided by 8)"]
    _01 = 1,
    #[doc = "2: CFC clock (divided by 8)"]
    _10 = 2,
    #[doc = "3: SUCLK (divided by 8)"]
    _11 = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::_00,
            1 => CLKSEL_A::_01,
            2 => CLKSEL_A::_10,
            3 => CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKSEL_A::_11
    }
}
#[doc = "Field `CLKSEL` writer - Observation Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTSUCALIB_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "Not selected (L fixed output)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKSEL_A::_00)
    }
    #[doc = "Measurement clock (divided by 8)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKSEL_A::_01)
    }
    #[doc = "CFC clock (divided by 8)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKSEL_A::_10)
    }
    #[doc = "SUCLK (divided by 8)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKSEL_A::_11)
    }
}
#[doc = "Field `SUCLKEN` reader - SUCLK Forced Oscillation Control"]
pub type SUCLKEN_R = crate::BitReader<SUCLKEN_A>;
#[doc = "SUCLK Forced Oscillation Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUCLKEN_A {
    #[doc = "0: SUCLK oscillation only during measurement"]
    _0 = 0,
    #[doc = "1: SUCLK always oscillates"]
    _1 = 1,
}
impl From<SUCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SUCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SUCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUCLKEN_A {
        match self.bits {
            false => SUCLKEN_A::_0,
            true => SUCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUCLKEN_A::_1
    }
}
#[doc = "Field `SUCLKEN` writer - SUCLK Forced Oscillation Control"]
pub type SUCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, SUCLKEN_A, O>;
impl<'a, const O: u8> SUCLKEN_W<'a, O> {
    #[doc = "SUCLK oscillation only during measurement"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUCLKEN_A::_0)
    }
    #[doc = "SUCLK always oscillates"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUCLKEN_A::_1)
    }
}
#[doc = "Field `TSOC` reader - Switched Capacitor Operation Calibration Select Bit"]
pub type TSOC_R = crate::BitReader<TSOC_A>;
#[doc = "Switched Capacitor Operation Calibration Select Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOC_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Switched capacitor operation calibration mode"]
    _1 = 1,
}
impl From<TSOC_A> for bool {
    #[inline(always)]
    fn from(variant: TSOC_A) -> Self {
        variant as u8 != 0
    }
}
impl TSOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSOC_A {
        match self.bits {
            false => TSOC_A::_0,
            true => TSOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSOC_A::_1
    }
}
#[doc = "Field `TSOC` writer - Switched Capacitor Operation Calibration Select Bit"]
pub type TSOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, TSOC_A, O>;
impl<'a, const O: u8> TSOC_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSOC_A::_0)
    }
    #[doc = "Switched capacitor operation calibration mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSOC_A::_1)
    }
}
#[doc = "Field `CNTRDSEL` reader - Read Count Select of Sensor Counter"]
pub type CNTRDSEL_R = crate::BitReader<CNTRDSEL_A>;
#[doc = "Read Count Select of Sensor Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTRDSEL_A {
    #[doc = "0: Read once"]
    _0 = 0,
    #[doc = "1: Read twice"]
    _1 = 1,
}
impl From<CNTRDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CNTRDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTRDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTRDSEL_A {
        match self.bits {
            false => CNTRDSEL_A::_0,
            true => CNTRDSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTRDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTRDSEL_A::_1
    }
}
#[doc = "Field `CNTRDSEL` writer - Read Count Select of Sensor Counter"]
pub type CNTRDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, CNTRDSEL_A, O>;
impl<'a, const O: u8> CNTRDSEL_W<'a, O> {
    #[doc = "Read once"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTRDSEL_A::_0)
    }
    #[doc = "Read twice"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTRDSEL_A::_1)
    }
}
#[doc = "Field `IOC` reader - TS Pin Fixed Output Value Set"]
pub type IOC_R = crate::BitReader<IOC_A>;
#[doc = "TS Pin Fixed Output Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOC_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<IOC_A> for bool {
    #[inline(always)]
    fn from(variant: IOC_A) -> Self {
        variant as u8 != 0
    }
}
impl IOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOC_A {
        match self.bits {
            false => IOC_A::_0,
            true => IOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOC_A::_1
    }
}
#[doc = "Field `IOC` writer - TS Pin Fixed Output Value Set"]
pub type IOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, IOC_A, O>;
impl<'a, const O: u8> IOC_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOC_A::_0)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOC_A::_1)
    }
}
#[doc = "Field `CFCRDMD` reader - CFC Counter Read Mode Select"]
pub type CFCRDMD_R = crate::BitReader<CFCRDMD_A>;
#[doc = "CFC Counter Read Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCRDMD_A {
    #[doc = "0: Except for mutual capacitance parallel measurement mode"]
    _0 = 0,
    #[doc = "1: Mutual capacitance parallel measurement mode"]
    _1 = 1,
}
impl From<CFCRDMD_A> for bool {
    #[inline(always)]
    fn from(variant: CFCRDMD_A) -> Self {
        variant as u8 != 0
    }
}
impl CFCRDMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCRDMD_A {
        match self.bits {
            false => CFCRDMD_A::_0,
            true => CFCRDMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFCRDMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFCRDMD_A::_1
    }
}
#[doc = "Field `CFCRDMD` writer - CFC Counter Read Mode Select"]
pub type CFCRDMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, CFCRDMD_A, O>;
impl<'a, const O: u8> CFCRDMD_W<'a, O> {
    #[doc = "Except for mutual capacitance parallel measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFCRDMD_A::_0)
    }
    #[doc = "Mutual capacitance parallel measurement mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFCRDMD_A::_1)
    }
}
#[doc = "Field `DCOFF` reader - Down Converter Control"]
pub type DCOFF_R = crate::BitReader<DCOFF_A>;
#[doc = "Down Converter Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOFF_A {
    #[doc = "0: Voltage down converter operation (TSCAP voltage generation)"]
    _0 = 0,
    #[doc = "1: The voltage down converter is off"]
    _1 = 1,
}
impl From<DCOFF_A> for bool {
    #[inline(always)]
    fn from(variant: DCOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOFF_A {
        match self.bits {
            false => DCOFF_A::_0,
            true => DCOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCOFF_A::_1
    }
}
#[doc = "Field `DCOFF` writer - Down Converter Control"]
pub type DCOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, DCOFF_A, O>;
impl<'a, const O: u8> DCOFF_W<'a, O> {
    #[doc = "Voltage down converter operation (TSCAP voltage generation)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOFF_A::_0)
    }
    #[doc = "The voltage down converter is off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOFF_A::_1)
    }
}
#[doc = "Field `CFCSEL` reader - Observation CFC Clock Select"]
pub type CFCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFCSEL` writer - Observation CFC Clock Select"]
pub type CFCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUCALIB_SPEC, u8, u8, 6, O>;
#[doc = "Field `CFCMODE` reader - CFC Oscillator Calibration Mode Select"]
pub type CFCMODE_R = crate::BitReader<CFCMODE_A>;
#[doc = "CFC Oscillator Calibration Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCMODE_A {
    #[doc = "0: CFC current measurement (Capacitance measurement mode)"]
    _0 = 0,
    #[doc = "1: External current measurement for calibration"]
    _1 = 1,
}
impl From<CFCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CFCMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCMODE_A {
        match self.bits {
            false => CFCMODE_A::_0,
            true => CFCMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFCMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFCMODE_A::_1
    }
}
#[doc = "Field `CFCMODE` writer - CFC Oscillator Calibration Mode Select"]
pub type CFCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, CFCMODE_A, O>;
impl<'a, const O: u8> CFCMODE_W<'a, O> {
    #[doc = "CFC current measurement (Capacitance measurement mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFCMODE_A::_0)
    }
    #[doc = "External current measurement for calibration"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFCMODE_A::_1)
    }
}
#[doc = "Field `DACMSEL` reader - Current Offset DAC Current Matrix Calibration Select"]
pub type DACMSEL_R = crate::BitReader<DACMSEL_A>;
#[doc = "Current Offset DAC Current Matrix Calibration Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACMSEL_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Current offset DAC current Calibration mode"]
    _1 = 1,
}
impl From<DACMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DACMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DACMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACMSEL_A {
        match self.bits {
            false => DACMSEL_A::_0,
            true => DACMSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACMSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACMSEL_A::_1
    }
}
#[doc = "Field `DACMSEL` writer - Current Offset DAC Current Matrix Calibration Select"]
pub type DACMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, DACMSEL_A, O>;
impl<'a, const O: u8> DACMSEL_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACMSEL_A::_0)
    }
    #[doc = "Current offset DAC current Calibration mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACMSEL_A::_1)
    }
}
#[doc = "Field `DACCARRY` reader - Offset Current Adjustment for Calibration"]
pub type DACCARRY_R = crate::BitReader<DACCARRY_A>;
#[doc = "Offset Current Adjustment for Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACCARRY_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: All current sources can be turned on"]
    _1 = 1,
}
impl From<DACCARRY_A> for bool {
    #[inline(always)]
    fn from(variant: DACCARRY_A) -> Self {
        variant as u8 != 0
    }
}
impl DACCARRY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCARRY_A {
        match self.bits {
            false => DACCARRY_A::_0,
            true => DACCARRY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACCARRY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACCARRY_A::_1
    }
}
#[doc = "Field `DACCARRY` writer - Offset Current Adjustment for Calibration"]
pub type DACCARRY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, DACCARRY_A, O>;
impl<'a, const O: u8> DACCARRY_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACCARRY_A::_0)
    }
    #[doc = "All current sources can be turned on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACCARRY_A::_1)
    }
}
#[doc = "Field `SUMSEL` reader - Current Control Oscillator Input Current Matrix Calibration Select"]
pub type SUMSEL_R = crate::BitReader<SUMSEL_A>;
#[doc = "Current Control Oscillator Input Current Matrix Calibration Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUMSEL_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Current control oscillator input current matrix calibration mode"]
    _1 = 1,
}
impl From<SUMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SUMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SUMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUMSEL_A {
        match self.bits {
            false => SUMSEL_A::_0,
            true => SUMSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUMSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUMSEL_A::_1
    }
}
#[doc = "Field `SUMSEL` writer - Current Control Oscillator Input Current Matrix Calibration Select"]
pub type SUMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, SUMSEL_A, O>;
impl<'a, const O: u8> SUMSEL_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUMSEL_A::_0)
    }
    #[doc = "Current control oscillator input current matrix calibration mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUMSEL_A::_1)
    }
}
#[doc = "Field `SUCARRY` reader - Current Control Oscillator Input Current Adjustment for SUCLK"]
pub type SUCARRY_R = crate::BitReader<SUCARRY_A>;
#[doc = "Current Control Oscillator Input Current Adjustment for SUCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUCARRY_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: All current sources can be turned on"]
    _1 = 1,
}
impl From<SUCARRY_A> for bool {
    #[inline(always)]
    fn from(variant: SUCARRY_A) -> Self {
        variant as u8 != 0
    }
}
impl SUCARRY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUCARRY_A {
        match self.bits {
            false => SUCARRY_A::_0,
            true => SUCARRY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUCARRY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUCARRY_A::_1
    }
}
#[doc = "Field `SUCARRY` writer - Current Control Oscillator Input Current Adjustment for SUCLK"]
pub type SUCARRY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, SUCARRY_A, O>;
impl<'a, const O: u8> SUCARRY_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUCARRY_A::_0)
    }
    #[doc = "All current sources can be turned on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUCARRY_A::_1)
    }
}
#[doc = "Field `DACCLK` reader - Modulation Clock Select for Offset Current Circuits"]
pub type DACCLK_R = crate::BitReader<DACCLK_A>;
#[doc = "Modulation Clock Select for Offset Current Circuits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACCLK_A {
    #[doc = "0: Operating clock selected by CTSUCRA.CLK \\[1:0\\]"]
    _0 = 0,
    #[doc = "1: SUCLK"]
    _1 = 1,
}
impl From<DACCLK_A> for bool {
    #[inline(always)]
    fn from(variant: DACCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl DACCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCLK_A {
        match self.bits {
            false => DACCLK_A::_0,
            true => DACCLK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACCLK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACCLK_A::_1
    }
}
#[doc = "Field `DACCLK` writer - Modulation Clock Select for Offset Current Circuits"]
pub type DACCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, DACCLK_A, O>;
impl<'a, const O: u8> DACCLK_W<'a, O> {
    #[doc = "Operating clock selected by CTSUCRA.CLK \\[1:0\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACCLK_A::_0)
    }
    #[doc = "SUCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACCLK_A::_1)
    }
}
#[doc = "Field `CCOCLK` reader - Modulation Clock Select for Current Controlled Oscillator Input Current of SUCLK"]
pub type CCOCLK_R = crate::BitReader<CCOCLK_A>;
#[doc = "Modulation Clock Select for Current Controlled Oscillator Input Current of SUCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCOCLK_A {
    #[doc = "0: Operating clock selected by CTSUCRA.CLK \\[1:0\\]"]
    _0 = 0,
    #[doc = "1: SUCLK"]
    _1 = 1,
}
impl From<CCOCLK_A> for bool {
    #[inline(always)]
    fn from(variant: CCOCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl CCOCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCOCLK_A {
        match self.bits {
            false => CCOCLK_A::_0,
            true => CCOCLK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCOCLK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCOCLK_A::_1
    }
}
#[doc = "Field `CCOCLK` writer - Modulation Clock Select for Current Controlled Oscillator Input Current of SUCLK"]
pub type CCOCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, CCOCLK_A, O>;
impl<'a, const O: u8> CCOCLK_W<'a, O> {
    #[doc = "Operating clock selected by CTSUCRA.CLK \\[1:0\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCOCLK_A::_0)
    }
    #[doc = "SUCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCOCLK_A::_1)
    }
}
#[doc = "Field `CCOCALIB` reader - Calibration Selection of Current Controlled Oscillator for Measurement"]
pub type CCOCALIB_R = crate::BitReader<CCOCALIB_A>;
#[doc = "Calibration Selection of Current Controlled Oscillator for Measurement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCOCALIB_A {
    #[doc = "0: Capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Oscillator calibration mode"]
    _1 = 1,
}
impl From<CCOCALIB_A> for bool {
    #[inline(always)]
    fn from(variant: CCOCALIB_A) -> Self {
        variant as u8 != 0
    }
}
impl CCOCALIB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCOCALIB_A {
        match self.bits {
            false => CCOCALIB_A::_0,
            true => CCOCALIB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCOCALIB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCOCALIB_A::_1
    }
}
#[doc = "Field `CCOCALIB` writer - Calibration Selection of Current Controlled Oscillator for Measurement"]
pub type CCOCALIB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, CCOCALIB_A, O>;
impl<'a, const O: u8> CCOCALIB_W<'a, O> {
    #[doc = "Capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCOCALIB_A::_0)
    }
    #[doc = "Oscillator calibration mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCOCALIB_A::_1)
    }
}
#[doc = "Field `TXREV` reader - Transmit Pin Inverted Output"]
pub type TXREV_R = crate::BitReader<TXREV_A>;
#[doc = "Transmit Pin Inverted Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXREV_A {
    #[doc = "0: Normal"]
    _0 = 0,
    #[doc = "1: Invert"]
    _1 = 1,
}
impl From<TXREV_A> for bool {
    #[inline(always)]
    fn from(variant: TXREV_A) -> Self {
        variant as u8 != 0
    }
}
impl TXREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXREV_A {
        match self.bits {
            false => TXREV_A::_0,
            true => TXREV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXREV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXREV_A::_1
    }
}
#[doc = "Field `TXREV` writer - Transmit Pin Inverted Output"]
pub type TXREV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCALIB_SPEC, TXREV_A, O>;
impl<'a, const O: u8> TXREV_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXREV_A::_0)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXREV_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - TS Pin Fixed Output"]
    #[inline(always)]
    pub fn tsod(&self) -> TSOD_R {
        TSOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Supply Calibration Select"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Observation Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SUCLK Forced Oscillation Control"]
    #[inline(always)]
    pub fn suclken(&self) -> SUCLKEN_R {
        SUCLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Switched Capacitor Operation Calibration Select Bit"]
    #[inline(always)]
    pub fn tsoc(&self) -> TSOC_R {
        TSOC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read Count Select of Sensor Counter"]
    #[inline(always)]
    pub fn cntrdsel(&self) -> CNTRDSEL_R {
        CNTRDSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TS Pin Fixed Output Value Set"]
    #[inline(always)]
    pub fn ioc(&self) -> IOC_R {
        IOC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CFC Counter Read Mode Select"]
    #[inline(always)]
    pub fn cfcrdmd(&self) -> CFCRDMD_R {
        CFCRDMD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Down Converter Control"]
    #[inline(always)]
    pub fn dcoff(&self) -> DCOFF_R {
        DCOFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Observation CFC Clock Select"]
    #[inline(always)]
    pub fn cfcsel(&self) -> CFCSEL_R {
        CFCSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - CFC Oscillator Calibration Mode Select"]
    #[inline(always)]
    pub fn cfcmode(&self) -> CFCMODE_R {
        CFCMODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Current Offset DAC Current Matrix Calibration Select"]
    #[inline(always)]
    pub fn dacmsel(&self) -> DACMSEL_R {
        DACMSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Offset Current Adjustment for Calibration"]
    #[inline(always)]
    pub fn daccarry(&self) -> DACCARRY_R {
        DACCARRY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Current Control Oscillator Input Current Matrix Calibration Select"]
    #[inline(always)]
    pub fn sumsel(&self) -> SUMSEL_R {
        SUMSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Current Control Oscillator Input Current Adjustment for SUCLK"]
    #[inline(always)]
    pub fn sucarry(&self) -> SUCARRY_R {
        SUCARRY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Modulation Clock Select for Offset Current Circuits"]
    #[inline(always)]
    pub fn dacclk(&self) -> DACCLK_R {
        DACCLK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Modulation Clock Select for Current Controlled Oscillator Input Current of SUCLK"]
    #[inline(always)]
    pub fn ccoclk(&self) -> CCOCLK_R {
        CCOCLK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Calibration Selection of Current Controlled Oscillator for Measurement"]
    #[inline(always)]
    pub fn ccocalib(&self) -> CCOCALIB_R {
        CCOCALIB_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit Pin Inverted Output"]
    #[inline(always)]
    pub fn txrev(&self) -> TXREV_R {
        TXREV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TS Pin Fixed Output"]
    #[inline(always)]
    #[must_use]
    pub fn tsod(&mut self) -> TSOD_W<2> {
        TSOD_W::new(self)
    }
    #[doc = "Bit 3 - Power Supply Calibration Select"]
    #[inline(always)]
    #[must_use]
    pub fn drv(&mut self) -> DRV_W<3> {
        DRV_W::new(self)
    }
    #[doc = "Bits 4:5 - Observation Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<4> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 6 - SUCLK Forced Oscillation Control"]
    #[inline(always)]
    #[must_use]
    pub fn suclken(&mut self) -> SUCLKEN_W<6> {
        SUCLKEN_W::new(self)
    }
    #[doc = "Bit 7 - Switched Capacitor Operation Calibration Select Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tsoc(&mut self) -> TSOC_W<7> {
        TSOC_W::new(self)
    }
    #[doc = "Bit 8 - Read Count Select of Sensor Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cntrdsel(&mut self) -> CNTRDSEL_W<8> {
        CNTRDSEL_W::new(self)
    }
    #[doc = "Bit 9 - TS Pin Fixed Output Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn ioc(&mut self) -> IOC_W<9> {
        IOC_W::new(self)
    }
    #[doc = "Bit 10 - CFC Counter Read Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn cfcrdmd(&mut self) -> CFCRDMD_W<10> {
        CFCRDMD_W::new(self)
    }
    #[doc = "Bit 11 - Down Converter Control"]
    #[inline(always)]
    #[must_use]
    pub fn dcoff(&mut self) -> DCOFF_W<11> {
        DCOFF_W::new(self)
    }
    #[doc = "Bits 16:21 - Observation CFC Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cfcsel(&mut self) -> CFCSEL_W<16> {
        CFCSEL_W::new(self)
    }
    #[doc = "Bit 22 - CFC Oscillator Calibration Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn cfcmode(&mut self) -> CFCMODE_W<22> {
        CFCMODE_W::new(self)
    }
    #[doc = "Bit 24 - Current Offset DAC Current Matrix Calibration Select"]
    #[inline(always)]
    #[must_use]
    pub fn dacmsel(&mut self) -> DACMSEL_W<24> {
        DACMSEL_W::new(self)
    }
    #[doc = "Bit 25 - Offset Current Adjustment for Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn daccarry(&mut self) -> DACCARRY_W<25> {
        DACCARRY_W::new(self)
    }
    #[doc = "Bit 26 - Current Control Oscillator Input Current Matrix Calibration Select"]
    #[inline(always)]
    #[must_use]
    pub fn sumsel(&mut self) -> SUMSEL_W<26> {
        SUMSEL_W::new(self)
    }
    #[doc = "Bit 27 - Current Control Oscillator Input Current Adjustment for SUCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sucarry(&mut self) -> SUCARRY_W<27> {
        SUCARRY_W::new(self)
    }
    #[doc = "Bit 28 - Modulation Clock Select for Offset Current Circuits"]
    #[inline(always)]
    #[must_use]
    pub fn dacclk(&mut self) -> DACCLK_W<28> {
        DACCLK_W::new(self)
    }
    #[doc = "Bit 29 - Modulation Clock Select for Current Controlled Oscillator Input Current of SUCLK"]
    #[inline(always)]
    #[must_use]
    pub fn ccoclk(&mut self) -> CCOCLK_W<29> {
        CCOCLK_W::new(self)
    }
    #[doc = "Bit 30 - Calibration Selection of Current Controlled Oscillator for Measurement"]
    #[inline(always)]
    #[must_use]
    pub fn ccocalib(&mut self) -> CCOCALIB_W<30> {
        CCOCALIB_W::new(self)
    }
    #[doc = "Bit 31 - Transmit Pin Inverted Output"]
    #[inline(always)]
    #[must_use]
    pub fn txrev(&mut self) -> TXREV_W<31> {
        TXREV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucalib](index.html) module"]
pub struct CTSUCALIB_SPEC;
impl crate::RegisterSpec for CTSUCALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsucalib::R](R) reader structure"]
impl crate::Readable for CTSUCALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsucalib::W](W) writer structure"]
impl crate::Writable for CTSUCALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCALIB to value 0"]
impl crate::Resettable for CTSUCALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
