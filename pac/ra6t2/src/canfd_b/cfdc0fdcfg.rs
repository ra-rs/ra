#[doc = "Register `CFDC0FDCFG` reader"]
pub struct R(crate::R<CFDC0FDCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0FDCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0FDCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0FDCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0FDCFG` writer"]
pub struct W(crate::W<CFDC0FDCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0FDCFG_SPEC>;
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
impl From<crate::W<CFDC0FDCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0FDCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOCCFG` reader - Error Occurrence Counter Configuration"]
pub type EOCCFG_R = crate::FieldReader<u8, EOCCFG_A>;
#[doc = "Error Occurrence Counter Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EOCCFG_A {
    #[doc = "0: All transmitter or receiver CAN frames"]
    _000 = 0,
    #[doc = "1: All transmitter CAN frames"]
    _001 = 1,
    #[doc = "2: All receiver CAN frames"]
    _010 = 2,
    #[doc = "3: Reserved"]
    _011 = 3,
    #[doc = "4: Only transmitter or receiver CANFD data-phase (fast bits)"]
    _100 = 4,
    #[doc = "5: Only transmitter CANFD data-phase (fast bits)"]
    _101 = 5,
    #[doc = "6: Only receiver CANFD data-phase (fast bits)"]
    _110 = 6,
    #[doc = "7: Reserved"]
    _111 = 7,
}
impl From<EOCCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: EOCCFG_A) -> Self {
        variant as _
    }
}
impl EOCCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCCFG_A {
        match self.bits {
            0 => EOCCFG_A::_000,
            1 => EOCCFG_A::_001,
            2 => EOCCFG_A::_010,
            3 => EOCCFG_A::_011,
            4 => EOCCFG_A::_100,
            5 => EOCCFG_A::_101,
            6 => EOCCFG_A::_110,
            7 => EOCCFG_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == EOCCFG_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == EOCCFG_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == EOCCFG_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == EOCCFG_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == EOCCFG_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == EOCCFG_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == EOCCFG_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == EOCCFG_A::_111
    }
}
#[doc = "Field `EOCCFG` writer - Error Occurrence Counter Configuration"]
pub type EOCCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDC0FDCFG_SPEC, u8, EOCCFG_A, 3, O>;
impl<'a, const O: u8> EOCCFG_W<'a, O> {
    #[doc = "All transmitter or receiver CAN frames"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(EOCCFG_A::_000)
    }
    #[doc = "All transmitter CAN frames"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(EOCCFG_A::_001)
    }
    #[doc = "All receiver CAN frames"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(EOCCFG_A::_010)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(EOCCFG_A::_011)
    }
    #[doc = "Only transmitter or receiver CANFD data-phase (fast bits)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(EOCCFG_A::_100)
    }
    #[doc = "Only transmitter CANFD data-phase (fast bits)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(EOCCFG_A::_101)
    }
    #[doc = "Only receiver CANFD data-phase (fast bits)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(EOCCFG_A::_110)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(EOCCFG_A::_111)
    }
}
#[doc = "Field `TDCOC` reader - Transceiver Delay Compensation Offset Configuration"]
pub type TDCOC_R = crate::BitReader<TDCOC_A>;
#[doc = "Transceiver Delay Compensation Offset Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCOC_A {
    #[doc = "0: Measured + offset"]
    _0 = 0,
    #[doc = "1: Offset-only"]
    _1 = 1,
}
impl From<TDCOC_A> for bool {
    #[inline(always)]
    fn from(variant: TDCOC_A) -> Self {
        variant as u8 != 0
    }
}
impl TDCOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCOC_A {
        match self.bits {
            false => TDCOC_A::_0,
            true => TDCOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDCOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDCOC_A::_1
    }
}
#[doc = "Field `TDCOC` writer - Transceiver Delay Compensation Offset Configuration"]
pub type TDCOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCFG_SPEC, TDCOC_A, O>;
impl<'a, const O: u8> TDCOC_W<'a, O> {
    #[doc = "Measured + offset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCOC_A::_0)
    }
    #[doc = "Offset-only"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCOC_A::_1)
    }
}
#[doc = "Field `TDCE` reader - Transceiver Delay Compensation Enable"]
pub type TDCE_R = crate::BitReader<TDCE_A>;
#[doc = "Transceiver Delay Compensation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCE_A {
    #[doc = "0: Transceiver delay compensation disabled"]
    _0 = 0,
    #[doc = "1: Transceiver delay compensation enabled"]
    _1 = 1,
}
impl From<TDCE_A> for bool {
    #[inline(always)]
    fn from(variant: TDCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCE_A {
        match self.bits {
            false => TDCE_A::_0,
            true => TDCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDCE_A::_1
    }
}
#[doc = "Field `TDCE` writer - Transceiver Delay Compensation Enable"]
pub type TDCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCFG_SPEC, TDCE_A, O>;
impl<'a, const O: u8> TDCE_W<'a, O> {
    #[doc = "Transceiver delay compensation disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCE_A::_0)
    }
    #[doc = "Transceiver delay compensation enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCE_A::_1)
    }
}
#[doc = "Field `ESIC` reader - Error State Indication Configuration"]
pub type ESIC_R = crate::BitReader<ESIC_A>;
#[doc = "Error State Indication Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESIC_A {
    #[doc = "0: The ESI bit in the frame represents the error state of the node itself"]
    _0 = 0,
    #[doc = "1: The ESI bit in the frame represents the error state of the message buffer if the node itself is not in error passive. If the node is in error passive, then the ESI bit is driven by the node itself."]
    _1 = 1,
}
impl From<ESIC_A> for bool {
    #[inline(always)]
    fn from(variant: ESIC_A) -> Self {
        variant as u8 != 0
    }
}
impl ESIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESIC_A {
        match self.bits {
            false => ESIC_A::_0,
            true => ESIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESIC_A::_1
    }
}
#[doc = "Field `ESIC` writer - Error State Indication Configuration"]
pub type ESIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCFG_SPEC, ESIC_A, O>;
impl<'a, const O: u8> ESIC_W<'a, O> {
    #[doc = "The ESI bit in the frame represents the error state of the node itself"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESIC_A::_0)
    }
    #[doc = "The ESI bit in the frame represents the error state of the message buffer if the node itself is not in error passive. If the node is in error passive, then the ESI bit is driven by the node itself."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESIC_A::_1)
    }
}
#[doc = "Field `TDCO` reader - Transceiver Delay Compensation Offset"]
pub type TDCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDCO` writer - Transceiver Delay Compensation Offset"]
pub type TDCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDC0FDCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `FDOE` reader - FD-Only Enable"]
pub type FDOE_R = crate::BitReader<FDOE_A>;
#[doc = "FD-Only Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDOE_A {
    #[doc = "0: FD-only mode disabled"]
    _0 = 0,
    #[doc = "1: FD-only mode enabled"]
    _1 = 1,
}
impl From<FDOE_A> for bool {
    #[inline(always)]
    fn from(variant: FDOE_A) -> Self {
        variant as u8 != 0
    }
}
impl FDOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDOE_A {
        match self.bits {
            false => FDOE_A::_0,
            true => FDOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDOE_A::_1
    }
}
#[doc = "Field `FDOE` writer - FD-Only Enable"]
pub type FDOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCFG_SPEC, FDOE_A, O>;
impl<'a, const O: u8> FDOE_W<'a, O> {
    #[doc = "FD-only mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDOE_A::_0)
    }
    #[doc = "FD-only mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDOE_A::_1)
    }
}
#[doc = "Field `REFE` reader - RX Edge Filter Enable"]
pub type REFE_R = crate::BitReader<REFE_A>;
#[doc = "RX Edge Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFE_A {
    #[doc = "0: RX edge filter disabled"]
    _0 = 0,
    #[doc = "1: RX edge filter enabled"]
    _1 = 1,
}
impl From<REFE_A> for bool {
    #[inline(always)]
    fn from(variant: REFE_A) -> Self {
        variant as u8 != 0
    }
}
impl REFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFE_A {
        match self.bits {
            false => REFE_A::_0,
            true => REFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REFE_A::_1
    }
}
#[doc = "Field `REFE` writer - RX Edge Filter Enable"]
pub type REFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCFG_SPEC, REFE_A, O>;
impl<'a, const O: u8> REFE_W<'a, O> {
    #[doc = "RX edge filter disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REFE_A::_0)
    }
    #[doc = "RX edge filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REFE_A::_1)
    }
}
#[doc = "Field `CLOE` reader - Classical CAN Enable"]
pub type CLOE_R = crate::BitReader<CLOE_A>;
#[doc = "Classical CAN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOE_A {
    #[doc = "0: Classical CAN mode disabled"]
    _0 = 0,
    #[doc = "1: Classical CAN mode enabled"]
    _1 = 1,
}
impl From<CLOE_A> for bool {
    #[inline(always)]
    fn from(variant: CLOE_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOE_A {
        match self.bits {
            false => CLOE_A::_0,
            true => CLOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLOE_A::_1
    }
}
#[doc = "Field `CLOE` writer - Classical CAN Enable"]
pub type CLOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDCFG_SPEC, CLOE_A, O>;
impl<'a, const O: u8> CLOE_W<'a, O> {
    #[doc = "Classical CAN mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLOE_A::_0)
    }
    #[doc = "Classical CAN mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLOE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Error Occurrence Counter Configuration"]
    #[inline(always)]
    pub fn eoccfg(&self) -> EOCCFG_R {
        EOCCFG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Transceiver Delay Compensation Offset Configuration"]
    #[inline(always)]
    pub fn tdcoc(&self) -> TDCOC_R {
        TDCOC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdce(&self) -> TDCE_R {
        TDCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error State Indication Configuration"]
    #[inline(always)]
    pub fn esic(&self) -> ESIC_R {
        ESIC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - FD-Only Enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RX Edge Filter Enable"]
    #[inline(always)]
    pub fn refe(&self) -> REFE_R {
        REFE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Classical CAN Enable"]
    #[inline(always)]
    pub fn cloe(&self) -> CLOE_R {
        CLOE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Error Occurrence Counter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn eoccfg(&mut self) -> EOCCFG_W<0> {
        EOCCFG_W::new(self)
    }
    #[doc = "Bit 8 - Transceiver Delay Compensation Offset Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn tdcoc(&mut self) -> TDCOC_W<8> {
        TDCOC_W::new(self)
    }
    #[doc = "Bit 9 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdce(&mut self) -> TDCE_W<9> {
        TDCE_W::new(self)
    }
    #[doc = "Bit 10 - Error State Indication Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn esic(&mut self) -> ESIC_W<10> {
        ESIC_W::new(self)
    }
    #[doc = "Bits 16:23 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TDCO_W<16> {
        TDCO_W::new(self)
    }
    #[doc = "Bit 28 - FD-Only Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<28> {
        FDOE_W::new(self)
    }
    #[doc = "Bit 29 - RX Edge Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refe(&mut self) -> REFE_W<29> {
        REFE_W::new(self)
    }
    #[doc = "Bit 30 - Classical CAN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cloe(&mut self) -> CLOE_W<30> {
        CLOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 CANFD Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0fdcfg](index.html) module"]
pub struct CFDC0FDCFG_SPEC;
impl crate::RegisterSpec for CFDC0FDCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0fdcfg::R](R) reader structure"]
impl crate::Readable for CFDC0FDCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0fdcfg::W](W) writer structure"]
impl crate::Writable for CFDC0FDCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0FDCFG to value 0"]
impl crate::Resettable for CFDC0FDCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
