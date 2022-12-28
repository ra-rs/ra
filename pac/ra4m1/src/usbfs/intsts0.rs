#[doc = "Register `INTSTS0` reader"]
pub struct R(crate::R<INTSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTS0` writer"]
pub struct W(crate::W<INTSTS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTS0_SPEC>;
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
impl From<crate::W<INTSTS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSQ` reader - Control Transfer Stage"]
pub type CTSQ_R = crate::FieldReader<u8, CTSQ_A>;
#[doc = "Control Transfer Stage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSQ_A {
    #[doc = "0: Idle or setup stage"]
    _000 = 0,
    #[doc = "1: Control read data stage"]
    _001 = 1,
    #[doc = "2: Control read status stage"]
    _010 = 2,
    #[doc = "3: Control write data stage"]
    _011 = 3,
    #[doc = "4: Control write status stage"]
    _100 = 4,
    #[doc = "5: Control write (no data) status stage"]
    _101 = 5,
    #[doc = "6: Control transfer sequence error"]
    _110 = 6,
}
impl From<CTSQ_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSQ_A) -> Self {
        variant as _
    }
}
impl CTSQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSQ_A {
        match self.bits {
            0 => CTSQ_A::_000,
            1 => CTSQ_A::_001,
            2 => CTSQ_A::_010,
            3 => CTSQ_A::_011,
            4 => CTSQ_A::_100,
            5 => CTSQ_A::_101,
            6 => CTSQ_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CTSQ_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CTSQ_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CTSQ_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CTSQ_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CTSQ_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CTSQ_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CTSQ_A::_110
    }
}
#[doc = "Field `VALID` reader - USB Request Reception"]
pub type VALID_R = crate::BitReader<VALID_A>;
#[doc = "USB Request Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALID_A {
    #[doc = "0: Setup packet is not received"]
    _0 = 0,
    #[doc = "1: Setup packet is received"]
    _1 = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::_0,
            true => VALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VALID_A::_1
    }
}
#[doc = "Field `VALID` writer - USB Request Reception"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTSTS0_SPEC, VALID_A, O>;
impl<'a, const O: u8> VALID_W<'a, O> {
    #[doc = "Setup packet is not received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VALID_A::_0)
    }
    #[doc = "Setup packet is received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VALID_A::_1)
    }
}
#[doc = "Field `DVSQ` reader - Device State"]
pub type DVSQ_R = crate::FieldReader<u8, DVSQ_A>;
#[doc = "Device State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVSQ_A {
    #[doc = "0: Powered state"]
    _000 = 0,
    #[doc = "1: Default state"]
    _001 = 1,
    #[doc = "2: Address state"]
    _010 = 2,
    #[doc = "3: Configured state"]
    _011 = 3,
}
impl From<DVSQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DVSQ_A) -> Self {
        variant as _
    }
}
impl DVSQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DVSQ_A> {
        match self.bits {
            0 => Some(DVSQ_A::_000),
            1 => Some(DVSQ_A::_001),
            2 => Some(DVSQ_A::_010),
            3 => Some(DVSQ_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVSQ_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVSQ_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVSQ_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVSQ_A::_011
    }
}
#[doc = "Field `VBSTS` reader - VBUS Input Status"]
pub type VBSTS_R = crate::BitReader<VBSTS_A>;
#[doc = "VBUS Input Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBSTS_A {
    #[doc = "0: USB_VBUS pin is low."]
    _0 = 0,
    #[doc = "1: USB_VBUS pin is high."]
    _1 = 1,
}
impl From<VBSTS_A> for bool {
    #[inline(always)]
    fn from(variant: VBSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl VBSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBSTS_A {
        match self.bits {
            false => VBSTS_A::_0,
            true => VBSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBSTS_A::_1
    }
}
#[doc = "Field `BRDY` reader - Buffer Ready Interrupt Status"]
pub type BRDY_R = crate::BitReader<BRDY_A>;
#[doc = "Buffer Ready Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDY_A {
    #[doc = "0: BRDY interrupts are not generated."]
    _0 = 0,
    #[doc = "1: BRDY interrupts are generated."]
    _1 = 1,
}
impl From<BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDY_A {
        match self.bits {
            false => BRDY_A::_0,
            true => BRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRDY_A::_1
    }
}
#[doc = "Field `NRDY` reader - Buffer Not Ready Interrupt Status"]
pub type NRDY_R = crate::BitReader<NRDY_A>;
#[doc = "Buffer Not Ready Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRDY_A {
    #[doc = "0: NRDY interrupts are not generated."]
    _0 = 0,
    #[doc = "1: NRDY interrupts are generated."]
    _1 = 1,
}
impl From<NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl NRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDY_A {
        match self.bits {
            false => NRDY_A::_0,
            true => NRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NRDY_A::_1
    }
}
#[doc = "Field `BEMP` reader - Buffer Empty Interrupt Status"]
pub type BEMP_R = crate::BitReader<BEMP_A>;
#[doc = "Buffer Empty Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEMP_A {
    #[doc = "0: BEMP interrupts are not generated."]
    _0 = 0,
    #[doc = "1: BEMP interrupts are generated."]
    _1 = 1,
}
impl From<BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEMP_A {
        match self.bits {
            false => BEMP_A::_0,
            true => BEMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEMP_A::_1
    }
}
#[doc = "Field `CTRT` reader - Control Transfer Stage Transition Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type CTRT_R = crate::BitReader<CTRT_A>;
#[doc = "Control Transfer Stage Transition Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRT_A {
    #[doc = "0: Control transfer stage transition interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Control transfer stage transition interrupts are generated."]
    _1 = 1,
}
impl From<CTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CTRT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRT_A {
        match self.bits {
            false => CTRT_A::_0,
            true => CTRT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTRT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTRT_A::_1
    }
}
#[doc = "Field `CTRT` writer - Control Transfer Stage Transition Interrupt Status"]
pub type CTRT_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, INTSTS0_SPEC, CTRT_A, O>;
impl<'a, const O: u8> CTRT_W<'a, O> {
    #[doc = "Control transfer stage transition interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTRT_A::_0)
    }
    #[doc = "Control transfer stage transition interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTRT_A::_1)
    }
}
#[doc = "Field `DVST` reader - Device State Transition Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type DVST_R = crate::BitReader<DVST_A>;
#[doc = "Device State Transition Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVST_A {
    #[doc = "0: Device state transition interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Device state transition interrupts are generated."]
    _1 = 1,
}
impl From<DVST_A> for bool {
    #[inline(always)]
    fn from(variant: DVST_A) -> Self {
        variant as u8 != 0
    }
}
impl DVST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVST_A {
        match self.bits {
            false => DVST_A::_0,
            true => DVST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVST_A::_1
    }
}
#[doc = "Field `DVST` writer - Device State Transition Interrupt Status"]
pub type DVST_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, INTSTS0_SPEC, DVST_A, O>;
impl<'a, const O: u8> DVST_W<'a, O> {
    #[doc = "Device state transition interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVST_A::_0)
    }
    #[doc = "Device state transition interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVST_A::_1)
    }
}
#[doc = "Field `SOFR` reader - Frame Number Refresh Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type SOFR_R = crate::BitReader<SOFR_A>;
#[doc = "Frame Number Refresh Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFR_A {
    #[doc = "0: SOF interrupts are not generated."]
    _0 = 0,
    #[doc = "1: SOF interrupts are generated."]
    _1 = 1,
}
impl From<SOFR_A> for bool {
    #[inline(always)]
    fn from(variant: SOFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFR_A {
        match self.bits {
            false => SOFR_A::_0,
            true => SOFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFR_A::_1
    }
}
#[doc = "Field `SOFR` writer - Frame Number Refresh Interrupt Status"]
pub type SOFR_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, INTSTS0_SPEC, SOFR_A, O>;
impl<'a, const O: u8> SOFR_W<'a, O> {
    #[doc = "SOF interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFR_A::_0)
    }
    #[doc = "SOF interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFR_A::_1)
    }
}
#[doc = "Field `RESM` reader - Resume Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type RESM_R = crate::BitReader<RESM_A>;
#[doc = "Resume Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESM_A {
    #[doc = "0: Resume interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Resume interrupts are generated."]
    _1 = 1,
}
impl From<RESM_A> for bool {
    #[inline(always)]
    fn from(variant: RESM_A) -> Self {
        variant as u8 != 0
    }
}
impl RESM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESM_A {
        match self.bits {
            false => RESM_A::_0,
            true => RESM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESM_A::_1
    }
}
#[doc = "Field `RESM` writer - Resume Interrupt Status"]
pub type RESM_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, INTSTS0_SPEC, RESM_A, O>;
impl<'a, const O: u8> RESM_W<'a, O> {
    #[doc = "Resume interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESM_A::_0)
    }
    #[doc = "Resume interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESM_A::_1)
    }
}
#[doc = "Field `VBINT` reader - VBUS Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type VBINT_R = crate::BitReader<VBINT_A>;
#[doc = "VBUS Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBINT_A {
    #[doc = "0: VBUS interrupts are not generated."]
    _0 = 0,
    #[doc = "1: VBUS interrupts are generated."]
    _1 = 1,
}
impl From<VBINT_A> for bool {
    #[inline(always)]
    fn from(variant: VBINT_A) -> Self {
        variant as u8 != 0
    }
}
impl VBINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBINT_A {
        match self.bits {
            false => VBINT_A::_0,
            true => VBINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBINT_A::_1
    }
}
#[doc = "Field `VBINT` writer - VBUS Interrupt Status"]
pub type VBINT_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, INTSTS0_SPEC, VBINT_A, O>;
impl<'a, const O: u8> VBINT_W<'a, O> {
    #[doc = "VBUS interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBINT_A::_0)
    }
    #[doc = "VBUS interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBINT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Control Transfer Stage"]
    #[inline(always)]
    pub fn ctsq(&self) -> CTSQ_R {
        CTSQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - USB Request Reception"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Device State"]
    #[inline(always)]
    pub fn dvsq(&self) -> DVSQ_R {
        DVSQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - VBUS Input Status"]
    #[inline(always)]
    pub fn vbsts(&self) -> VBSTS_R {
        VBSTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffer Ready Interrupt Status"]
    #[inline(always)]
    pub fn brdy(&self) -> BRDY_R {
        BRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer Not Ready Interrupt Status"]
    #[inline(always)]
    pub fn nrdy(&self) -> NRDY_R {
        NRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Empty Interrupt Status"]
    #[inline(always)]
    pub fn bemp(&self) -> BEMP_R {
        BEMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Status"]
    #[inline(always)]
    pub fn ctrt(&self) -> CTRT_R {
        CTRT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Status"]
    #[inline(always)]
    pub fn dvst(&self) -> DVST_R {
        DVST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Number Refresh Interrupt Status"]
    #[inline(always)]
    pub fn sofr(&self) -> SOFR_R {
        SOFR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Resume Interrupt Status"]
    #[inline(always)]
    pub fn resm(&self) -> RESM_R {
        RESM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VBUS Interrupt Status"]
    #[inline(always)]
    pub fn vbint(&self) -> VBINT_R {
        VBINT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - USB Request Reception"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<3> {
        VALID_W::new(self)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ctrt(&mut self) -> CTRT_W<11> {
        CTRT_W::new(self)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dvst(&mut self) -> DVST_W<12> {
        DVST_W::new(self)
    }
    #[doc = "Bit 13 - Frame Number Refresh Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sofr(&mut self) -> SOFR_W<13> {
        SOFR_W::new(self)
    }
    #[doc = "Bit 14 - Resume Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn resm(&mut self) -> RESM_W<14> {
        RESM_W::new(self)
    }
    #[doc = "Bit 15 - VBUS Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn vbint(&mut self) -> VBINT_W<15> {
        VBINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts0](index.html) module"]
pub struct INTSTS0_SPEC;
impl crate::RegisterSpec for INTSTS0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intsts0::R](R) reader structure"]
impl crate::Readable for INTSTS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsts0::W](W) writer structure"]
impl crate::Writable for INTSTS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf800;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSTS0 to value 0"]
impl crate::Resettable for INTSTS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
