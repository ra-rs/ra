#[doc = "Register `EESR` reader"]
pub struct R(crate::R<EESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EESR` writer"]
pub struct W(crate::W<EESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EESR_SPEC>;
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
impl From<crate::W<EESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYPE` reader - PTP v2 Message Type Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "PTP v2 Message Type Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Sync"]
    _0000 = 0,
    #[doc = "1: Delay_Req"]
    _0001 = 1,
    #[doc = "2: Pdelay_Req"]
    _0010 = 2,
    #[doc = "3: Pdelay_Resp"]
    _0011 = 3,
    #[doc = "8: Follow_Up"]
    _1000 = 8,
    #[doc = "9: Delay_Resp"]
    _1001 = 9,
    #[doc = "10: Pdelay_Resp_Follow_Up"]
    _1010 = 10,
    #[doc = "11: Announce"]
    _1011 = 11,
    #[doc = "12: Signaling"]
    _1100 = 12,
    #[doc = "13: Management"]
    _1101 = 13,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPE_A> {
        match self.bits {
            0 => Some(TYPE_A::_0000),
            1 => Some(TYPE_A::_0001),
            2 => Some(TYPE_A::_0010),
            3 => Some(TYPE_A::_0011),
            8 => Some(TYPE_A::_1000),
            9 => Some(TYPE_A::_1001),
            10 => Some(TYPE_A::_1010),
            11 => Some(TYPE_A::_1011),
            12 => Some(TYPE_A::_1100),
            13 => Some(TYPE_A::_1101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TYPE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TYPE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TYPE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TYPE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == TYPE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TYPE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TYPE_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == TYPE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == TYPE_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TYPE_A::_1101
    }
}
#[doc = "Field `TYPE` writer - PTP v2 Message Type Flag"]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EESR_SPEC, u8, TYPE_A, 4, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "Sync"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TYPE_A::_0000)
    }
    #[doc = "Delay_Req"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TYPE_A::_0001)
    }
    #[doc = "Pdelay_Req"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TYPE_A::_0010)
    }
    #[doc = "Pdelay_Resp"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TYPE_A::_0011)
    }
    #[doc = "Follow_Up"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TYPE_A::_1000)
    }
    #[doc = "Delay_Resp"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TYPE_A::_1001)
    }
    #[doc = "Pdelay_Resp_Follow_Up"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TYPE_A::_1010)
    }
    #[doc = "Announce"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TYPE_A::_1011)
    }
    #[doc = "Signaling"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TYPE_A::_1100)
    }
    #[doc = "Management"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TYPE_A::_1101)
    }
}
#[doc = "Field `PVER` reader - PTP v2 Packet Flag\n\nThe field is **modified** in some way after a read operation."]
pub type PVER_R = crate::BitReader<PVER_A>;
#[doc = "PTP v2 Packet Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVER_A {
    #[doc = "0: The current packet is not a PTP v2 packet."]
    _0 = 0,
    #[doc = "1: The current packet is a PTP v2 packet."]
    _1 = 1,
}
impl From<PVER_A> for bool {
    #[inline(always)]
    fn from(variant: PVER_A) -> Self {
        variant as u8 != 0
    }
}
impl PVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVER_A {
        match self.bits {
            false => PVER_A::_0,
            true => PVER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PVER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PVER_A::_1
    }
}
#[doc = "Field `PVER` writer - PTP v2 Packet Flag"]
pub type PVER_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, PVER_A, O>;
impl<'a, const O: u8> PVER_W<'a, O> {
    #[doc = "The current packet is not a PTP v2 packet."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PVER_A::_0)
    }
    #[doc = "The current packet is a PTP v2 packet."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PVER_A::_1)
    }
}
#[doc = "Field `MACE` reader - MAC Address Mismatch Flag\n\nThe field is **modified** in some way after a read operation."]
pub type MACE_R = crate::BitReader<MACE_A>;
#[doc = "MAC Address Mismatch Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MACE_A {
    #[doc = "0: The source MAC address of transmit frame data matches the set value."]
    _0 = 0,
    #[doc = "1: The source MAC address of transmit frame data does not match the set value."]
    _1 = 1,
}
impl From<MACE_A> for bool {
    #[inline(always)]
    fn from(variant: MACE_A) -> Self {
        variant as u8 != 0
    }
}
impl MACE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MACE_A {
        match self.bits {
            false => MACE_A::_0,
            true => MACE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MACE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MACE_A::_1
    }
}
#[doc = "Field `MACE` writer - MAC Address Mismatch Flag"]
pub type MACE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, MACE_A, O>;
impl<'a, const O: u8> MACE_W<'a, O> {
    #[doc = "The source MAC address of transmit frame data matches the set value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MACE_A::_0)
    }
    #[doc = "The source MAC address of transmit frame data does not match the set value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MACE_A::_1)
    }
}
#[doc = "Field `RFOF` reader - Receive FIFO Overflow Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RFOF_R = crate::BitReader<RFOF_A>;
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOF_A {
    #[doc = "0: Overflow has not occurred."]
    _0 = 0,
    #[doc = "1: Overflow has occurred."]
    _1 = 1,
}
impl From<RFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOF_A {
        match self.bits {
            false => RFOF_A::_0,
            true => RFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOF_A::_1
    }
}
#[doc = "Field `RFOF` writer - Receive FIFO Overflow Flag"]
pub type RFOF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, RFOF_A, O>;
impl<'a, const O: u8> RFOF_W<'a, O> {
    #[doc = "Overflow has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_A::_0)
    }
    #[doc = "Overflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_A::_1)
    }
}
#[doc = "Field `RDE` reader - Receive Descriptor Empty Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RDE_R = crate::BitReader<RDE_A>;
#[doc = "Receive Descriptor Empty Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDE_A {
    #[doc = "0: The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 1."]
    _0 = 0,
    #[doc = "1: The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 0."]
    _1 = 1,
}
impl From<RDE_A> for bool {
    #[inline(always)]
    fn from(variant: RDE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDE_A {
        match self.bits {
            false => RDE_A::_0,
            true => RDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDE_A::_1
    }
}
#[doc = "Field `RDE` writer - Receive Descriptor Empty Flag"]
pub type RDE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, RDE_A, O>;
impl<'a, const O: u8> RDE_W<'a, O> {
    #[doc = "The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDE_A::_0)
    }
    #[doc = "The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDE_A::_1)
    }
}
#[doc = "Field `FR` reader - Frame Receive Flag\n\nThe field is **modified** in some way after a read operation."]
pub type FR_R = crate::BitReader<FR_A>;
#[doc = "Frame Receive Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FR_A {
    #[doc = "0: Frame has not been received."]
    _0 = 0,
    #[doc = "1: Frame has been received. The receive descriptor has been updated."]
    _1 = 1,
}
impl From<FR_A> for bool {
    #[inline(always)]
    fn from(variant: FR_A) -> Self {
        variant as u8 != 0
    }
}
impl FR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FR_A {
        match self.bits {
            false => FR_A::_0,
            true => FR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FR_A::_1
    }
}
#[doc = "Field `FR` writer - Frame Receive Flag"]
pub type FR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, FR_A, O>;
impl<'a, const O: u8> FR_W<'a, O> {
    #[doc = "Frame has not been received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FR_A::_0)
    }
    #[doc = "Frame has been received. The receive descriptor has been updated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FR_A::_1)
    }
}
#[doc = "Field `TFUF` reader - Transmit FIFO Underflow Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TFUF_R = crate::BitReader<TFUF_A>;
#[doc = "Transmit FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUF_A {
    #[doc = "0: Underflow has not occurred."]
    _0 = 0,
    #[doc = "1: Underflow has occurred."]
    _1 = 1,
}
impl From<TFUF_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_A) -> Self {
        variant as u8 != 0
    }
}
impl TFUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUF_A {
        match self.bits {
            false => TFUF_A::_0,
            true => TFUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUF_A::_1
    }
}
#[doc = "Field `TFUF` writer - Transmit FIFO Underflow Flag"]
pub type TFUF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, TFUF_A, O>;
impl<'a, const O: u8> TFUF_W<'a, O> {
    #[doc = "Underflow has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_A::_0)
    }
    #[doc = "Underflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_A::_1)
    }
}
#[doc = "Field `TDE` reader - Transmit Descriptor Empty Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TDE_R = crate::BitReader<TDE_A>;
#[doc = "Transmit Descriptor Empty Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    #[doc = "0: The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 1."]
    _0 = 0,
    #[doc = "1: The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 0."]
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
#[doc = "Field `TDE` writer - Transmit Descriptor Empty Flag"]
pub type TDE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, TDE_A, O>;
impl<'a, const O: u8> TDE_W<'a, O> {
    #[doc = "The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDE_A::_0)
    }
    #[doc = "The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDE_A::_1)
    }
}
#[doc = "Field `TC` reader - Frame Transfer Complete Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Frame Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transfer has not been completed, or transfer has not been requested."]
    _0 = 0,
    #[doc = "1: All frames indicated by the transmit descriptor have been completely transferred to the transmit FIFO."]
    _1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TC_A::_1
    }
}
#[doc = "Field `TC` writer - Frame Transfer Complete Flag"]
pub type TC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, TC_A, O>;
impl<'a, const O: u8> TC_W<'a, O> {
    #[doc = "Transfer has not been completed, or transfer has not been requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TC_A::_0)
    }
    #[doc = "All frames indicated by the transmit descriptor have been completely transferred to the transmit FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TC_A::_1)
    }
}
#[doc = "Field `ADE` reader - Address Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type ADE_R = crate::BitReader<ADE_A>;
#[doc = "Address Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADE_A {
    #[doc = "0: Invalid memory address has not been detected (normal operation)."]
    _0 = 0,
    #[doc = "1: Invalid memory address has been detected."]
    _1 = 1,
}
impl From<ADE_A> for bool {
    #[inline(always)]
    fn from(variant: ADE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADE_A {
        match self.bits {
            false => ADE_A::_0,
            true => ADE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADE_A::_1
    }
}
#[doc = "Field `ADE` writer - Address Error Flag"]
pub type ADE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, ADE_A, O>;
impl<'a, const O: u8> ADE_W<'a, O> {
    #[doc = "Invalid memory address has not been detected (normal operation)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADE_A::_0)
    }
    #[doc = "Invalid memory address has been detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADE_A::_1)
    }
}
#[doc = "Field `RFCOF` reader - Receive Frame Counter Overflow Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RFCOF_R = crate::BitReader<RFCOF_A>;
#[doc = "Receive Frame Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCOF_A {
    #[doc = "0: Receive frame counter has not overflowed."]
    _0 = 0,
    #[doc = "1: Receive frame counter has overflowed."]
    _1 = 1,
}
impl From<RFCOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFCOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFCOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCOF_A {
        match self.bits {
            false => RFCOF_A::_0,
            true => RFCOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFCOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFCOF_A::_1
    }
}
#[doc = "Field `RFCOF` writer - Receive Frame Counter Overflow Flag"]
pub type RFCOF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, RFCOF_A, O>;
impl<'a, const O: u8> RFCOF_W<'a, O> {
    #[doc = "Receive frame counter has not overflowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFCOF_A::_0)
    }
    #[doc = "Receive frame counter has overflowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFCOF_A::_1)
    }
}
#[doc = "Field `TABT` reader - Transmit Abort Detect Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TABT_R = crate::BitReader<TABT_A>;
#[doc = "Transmit Abort Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABT_A {
    #[doc = "0: Frame transmission has not been aborted or transmission has not been requested."]
    _0 = 0,
    #[doc = "1: Frame transmission has been aborted."]
    _1 = 1,
}
impl From<TABT_A> for bool {
    #[inline(always)]
    fn from(variant: TABT_A) -> Self {
        variant as u8 != 0
    }
}
impl TABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABT_A {
        match self.bits {
            false => TABT_A::_0,
            true => TABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABT_A::_1
    }
}
#[doc = "Field `TABT` writer - Transmit Abort Detect Flag"]
pub type TABT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, TABT_A, O>;
impl<'a, const O: u8> TABT_W<'a, O> {
    #[doc = "Frame transmission has not been aborted or transmission has not been requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABT_A::_0)
    }
    #[doc = "Frame transmission has been aborted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABT_A::_1)
    }
}
#[doc = "Field `TWB` reader - Write-Back Complete Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TWB_R = crate::BitReader<TWB_A>;
#[doc = "Write-Back Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWB_A {
    #[doc = "0: Write-back has not been completed, or transmission has not been requested."]
    _0 = 0,
    #[doc = "1: Write-back to the transmit descriptor has been completed."]
    _1 = 1,
}
impl From<TWB_A> for bool {
    #[inline(always)]
    fn from(variant: TWB_A) -> Self {
        variant as u8 != 0
    }
}
impl TWB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWB_A {
        match self.bits {
            false => TWB_A::_0,
            true => TWB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWB_A::_1
    }
}
#[doc = "Field `TWB` writer - Write-Back Complete Flag"]
pub type TWB_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EESR_SPEC, TWB_A, O>;
impl<'a, const O: u8> TWB_W<'a, O> {
    #[doc = "Write-back has not been completed, or transmission has not been requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWB_A::_0)
    }
    #[doc = "Write-back to the transmit descriptor has been completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - PTP v2 Message Type Flag"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PTP v2 Packet Flag"]
    #[inline(always)]
    pub fn pver(&self) -> PVER_R {
        PVER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MAC Address Mismatch Flag"]
    #[inline(always)]
    pub fn mace(&self) -> MACE_R {
        MACE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&self) -> RFOF_R {
        RFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Flag"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame Receive Flag"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&self) -> TFUF_R {
        TFUF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Flag"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Address Error Flag"]
    #[inline(always)]
    pub fn ade(&self) -> ADE_R {
        ADE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn rfcof(&self) -> RFCOF_R {
        RFCOF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Flag"]
    #[inline(always)]
    pub fn tabt(&self) -> TABT_R {
        TABT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Write-Back Complete Flag"]
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PTP v2 Message Type Flag"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<0> {
        TYPE_W::new(self)
    }
    #[doc = "Bit 4 - PTP v2 Packet Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pver(&mut self) -> PVER_W<4> {
        PVER_W::new(self)
    }
    #[doc = "Bit 8 - MAC Address Mismatch Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mace(&mut self) -> MACE_W<8> {
        MACE_W::new(self)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfof(&mut self) -> RFOF_W<16> {
        RFOF_W::new(self)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RDE_W<17> {
        RDE_W::new(self)
    }
    #[doc = "Bit 18 - Frame Receive Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FR_W<18> {
        FR_W::new(self)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfuf(&mut self) -> TFUF_W<19> {
        TFUF_W::new(self)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<20> {
        TDE_W::new(self)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<21> {
        TC_W::new(self)
    }
    #[doc = "Bit 23 - Address Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ade(&mut self) -> ADE_W<23> {
        ADE_W::new(self)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfcof(&mut self) -> RFCOF_W<24> {
        RFCOF_W::new(self)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tabt(&mut self) -> TABT_W<26> {
        TABT_W::new(self)
    }
    #[doc = "Bit 30 - Write-Back Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn twb(&mut self) -> TWB_W<30> {
        TWB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP/EDMAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eesr](index.html) module"]
pub struct EESR_SPEC;
impl crate::RegisterSpec for EESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eesr::R](R) reader structure"]
impl crate::Readable for EESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eesr::W](W) writer structure"]
impl crate::Writable for EESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x45bf_011f;
}
#[doc = "`reset()` method sets EESR to value 0"]
impl crate::Resettable for EESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
