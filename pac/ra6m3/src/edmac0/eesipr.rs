#[doc = "Register `EESIPR` reader"]
pub struct R(crate::R<EESIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EESIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EESIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EESIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EESIPR` writer"]
pub struct W(crate::W<EESIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EESIPR_SPEC>;
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
impl From<crate::W<EESIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EESIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERFIP` reader - CRC Error Interrupt Request Enable"]
pub type CERFIP_R = crate::BitReader<CERFIP_A>;
#[doc = "CRC Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERFIP_A {
    #[doc = "0: CRC error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: CRC error interrupt request is enabled."]
    _1 = 1,
}
impl From<CERFIP_A> for bool {
    #[inline(always)]
    fn from(variant: CERFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl CERFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERFIP_A {
        match self.bits {
            false => CERFIP_A::_0,
            true => CERFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERFIP_A::_1
    }
}
#[doc = "Field `CERFIP` writer - CRC Error Interrupt Request Enable"]
pub type CERFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, CERFIP_A, O>;
impl<'a, const O: u8> CERFIP_W<'a, O> {
    #[doc = "CRC error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CERFIP_A::_0)
    }
    #[doc = "CRC error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CERFIP_A::_1)
    }
}
#[doc = "Field `PREIP` reader - PHY-LSI Receive Error Interrupt Request Enable"]
pub type PREIP_R = crate::BitReader<PREIP_A>;
#[doc = "PHY-LSI Receive Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREIP_A {
    #[doc = "0: PHY-LSI receive error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: PHY-LSI receive error interrupt request is enabled."]
    _1 = 1,
}
impl From<PREIP_A> for bool {
    #[inline(always)]
    fn from(variant: PREIP_A) -> Self {
        variant as u8 != 0
    }
}
impl PREIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREIP_A {
        match self.bits {
            false => PREIP_A::_0,
            true => PREIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PREIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PREIP_A::_1
    }
}
#[doc = "Field `PREIP` writer - PHY-LSI Receive Error Interrupt Request Enable"]
pub type PREIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, PREIP_A, O>;
impl<'a, const O: u8> PREIP_W<'a, O> {
    #[doc = "PHY-LSI receive error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PREIP_A::_0)
    }
    #[doc = "PHY-LSI receive error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PREIP_A::_1)
    }
}
#[doc = "Field `RTSFIP` reader - Frame-Too-Short Error Interrupt Request Enable"]
pub type RTSFIP_R = crate::BitReader<RTSFIP_A>;
#[doc = "Frame-Too-Short Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSFIP_A {
    #[doc = "0: Frame-too-short error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Frame-too-short error interrupt request is enabled."]
    _1 = 1,
}
impl From<RTSFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RTSFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSFIP_A {
        match self.bits {
            false => RTSFIP_A::_0,
            true => RTSFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTSFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTSFIP_A::_1
    }
}
#[doc = "Field `RTSFIP` writer - Frame-Too-Short Error Interrupt Request Enable"]
pub type RTSFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RTSFIP_A, O>;
impl<'a, const O: u8> RTSFIP_W<'a, O> {
    #[doc = "Frame-too-short error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSFIP_A::_0)
    }
    #[doc = "Frame-too-short error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSFIP_A::_1)
    }
}
#[doc = "Field `RTLFIP` reader - Frame-Too-Long Error Interrupt Request Enable"]
pub type RTLFIP_R = crate::BitReader<RTLFIP_A>;
#[doc = "Frame-Too-Long Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTLFIP_A {
    #[doc = "0: Frame-too-long error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Frame-too-long error interrupt request is enabled."]
    _1 = 1,
}
impl From<RTLFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RTLFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RTLFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTLFIP_A {
        match self.bits {
            false => RTLFIP_A::_0,
            true => RTLFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTLFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTLFIP_A::_1
    }
}
#[doc = "Field `RTLFIP` writer - Frame-Too-Long Error Interrupt Request Enable"]
pub type RTLFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RTLFIP_A, O>;
impl<'a, const O: u8> RTLFIP_W<'a, O> {
    #[doc = "Frame-too-long error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTLFIP_A::_0)
    }
    #[doc = "Frame-too-long error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTLFIP_A::_1)
    }
}
#[doc = "Field `RRFIP` reader - Alignment Error Interrupt Request Enable"]
pub type RRFIP_R = crate::BitReader<RRFIP_A>;
#[doc = "Alignment Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFIP_A {
    #[doc = "0: Alignment error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Alignment error interrupt request is enabled."]
    _1 = 1,
}
impl From<RRFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RRFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RRFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFIP_A {
        match self.bits {
            false => RRFIP_A::_0,
            true => RRFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRFIP_A::_1
    }
}
#[doc = "Field `RRFIP` writer - Alignment Error Interrupt Request Enable"]
pub type RRFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RRFIP_A, O>;
impl<'a, const O: u8> RRFIP_W<'a, O> {
    #[doc = "Alignment error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRFIP_A::_0)
    }
    #[doc = "Alignment error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRFIP_A::_1)
    }
}
#[doc = "Field `RMAFIP` reader - Multicast Address Frame Receive Interrupt Request Enable"]
pub type RMAFIP_R = crate::BitReader<RMAFIP_A>;
#[doc = "Multicast Address Frame Receive Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMAFIP_A {
    #[doc = "0: Multicast address frame receive interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Multicast address frame receive interrupt request is enabled."]
    _1 = 1,
}
impl From<RMAFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RMAFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RMAFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMAFIP_A {
        match self.bits {
            false => RMAFIP_A::_0,
            true => RMAFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMAFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMAFIP_A::_1
    }
}
#[doc = "Field `RMAFIP` writer - Multicast Address Frame Receive Interrupt Request Enable"]
pub type RMAFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RMAFIP_A, O>;
impl<'a, const O: u8> RMAFIP_W<'a, O> {
    #[doc = "Multicast address frame receive interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMAFIP_A::_0)
    }
    #[doc = "Multicast address frame receive interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMAFIP_A::_1)
    }
}
#[doc = "Field `TROIP` reader - Transmit Retry Over Interrupt Request Enable"]
pub type TROIP_R = crate::BitReader<TROIP_A>;
#[doc = "Transmit Retry Over Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROIP_A {
    #[doc = "0: Transmit retry over interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Transmit retry over interrupt request is enabled."]
    _1 = 1,
}
impl From<TROIP_A> for bool {
    #[inline(always)]
    fn from(variant: TROIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TROIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TROIP_A {
        match self.bits {
            false => TROIP_A::_0,
            true => TROIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TROIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TROIP_A::_1
    }
}
#[doc = "Field `TROIP` writer - Transmit Retry Over Interrupt Request Enable"]
pub type TROIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TROIP_A, O>;
impl<'a, const O: u8> TROIP_W<'a, O> {
    #[doc = "Transmit retry over interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TROIP_A::_0)
    }
    #[doc = "Transmit retry over interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TROIP_A::_1)
    }
}
#[doc = "Field `CDIP` reader - Late Collision Detect Interrupt Request Enable"]
pub type CDIP_R = crate::BitReader<CDIP_A>;
#[doc = "Late Collision Detect Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDIP_A {
    #[doc = "0: Late collision detect interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Late collision detect interrupt request is enabled."]
    _1 = 1,
}
impl From<CDIP_A> for bool {
    #[inline(always)]
    fn from(variant: CDIP_A) -> Self {
        variant as u8 != 0
    }
}
impl CDIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIP_A {
        match self.bits {
            false => CDIP_A::_0,
            true => CDIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDIP_A::_1
    }
}
#[doc = "Field `CDIP` writer - Late Collision Detect Interrupt Request Enable"]
pub type CDIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, CDIP_A, O>;
impl<'a, const O: u8> CDIP_W<'a, O> {
    #[doc = "Late collision detect interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDIP_A::_0)
    }
    #[doc = "Late collision detect interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDIP_A::_1)
    }
}
#[doc = "Field `DLCIP` reader - Loss of Carrier Detect Interrupt Request Enable"]
pub type DLCIP_R = crate::BitReader<DLCIP_A>;
#[doc = "Loss of Carrier Detect Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLCIP_A {
    #[doc = "0: Loss of carrier detect interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Loss of carrier detect interrupt request is enabled."]
    _1 = 1,
}
impl From<DLCIP_A> for bool {
    #[inline(always)]
    fn from(variant: DLCIP_A) -> Self {
        variant as u8 != 0
    }
}
impl DLCIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLCIP_A {
        match self.bits {
            false => DLCIP_A::_0,
            true => DLCIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLCIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLCIP_A::_1
    }
}
#[doc = "Field `DLCIP` writer - Loss of Carrier Detect Interrupt Request Enable"]
pub type DLCIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, DLCIP_A, O>;
impl<'a, const O: u8> DLCIP_W<'a, O> {
    #[doc = "Loss of carrier detect interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLCIP_A::_0)
    }
    #[doc = "Loss of carrier detect interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLCIP_A::_1)
    }
}
#[doc = "Field `CNDIP` reader - Carrier Not Detect Interrupt Request Enable"]
pub type CNDIP_R = crate::BitReader<CNDIP_A>;
#[doc = "Carrier Not Detect Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNDIP_A {
    #[doc = "0: Carrier not detect interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Carrier not detect interrupt request is enabled."]
    _1 = 1,
}
impl From<CNDIP_A> for bool {
    #[inline(always)]
    fn from(variant: CNDIP_A) -> Self {
        variant as u8 != 0
    }
}
impl CNDIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNDIP_A {
        match self.bits {
            false => CNDIP_A::_0,
            true => CNDIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNDIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNDIP_A::_1
    }
}
#[doc = "Field `CNDIP` writer - Carrier Not Detect Interrupt Request Enable"]
pub type CNDIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, CNDIP_A, O>;
impl<'a, const O: u8> CNDIP_W<'a, O> {
    #[doc = "Carrier not detect interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNDIP_A::_0)
    }
    #[doc = "Carrier not detect interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNDIP_A::_1)
    }
}
#[doc = "Field `RFOFIP` reader - Receive FIFO Overflow Interrupt Request Enable"]
pub type RFOFIP_R = crate::BitReader<RFOFIP_A>;
#[doc = "Receive FIFO Overflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOFIP_A {
    #[doc = "0: Overflow interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Overflow interrupt request is enabled."]
    _1 = 1,
}
impl From<RFOFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RFOFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOFIP_A {
        match self.bits {
            false => RFOFIP_A::_0,
            true => RFOFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOFIP_A::_1
    }
}
#[doc = "Field `RFOFIP` writer - Receive FIFO Overflow Interrupt Request Enable"]
pub type RFOFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RFOFIP_A, O>;
impl<'a, const O: u8> RFOFIP_W<'a, O> {
    #[doc = "Overflow interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOFIP_A::_0)
    }
    #[doc = "Overflow interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOFIP_A::_1)
    }
}
#[doc = "Field `RDEIP` reader - Receive Descriptor Empty Interrupt Request Enable"]
pub type RDEIP_R = crate::BitReader<RDEIP_A>;
#[doc = "Receive Descriptor Empty Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDEIP_A {
    #[doc = "0: Receive descriptor empty interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Receive descriptor empty interrupt request is enabled."]
    _1 = 1,
}
impl From<RDEIP_A> for bool {
    #[inline(always)]
    fn from(variant: RDEIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RDEIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDEIP_A {
        match self.bits {
            false => RDEIP_A::_0,
            true => RDEIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDEIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDEIP_A::_1
    }
}
#[doc = "Field `RDEIP` writer - Receive Descriptor Empty Interrupt Request Enable"]
pub type RDEIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RDEIP_A, O>;
impl<'a, const O: u8> RDEIP_W<'a, O> {
    #[doc = "Receive descriptor empty interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDEIP_A::_0)
    }
    #[doc = "Receive descriptor empty interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDEIP_A::_1)
    }
}
#[doc = "Field `FRIP` reader - Frame Receive Interrupt Request Enable"]
pub type FRIP_R = crate::BitReader<FRIP_A>;
#[doc = "Frame Receive Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRIP_A {
    #[doc = "0: Frame reception interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Frame reception interrupt request is enabled."]
    _1 = 1,
}
impl From<FRIP_A> for bool {
    #[inline(always)]
    fn from(variant: FRIP_A) -> Self {
        variant as u8 != 0
    }
}
impl FRIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRIP_A {
        match self.bits {
            false => FRIP_A::_0,
            true => FRIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRIP_A::_1
    }
}
#[doc = "Field `FRIP` writer - Frame Receive Interrupt Request Enable"]
pub type FRIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, FRIP_A, O>;
impl<'a, const O: u8> FRIP_W<'a, O> {
    #[doc = "Frame reception interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRIP_A::_0)
    }
    #[doc = "Frame reception interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRIP_A::_1)
    }
}
#[doc = "Field `TFUFIP` reader - Transmit FIFO Underflow Interrupt Request Enable"]
pub type TFUFIP_R = crate::BitReader<TFUFIP_A>;
#[doc = "Transmit FIFO Underflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUFIP_A {
    #[doc = "0: Underflow interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Underflow interrupt request is enabled."]
    _1 = 1,
}
impl From<TFUFIP_A> for bool {
    #[inline(always)]
    fn from(variant: TFUFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TFUFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUFIP_A {
        match self.bits {
            false => TFUFIP_A::_0,
            true => TFUFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUFIP_A::_1
    }
}
#[doc = "Field `TFUFIP` writer - Transmit FIFO Underflow Interrupt Request Enable"]
pub type TFUFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TFUFIP_A, O>;
impl<'a, const O: u8> TFUFIP_W<'a, O> {
    #[doc = "Underflow interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUFIP_A::_0)
    }
    #[doc = "Underflow interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUFIP_A::_1)
    }
}
#[doc = "Field `TDEIP` reader - Transmit Descriptor Empty Interrupt Request Enable"]
pub type TDEIP_R = crate::BitReader<TDEIP_A>;
#[doc = "Transmit Descriptor Empty Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDEIP_A {
    #[doc = "0: Transmit descriptor empty interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Transmit descriptor empty interrupt request is enabled."]
    _1 = 1,
}
impl From<TDEIP_A> for bool {
    #[inline(always)]
    fn from(variant: TDEIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TDEIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDEIP_A {
        match self.bits {
            false => TDEIP_A::_0,
            true => TDEIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDEIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDEIP_A::_1
    }
}
#[doc = "Field `TDEIP` writer - Transmit Descriptor Empty Interrupt Request Enable"]
pub type TDEIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TDEIP_A, O>;
impl<'a, const O: u8> TDEIP_W<'a, O> {
    #[doc = "Transmit descriptor empty interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDEIP_A::_0)
    }
    #[doc = "Transmit descriptor empty interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDEIP_A::_1)
    }
}
#[doc = "Field `TCIP` reader - Frame Transfer Complete Interrupt Request Enable"]
pub type TCIP_R = crate::BitReader<TCIP_A>;
#[doc = "Frame Transfer Complete Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIP_A {
    #[doc = "0: Frame transmission complete interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Frame transmission complete interrupt request is enabled."]
    _1 = 1,
}
impl From<TCIP_A> for bool {
    #[inline(always)]
    fn from(variant: TCIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIP_A {
        match self.bits {
            false => TCIP_A::_0,
            true => TCIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCIP_A::_1
    }
}
#[doc = "Field `TCIP` writer - Frame Transfer Complete Interrupt Request Enable"]
pub type TCIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TCIP_A, O>;
impl<'a, const O: u8> TCIP_W<'a, O> {
    #[doc = "Frame transmission complete interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIP_A::_0)
    }
    #[doc = "Frame transmission complete interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIP_A::_1)
    }
}
#[doc = "Field `ECIIP` reader - ETHERC Status Register Source Interrupt Request Enable"]
pub type ECIIP_R = crate::BitReader<ECIIP_A>;
#[doc = "ETHERC Status Register Source Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECIIP_A {
    #[doc = "0: ETHERC status interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: ETHERC status interrupt request is enabled."]
    _1 = 1,
}
impl From<ECIIP_A> for bool {
    #[inline(always)]
    fn from(variant: ECIIP_A) -> Self {
        variant as u8 != 0
    }
}
impl ECIIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECIIP_A {
        match self.bits {
            false => ECIIP_A::_0,
            true => ECIIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECIIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECIIP_A::_1
    }
}
#[doc = "Field `ECIIP` writer - ETHERC Status Register Source Interrupt Request Enable"]
pub type ECIIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, ECIIP_A, O>;
impl<'a, const O: u8> ECIIP_W<'a, O> {
    #[doc = "ETHERC status interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECIIP_A::_0)
    }
    #[doc = "ETHERC status interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECIIP_A::_1)
    }
}
#[doc = "Field `ADEIP` reader - Address Error Interrupt Request Enable"]
pub type ADEIP_R = crate::BitReader<ADEIP_A>;
#[doc = "Address Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEIP_A {
    #[doc = "0: Address error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Address error interrupt request is enabled."]
    _1 = 1,
}
impl From<ADEIP_A> for bool {
    #[inline(always)]
    fn from(variant: ADEIP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEIP_A {
        match self.bits {
            false => ADEIP_A::_0,
            true => ADEIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEIP_A::_1
    }
}
#[doc = "Field `ADEIP` writer - Address Error Interrupt Request Enable"]
pub type ADEIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, ADEIP_A, O>;
impl<'a, const O: u8> ADEIP_W<'a, O> {
    #[doc = "Address error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEIP_A::_0)
    }
    #[doc = "Address error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEIP_A::_1)
    }
}
#[doc = "Field `RFCOFIP` reader - Receive Frame Counter Overflow Interrupt Request Enable"]
pub type RFCOFIP_R = crate::BitReader<RFCOFIP_A>;
#[doc = "Receive Frame Counter Overflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCOFIP_A {
    #[doc = "0: Receive frame counter overflow interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Receive frame counter overflow interrupt request is enabled."]
    _1 = 1,
}
impl From<RFCOFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RFCOFIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RFCOFIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCOFIP_A {
        match self.bits {
            false => RFCOFIP_A::_0,
            true => RFCOFIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFCOFIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFCOFIP_A::_1
    }
}
#[doc = "Field `RFCOFIP` writer - Receive Frame Counter Overflow Interrupt Request Enable"]
pub type RFCOFIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RFCOFIP_A, O>;
impl<'a, const O: u8> RFCOFIP_W<'a, O> {
    #[doc = "Receive frame counter overflow interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFCOFIP_A::_0)
    }
    #[doc = "Receive frame counter overflow interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFCOFIP_A::_1)
    }
}
#[doc = "Field `RABTIP` reader - Receive Abort Detect Interrupt Request Enable"]
pub type RABTIP_R = crate::BitReader<RABTIP_A>;
#[doc = "Receive Abort Detect Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RABTIP_A {
    #[doc = "0: Receive abort detect interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Receive abort detect interrupt request is enabled."]
    _1 = 1,
}
impl From<RABTIP_A> for bool {
    #[inline(always)]
    fn from(variant: RABTIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RABTIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RABTIP_A {
        match self.bits {
            false => RABTIP_A::_0,
            true => RABTIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RABTIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RABTIP_A::_1
    }
}
#[doc = "Field `RABTIP` writer - Receive Abort Detect Interrupt Request Enable"]
pub type RABTIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, RABTIP_A, O>;
impl<'a, const O: u8> RABTIP_W<'a, O> {
    #[doc = "Receive abort detect interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RABTIP_A::_0)
    }
    #[doc = "Receive abort detect interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RABTIP_A::_1)
    }
}
#[doc = "Field `TABTIP` reader - Transmit Abort Detect Interrupt Request Enable"]
pub type TABTIP_R = crate::BitReader<TABTIP_A>;
#[doc = "Transmit Abort Detect Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABTIP_A {
    #[doc = "0: Transmit abort detect interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Transmit abort detect interrupt request is enabled."]
    _1 = 1,
}
impl From<TABTIP_A> for bool {
    #[inline(always)]
    fn from(variant: TABTIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TABTIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABTIP_A {
        match self.bits {
            false => TABTIP_A::_0,
            true => TABTIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABTIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABTIP_A::_1
    }
}
#[doc = "Field `TABTIP` writer - Transmit Abort Detect Interrupt Request Enable"]
pub type TABTIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TABTIP_A, O>;
impl<'a, const O: u8> TABTIP_W<'a, O> {
    #[doc = "Transmit abort detect interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABTIP_A::_0)
    }
    #[doc = "Transmit abort detect interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABTIP_A::_1)
    }
}
#[doc = "Field `TWBIP` reader - Write-Back Complete Interrupt Request Enable"]
pub type TWBIP_R = crate::BitReader<TWBIP_A>;
#[doc = "Write-Back Complete Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWBIP_A {
    #[doc = "0: Write-back complete interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Write-back complete interrupt request is enabled."]
    _1 = 1,
}
impl From<TWBIP_A> for bool {
    #[inline(always)]
    fn from(variant: TWBIP_A) -> Self {
        variant as u8 != 0
    }
}
impl TWBIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWBIP_A {
        match self.bits {
            false => TWBIP_A::_0,
            true => TWBIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWBIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWBIP_A::_1
    }
}
#[doc = "Field `TWBIP` writer - Write-Back Complete Interrupt Request Enable"]
pub type TWBIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESIPR_SPEC, TWBIP_A, O>;
impl<'a, const O: u8> TWBIP_W<'a, O> {
    #[doc = "Write-back complete interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWBIP_A::_0)
    }
    #[doc = "Write-back complete interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWBIP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn cerfip(&self) -> CERFIP_R {
        CERFIP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PHY-LSI Receive Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn preip(&self) -> PREIP_R {
        PREIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame-Too-Short Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn rtsfip(&self) -> RTSFIP_R {
        RTSFIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame-Too-Long Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn rtlfip(&self) -> RTLFIP_R {
        RTLFIP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alignment Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn rrfip(&self) -> RRFIP_R {
        RRFIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Multicast Address Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    pub fn rmafip(&self) -> RMAFIP_R {
        RMAFIP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Retry Over Interrupt Request Enable"]
    #[inline(always)]
    pub fn troip(&self) -> TROIP_R {
        TROIP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Late Collision Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn cdip(&self) -> CDIP_R {
        CDIP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Loss of Carrier Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn dlcip(&self) -> DLCIP_R {
        DLCIP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Carrier Not Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn cndip(&self) -> CNDIP_R {
        CNDIP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn rfofip(&self) -> RFOFIP_R {
        RFOFIP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn rdeip(&self) -> RDEIP_R {
        RDEIP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    pub fn frip(&self) -> FRIP_R {
        FRIP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn tfufip(&self) -> TFUFIP_R {
        TFUFIP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn tdeip(&self) -> TDEIP_R {
        TDEIP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Interrupt Request Enable"]
    #[inline(always)]
    pub fn tcip(&self) -> TCIP_R {
        TCIP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ETHERC Status Register Source Interrupt Request Enable"]
    #[inline(always)]
    pub fn eciip(&self) -> ECIIP_R {
        ECIIP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Address Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn adeip(&self) -> ADEIP_R {
        ADEIP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn rfcofip(&self) -> RFCOFIP_R {
        RFCOFIP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn rabtip(&self) -> RABTIP_R {
        RABTIP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn tabtip(&self) -> TABTIP_R {
        TABTIP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Write-Back Complete Interrupt Request Enable"]
    #[inline(always)]
    pub fn twbip(&self) -> TWBIP_R {
        TWBIP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cerfip(&mut self) -> CERFIP_W<0> {
        CERFIP_W::new(self)
    }
    #[doc = "Bit 1 - PHY-LSI Receive Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn preip(&mut self) -> PREIP_W<1> {
        PREIP_W::new(self)
    }
    #[doc = "Bit 2 - Frame-Too-Short Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsfip(&mut self) -> RTSFIP_W<2> {
        RTSFIP_W::new(self)
    }
    #[doc = "Bit 3 - Frame-Too-Long Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtlfip(&mut self) -> RTLFIP_W<3> {
        RTLFIP_W::new(self)
    }
    #[doc = "Bit 4 - Alignment Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrfip(&mut self) -> RRFIP_W<4> {
        RRFIP_W::new(self)
    }
    #[doc = "Bit 7 - Multicast Address Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmafip(&mut self) -> RMAFIP_W<7> {
        RMAFIP_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Retry Over Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn troip(&mut self) -> TROIP_W<8> {
        TROIP_W::new(self)
    }
    #[doc = "Bit 9 - Late Collision Detect Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdip(&mut self) -> CDIP_W<9> {
        CDIP_W::new(self)
    }
    #[doc = "Bit 10 - Loss of Carrier Detect Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlcip(&mut self) -> DLCIP_W<10> {
        DLCIP_W::new(self)
    }
    #[doc = "Bit 11 - Carrier Not Detect Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cndip(&mut self) -> CNDIP_W<11> {
        CNDIP_W::new(self)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfofip(&mut self) -> RFOFIP_W<16> {
        RFOFIP_W::new(self)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdeip(&mut self) -> RDEIP_W<17> {
        RDEIP_W::new(self)
    }
    #[doc = "Bit 18 - Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frip(&mut self) -> FRIP_W<18> {
        FRIP_W::new(self)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfufip(&mut self) -> TFUFIP_W<19> {
        TFUFIP_W::new(self)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdeip(&mut self) -> TDEIP_W<20> {
        TDEIP_W::new(self)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcip(&mut self) -> TCIP_W<21> {
        TCIP_W::new(self)
    }
    #[doc = "Bit 22 - ETHERC Status Register Source Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eciip(&mut self) -> ECIIP_W<22> {
        ECIIP_W::new(self)
    }
    #[doc = "Bit 23 - Address Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adeip(&mut self) -> ADEIP_W<23> {
        ADEIP_W::new(self)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfcofip(&mut self) -> RFCOFIP_W<24> {
        RFCOFIP_W::new(self)
    }
    #[doc = "Bit 25 - Receive Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rabtip(&mut self) -> RABTIP_W<25> {
        RABTIP_W::new(self)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tabtip(&mut self) -> TABTIP_W<26> {
        TABTIP_W::new(self)
    }
    #[doc = "Bit 30 - Write-Back Complete Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twbip(&mut self) -> TWBIP_W<30> {
        TWBIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETHERC/EDMAC Status Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eesipr](index.html) module"]
pub struct EESIPR_SPEC;
impl crate::RegisterSpec for EESIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eesipr::R](R) reader structure"]
impl crate::Readable for EESIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eesipr::W](W) writer structure"]
impl crate::Writable for EESIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EESIPR to value 0"]
impl crate::Resettable for EESIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
