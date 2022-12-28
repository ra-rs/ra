#[doc = "Register `CTSUCRA` reader"]
pub struct R(crate::R<CTSUCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCRA` writer"]
pub struct W(crate::W<CTSUCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCRA_SPEC>;
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
impl From<crate::W<CTSUCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRT` reader - CTSU Measurement Operation Start"]
pub type STRT_R = crate::BitReader<STRT_A>;
#[doc = "CTSU Measurement Operation Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRT_A {
    #[doc = "0: Stop measurement operation"]
    _0 = 0,
    #[doc = "1: Start measurement operation"]
    _1 = 1,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRT_A {
        match self.bits {
            false => STRT_A::_0,
            true => STRT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STRT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STRT_A::_1
    }
}
#[doc = "Field `STRT` writer - CTSU Measurement Operation Start"]
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, STRT_A, O>;
impl<'a, const O: u8> STRT_W<'a, O> {
    #[doc = "Stop measurement operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STRT_A::_0)
    }
    #[doc = "Start measurement operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STRT_A::_1)
    }
}
#[doc = "Field `CAP` reader - CTSU Measurement Operation Start Trigger Select"]
pub type CAP_R = crate::BitReader<CAP_A>;
#[doc = "CTSU Measurement Operation Start Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP_A {
    #[doc = "0: Software trigger"]
    _0 = 0,
    #[doc = "1: External trigger"]
    _1 = 1,
}
impl From<CAP_A> for bool {
    #[inline(always)]
    fn from(variant: CAP_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP_A {
        match self.bits {
            false => CAP_A::_0,
            true => CAP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CAP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CAP_A::_1
    }
}
#[doc = "Field `CAP` writer - CTSU Measurement Operation Start Trigger Select"]
pub type CAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, CAP_A, O>;
impl<'a, const O: u8> CAP_W<'a, O> {
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAP_A::_0)
    }
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAP_A::_1)
    }
}
#[doc = "Field `SNZ` reader - CTSU Wait State Power-Saving Enable"]
pub type SNZ_R = crate::BitReader<SNZ_A>;
#[doc = "CTSU Wait State Power-Saving Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZ_A {
    #[doc = "0: Disable power-saving function during wait state"]
    _0 = 0,
    #[doc = "1: Enable power-saving function during wait state"]
    _1 = 1,
}
impl From<SNZ_A> for bool {
    #[inline(always)]
    fn from(variant: SNZ_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNZ_A {
        match self.bits {
            false => SNZ_A::_0,
            true => SNZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZ_A::_1
    }
}
#[doc = "Field `SNZ` writer - CTSU Wait State Power-Saving Enable"]
pub type SNZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, SNZ_A, O>;
impl<'a, const O: u8> SNZ_W<'a, O> {
    #[doc = "Disable power-saving function during wait state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNZ_A::_0)
    }
    #[doc = "Enable power-saving function during wait state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNZ_A::_1)
    }
}
#[doc = "Field `CFCON` reader - CTSU CFC Power On Control"]
pub type CFCON_R = crate::BitReader<CFCON_A>;
#[doc = "CTSU CFC Power On Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCON_A {
    #[doc = "0: CFC power off"]
    _0 = 0,
    #[doc = "1: CFC power on"]
    _1 = 1,
}
impl From<CFCON_A> for bool {
    #[inline(always)]
    fn from(variant: CFCON_A) -> Self {
        variant as u8 != 0
    }
}
impl CFCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCON_A {
        match self.bits {
            false => CFCON_A::_0,
            true => CFCON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFCON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFCON_A::_1
    }
}
#[doc = "Field `CFCON` writer - CTSU CFC Power On Control"]
pub type CFCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, CFCON_A, O>;
impl<'a, const O: u8> CFCON_W<'a, O> {
    #[doc = "CFC power off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFCON_A::_0)
    }
    #[doc = "CFC power on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFCON_A::_1)
    }
}
#[doc = "Field `INIT` writer - CTSU Control Block Initialization"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, bool, O>;
#[doc = "Field `PUMPON` reader - CTSU Boost Circuit Control"]
pub type PUMPON_R = crate::BitReader<PUMPON_A>;
#[doc = "CTSU Boost Circuit Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUMPON_A {
    #[doc = "0: Boost circuit off"]
    _0 = 0,
    #[doc = "1: Boost circuit on"]
    _1 = 1,
}
impl From<PUMPON_A> for bool {
    #[inline(always)]
    fn from(variant: PUMPON_A) -> Self {
        variant as u8 != 0
    }
}
impl PUMPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUMPON_A {
        match self.bits {
            false => PUMPON_A::_0,
            true => PUMPON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PUMPON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PUMPON_A::_1
    }
}
#[doc = "Field `PUMPON` writer - CTSU Boost Circuit Control"]
pub type PUMPON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, PUMPON_A, O>;
impl<'a, const O: u8> PUMPON_W<'a, O> {
    #[doc = "Boost circuit off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUMPON_A::_0)
    }
    #[doc = "Boost circuit on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUMPON_A::_1)
    }
}
#[doc = "Field `TXVSEL` reader - CTSU Transmission Power Supply Selection"]
pub type TXVSEL_R = crate::FieldReader<u8, TXVSEL_A>;
#[doc = "CTSU Transmission Power Supply Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXVSEL_A {
    #[doc = "0: Selecting VCC as the power supply for the transmit pins of mutual capacitance method."]
    _00 = 0,
    #[doc = "1: Selecting VCC as the power supply for the transmit pins of the mutual capacitance method. In addition, noise is reduced during GPIO operation. (Recommended)"]
    _01 = 1,
    #[doc = "2: Select VCC as the power source for the transmitter pins used as the active shield."]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<TXVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXVSEL_A) -> Self {
        variant as _
    }
}
impl TXVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXVSEL_A {
        match self.bits {
            0 => TXVSEL_A::_00,
            1 => TXVSEL_A::_01,
            2 => TXVSEL_A::_10,
            3 => TXVSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TXVSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TXVSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TXVSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TXVSEL_A::_11
    }
}
#[doc = "Field `TXVSEL` writer - CTSU Transmission Power Supply Selection"]
pub type TXVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTSUCRA_SPEC, u8, TXVSEL_A, 2, O>;
impl<'a, const O: u8> TXVSEL_W<'a, O> {
    #[doc = "Selecting VCC as the power supply for the transmit pins of mutual capacitance method."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TXVSEL_A::_00)
    }
    #[doc = "Selecting VCC as the power supply for the transmit pins of the mutual capacitance method. In addition, noise is reduced during GPIO operation. (Recommended)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TXVSEL_A::_01)
    }
    #[doc = "Select VCC as the power source for the transmitter pins used as the active shield."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TXVSEL_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TXVSEL_A::_11)
    }
}
#[doc = "Field `PON` reader - CTSU Power On Control"]
pub type PON_R = crate::BitReader<PON_A>;
#[doc = "CTSU Power On Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PON_A {
    #[doc = "0: Power off the CTSU"]
    _0 = 0,
    #[doc = "1: Power on the CTSU"]
    _1 = 1,
}
impl From<PON_A> for bool {
    #[inline(always)]
    fn from(variant: PON_A) -> Self {
        variant as u8 != 0
    }
}
impl PON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PON_A {
        match self.bits {
            false => PON_A::_0,
            true => PON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PON_A::_1
    }
}
#[doc = "Field `PON` writer - CTSU Power On Control"]
pub type PON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, PON_A, O>;
impl<'a, const O: u8> PON_W<'a, O> {
    #[doc = "Power off the CTSU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PON_A::_0)
    }
    #[doc = "Power on the CTSU"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PON_A::_1)
    }
}
#[doc = "Field `CSW` reader - TSCAP Pin Enable"]
pub type CSW_R = crate::BitReader<CSW_A>;
#[doc = "TSCAP Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSW_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<CSW_A> for bool {
    #[inline(always)]
    fn from(variant: CSW_A) -> Self {
        variant as u8 != 0
    }
}
impl CSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSW_A {
        match self.bits {
            false => CSW_A::_0,
            true => CSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSW_A::_1
    }
}
#[doc = "Field `CSW` writer - TSCAP Pin Enable"]
pub type CSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, CSW_A, O>;
impl<'a, const O: u8> CSW_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSW_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSW_A::_1)
    }
}
#[doc = "Field `ATUNE0` reader - CTSU Power Supply Operating Mode Setting"]
pub type ATUNE0_R = crate::BitReader<ATUNE0_A>;
#[doc = "CTSU Power Supply Operating Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATUNE0_A {
    #[doc = "0: VCC ≥ 2.4 V: Normal voltage operating mode VCC < 2.4 V: Setting prohibited"]
    _0 = 0,
    #[doc = "1: Low-voltage operating mode"]
    _1 = 1,
}
impl From<ATUNE0_A> for bool {
    #[inline(always)]
    fn from(variant: ATUNE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ATUNE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATUNE0_A {
        match self.bits {
            false => ATUNE0_A::_0,
            true => ATUNE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATUNE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATUNE0_A::_1
    }
}
#[doc = "Field `ATUNE0` writer - CTSU Power Supply Operating Mode Setting"]
pub type ATUNE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, ATUNE0_A, O>;
impl<'a, const O: u8> ATUNE0_W<'a, O> {
    #[doc = "VCC ≥ 2.4 V: Normal voltage operating mode VCC < 2.4 V: Setting prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATUNE0_A::_0)
    }
    #[doc = "Low-voltage operating mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATUNE0_A::_1)
    }
}
#[doc = "Field `ATUNE1` reader - CTSU Current Range Adjustment"]
pub type ATUNE1_R = crate::BitReader<ATUNE1_A>;
#[doc = "CTSU Current Range Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATUNE1_A {
    #[doc = "0: 80 µA when CTSUATUNE2 = 0 20 µA when CTSUATUNE2 = 1"]
    _0 = 0,
    #[doc = "1: 40 µA when CTSUATUNE2 = 0 160 µA when CTSUATUNE2 = 1"]
    _1 = 1,
}
impl From<ATUNE1_A> for bool {
    #[inline(always)]
    fn from(variant: ATUNE1_A) -> Self {
        variant as u8 != 0
    }
}
impl ATUNE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATUNE1_A {
        match self.bits {
            false => ATUNE1_A::_0,
            true => ATUNE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATUNE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATUNE1_A::_1
    }
}
#[doc = "Field `ATUNE1` writer - CTSU Current Range Adjustment"]
pub type ATUNE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, ATUNE1_A, O>;
impl<'a, const O: u8> ATUNE1_W<'a, O> {
    #[doc = "80 µA when CTSUATUNE2 = 0 20 µA when CTSUATUNE2 = 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATUNE1_A::_0)
    }
    #[doc = "40 µA when CTSUATUNE2 = 0 160 µA when CTSUATUNE2 = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATUNE1_A::_1)
    }
}
#[doc = "Field `CLK` reader - CTSU Operating Clock Select"]
pub type CLK_R = crate::FieldReader<u8, CLK_A>;
#[doc = "CTSU Operating Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_A {
    #[doc = "0: PCLKB"]
    _00 = 0,
    #[doc = "1: PCLKB/2 (PCLKB divided by 2)"]
    _01 = 1,
    #[doc = "2: PCLKB/4 (PCLKB divided by 4)"]
    _10 = 2,
    #[doc = "3: PCLKB/8 (PCLKB divided by 8)"]
    _11 = 3,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        variant as _
    }
}
impl CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_A {
        match self.bits {
            0 => CLK_A::_00,
            1 => CLK_A::_01,
            2 => CLK_A::_10,
            3 => CLK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLK_A::_11
    }
}
#[doc = "Field `CLK` writer - CTSU Operating Clock Select"]
pub type CLK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTSUCRA_SPEC, u8, CLK_A, 2, O>;
impl<'a, const O: u8> CLK_W<'a, O> {
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLK_A::_00)
    }
    #[doc = "PCLKB/2 (PCLKB divided by 2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLK_A::_01)
    }
    #[doc = "PCLKB/4 (PCLKB divided by 4)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLK_A::_10)
    }
    #[doc = "PCLKB/8 (PCLKB divided by 8)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLK_A::_11)
    }
}
#[doc = "Field `MD0` reader - CTSU Measurement Mode Select 0"]
pub type MD0_R = crate::BitReader<MD0_A>;
#[doc = "CTSU Measurement Mode Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MD0_A {
    #[doc = "0: Single scan mode"]
    _0 = 0,
    #[doc = "1: Multi-scan mode"]
    _1 = 1,
}
impl From<MD0_A> for bool {
    #[inline(always)]
    fn from(variant: MD0_A) -> Self {
        variant as u8 != 0
    }
}
impl MD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD0_A {
        match self.bits {
            false => MD0_A::_0,
            true => MD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MD0_A::_1
    }
}
#[doc = "Field `MD0` writer - CTSU Measurement Mode Select 0"]
pub type MD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, MD0_A, O>;
impl<'a, const O: u8> MD0_W<'a, O> {
    #[doc = "Single scan mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MD0_A::_0)
    }
    #[doc = "Multi-scan mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MD0_A::_1)
    }
}
#[doc = "Field `MD1` reader - CTSU Measurement Mode Select 1"]
pub type MD1_R = crate::BitReader<MD1_A>;
#[doc = "CTSU Measurement Mode Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MD1_A {
    #[doc = "0: One-time measurement (self-capacitance method)"]
    _0 = 0,
    #[doc = "1: Two times measurement (mutual capacitance method)"]
    _1 = 1,
}
impl From<MD1_A> for bool {
    #[inline(always)]
    fn from(variant: MD1_A) -> Self {
        variant as u8 != 0
    }
}
impl MD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD1_A {
        match self.bits {
            false => MD1_A::_0,
            true => MD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MD1_A::_1
    }
}
#[doc = "Field `MD1` writer - CTSU Measurement Mode Select 1"]
pub type MD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, MD1_A, O>;
impl<'a, const O: u8> MD1_W<'a, O> {
    #[doc = "One-time measurement (self-capacitance method)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MD1_A::_0)
    }
    #[doc = "Two times measurement (mutual capacitance method)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MD1_A::_1)
    }
}
#[doc = "Field `MD2` reader - CTSU Measurement Mode Select 2"]
pub type MD2_R = crate::BitReader<MD2_A>;
#[doc = "CTSU Measurement Mode Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MD2_A {
    #[doc = "0: Measure the switched capacitor current and the DC current"]
    _0 = 0,
    #[doc = "1: Measure the charge transfer by CFC circuit (parallel measurement)"]
    _1 = 1,
}
impl From<MD2_A> for bool {
    #[inline(always)]
    fn from(variant: MD2_A) -> Self {
        variant as u8 != 0
    }
}
impl MD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD2_A {
        match self.bits {
            false => MD2_A::_0,
            true => MD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MD2_A::_1
    }
}
#[doc = "Field `MD2` writer - CTSU Measurement Mode Select 2"]
pub type MD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, MD2_A, O>;
impl<'a, const O: u8> MD2_W<'a, O> {
    #[doc = "Measure the switched capacitor current and the DC current"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MD2_A::_0)
    }
    #[doc = "Measure the charge transfer by CFC circuit (parallel measurement)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MD2_A::_1)
    }
}
#[doc = "Field `ATUNE2` reader - CTSU Current Range Adjustment"]
pub type ATUNE2_R = crate::BitReader<ATUNE2_A>;
#[doc = "CTSU Current Range Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATUNE2_A {
    #[doc = "0: 80 µA when CTSUATUNE1 = 0 40 µA when CTSUATUNE1 = 1"]
    _0 = 0,
    #[doc = "1: 20 µA when CTSUATUNE1 = 0 160 µA when CTSUATUNE1 = 1"]
    _1 = 1,
}
impl From<ATUNE2_A> for bool {
    #[inline(always)]
    fn from(variant: ATUNE2_A) -> Self {
        variant as u8 != 0
    }
}
impl ATUNE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATUNE2_A {
        match self.bits {
            false => ATUNE2_A::_0,
            true => ATUNE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATUNE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATUNE2_A::_1
    }
}
#[doc = "Field `ATUNE2` writer - CTSU Current Range Adjustment"]
pub type ATUNE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, ATUNE2_A, O>;
impl<'a, const O: u8> ATUNE2_W<'a, O> {
    #[doc = "80 µA when CTSUATUNE1 = 0 40 µA when CTSUATUNE1 = 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATUNE2_A::_0)
    }
    #[doc = "20 µA when CTSUATUNE1 = 0 160 µA when CTSUATUNE1 = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATUNE2_A::_1)
    }
}
#[doc = "Field `LOAD` reader - CTSU Load Control During Measurement"]
pub type LOAD_R = crate::FieldReader<u8, LOAD_A>;
#[doc = "CTSU Load Control During Measurement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOAD_A {
    #[doc = "0: 2.5 µA constant current load"]
    _00 = 0,
    #[doc = "1: No load"]
    _01 = 1,
    #[doc = "2: 20 µA constant current load and overcurrent detector disabled"]
    _10 = 2,
    #[doc = "3: Resistance load for calibration. To set LOAD\\[1:0\\]
bits to resistance load for calibration, set these bits to 10b before they are set to 11b."]
    _11 = 3,
}
impl From<LOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: LOAD_A) -> Self {
        variant as _
    }
}
impl LOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOAD_A {
        match self.bits {
            0 => LOAD_A::_00,
            1 => LOAD_A::_01,
            2 => LOAD_A::_10,
            3 => LOAD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LOAD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LOAD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LOAD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LOAD_A::_11
    }
}
#[doc = "Field `LOAD` writer - CTSU Load Control During Measurement"]
pub type LOAD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTSUCRA_SPEC, u8, LOAD_A, 2, O>;
impl<'a, const O: u8> LOAD_W<'a, O> {
    #[doc = "2.5 µA constant current load"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LOAD_A::_00)
    }
    #[doc = "No load"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LOAD_A::_01)
    }
    #[doc = "20 µA constant current load and overcurrent detector disabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LOAD_A::_10)
    }
    #[doc = "Resistance load for calibration. To set LOAD\\[1:0\\]
bits to resistance load for calibration, set these bits to 10b before they are set to 11b."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LOAD_A::_11)
    }
}
#[doc = "Field `POSEL` reader - CTSU Non-Measured Channel Output Select"]
pub type POSEL_R = crate::FieldReader<u8, POSEL_A>;
#[doc = "CTSU Non-Measured Channel Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POSEL_A {
    #[doc = "0: Output low"]
    _00 = 0,
    #[doc = "1: Hi-Z"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: Output a pulse in phase with the transmit channel"]
    _11 = 3,
}
impl From<POSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POSEL_A) -> Self {
        variant as _
    }
}
impl POSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSEL_A {
        match self.bits {
            0 => POSEL_A::_00,
            1 => POSEL_A::_01,
            2 => POSEL_A::_10,
            3 => POSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == POSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == POSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == POSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == POSEL_A::_11
    }
}
#[doc = "Field `POSEL` writer - CTSU Non-Measured Channel Output Select"]
pub type POSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTSUCRA_SPEC, u8, POSEL_A, 2, O>;
impl<'a, const O: u8> POSEL_W<'a, O> {
    #[doc = "Output low"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(POSEL_A::_00)
    }
    #[doc = "Hi-Z"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(POSEL_A::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(POSEL_A::_10)
    }
    #[doc = "Output a pulse in phase with the transmit channel"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(POSEL_A::_11)
    }
}
#[doc = "Field `SDPSEL` reader - CTSU Sensor Drive Pulse Select"]
pub type SDPSEL_R = crate::BitReader<SDPSEL_A>;
#[doc = "CTSU Sensor Drive Pulse Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDPSEL_A {
    #[doc = "0: Random pulse"]
    _0 = 0,
    #[doc = "1: Normal pulse using the sensor unit clock"]
    _1 = 1,
}
impl From<SDPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDPSEL_A {
        match self.bits {
            false => SDPSEL_A::_0,
            true => SDPSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDPSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDPSEL_A::_1
    }
}
#[doc = "Field `SDPSEL` writer - CTSU Sensor Drive Pulse Select"]
pub type SDPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, SDPSEL_A, O>;
impl<'a, const O: u8> SDPSEL_W<'a, O> {
    #[doc = "Random pulse"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDPSEL_A::_0)
    }
    #[doc = "Normal pulse using the sensor unit clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDPSEL_A::_1)
    }
}
#[doc = "Field `PCSEL` reader - CTSU Boost Circuit Clock Select"]
pub type PCSEL_R = crate::BitReader<PCSEL_A>;
#[doc = "CTSU Boost Circuit Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSEL_A {
    #[doc = "0: Sensor drive pulse divided by 2"]
    _0 = 0,
    #[doc = "1: STCLK"]
    _1 = 1,
}
impl From<PCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSEL_A {
        match self.bits {
            false => PCSEL_A::_0,
            true => PCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSEL_A::_1
    }
}
#[doc = "Field `PCSEL` writer - CTSU Boost Circuit Clock Select"]
pub type PCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, PCSEL_A, O>;
impl<'a, const O: u8> PCSEL_W<'a, O> {
    #[doc = "Sensor drive pulse divided by 2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSEL_A::_0)
    }
    #[doc = "STCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSEL_A::_1)
    }
}
#[doc = "Field `STCLK` reader - CTSU STCLK Select"]
pub type STCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STCLK` writer - CTSU STCLK Select"]
pub type STCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUCRA_SPEC, u8, u8, 6, O>;
#[doc = "Field `DCMODE` reader - CTSU Current Measurement Mode Select"]
pub type DCMODE_R = crate::BitReader<DCMODE_A>;
#[doc = "CTSU Current Measurement Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMODE_A {
    #[doc = "0: Electrostatic capacitance measurement mode"]
    _0 = 0,
    #[doc = "1: Current measurement mode"]
    _1 = 1,
}
impl From<DCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DCMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMODE_A {
        match self.bits {
            false => DCMODE_A::_0,
            true => DCMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCMODE_A::_1
    }
}
#[doc = "Field `DCMODE` writer - CTSU Current Measurement Mode Select"]
pub type DCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, DCMODE_A, O>;
impl<'a, const O: u8> DCMODE_W<'a, O> {
    #[doc = "Electrostatic capacitance measurement mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCMODE_A::_0)
    }
    #[doc = "Current measurement mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCMODE_A::_1)
    }
}
#[doc = "Field `DCBACK` reader - CTSU Current Measurement Feedback Select"]
pub type DCBACK_R = crate::BitReader<DCBACK_A>;
#[doc = "CTSU Current Measurement Feedback Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCBACK_A {
    #[doc = "0: TSCAP pin is selected"]
    _0 = 0,
    #[doc = "1: Measurement pin is selected. It is recommended in the current measurement mode."]
    _1 = 1,
}
impl From<DCBACK_A> for bool {
    #[inline(always)]
    fn from(variant: DCBACK_A) -> Self {
        variant as u8 != 0
    }
}
impl DCBACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCBACK_A {
        match self.bits {
            false => DCBACK_A::_0,
            true => DCBACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCBACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCBACK_A::_1
    }
}
#[doc = "Field `DCBACK` writer - CTSU Current Measurement Feedback Select"]
pub type DCBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCRA_SPEC, DCBACK_A, O>;
impl<'a, const O: u8> DCBACK_W<'a, O> {
    #[doc = "TSCAP pin is selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCBACK_A::_0)
    }
    #[doc = "Measurement pin is selected. It is recommended in the current measurement mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCBACK_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    pub fn snz(&self) -> SNZ_R {
        SNZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTSU CFC Power On Control"]
    #[inline(always)]
    pub fn cfcon(&self) -> CFCON_R {
        CFCON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CTSU Boost Circuit Control"]
    #[inline(always)]
    pub fn pumpon(&self) -> PUMPON_R {
        PUMPON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CTSU Transmission Power Supply Selection"]
    #[inline(always)]
    pub fn txvsel(&self) -> TXVSEL_R {
        TXVSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CTSU Power On Control"]
    #[inline(always)]
    pub fn pon(&self) -> PON_R {
        PON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCAP Pin Enable"]
    #[inline(always)]
    pub fn csw(&self) -> CSW_R {
        CSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSU Power Supply Operating Mode Setting"]
    #[inline(always)]
    pub fn atune0(&self) -> ATUNE0_R {
        ATUNE0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTSU Current Range Adjustment"]
    #[inline(always)]
    pub fn atune1(&self) -> ATUNE1_R {
        ATUNE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CTSU Operating Clock Select"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - CTSU Measurement Mode Select 0"]
    #[inline(always)]
    pub fn md0(&self) -> MD0_R {
        MD0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTSU Measurement Mode Select 1"]
    #[inline(always)]
    pub fn md1(&self) -> MD1_R {
        MD1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CTSU Measurement Mode Select 2"]
    #[inline(always)]
    pub fn md2(&self) -> MD2_R {
        MD2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CTSU Current Range Adjustment"]
    #[inline(always)]
    pub fn atune2(&self) -> ATUNE2_R {
        ATUNE2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - CTSU Load Control During Measurement"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - CTSU Non-Measured Channel Output Select"]
    #[inline(always)]
    pub fn posel(&self) -> POSEL_R {
        POSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - CTSU Sensor Drive Pulse Select"]
    #[inline(always)]
    pub fn sdpsel(&self) -> SDPSEL_R {
        SDPSEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CTSU Boost Circuit Clock Select"]
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - CTSU STCLK Select"]
    #[inline(always)]
    pub fn stclk(&self) -> STCLK_R {
        STCLK_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - CTSU Current Measurement Mode Select"]
    #[inline(always)]
    pub fn dcmode(&self) -> DCMODE_R {
        DCMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CTSU Current Measurement Feedback Select"]
    #[inline(always)]
    pub fn dcback(&self) -> DCBACK_R {
        DCBACK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<0> {
        STRT_W::new(self)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CAP_W<1> {
        CAP_W::new(self)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    #[must_use]
    pub fn snz(&mut self) -> SNZ_W<2> {
        SNZ_W::new(self)
    }
    #[doc = "Bit 3 - CTSU CFC Power On Control"]
    #[inline(always)]
    #[must_use]
    pub fn cfcon(&mut self) -> CFCON_W<3> {
        CFCON_W::new(self)
    }
    #[doc = "Bit 4 - CTSU Control Block Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<4> {
        INIT_W::new(self)
    }
    #[doc = "Bit 5 - CTSU Boost Circuit Control"]
    #[inline(always)]
    #[must_use]
    pub fn pumpon(&mut self) -> PUMPON_W<5> {
        PUMPON_W::new(self)
    }
    #[doc = "Bits 6:7 - CTSU Transmission Power Supply Selection"]
    #[inline(always)]
    #[must_use]
    pub fn txvsel(&mut self) -> TXVSEL_W<6> {
        TXVSEL_W::new(self)
    }
    #[doc = "Bit 8 - CTSU Power On Control"]
    #[inline(always)]
    #[must_use]
    pub fn pon(&mut self) -> PON_W<8> {
        PON_W::new(self)
    }
    #[doc = "Bit 9 - TSCAP Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csw(&mut self) -> CSW_W<9> {
        CSW_W::new(self)
    }
    #[doc = "Bit 10 - CTSU Power Supply Operating Mode Setting"]
    #[inline(always)]
    #[must_use]
    pub fn atune0(&mut self) -> ATUNE0_W<10> {
        ATUNE0_W::new(self)
    }
    #[doc = "Bit 11 - CTSU Current Range Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn atune1(&mut self) -> ATUNE1_W<11> {
        ATUNE1_W::new(self)
    }
    #[doc = "Bits 12:13 - CTSU Operating Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<12> {
        CLK_W::new(self)
    }
    #[doc = "Bit 14 - CTSU Measurement Mode Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn md0(&mut self) -> MD0_W<14> {
        MD0_W::new(self)
    }
    #[doc = "Bit 15 - CTSU Measurement Mode Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn md1(&mut self) -> MD1_W<15> {
        MD1_W::new(self)
    }
    #[doc = "Bit 16 - CTSU Measurement Mode Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn md2(&mut self) -> MD2_W<16> {
        MD2_W::new(self)
    }
    #[doc = "Bit 17 - CTSU Current Range Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn atune2(&mut self) -> ATUNE2_W<17> {
        ATUNE2_W::new(self)
    }
    #[doc = "Bits 18:19 - CTSU Load Control During Measurement"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<18> {
        LOAD_W::new(self)
    }
    #[doc = "Bits 20:21 - CTSU Non-Measured Channel Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn posel(&mut self) -> POSEL_W<20> {
        POSEL_W::new(self)
    }
    #[doc = "Bit 22 - CTSU Sensor Drive Pulse Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdpsel(&mut self) -> SDPSEL_W<22> {
        SDPSEL_W::new(self)
    }
    #[doc = "Bit 23 - CTSU Boost Circuit Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel(&mut self) -> PCSEL_W<23> {
        PCSEL_W::new(self)
    }
    #[doc = "Bits 24:29 - CTSU STCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn stclk(&mut self) -> STCLK_W<24> {
        STCLK_W::new(self)
    }
    #[doc = "Bit 30 - CTSU Current Measurement Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dcmode(&mut self) -> DCMODE_W<30> {
        DCMODE_W::new(self)
    }
    #[doc = "Bit 31 - CTSU Current Measurement Feedback Select"]
    #[inline(always)]
    #[must_use]
    pub fn dcback(&mut self) -> DCBACK_W<31> {
        DCBACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucra](index.html) module"]
pub struct CTSUCRA_SPEC;
impl crate::RegisterSpec for CTSUCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsucra::R](R) reader structure"]
impl crate::Readable for CTSUCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsucra::W](W) writer structure"]
impl crate::Writable for CTSUCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCRA to value 0"]
impl crate::Resettable for CTSUCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
