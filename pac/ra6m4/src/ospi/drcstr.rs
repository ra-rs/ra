#[doc = "Register `DRCSTR` reader"]
pub struct R(crate::R<DRCSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRCSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRCSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRCSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRCSTR` writer"]
pub struct W(crate::W<DRCSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRCSTR_SPEC>;
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
impl From<crate::W<DRCSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRCSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRW0` reader - Device 0 single continuous read waiting cycle setting in PCLKA units"]
pub type CTRW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRW0` writer - Device 0 single continuous read waiting cycle setting in PCLKA units"]
pub type CTRW0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRCSTR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CTR0` reader - Device 0 single continuous read mode setting"]
pub type CTR0_R = crate::BitReader<CTR0_A>;
#[doc = "Device 0 single continuous read mode setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTR0_A {
    #[doc = "0: Single continuous read mode is disabled for device 0."]
    _0 = 0,
    #[doc = "1: Single continuous read mode is enabled for device 0."]
    _1 = 1,
}
impl From<CTR0_A> for bool {
    #[inline(always)]
    fn from(variant: CTR0_A) -> Self {
        variant as u8 != 0
    }
}
impl CTR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTR0_A {
        match self.bits {
            false => CTR0_A::_0,
            true => CTR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTR0_A::_1
    }
}
#[doc = "Field `CTR0` writer - Device 0 single continuous read mode setting"]
pub type CTR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRCSTR_SPEC, CTR0_A, O>;
impl<'a, const O: u8> CTR0_W<'a, O> {
    #[doc = "Single continuous read mode is disabled for device 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTR0_A::_0)
    }
    #[doc = "Single continuous read mode is enabled for device 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTR0_A::_1)
    }
}
#[doc = "Field `DVRDCMD0` reader - Device 0 Command execution interval setting"]
pub type DVRDCMD0_R = crate::FieldReader<u8, DVRDCMD0_A>;
#[doc = "Device 0 Command execution interval setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVRDCMD0_A {
    #[doc = "0: 2 clock cycles"]
    _000 = 0,
    #[doc = "1: 5 clock cycles"]
    _001 = 1,
    #[doc = "2: 7 clock cycles"]
    _010 = 2,
    #[doc = "3: 9 clock cycles"]
    _011 = 3,
    #[doc = "4: 11 clock cycles"]
    _100 = 4,
    #[doc = "5: 13 clock cycles"]
    _101 = 5,
    #[doc = "6: 15 clock cycles"]
    _110 = 6,
    #[doc = "7: 17 clock cycles"]
    _111 = 7,
}
impl From<DVRDCMD0_A> for u8 {
    #[inline(always)]
    fn from(variant: DVRDCMD0_A) -> Self {
        variant as _
    }
}
impl DVRDCMD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVRDCMD0_A {
        match self.bits {
            0 => DVRDCMD0_A::_000,
            1 => DVRDCMD0_A::_001,
            2 => DVRDCMD0_A::_010,
            3 => DVRDCMD0_A::_011,
            4 => DVRDCMD0_A::_100,
            5 => DVRDCMD0_A::_101,
            6 => DVRDCMD0_A::_110,
            7 => DVRDCMD0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVRDCMD0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVRDCMD0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVRDCMD0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVRDCMD0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVRDCMD0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVRDCMD0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVRDCMD0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVRDCMD0_A::_111
    }
}
#[doc = "Field `DVRDCMD0` writer - Device 0 Command execution interval setting"]
pub type DVRDCMD0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DRCSTR_SPEC, u8, DVRDCMD0_A, 3, O>;
impl<'a, const O: u8> DVRDCMD0_W<'a, O> {
    #[doc = "2 clock cycles"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_000)
    }
    #[doc = "5 clock cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_001)
    }
    #[doc = "7 clock cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_010)
    }
    #[doc = "9 clock cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_011)
    }
    #[doc = "11 clock cycles"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_100)
    }
    #[doc = "13 clock cycles"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_101)
    }
    #[doc = "15 clock cycles"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_110)
    }
    #[doc = "17 clock cycles"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVRDCMD0_A::_111)
    }
}
#[doc = "Field `DVRDHI0` reader - Device 0 select signal pull-up timing setting"]
pub type DVRDHI0_R = crate::FieldReader<u8, DVRDHI0_A>;
#[doc = "Device 0 select signal pull-up timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVRDHI0_A {
    #[doc = "0: Setting prohibit"]
    _000 = 0,
    #[doc = "1: Setting prohibit"]
    _001 = 1,
    #[doc = "2: Setting prohibit"]
    _010 = 2,
    #[doc = "3: Setting prohibit (DOPI mode) 5 clock cycles (Other mode)"]
    _011 = 3,
    #[doc = "4: Setting prohibit (DOPI mode) 6 clock cycles (Other mode)"]
    _100 = 4,
    #[doc = "5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    _101 = 5,
    #[doc = "6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    _110 = 6,
    #[doc = "7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    _111 = 7,
}
impl From<DVRDHI0_A> for u8 {
    #[inline(always)]
    fn from(variant: DVRDHI0_A) -> Self {
        variant as _
    }
}
impl DVRDHI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVRDHI0_A {
        match self.bits {
            0 => DVRDHI0_A::_000,
            1 => DVRDHI0_A::_001,
            2 => DVRDHI0_A::_010,
            3 => DVRDHI0_A::_011,
            4 => DVRDHI0_A::_100,
            5 => DVRDHI0_A::_101,
            6 => DVRDHI0_A::_110,
            7 => DVRDHI0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVRDHI0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVRDHI0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVRDHI0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVRDHI0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVRDHI0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVRDHI0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVRDHI0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVRDHI0_A::_111
    }
}
#[doc = "Field `DVRDHI0` writer - Device 0 select signal pull-up timing setting"]
pub type DVRDHI0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DRCSTR_SPEC, u8, DVRDHI0_A, 3, O>;
impl<'a, const O: u8> DVRDHI0_W<'a, O> {
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_000)
    }
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_001)
    }
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_010)
    }
    #[doc = "Setting prohibit (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_011)
    }
    #[doc = "Setting prohibit (DOPI mode) 6 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_100)
    }
    #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_101)
    }
    #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_110)
    }
    #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVRDHI0_A::_111)
    }
}
#[doc = "Field `DVRDLO0` reader - Device 0 select signal pull-down timing setting"]
pub type DVRDLO0_R = crate::FieldReader<u8, DVRDLO0_A>;
#[doc = "Device 0 select signal pull-down timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVRDLO0_A {
    #[doc = "0: Setting prohibit"]
    _00 = 0,
    #[doc = "1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    _01 = 1,
    #[doc = "2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    _10 = 2,
    #[doc = "3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    _11 = 3,
}
impl From<DVRDLO0_A> for u8 {
    #[inline(always)]
    fn from(variant: DVRDLO0_A) -> Self {
        variant as _
    }
}
impl DVRDLO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVRDLO0_A {
        match self.bits {
            0 => DVRDLO0_A::_00,
            1 => DVRDLO0_A::_01,
            2 => DVRDLO0_A::_10,
            3 => DVRDLO0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DVRDLO0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DVRDLO0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DVRDLO0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DVRDLO0_A::_11
    }
}
#[doc = "Field `DVRDLO0` writer - Device 0 select signal pull-down timing setting"]
pub type DVRDLO0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DRCSTR_SPEC, u8, DVRDLO0_A, 2, O>;
impl<'a, const O: u8> DVRDLO0_W<'a, O> {
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DVRDLO0_A::_00)
    }
    #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DVRDLO0_A::_01)
    }
    #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DVRDLO0_A::_10)
    }
    #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DVRDLO0_A::_11)
    }
}
#[doc = "Field `CTRW1` reader - Device 1 single continuous read waiting cycle setting in PCLKA units"]
pub type CTRW1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRW1` writer - Device 1 single continuous read waiting cycle setting in PCLKA units"]
pub type CTRW1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRCSTR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CTR1` reader - Device 1 single continuous read mode setting"]
pub type CTR1_R = crate::BitReader<CTR1_A>;
#[doc = "Device 1 single continuous read mode setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTR1_A {
    #[doc = "0: Single continuous read mode is disabled for device 1."]
    _0 = 0,
    #[doc = "1: Single continuous read mode is enabled for device 1."]
    _1 = 1,
}
impl From<CTR1_A> for bool {
    #[inline(always)]
    fn from(variant: CTR1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTR1_A {
        match self.bits {
            false => CTR1_A::_0,
            true => CTR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTR1_A::_1
    }
}
#[doc = "Field `CTR1` writer - Device 1 single continuous read mode setting"]
pub type CTR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRCSTR_SPEC, CTR1_A, O>;
impl<'a, const O: u8> CTR1_W<'a, O> {
    #[doc = "Single continuous read mode is disabled for device 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTR1_A::_0)
    }
    #[doc = "Single continuous read mode is enabled for device 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTR1_A::_1)
    }
}
#[doc = "Field `DVRDCMD1` reader - Device 1 Command execution interval"]
pub type DVRDCMD1_R = crate::FieldReader<u8, DVRDCMD1_A>;
#[doc = "Device 1 Command execution interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVRDCMD1_A {
    #[doc = "0: 2 clock cycles"]
    _000 = 0,
    #[doc = "1: 5 clock cycles"]
    _001 = 1,
    #[doc = "2: 7 clock cycles"]
    _010 = 2,
    #[doc = "3: 9 clock cycles"]
    _011 = 3,
    #[doc = "4: 11 clock cycles"]
    _100 = 4,
    #[doc = "5: 13 clock cycles"]
    _101 = 5,
    #[doc = "6: 15 clock cycles"]
    _110 = 6,
    #[doc = "7: 17 clock cycles"]
    _111 = 7,
}
impl From<DVRDCMD1_A> for u8 {
    #[inline(always)]
    fn from(variant: DVRDCMD1_A) -> Self {
        variant as _
    }
}
impl DVRDCMD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVRDCMD1_A {
        match self.bits {
            0 => DVRDCMD1_A::_000,
            1 => DVRDCMD1_A::_001,
            2 => DVRDCMD1_A::_010,
            3 => DVRDCMD1_A::_011,
            4 => DVRDCMD1_A::_100,
            5 => DVRDCMD1_A::_101,
            6 => DVRDCMD1_A::_110,
            7 => DVRDCMD1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVRDCMD1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVRDCMD1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVRDCMD1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVRDCMD1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVRDCMD1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVRDCMD1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVRDCMD1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVRDCMD1_A::_111
    }
}
#[doc = "Field `DVRDCMD1` writer - Device 1 Command execution interval"]
pub type DVRDCMD1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DRCSTR_SPEC, u8, DVRDCMD1_A, 3, O>;
impl<'a, const O: u8> DVRDCMD1_W<'a, O> {
    #[doc = "2 clock cycles"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_000)
    }
    #[doc = "5 clock cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_001)
    }
    #[doc = "7 clock cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_010)
    }
    #[doc = "9 clock cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_011)
    }
    #[doc = "11 clock cycles"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_100)
    }
    #[doc = "13 clock cycles"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_101)
    }
    #[doc = "15 clock cycles"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_110)
    }
    #[doc = "17 clock cycles"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVRDCMD1_A::_111)
    }
}
#[doc = "Field `DVRDHI1` reader - Device 1 select signal High timing setting"]
pub type DVRDHI1_R = crate::FieldReader<u8, DVRDHI1_A>;
#[doc = "Device 1 select signal High timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVRDHI1_A {
    #[doc = "0: Setting prohibit"]
    _000 = 0,
    #[doc = "1: Setting prohibit"]
    _001 = 1,
    #[doc = "2: Setting prohibit"]
    _010 = 2,
    #[doc = "3: Setting prohibit (DOPI mode) 5 clock cycles (Other mode)"]
    _011 = 3,
    #[doc = "4: Setting prohibit (DOPI mode) 6 clock cycles (Other mode)"]
    _100 = 4,
    #[doc = "5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    _101 = 5,
    #[doc = "6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    _110 = 6,
    #[doc = "7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    _111 = 7,
}
impl From<DVRDHI1_A> for u8 {
    #[inline(always)]
    fn from(variant: DVRDHI1_A) -> Self {
        variant as _
    }
}
impl DVRDHI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVRDHI1_A {
        match self.bits {
            0 => DVRDHI1_A::_000,
            1 => DVRDHI1_A::_001,
            2 => DVRDHI1_A::_010,
            3 => DVRDHI1_A::_011,
            4 => DVRDHI1_A::_100,
            5 => DVRDHI1_A::_101,
            6 => DVRDHI1_A::_110,
            7 => DVRDHI1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVRDHI1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVRDHI1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVRDHI1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVRDHI1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVRDHI1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVRDHI1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVRDHI1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVRDHI1_A::_111
    }
}
#[doc = "Field `DVRDHI1` writer - Device 1 select signal High timing setting"]
pub type DVRDHI1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DRCSTR_SPEC, u8, DVRDHI1_A, 3, O>;
impl<'a, const O: u8> DVRDHI1_W<'a, O> {
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_000)
    }
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_001)
    }
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_010)
    }
    #[doc = "Setting prohibit (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_011)
    }
    #[doc = "Setting prohibit (DOPI mode) 6 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_100)
    }
    #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_101)
    }
    #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_110)
    }
    #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVRDHI1_A::_111)
    }
}
#[doc = "Field `DVRDLO1` reader - Device 1 select signal pull-down timing setting"]
pub type DVRDLO1_R = crate::FieldReader<u8, DVRDLO1_A>;
#[doc = "Device 1 select signal pull-down timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVRDLO1_A {
    #[doc = "0: Setting prohibited"]
    _00 = 0,
    #[doc = "1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    _01 = 1,
    #[doc = "2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    _10 = 2,
    #[doc = "3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    _11 = 3,
}
impl From<DVRDLO1_A> for u8 {
    #[inline(always)]
    fn from(variant: DVRDLO1_A) -> Self {
        variant as _
    }
}
impl DVRDLO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVRDLO1_A {
        match self.bits {
            0 => DVRDLO1_A::_00,
            1 => DVRDLO1_A::_01,
            2 => DVRDLO1_A::_10,
            3 => DVRDLO1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DVRDLO1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DVRDLO1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DVRDLO1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DVRDLO1_A::_11
    }
}
#[doc = "Field `DVRDLO1` writer - Device 1 select signal pull-down timing setting"]
pub type DVRDLO1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DRCSTR_SPEC, u8, DVRDLO1_A, 2, O>;
impl<'a, const O: u8> DVRDLO1_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DVRDLO1_A::_00)
    }
    #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DVRDLO1_A::_01)
    }
    #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DVRDLO1_A::_10)
    }
    #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DVRDLO1_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:6 - Device 0 single continuous read waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctrw0(&self) -> CTRW0_R {
        CTRW0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Device 0 single continuous read mode setting"]
    #[inline(always)]
    pub fn ctr0(&self) -> CTR0_R {
        CTR0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Device 0 Command execution interval setting"]
    #[inline(always)]
    pub fn dvrdcmd0(&self) -> DVRDCMD0_R {
        DVRDCMD0_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Device 0 select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvrdhi0(&self) -> DVRDHI0_R {
        DVRDHI0_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Device 0 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvrdlo0(&self) -> DVRDLO0_R {
        DVRDLO0_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Device 1 single continuous read waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctrw1(&self) -> CTRW1_R {
        CTRW1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Device 1 single continuous read mode setting"]
    #[inline(always)]
    pub fn ctr1(&self) -> CTR1_R {
        CTR1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Device 1 Command execution interval"]
    #[inline(always)]
    pub fn dvrdcmd1(&self) -> DVRDCMD1_R {
        DVRDCMD1_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Device 1 select signal High timing setting"]
    #[inline(always)]
    pub fn dvrdhi1(&self) -> DVRDHI1_R {
        DVRDHI1_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Device 1 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvrdlo1(&self) -> DVRDLO1_R {
        DVRDLO1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device 0 single continuous read waiting cycle setting in PCLKA units"]
    #[inline(always)]
    #[must_use]
    pub fn ctrw0(&mut self) -> CTRW0_W<0> {
        CTRW0_W::new(self)
    }
    #[doc = "Bit 7 - Device 0 single continuous read mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctr0(&mut self) -> CTR0_W<7> {
        CTR0_W::new(self)
    }
    #[doc = "Bits 8:10 - Device 0 Command execution interval setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvrdcmd0(&mut self) -> DVRDCMD0_W<8> {
        DVRDCMD0_W::new(self)
    }
    #[doc = "Bits 11:13 - Device 0 select signal pull-up timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvrdhi0(&mut self) -> DVRDHI0_W<11> {
        DVRDHI0_W::new(self)
    }
    #[doc = "Bits 14:15 - Device 0 select signal pull-down timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvrdlo0(&mut self) -> DVRDLO0_W<14> {
        DVRDLO0_W::new(self)
    }
    #[doc = "Bits 16:22 - Device 1 single continuous read waiting cycle setting in PCLKA units"]
    #[inline(always)]
    #[must_use]
    pub fn ctrw1(&mut self) -> CTRW1_W<16> {
        CTRW1_W::new(self)
    }
    #[doc = "Bit 23 - Device 1 single continuous read mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctr1(&mut self) -> CTR1_W<23> {
        CTR1_W::new(self)
    }
    #[doc = "Bits 24:26 - Device 1 Command execution interval"]
    #[inline(always)]
    #[must_use]
    pub fn dvrdcmd1(&mut self) -> DVRDCMD1_W<24> {
        DVRDCMD1_W::new(self)
    }
    #[doc = "Bits 27:29 - Device 1 select signal High timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvrdhi1(&mut self) -> DVRDHI1_W<27> {
        DVRDHI1_W::new(self)
    }
    #[doc = "Bits 30:31 - Device 1 select signal pull-down timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvrdlo1(&mut self) -> DVRDLO1_W<30> {
        DVRDLO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Memory Map Read Chip Select Timing Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drcstr](index.html) module"]
pub struct DRCSTR_SPEC;
impl crate::RegisterSpec for DRCSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drcstr::R](R) reader structure"]
impl crate::Readable for DRCSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drcstr::W](W) writer structure"]
impl crate::Writable for DRCSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRCSTR to value 0"]
impl crate::Resettable for DRCSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
