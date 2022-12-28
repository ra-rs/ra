#[doc = "Register `SRCCTRL` reader"]
pub struct R(crate::R<SRCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCCTRL` writer"]
pub struct W(crate::W<SRCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCCTRL_SPEC>;
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
impl From<crate::W<SRCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFS` reader - Output Sampling Rate"]
pub type OFS_R = crate::FieldReader<u8, OFS_A>;
#[doc = "Output Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OFS_A {
    #[doc = "0: 44.1 kHz"]
    _000 = 0,
    #[doc = "1: 48.0 kHz"]
    _001 = 1,
    #[doc = "2: 32.0 kHz"]
    _010 = 2,
    #[doc = "3: Setting prohibited"]
    _011 = 3,
    #[doc = "4: 8.0 kHz ( Valid only when IFS\\[3:0\\]
=1001b )"]
    _100 = 4,
    #[doc = "5: 16.0 kHz ( Valid only when IFS\\[3:0\\]
=1001b )"]
    _101 = 5,
}
impl From<OFS_A> for u8 {
    #[inline(always)]
    fn from(variant: OFS_A) -> Self {
        variant as _
    }
}
impl OFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OFS_A> {
        match self.bits {
            0 => Some(OFS_A::_000),
            1 => Some(OFS_A::_001),
            2 => Some(OFS_A::_010),
            3 => Some(OFS_A::_011),
            4 => Some(OFS_A::_100),
            5 => Some(OFS_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == OFS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == OFS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == OFS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == OFS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == OFS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == OFS_A::_101
    }
}
#[doc = "Field `OFS` writer - Output Sampling Rate"]
pub type OFS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SRCCTRL_SPEC, u8, OFS_A, 3, O>;
impl<'a, const O: u8> OFS_W<'a, O> {
    #[doc = "44.1 kHz"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(OFS_A::_000)
    }
    #[doc = "48.0 kHz"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(OFS_A::_001)
    }
    #[doc = "32.0 kHz"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(OFS_A::_010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(OFS_A::_011)
    }
    #[doc = "8.0 kHz ( Valid only when IFS\\[3:0\\]
=1001b )"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(OFS_A::_100)
    }
    #[doc = "16.0 kHz ( Valid only when IFS\\[3:0\\]
=1001b )"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(OFS_A::_101)
    }
}
#[doc = "Field `IFS` reader - Input Sampling Rate"]
pub type IFS_R = crate::FieldReader<u8, IFS_A>;
#[doc = "Input Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFS_A {
    #[doc = "0: 8.0 kHz"]
    _0000 = 0,
    #[doc = "1: 11.025 kHz"]
    _0001 = 1,
    #[doc = "2: 12.0 kHz"]
    _0010 = 2,
    #[doc = "3: Setting prohibited"]
    _0011 = 3,
    #[doc = "4: 16.0 kHz"]
    _0100 = 4,
    #[doc = "5: 22.05 kHz"]
    _0101 = 5,
    #[doc = "6: 24.0 kHz"]
    _0110 = 6,
    #[doc = "7: Setting prohibited"]
    _0111 = 7,
    #[doc = "8: 32.0 kHz"]
    _1000 = 8,
    #[doc = "9: 44.1 kHz"]
    _1001 = 9,
    #[doc = "10: 48.0 kHz"]
    _1010 = 10,
}
impl From<IFS_A> for u8 {
    #[inline(always)]
    fn from(variant: IFS_A) -> Self {
        variant as _
    }
}
impl IFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IFS_A> {
        match self.bits {
            0 => Some(IFS_A::_0000),
            1 => Some(IFS_A::_0001),
            2 => Some(IFS_A::_0010),
            3 => Some(IFS_A::_0011),
            4 => Some(IFS_A::_0100),
            5 => Some(IFS_A::_0101),
            6 => Some(IFS_A::_0110),
            7 => Some(IFS_A::_0111),
            8 => Some(IFS_A::_1000),
            9 => Some(IFS_A::_1001),
            10 => Some(IFS_A::_1010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == IFS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == IFS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == IFS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == IFS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == IFS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == IFS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == IFS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == IFS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == IFS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == IFS_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == IFS_A::_1010
    }
}
#[doc = "Field `IFS` writer - Input Sampling Rate"]
pub type IFS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SRCCTRL_SPEC, u8, IFS_A, 4, O>;
impl<'a, const O: u8> IFS_W<'a, O> {
    #[doc = "8.0 kHz"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(IFS_A::_0000)
    }
    #[doc = "11.025 kHz"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(IFS_A::_0001)
    }
    #[doc = "12.0 kHz"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(IFS_A::_0010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(IFS_A::_0011)
    }
    #[doc = "16.0 kHz"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(IFS_A::_0100)
    }
    #[doc = "22.05 kHz"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(IFS_A::_0101)
    }
    #[doc = "24.0 kHz"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(IFS_A::_0110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(IFS_A::_0111)
    }
    #[doc = "32.0 kHz"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(IFS_A::_1000)
    }
    #[doc = "44.1 kHz"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(IFS_A::_1001)
    }
    #[doc = "48.0 kHz"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(IFS_A::_1010)
    }
}
#[doc = "Internal Work Memory Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CL_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: Clears the input FIFO, output FIFO, input buffer memory, intermediate memory and accumulator."]
    _1 = 1,
}
impl From<CL_AW> for bool {
    #[inline(always)]
    fn from(variant: CL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CL` writer - Internal Work Memory Clear"]
pub type CL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCCTRL_SPEC, CL_AW, O>;
impl<'a, const O: u8> CL_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CL_AW::_0)
    }
    #[doc = "Clears the input FIFO, output FIFO, input buffer memory, intermediate memory and accumulator."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CL_AW::_1)
    }
}
#[doc = "Internal Work Memory Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FL_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: starts converting the sampling rate of all the data in the input FIFO, input buffer memory, and intermediate memory(i.e., flush processing)."]
    _1 = 1,
}
impl From<FL_AW> for bool {
    #[inline(always)]
    fn from(variant: FL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FL` writer - Internal Work Memory Flush"]
pub type FL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCCTRL_SPEC, FL_AW, O>;
impl<'a, const O: u8> FL_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FL_AW::_0)
    }
    #[doc = "starts converting the sampling rate of all the data in the input FIFO, input buffer memory, and intermediate memory(i.e., flush processing)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FL_AW::_1)
    }
}
#[doc = "Field `OVEN` reader - Output Data FIFO Overwrite Interrupt Enable"]
pub type OVEN_R = crate::BitReader<OVEN_A>;
#[doc = "Output Data FIFO Overwrite Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVEN_A {
    #[doc = "0: Output data FIFO overwrite interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Output data FIFO overwrite interrupt is enabled."]
    _1 = 1,
}
impl From<OVEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVEN_A {
        match self.bits {
            false => OVEN_A::_0,
            true => OVEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVEN_A::_1
    }
}
#[doc = "Field `OVEN` writer - Output Data FIFO Overwrite Interrupt Enable"]
pub type OVEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCCTRL_SPEC, OVEN_A, O>;
impl<'a, const O: u8> OVEN_W<'a, O> {
    #[doc = "Output data FIFO overwrite interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVEN_A::_0)
    }
    #[doc = "Output data FIFO overwrite interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVEN_A::_1)
    }
}
#[doc = "Field `UDEN` reader - Output Data FIFO Underflow Interrupt Enable"]
pub type UDEN_R = crate::BitReader<UDEN_A>;
#[doc = "Output Data FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDEN_A {
    #[doc = "0: Disables output data FIFO underflow interrupt requests."]
    _0 = 0,
    #[doc = "1: Enables output data FIFO underflow interrupt requests."]
    _1 = 1,
}
impl From<UDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDEN_A {
        match self.bits {
            false => UDEN_A::_0,
            true => UDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDEN_A::_1
    }
}
#[doc = "Field `UDEN` writer - Output Data FIFO Underflow Interrupt Enable"]
pub type UDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCCTRL_SPEC, UDEN_A, O>;
impl<'a, const O: u8> UDEN_W<'a, O> {
    #[doc = "Disables output data FIFO underflow interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UDEN_A::_0)
    }
    #[doc = "Enables output data FIFO underflow interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UDEN_A::_1)
    }
}
#[doc = "Field `SRCEN` reader - Module Enable"]
pub type SRCEN_R = crate::BitReader<SRCEN_A>;
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCEN_A {
    #[doc = "0: Disables this module operation."]
    _0 = 0,
    #[doc = "1: Enables this module operation."]
    _1 = 1,
}
impl From<SRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCEN_A {
        match self.bits {
            false => SRCEN_A::_0,
            true => SRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRCEN_A::_1
    }
}
#[doc = "Field `SRCEN` writer - Module Enable"]
pub type SRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCCTRL_SPEC, SRCEN_A, O>;
impl<'a, const O: u8> SRCEN_W<'a, O> {
    #[doc = "Disables this module operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRCEN_A::_0)
    }
    #[doc = "Enables this module operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRCEN_A::_1)
    }
}
#[doc = "Field `CEEN` reader - Conversion End Interrupt Enable"]
pub type CEEN_R = crate::BitReader<CEEN_A>;
#[doc = "Conversion End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEEN_A {
    #[doc = "0: Disables conversion end interrupt requests."]
    _0 = 0,
    #[doc = "1: Enables conversion end interrupt requests."]
    _1 = 1,
}
impl From<CEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEEN_A {
        match self.bits {
            false => CEEN_A::_0,
            true => CEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEEN_A::_1
    }
}
#[doc = "Field `CEEN` writer - Conversion End Interrupt Enable"]
pub type CEEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCCTRL_SPEC, CEEN_A, O>;
impl<'a, const O: u8> CEEN_W<'a, O> {
    #[doc = "Disables conversion end interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEEN_A::_0)
    }
    #[doc = "Enables conversion end interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEEN_A::_1)
    }
}
#[doc = "Field `FICRAE` reader - Filter Coefficient Table Access Enable"]
pub type FICRAE_R = crate::BitReader<FICRAE_A>;
#[doc = "Filter Coefficient Table Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FICRAE_A {
    #[doc = "0: Reading/writing to filter coefficient table RAM is disabled."]
    _0 = 0,
    #[doc = "1: Reading/writing to filter coefficient table RAM is enabled."]
    _1 = 1,
}
impl From<FICRAE_A> for bool {
    #[inline(always)]
    fn from(variant: FICRAE_A) -> Self {
        variant as u8 != 0
    }
}
impl FICRAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FICRAE_A {
        match self.bits {
            false => FICRAE_A::_0,
            true => FICRAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FICRAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FICRAE_A::_1
    }
}
#[doc = "Field `FICRAE` writer - Filter Coefficient Table Access Enable"]
pub type FICRAE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCCTRL_SPEC, FICRAE_A, O>;
impl<'a, const O: u8> FICRAE_W<'a, O> {
    #[doc = "Reading/writing to filter coefficient table RAM is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FICRAE_A::_0)
    }
    #[doc = "Reading/writing to filter coefficient table RAM is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FICRAE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Output Sampling Rate"]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Input Sampling Rate"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Output Data FIFO Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn oven(&self) -> OVEN_R {
        OVEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Data FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Module Enable"]
    #[inline(always)]
    pub fn srcen(&self) -> SRCEN_R {
        SRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Conversion End Interrupt Enable"]
    #[inline(always)]
    pub fn ceen(&self) -> CEEN_R {
        CEEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter Coefficient Table Access Enable"]
    #[inline(always)]
    pub fn ficrae(&self) -> FICRAE_R {
        FICRAE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Output Sampling Rate"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OFS_W<0> {
        OFS_W::new(self)
    }
    #[doc = "Bits 4:7 - Input Sampling Rate"]
    #[inline(always)]
    #[must_use]
    pub fn ifs(&mut self) -> IFS_W<4> {
        IFS_W::new(self)
    }
    #[doc = "Bit 8 - Internal Work Memory Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cl(&mut self) -> CL_W<8> {
        CL_W::new(self)
    }
    #[doc = "Bit 9 - Internal Work Memory Flush"]
    #[inline(always)]
    #[must_use]
    pub fn fl(&mut self) -> FL_W<9> {
        FL_W::new(self)
    }
    #[doc = "Bit 10 - Output Data FIFO Overwrite Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oven(&mut self) -> OVEN_W<10> {
        OVEN_W::new(self)
    }
    #[doc = "Bit 11 - Output Data FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uden(&mut self) -> UDEN_W<11> {
        UDEN_W::new(self)
    }
    #[doc = "Bit 12 - Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srcen(&mut self) -> SRCEN_W<12> {
        SRCEN_W::new(self)
    }
    #[doc = "Bit 13 - Conversion End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ceen(&mut self) -> CEEN_W<13> {
        CEEN_W::new(self)
    }
    #[doc = "Bit 15 - Filter Coefficient Table Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ficrae(&mut self) -> FICRAE_W<15> {
        FICRAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcctrl](index.html) module"]
pub struct SRCCTRL_SPEC;
impl crate::RegisterSpec for SRCCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [srcctrl::R](R) reader structure"]
impl crate::Readable for SRCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcctrl::W](W) writer structure"]
impl crate::Writable for SRCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCCTRL to value 0"]
impl crate::Resettable for SRCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
