#[doc = "Register `CFDC0CTR` reader"]
pub struct R(crate::R<CFDC0CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0CTR` writer"]
pub struct W(crate::W<CFDC0CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0CTR_SPEC>;
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
impl From<crate::W<CFDC0CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMDC` reader - Channel Mode Control"]
pub type CHMDC_R = crate::FieldReader<u8, CHMDC_A>;
#[doc = "Channel Mode Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMDC_A {
    #[doc = "0: Channel operation mode request"]
    _00 = 0,
    #[doc = "1: Channel reset request"]
    _01 = 1,
    #[doc = "2: Channel halt request"]
    _10 = 2,
    #[doc = "3: Keep current value"]
    _11 = 3,
}
impl From<CHMDC_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMDC_A) -> Self {
        variant as _
    }
}
impl CHMDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMDC_A {
        match self.bits {
            0 => CHMDC_A::_00,
            1 => CHMDC_A::_01,
            2 => CHMDC_A::_10,
            3 => CHMDC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CHMDC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CHMDC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CHMDC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CHMDC_A::_11
    }
}
#[doc = "Field `CHMDC` writer - Channel Mode Control"]
pub type CHMDC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDC0CTR_SPEC, u8, CHMDC_A, 2, O>;
impl<'a, const O: u8> CHMDC_W<'a, O> {
    #[doc = "Channel operation mode request"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CHMDC_A::_00)
    }
    #[doc = "Channel reset request"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CHMDC_A::_01)
    }
    #[doc = "Channel halt request"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CHMDC_A::_10)
    }
    #[doc = "Keep current value"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CHMDC_A::_11)
    }
}
#[doc = "Field `CSLPR` reader - Channel Sleep Request"]
pub type CSLPR_R = crate::BitReader<CSLPR_A>;
#[doc = "Channel Sleep Request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSLPR_A {
    #[doc = "0: Channel sleep request disabled"]
    _0 = 0,
    #[doc = "1: Channel sleep request enabled"]
    _1 = 1,
}
impl From<CSLPR_A> for bool {
    #[inline(always)]
    fn from(variant: CSLPR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSLPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSLPR_A {
        match self.bits {
            false => CSLPR_A::_0,
            true => CSLPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSLPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSLPR_A::_1
    }
}
#[doc = "Field `CSLPR` writer - Channel Sleep Request"]
pub type CSLPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, CSLPR_A, O>;
impl<'a, const O: u8> CSLPR_W<'a, O> {
    #[doc = "Channel sleep request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSLPR_A::_0)
    }
    #[doc = "Channel sleep request enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSLPR_A::_1)
    }
}
#[doc = "Field `RTBO` reader - Return from Bus-Off"]
pub type RTBO_R = crate::BitReader<RTBO_A>;
#[doc = "Return from Bus-Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTBO_A {
    #[doc = "0: Channel is not forced to return from bus-off"]
    _0 = 0,
    #[doc = "1: Channel is forced to return from bus-off"]
    _1 = 1,
}
impl From<RTBO_A> for bool {
    #[inline(always)]
    fn from(variant: RTBO_A) -> Self {
        variant as u8 != 0
    }
}
impl RTBO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTBO_A {
        match self.bits {
            false => RTBO_A::_0,
            true => RTBO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTBO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTBO_A::_1
    }
}
#[doc = "Field `RTBO` writer - Return from Bus-Off"]
pub type RTBO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, RTBO_A, O>;
impl<'a, const O: u8> RTBO_W<'a, O> {
    #[doc = "Channel is not forced to return from bus-off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTBO_A::_0)
    }
    #[doc = "Channel is forced to return from bus-off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTBO_A::_1)
    }
}
#[doc = "Field `BEIE` reader - Bus Error Interrupt Enable"]
pub type BEIE_R = crate::BitReader<BEIE_A>;
#[doc = "Bus Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEIE_A {
    #[doc = "0: Bus error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus error interrupt enabled"]
    _1 = 1,
}
impl From<BEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEIE_A {
        match self.bits {
            false => BEIE_A::_0,
            true => BEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEIE_A::_1
    }
}
#[doc = "Field `BEIE` writer - Bus Error Interrupt Enable"]
pub type BEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, BEIE_A, O>;
impl<'a, const O: u8> BEIE_W<'a, O> {
    #[doc = "Bus error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEIE_A::_0)
    }
    #[doc = "Bus error interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEIE_A::_1)
    }
}
#[doc = "Field `EWIE` reader - Error Warning Interrupt Enable"]
pub type EWIE_R = crate::BitReader<EWIE_A>;
#[doc = "Error Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIE_A {
    #[doc = "0: Error warning interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error warning interrupt enabled"]
    _1 = 1,
}
impl From<EWIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWIE_A {
        match self.bits {
            false => EWIE_A::_0,
            true => EWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWIE_A::_1
    }
}
#[doc = "Field `EWIE` writer - Error Warning Interrupt Enable"]
pub type EWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, EWIE_A, O>;
impl<'a, const O: u8> EWIE_W<'a, O> {
    #[doc = "Error warning interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EWIE_A::_0)
    }
    #[doc = "Error warning interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EWIE_A::_1)
    }
}
#[doc = "Field `EPIE` reader - Error Passive Interrupt Enable"]
pub type EPIE_R = crate::BitReader<EPIE_A>;
#[doc = "Error Passive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIE_A {
    #[doc = "0: Error passive interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error passive interrupt enabled"]
    _1 = 1,
}
impl From<EPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIE_A {
        match self.bits {
            false => EPIE_A::_0,
            true => EPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPIE_A::_1
    }
}
#[doc = "Field `EPIE` writer - Error Passive Interrupt Enable"]
pub type EPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, EPIE_A, O>;
impl<'a, const O: u8> EPIE_W<'a, O> {
    #[doc = "Error passive interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPIE_A::_0)
    }
    #[doc = "Error passive interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPIE_A::_1)
    }
}
#[doc = "Field `BOEIE` reader - Bus-Off Entry Interrupt Enable"]
pub type BOEIE_R = crate::BitReader<BOEIE_A>;
#[doc = "Bus-Off Entry Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOEIE_A {
    #[doc = "0: Bus-off entry interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus-off entry interrupt enabled"]
    _1 = 1,
}
impl From<BOEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BOEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOEIE_A {
        match self.bits {
            false => BOEIE_A::_0,
            true => BOEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOEIE_A::_1
    }
}
#[doc = "Field `BOEIE` writer - Bus-Off Entry Interrupt Enable"]
pub type BOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, BOEIE_A, O>;
impl<'a, const O: u8> BOEIE_W<'a, O> {
    #[doc = "Bus-off entry interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOEIE_A::_0)
    }
    #[doc = "Bus-off entry interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOEIE_A::_1)
    }
}
#[doc = "Field `BORIE` reader - Bus-Off Recovery Interrupt Enable"]
pub type BORIE_R = crate::BitReader<BORIE_A>;
#[doc = "Bus-Off Recovery Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORIE_A {
    #[doc = "0: Bus-off recovery interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus-off recovery interrupt enabled"]
    _1 = 1,
}
impl From<BORIE_A> for bool {
    #[inline(always)]
    fn from(variant: BORIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BORIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORIE_A {
        match self.bits {
            false => BORIE_A::_0,
            true => BORIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BORIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BORIE_A::_1
    }
}
#[doc = "Field `BORIE` writer - Bus-Off Recovery Interrupt Enable"]
pub type BORIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, BORIE_A, O>;
impl<'a, const O: u8> BORIE_W<'a, O> {
    #[doc = "Bus-off recovery interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BORIE_A::_0)
    }
    #[doc = "Bus-off recovery interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BORIE_A::_1)
    }
}
#[doc = "Field `OLIE` reader - Overload Interrupt Enable"]
pub type OLIE_R = crate::BitReader<OLIE_A>;
#[doc = "Overload Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OLIE_A {
    #[doc = "0: Overload interrupt disabled"]
    _0 = 0,
    #[doc = "1: Overload interrupt enabled"]
    _1 = 1,
}
impl From<OLIE_A> for bool {
    #[inline(always)]
    fn from(variant: OLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OLIE_A {
        match self.bits {
            false => OLIE_A::_0,
            true => OLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OLIE_A::_1
    }
}
#[doc = "Field `OLIE` writer - Overload Interrupt Enable"]
pub type OLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, OLIE_A, O>;
impl<'a, const O: u8> OLIE_W<'a, O> {
    #[doc = "Overload interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OLIE_A::_0)
    }
    #[doc = "Overload interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OLIE_A::_1)
    }
}
#[doc = "Field `BLIE` reader - Bus Lock Interrupt Enable"]
pub type BLIE_R = crate::BitReader<BLIE_A>;
#[doc = "Bus Lock Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLIE_A {
    #[doc = "0: Bus lock interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus lock interrupt enabled"]
    _1 = 1,
}
impl From<BLIE_A> for bool {
    #[inline(always)]
    fn from(variant: BLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLIE_A {
        match self.bits {
            false => BLIE_A::_0,
            true => BLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLIE_A::_1
    }
}
#[doc = "Field `BLIE` writer - Bus Lock Interrupt Enable"]
pub type BLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, BLIE_A, O>;
impl<'a, const O: u8> BLIE_W<'a, O> {
    #[doc = "Bus lock interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLIE_A::_0)
    }
    #[doc = "Bus lock interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLIE_A::_1)
    }
}
#[doc = "Field `ALIE` reader - Arbitration Lost Interrupt Enable"]
pub type ALIE_R = crate::BitReader<ALIE_A>;
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIE_A {
    #[doc = "0: Arbitration lost interrupt disabled"]
    _0 = 0,
    #[doc = "1: Arbitration lost interrupt enabled"]
    _1 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::_0,
            true => ALIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALIE_A::_1
    }
}
#[doc = "Field `ALIE` writer - Arbitration Lost Interrupt Enable"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, ALIE_A, O>;
impl<'a, const O: u8> ALIE_W<'a, O> {
    #[doc = "Arbitration lost interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIE_A::_0)
    }
    #[doc = "Arbitration lost interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIE_A::_1)
    }
}
#[doc = "Field `TAIE` reader - Transmission Abort Interrupt Enable"]
pub type TAIE_R = crate::BitReader<TAIE_A>;
#[doc = "Transmission Abort Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAIE_A {
    #[doc = "0: TX abort interrupt disabled"]
    _0 = 0,
    #[doc = "1: TX abort interrupt enabled"]
    _1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::_0,
            true => TAIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TAIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAIE_A::_1
    }
}
#[doc = "Field `TAIE` writer - Transmission Abort Interrupt Enable"]
pub type TAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, TAIE_A, O>;
impl<'a, const O: u8> TAIE_W<'a, O> {
    #[doc = "TX abort interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAIE_A::_0)
    }
    #[doc = "TX abort interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIE_A::_1)
    }
}
#[doc = "Field `EOCOIE` reader - Error Occurrence Counter Overflow Interrupt Enable"]
pub type EOCOIE_R = crate::BitReader<EOCOIE_A>;
#[doc = "Error Occurrence Counter Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCOIE_A {
    #[doc = "0: Error occurrence counter overflow interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error occurrence counter overflow interrupt enabled"]
    _1 = 1,
}
impl From<EOCOIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCOIE_A {
        match self.bits {
            false => EOCOIE_A::_0,
            true => EOCOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOCOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOCOIE_A::_1
    }
}
#[doc = "Field `EOCOIE` writer - Error Occurrence Counter Overflow Interrupt Enable"]
pub type EOCOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, EOCOIE_A, O>;
impl<'a, const O: u8> EOCOIE_W<'a, O> {
    #[doc = "Error occurrence counter overflow interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOCOIE_A::_0)
    }
    #[doc = "Error occurrence counter overflow interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOCOIE_A::_1)
    }
}
#[doc = "Field `SOCOIE` reader - Successful Occurrence Counter Overflow Interrupt Enable"]
pub type SOCOIE_R = crate::BitReader<SOCOIE_A>;
#[doc = "Successful Occurrence Counter Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOCOIE_A {
    #[doc = "0: Successful occurrence counter overflow interrupt disabled"]
    _0 = 0,
    #[doc = "1: Successful occurrence counter overflow interrupt enabled"]
    _1 = 1,
}
impl From<SOCOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOCOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOCOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOCOIE_A {
        match self.bits {
            false => SOCOIE_A::_0,
            true => SOCOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOCOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOCOIE_A::_1
    }
}
#[doc = "Field `SOCOIE` writer - Successful Occurrence Counter Overflow Interrupt Enable"]
pub type SOCOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, SOCOIE_A, O>;
impl<'a, const O: u8> SOCOIE_W<'a, O> {
    #[doc = "Successful occurrence counter overflow interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOCOIE_A::_0)
    }
    #[doc = "Successful occurrence counter overflow interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOCOIE_A::_1)
    }
}
#[doc = "Field `TDCVFIE` reader - Transceiver Delay Compensation Violation Interrupt Enable"]
pub type TDCVFIE_R = crate::BitReader<TDCVFIE_A>;
#[doc = "Transceiver Delay Compensation Violation Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCVFIE_A {
    #[doc = "0: Transceiver delay compensation violation interrupt disabled"]
    _0 = 0,
    #[doc = "1: Transceiver delay compensation violation interrupt enabled"]
    _1 = 1,
}
impl From<TDCVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDCVFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDCVFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCVFIE_A {
        match self.bits {
            false => TDCVFIE_A::_0,
            true => TDCVFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDCVFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDCVFIE_A::_1
    }
}
#[doc = "Field `TDCVFIE` writer - Transceiver Delay Compensation Violation Interrupt Enable"]
pub type TDCVFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, TDCVFIE_A, O>;
impl<'a, const O: u8> TDCVFIE_W<'a, O> {
    #[doc = "Transceiver delay compensation violation interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCVFIE_A::_0)
    }
    #[doc = "Transceiver delay compensation violation interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCVFIE_A::_1)
    }
}
#[doc = "Field `BOM` reader - Channel Bus-Off Mode"]
pub type BOM_R = crate::FieldReader<u8, BOM_A>;
#[doc = "Channel Bus-Off Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOM_A {
    #[doc = "0: Normal mode (comply with ISO 11898-1)"]
    _00 = 0,
    #[doc = "1: Entry to Halt mode automatically at bus-off start"]
    _01 = 1,
    #[doc = "2: Entry to Halt mode automatically at bus-off end"]
    _10 = 2,
    #[doc = "3: Entry to Halt mode (during bus-off recovery period) by software"]
    _11 = 3,
}
impl From<BOM_A> for u8 {
    #[inline(always)]
    fn from(variant: BOM_A) -> Self {
        variant as _
    }
}
impl BOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOM_A {
        match self.bits {
            0 => BOM_A::_00,
            1 => BOM_A::_01,
            2 => BOM_A::_10,
            3 => BOM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BOM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BOM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BOM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BOM_A::_11
    }
}
#[doc = "Field `BOM` writer - Channel Bus-Off Mode"]
pub type BOM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFDC0CTR_SPEC, u8, BOM_A, 2, O>;
impl<'a, const O: u8> BOM_W<'a, O> {
    #[doc = "Normal mode (comply with ISO 11898-1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BOM_A::_00)
    }
    #[doc = "Entry to Halt mode automatically at bus-off start"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BOM_A::_01)
    }
    #[doc = "Entry to Halt mode automatically at bus-off end"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BOM_A::_10)
    }
    #[doc = "Entry to Halt mode (during bus-off recovery period) by software"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BOM_A::_11)
    }
}
#[doc = "Field `ERRD` reader - Channel Error Display"]
pub type ERRD_R = crate::BitReader<ERRD_A>;
#[doc = "Channel Error Display\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRD_A {
    #[doc = "0: Only the first set of error codes displayed"]
    _0 = 0,
    #[doc = "1: Accumulated error codes displayed"]
    _1 = 1,
}
impl From<ERRD_A> for bool {
    #[inline(always)]
    fn from(variant: ERRD_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRD_A {
        match self.bits {
            false => ERRD_A::_0,
            true => ERRD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRD_A::_1
    }
}
#[doc = "Field `ERRD` writer - Channel Error Display"]
pub type ERRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, ERRD_A, O>;
impl<'a, const O: u8> ERRD_W<'a, O> {
    #[doc = "Only the first set of error codes displayed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRD_A::_0)
    }
    #[doc = "Accumulated error codes displayed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRD_A::_1)
    }
}
#[doc = "Field `CTME` reader - Channel Test Mode Enable"]
pub type CTME_R = crate::BitReader<CTME_A>;
#[doc = "Channel Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTME_A {
    #[doc = "0: Channel test mode disabled"]
    _0 = 0,
    #[doc = "1: Channel test mode enabled"]
    _1 = 1,
}
impl From<CTME_A> for bool {
    #[inline(always)]
    fn from(variant: CTME_A) -> Self {
        variant as u8 != 0
    }
}
impl CTME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTME_A {
        match self.bits {
            false => CTME_A::_0,
            true => CTME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTME_A::_1
    }
}
#[doc = "Field `CTME` writer - Channel Test Mode Enable"]
pub type CTME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, CTME_A, O>;
impl<'a, const O: u8> CTME_W<'a, O> {
    #[doc = "Channel test mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTME_A::_0)
    }
    #[doc = "Channel test mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTME_A::_1)
    }
}
#[doc = "Field `CTMS` reader - Channel Test Mode Select"]
pub type CTMS_R = crate::FieldReader<u8, CTMS_A>;
#[doc = "Channel Test Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTMS_A {
    #[doc = "0: Basic test mode"]
    _00 = 0,
    #[doc = "1: Listen-only mode"]
    _01 = 1,
    #[doc = "2: Self-test mode 0 (External loopback mode)"]
    _10 = 2,
    #[doc = "3: Self-test mode 1 (Internal loopback mode)"]
    _11 = 3,
}
impl From<CTMS_A> for u8 {
    #[inline(always)]
    fn from(variant: CTMS_A) -> Self {
        variant as _
    }
}
impl CTMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTMS_A {
        match self.bits {
            0 => CTMS_A::_00,
            1 => CTMS_A::_01,
            2 => CTMS_A::_10,
            3 => CTMS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTMS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTMS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTMS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTMS_A::_11
    }
}
#[doc = "Field `CTMS` writer - Channel Test Mode Select"]
pub type CTMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFDC0CTR_SPEC, u8, CTMS_A, 2, O>;
impl<'a, const O: u8> CTMS_W<'a, O> {
    #[doc = "Basic test mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CTMS_A::_00)
    }
    #[doc = "Listen-only mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CTMS_A::_01)
    }
    #[doc = "Self-test mode 0 (External loopback mode)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CTMS_A::_10)
    }
    #[doc = "Self-test mode 1 (Internal loopback mode)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CTMS_A::_11)
    }
}
#[doc = "Field `BFT` reader - Bit Flip Test"]
pub type BFT_R = crate::BitReader<BFT_A>;
#[doc = "Bit Flip Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFT_A {
    #[doc = "0: First data bit of reception stream not inverted"]
    _0 = 0,
    #[doc = "1: First data bit of reception stream inverted"]
    _1 = 1,
}
impl From<BFT_A> for bool {
    #[inline(always)]
    fn from(variant: BFT_A) -> Self {
        variant as u8 != 0
    }
}
impl BFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFT_A {
        match self.bits {
            false => BFT_A::_0,
            true => BFT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFT_A::_1
    }
}
#[doc = "Field `BFT` writer - Bit Flip Test"]
pub type BFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, BFT_A, O>;
impl<'a, const O: u8> BFT_W<'a, O> {
    #[doc = "First data bit of reception stream not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFT_A::_0)
    }
    #[doc = "First data bit of reception stream inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFT_A::_1)
    }
}
#[doc = "Field `ROM` reader - Restricted Operation Mode"]
pub type ROM_R = crate::BitReader<ROM_A>;
#[doc = "Restricted Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_A {
    #[doc = "0: Restricted operation mode disabled"]
    _0 = 0,
    #[doc = "1: Restricted operation mode enabled"]
    _1 = 1,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::_0,
            true => ROM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROM_A::_1
    }
}
#[doc = "Field `ROM` writer - Restricted Operation Mode"]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0CTR_SPEC, ROM_A, O>;
impl<'a, const O: u8> ROM_W<'a, O> {
    #[doc = "Restricted operation mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROM_A::_0)
    }
    #[doc = "Restricted operation mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel Mode Control"]
    #[inline(always)]
    pub fn chmdc(&self) -> CHMDC_R {
        CHMDC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel Sleep Request"]
    #[inline(always)]
    pub fn cslpr(&self) -> CSLPR_R {
        CSLPR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Return from Bus-Off"]
    #[inline(always)]
    pub fn rtbo(&self) -> RTBO_R {
        RTBO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Warning Interrupt Enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub fn boeie(&self) -> BOEIE_R {
        BOEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub fn borie(&self) -> BORIE_R {
        BORIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overload Interrupt Enable"]
    #[inline(always)]
    pub fn olie(&self) -> OLIE_R {
        OLIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub fn blie(&self) -> BLIE_R {
        BLIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Abort Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error Occurrence Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn eocoie(&self) -> EOCOIE_R {
        EOCOIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Successful Occurrence Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn socoie(&self) -> SOCOIE_R {
        SOCOIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transceiver Delay Compensation Violation Interrupt Enable"]
    #[inline(always)]
    pub fn tdcvfie(&self) -> TDCVFIE_R {
        TDCVFIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Channel Bus-Off Mode"]
    #[inline(always)]
    pub fn bom(&self) -> BOM_R {
        BOM_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Channel Error Display"]
    #[inline(always)]
    pub fn errd(&self) -> ERRD_R {
        ERRD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel Test Mode Enable"]
    #[inline(always)]
    pub fn ctme(&self) -> CTME_R {
        CTME_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Channel Test Mode Select"]
    #[inline(always)]
    pub fn ctms(&self) -> CTMS_R {
        CTMS_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 30 - Bit Flip Test"]
    #[inline(always)]
    pub fn bft(&self) -> BFT_R {
        BFT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Restricted Operation Mode"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn chmdc(&mut self) -> CHMDC_W<0> {
        CHMDC_W::new(self)
    }
    #[doc = "Bit 2 - Channel Sleep Request"]
    #[inline(always)]
    #[must_use]
    pub fn cslpr(&mut self) -> CSLPR_W<2> {
        CSLPR_W::new(self)
    }
    #[doc = "Bit 3 - Return from Bus-Off"]
    #[inline(always)]
    #[must_use]
    pub fn rtbo(&mut self) -> RTBO_W<3> {
        RTBO_W::new(self)
    }
    #[doc = "Bit 8 - Bus Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn beie(&mut self) -> BEIE_W<8> {
        BEIE_W::new(self)
    }
    #[doc = "Bit 9 - Error Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<9> {
        EWIE_W::new(self)
    }
    #[doc = "Bit 10 - Error Passive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epie(&mut self) -> EPIE_W<10> {
        EPIE_W::new(self)
    }
    #[doc = "Bit 11 - Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn boeie(&mut self) -> BOEIE_W<11> {
        BOEIE_W::new(self)
    }
    #[doc = "Bit 12 - Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn borie(&mut self) -> BORIE_W<12> {
        BORIE_W::new(self)
    }
    #[doc = "Bit 13 - Overload Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn olie(&mut self) -> OLIE_W<13> {
        OLIE_W::new(self)
    }
    #[doc = "Bit 14 - Bus Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blie(&mut self) -> BLIE_W<14> {
        BLIE_W::new(self)
    }
    #[doc = "Bit 15 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<15> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 16 - Transmission Abort Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taie(&mut self) -> TAIE_W<16> {
        TAIE_W::new(self)
    }
    #[doc = "Bit 17 - Error Occurrence Counter Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocoie(&mut self) -> EOCOIE_W<17> {
        EOCOIE_W::new(self)
    }
    #[doc = "Bit 18 - Successful Occurrence Counter Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn socoie(&mut self) -> SOCOIE_W<18> {
        SOCOIE_W::new(self)
    }
    #[doc = "Bit 19 - Transceiver Delay Compensation Violation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcvfie(&mut self) -> TDCVFIE_W<19> {
        TDCVFIE_W::new(self)
    }
    #[doc = "Bits 21:22 - Channel Bus-Off Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bom(&mut self) -> BOM_W<21> {
        BOM_W::new(self)
    }
    #[doc = "Bit 23 - Channel Error Display"]
    #[inline(always)]
    #[must_use]
    pub fn errd(&mut self) -> ERRD_W<23> {
        ERRD_W::new(self)
    }
    #[doc = "Bit 24 - Channel Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctme(&mut self) -> CTME_W<24> {
        CTME_W::new(self)
    }
    #[doc = "Bits 25:26 - Channel Test Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ctms(&mut self) -> CTMS_W<25> {
        CTMS_W::new(self)
    }
    #[doc = "Bit 30 - Bit Flip Test"]
    #[inline(always)]
    #[must_use]
    pub fn bft(&mut self) -> BFT_W<30> {
        BFT_W::new(self)
    }
    #[doc = "Bit 31 - Restricted Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<31> {
        ROM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0ctr](index.html) module"]
pub struct CFDC0CTR_SPEC;
impl crate::RegisterSpec for CFDC0CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0ctr::R](R) reader structure"]
impl crate::Readable for CFDC0CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0ctr::W](W) writer structure"]
impl crate::Writable for CFDC0CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0CTR to value 0x05"]
impl crate::Resettable for CFDC0CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
