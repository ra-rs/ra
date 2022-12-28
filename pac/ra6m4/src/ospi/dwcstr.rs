#[doc = "Register `DWCSTR` reader"]
pub struct R(crate::R<DWCSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DWCSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DWCSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DWCSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DWCSTR` writer"]
pub struct W(crate::W<DWCSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DWCSTR_SPEC>;
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
impl From<crate::W<DWCSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DWCSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTWW0` reader - Device 0 single continuous write waiting cycle setting in PCLKA units"]
pub type CTWW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTWW0` writer - Device 0 single continuous write waiting cycle setting in PCLKA units"]
pub type CTWW0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DWCSTR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CTW0` reader - Device 0 single continuous write mode setting"]
pub type CTW0_R = crate::BitReader<CTW0_A>;
#[doc = "Device 0 single continuous write mode setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTW0_A {
    #[doc = "0: Single continuous write mode is disabled for device 0"]
    _0 = 0,
    #[doc = "1: Single continuous write mode is enabled for device 0"]
    _1 = 1,
}
impl From<CTW0_A> for bool {
    #[inline(always)]
    fn from(variant: CTW0_A) -> Self {
        variant as u8 != 0
    }
}
impl CTW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTW0_A {
        match self.bits {
            false => CTW0_A::_0,
            true => CTW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTW0_A::_1
    }
}
#[doc = "Field `CTW0` writer - Device 0 single continuous write mode setting"]
pub type CTW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DWCSTR_SPEC, CTW0_A, O>;
impl<'a, const O: u8> CTW0_W<'a, O> {
    #[doc = "Single continuous write mode is disabled for device 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTW0_A::_0)
    }
    #[doc = "Single continuous write mode is enabled for device 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTW0_A::_1)
    }
}
#[doc = "Field `DVWCMD0` reader - Device 0 Command execution interval setting"]
pub type DVWCMD0_R = crate::FieldReader<u8, DVWCMD0_A>;
#[doc = "Device 0 Command execution interval setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVWCMD0_A {
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
impl From<DVWCMD0_A> for u8 {
    #[inline(always)]
    fn from(variant: DVWCMD0_A) -> Self {
        variant as _
    }
}
impl DVWCMD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVWCMD0_A {
        match self.bits {
            0 => DVWCMD0_A::_000,
            1 => DVWCMD0_A::_001,
            2 => DVWCMD0_A::_010,
            3 => DVWCMD0_A::_011,
            4 => DVWCMD0_A::_100,
            5 => DVWCMD0_A::_101,
            6 => DVWCMD0_A::_110,
            7 => DVWCMD0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVWCMD0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVWCMD0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVWCMD0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVWCMD0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVWCMD0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVWCMD0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVWCMD0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVWCMD0_A::_111
    }
}
#[doc = "Field `DVWCMD0` writer - Device 0 Command execution interval setting"]
pub type DVWCMD0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DWCSTR_SPEC, u8, DVWCMD0_A, 3, O>;
impl<'a, const O: u8> DVWCMD0_W<'a, O> {
    #[doc = "2 clock cycles"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_000)
    }
    #[doc = "5 clock cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_001)
    }
    #[doc = "7 clock cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_010)
    }
    #[doc = "9 clock cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_011)
    }
    #[doc = "11 clock cycles"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_100)
    }
    #[doc = "13 clock cycles"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_101)
    }
    #[doc = "15 clock cycles"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_110)
    }
    #[doc = "17 clock cycles"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVWCMD0_A::_111)
    }
}
#[doc = "Field `DVWHI0` reader - Device 0 select signal pull-up timing setting"]
pub type DVWHI0_R = crate::FieldReader<u8, DVWHI0_A>;
#[doc = "Device 0 select signal pull-up timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVWHI0_A {
    #[doc = "0: 1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)"]
    _000 = 0,
    #[doc = "1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    _001 = 1,
    #[doc = "2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    _010 = 2,
    #[doc = "3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    _011 = 3,
    #[doc = "4: 5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)"]
    _100 = 4,
    #[doc = "5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    _101 = 5,
    #[doc = "6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    _110 = 6,
    #[doc = "7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    _111 = 7,
}
impl From<DVWHI0_A> for u8 {
    #[inline(always)]
    fn from(variant: DVWHI0_A) -> Self {
        variant as _
    }
}
impl DVWHI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVWHI0_A {
        match self.bits {
            0 => DVWHI0_A::_000,
            1 => DVWHI0_A::_001,
            2 => DVWHI0_A::_010,
            3 => DVWHI0_A::_011,
            4 => DVWHI0_A::_100,
            5 => DVWHI0_A::_101,
            6 => DVWHI0_A::_110,
            7 => DVWHI0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVWHI0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVWHI0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVWHI0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVWHI0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVWHI0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVWHI0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVWHI0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVWHI0_A::_111
    }
}
#[doc = "Field `DVWHI0` writer - Device 0 select signal pull-up timing setting"]
pub type DVWHI0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DWCSTR_SPEC, u8, DVWHI0_A, 3, O>;
impl<'a, const O: u8> DVWHI0_W<'a, O> {
    #[doc = "1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVWHI0_A::_000)
    }
    #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVWHI0_A::_001)
    }
    #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVWHI0_A::_010)
    }
    #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVWHI0_A::_011)
    }
    #[doc = "5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVWHI0_A::_100)
    }
    #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVWHI0_A::_101)
    }
    #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVWHI0_A::_110)
    }
    #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVWHI0_A::_111)
    }
}
#[doc = "Field `DVWLO0` reader - Device 0 select signal pull-down timing setting"]
pub type DVWLO0_R = crate::FieldReader<u8, DVWLO0_A>;
#[doc = "Device 0 select signal pull-down timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVWLO0_A {
    #[doc = "0: Setting prohibit"]
    _00 = 0,
    #[doc = "1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    _01 = 1,
    #[doc = "2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    _10 = 2,
    #[doc = "3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    _11 = 3,
}
impl From<DVWLO0_A> for u8 {
    #[inline(always)]
    fn from(variant: DVWLO0_A) -> Self {
        variant as _
    }
}
impl DVWLO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVWLO0_A {
        match self.bits {
            0 => DVWLO0_A::_00,
            1 => DVWLO0_A::_01,
            2 => DVWLO0_A::_10,
            3 => DVWLO0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DVWLO0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DVWLO0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DVWLO0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DVWLO0_A::_11
    }
}
#[doc = "Field `DVWLO0` writer - Device 0 select signal pull-down timing setting"]
pub type DVWLO0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DWCSTR_SPEC, u8, DVWLO0_A, 2, O>;
impl<'a, const O: u8> DVWLO0_W<'a, O> {
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DVWLO0_A::_00)
    }
    #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DVWLO0_A::_01)
    }
    #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DVWLO0_A::_10)
    }
    #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DVWLO0_A::_11)
    }
}
#[doc = "Field `CTWW1` reader - Device 1 single continuous write waiting cycle setting in PCLKA units"]
pub type CTWW1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTWW1` writer - Device 1 single continuous write waiting cycle setting in PCLKA units"]
pub type CTWW1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DWCSTR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CTW1` reader - Device 1 single continuous write mode setting"]
pub type CTW1_R = crate::BitReader<CTW1_A>;
#[doc = "Device 1 single continuous write mode setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTW1_A {
    #[doc = "0: Single continuous write mode is disabled for device 1"]
    _0 = 0,
    #[doc = "1: Single continuous write mode is enabled for device 1"]
    _1 = 1,
}
impl From<CTW1_A> for bool {
    #[inline(always)]
    fn from(variant: CTW1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTW1_A {
        match self.bits {
            false => CTW1_A::_0,
            true => CTW1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTW1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTW1_A::_1
    }
}
#[doc = "Field `CTW1` writer - Device 1 single continuous write mode setting"]
pub type CTW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DWCSTR_SPEC, CTW1_A, O>;
impl<'a, const O: u8> CTW1_W<'a, O> {
    #[doc = "Single continuous write mode is disabled for device 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTW1_A::_0)
    }
    #[doc = "Single continuous write mode is enabled for device 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTW1_A::_1)
    }
}
#[doc = "Field `DVWCMD1` reader - Device 1 Command execution interval setting"]
pub type DVWCMD1_R = crate::FieldReader<u8, DVWCMD1_A>;
#[doc = "Device 1 Command execution interval setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVWCMD1_A {
    #[doc = "0: setting prohibited"]
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
impl From<DVWCMD1_A> for u8 {
    #[inline(always)]
    fn from(variant: DVWCMD1_A) -> Self {
        variant as _
    }
}
impl DVWCMD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVWCMD1_A {
        match self.bits {
            0 => DVWCMD1_A::_000,
            1 => DVWCMD1_A::_001,
            2 => DVWCMD1_A::_010,
            3 => DVWCMD1_A::_011,
            4 => DVWCMD1_A::_100,
            5 => DVWCMD1_A::_101,
            6 => DVWCMD1_A::_110,
            7 => DVWCMD1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVWCMD1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVWCMD1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVWCMD1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVWCMD1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVWCMD1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVWCMD1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVWCMD1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVWCMD1_A::_111
    }
}
#[doc = "Field `DVWCMD1` writer - Device 1 Command execution interval setting"]
pub type DVWCMD1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DWCSTR_SPEC, u8, DVWCMD1_A, 3, O>;
impl<'a, const O: u8> DVWCMD1_W<'a, O> {
    #[doc = "setting prohibited"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_000)
    }
    #[doc = "5 clock cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_001)
    }
    #[doc = "7 clock cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_010)
    }
    #[doc = "9 clock cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_011)
    }
    #[doc = "11 clock cycles"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_100)
    }
    #[doc = "13 clock cycles"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_101)
    }
    #[doc = "15 clock cycles"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_110)
    }
    #[doc = "17 clock cycles"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVWCMD1_A::_111)
    }
}
#[doc = "Field `DVWHI1` reader - Device 1 select signal pull-up timing setting"]
pub type DVWHI1_R = crate::FieldReader<u8, DVWHI1_A>;
#[doc = "Device 1 select signal pull-up timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVWHI1_A {
    #[doc = "0: 1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)"]
    _000 = 0,
    #[doc = "1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    _001 = 1,
    #[doc = "2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    _010 = 2,
    #[doc = "3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    _011 = 3,
    #[doc = "4: 5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)"]
    _100 = 4,
    #[doc = "5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    _101 = 5,
    #[doc = "6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    _110 = 6,
    #[doc = "7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    _111 = 7,
}
impl From<DVWHI1_A> for u8 {
    #[inline(always)]
    fn from(variant: DVWHI1_A) -> Self {
        variant as _
    }
}
impl DVWHI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVWHI1_A {
        match self.bits {
            0 => DVWHI1_A::_000,
            1 => DVWHI1_A::_001,
            2 => DVWHI1_A::_010,
            3 => DVWHI1_A::_011,
            4 => DVWHI1_A::_100,
            5 => DVWHI1_A::_101,
            6 => DVWHI1_A::_110,
            7 => DVWHI1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVWHI1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVWHI1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVWHI1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVWHI1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DVWHI1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DVWHI1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DVWHI1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DVWHI1_A::_111
    }
}
#[doc = "Field `DVWHI1` writer - Device 1 select signal pull-up timing setting"]
pub type DVWHI1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DWCSTR_SPEC, u8, DVWHI1_A, 3, O>;
impl<'a, const O: u8> DVWHI1_W<'a, O> {
    #[doc = "1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DVWHI1_A::_000)
    }
    #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DVWHI1_A::_001)
    }
    #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DVWHI1_A::_010)
    }
    #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DVWHI1_A::_011)
    }
    #[doc = "5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DVWHI1_A::_100)
    }
    #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DVWHI1_A::_101)
    }
    #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DVWHI1_A::_110)
    }
    #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DVWHI1_A::_111)
    }
}
#[doc = "Field `DVWLO1` reader - Device 1 select signal pull-down timing setting"]
pub type DVWLO1_R = crate::FieldReader<u8, DVWLO1_A>;
#[doc = "Device 1 select signal pull-down timing setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVWLO1_A {
    #[doc = "0: Setting prohibit"]
    _00 = 0,
    #[doc = "1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    _01 = 1,
    #[doc = "2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    _10 = 2,
    #[doc = "3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    _11 = 3,
}
impl From<DVWLO1_A> for u8 {
    #[inline(always)]
    fn from(variant: DVWLO1_A) -> Self {
        variant as _
    }
}
impl DVWLO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVWLO1_A {
        match self.bits {
            0 => DVWLO1_A::_00,
            1 => DVWLO1_A::_01,
            2 => DVWLO1_A::_10,
            3 => DVWLO1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DVWLO1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DVWLO1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DVWLO1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DVWLO1_A::_11
    }
}
#[doc = "Field `DVWLO1` writer - Device 1 select signal pull-down timing setting"]
pub type DVWLO1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DWCSTR_SPEC, u8, DVWLO1_A, 2, O>;
impl<'a, const O: u8> DVWLO1_W<'a, O> {
    #[doc = "Setting prohibit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DVWLO1_A::_00)
    }
    #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DVWLO1_A::_01)
    }
    #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DVWLO1_A::_10)
    }
    #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DVWLO1_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:6 - Device 0 single continuous write waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctww0(&self) -> CTWW0_R {
        CTWW0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Device 0 single continuous write mode setting"]
    #[inline(always)]
    pub fn ctw0(&self) -> CTW0_R {
        CTW0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Device 0 Command execution interval setting"]
    #[inline(always)]
    pub fn dvwcmd0(&self) -> DVWCMD0_R {
        DVWCMD0_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Device 0 select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvwhi0(&self) -> DVWHI0_R {
        DVWHI0_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Device 0 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvwlo0(&self) -> DVWLO0_R {
        DVWLO0_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Device 1 single continuous write waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctww1(&self) -> CTWW1_R {
        CTWW1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Device 1 single continuous write mode setting"]
    #[inline(always)]
    pub fn ctw1(&self) -> CTW1_R {
        CTW1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Device 1 Command execution interval setting"]
    #[inline(always)]
    pub fn dvwcmd1(&self) -> DVWCMD1_R {
        DVWCMD1_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Device 1 select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvwhi1(&self) -> DVWHI1_R {
        DVWHI1_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Device 1 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvwlo1(&self) -> DVWLO1_R {
        DVWLO1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device 0 single continuous write waiting cycle setting in PCLKA units"]
    #[inline(always)]
    #[must_use]
    pub fn ctww0(&mut self) -> CTWW0_W<0> {
        CTWW0_W::new(self)
    }
    #[doc = "Bit 7 - Device 0 single continuous write mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctw0(&mut self) -> CTW0_W<7> {
        CTW0_W::new(self)
    }
    #[doc = "Bits 8:10 - Device 0 Command execution interval setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvwcmd0(&mut self) -> DVWCMD0_W<8> {
        DVWCMD0_W::new(self)
    }
    #[doc = "Bits 11:13 - Device 0 select signal pull-up timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvwhi0(&mut self) -> DVWHI0_W<11> {
        DVWHI0_W::new(self)
    }
    #[doc = "Bits 14:15 - Device 0 select signal pull-down timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvwlo0(&mut self) -> DVWLO0_W<14> {
        DVWLO0_W::new(self)
    }
    #[doc = "Bits 16:22 - Device 1 single continuous write waiting cycle setting in PCLKA units"]
    #[inline(always)]
    #[must_use]
    pub fn ctww1(&mut self) -> CTWW1_W<16> {
        CTWW1_W::new(self)
    }
    #[doc = "Bit 23 - Device 1 single continuous write mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctw1(&mut self) -> CTW1_W<23> {
        CTW1_W::new(self)
    }
    #[doc = "Bits 24:26 - Device 1 Command execution interval setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvwcmd1(&mut self) -> DVWCMD1_W<24> {
        DVWCMD1_W::new(self)
    }
    #[doc = "Bits 27:29 - Device 1 select signal pull-up timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvwhi1(&mut self) -> DVWHI1_W<27> {
        DVWHI1_W::new(self)
    }
    #[doc = "Bits 30:31 - Device 1 select signal pull-down timing setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvwlo1(&mut self) -> DVWLO1_W<30> {
        DVWLO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Memory Map Write Chip Select Timing Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dwcstr](index.html) module"]
pub struct DWCSTR_SPEC;
impl crate::RegisterSpec for DWCSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dwcstr::R](R) reader structure"]
impl crate::Readable for DWCSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dwcstr::W](W) writer structure"]
impl crate::Writable for DWCSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DWCSTR to value 0"]
impl crate::Resettable for DWCSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
