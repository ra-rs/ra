#[doc = "Register `CFDRFCC%s` reader"]
pub struct R(crate::R<CFDRFCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRFCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRFCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRFCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDRFCC%s` writer"]
pub struct W(crate::W<CFDRFCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDRFCC_SPEC>;
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
impl From<crate::W<CFDRFCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDRFCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFE` reader - RX FIFO Enable"]
pub type RFE_R = crate::BitReader<RFE_A>;
#[doc = "RX FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFE_A {
    #[doc = "0: FIFO disabled"]
    _0 = 0,
    #[doc = "1: FIFO enabled"]
    _1 = 1,
}
impl From<RFE_A> for bool {
    #[inline(always)]
    fn from(variant: RFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFE_A {
        match self.bits {
            false => RFE_A::_0,
            true => RFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFE_A::_1
    }
}
#[doc = "Field `RFE` writer - RX FIFO Enable"]
pub type RFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDRFCC_SPEC, RFE_A, O>;
impl<'a, const O: u8> RFE_W<'a, O> {
    #[doc = "FIFO disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFE_A::_0)
    }
    #[doc = "FIFO enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFE_A::_1)
    }
}
#[doc = "Field `RFIE` reader - RX FIFO Interrupt Enable"]
pub type RFIE_R = crate::BitReader<RFIE_A>;
#[doc = "RX FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFIE_A {
    #[doc = "0: FIFO interrupt generation disabled"]
    _0 = 0,
    #[doc = "1: FIFO interrupt generation enabled"]
    _1 = 1,
}
impl From<RFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFIE_A {
        match self.bits {
            false => RFIE_A::_0,
            true => RFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFIE_A::_1
    }
}
#[doc = "Field `RFIE` writer - RX FIFO Interrupt Enable"]
pub type RFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDRFCC_SPEC, RFIE_A, O>;
impl<'a, const O: u8> RFIE_W<'a, O> {
    #[doc = "FIFO interrupt generation disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFIE_A::_0)
    }
    #[doc = "FIFO interrupt generation enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFIE_A::_1)
    }
}
#[doc = "Field `RFPLS` reader - Rx FIFO Payload Data Size Configuration"]
pub type RFPLS_R = crate::FieldReader<u8, RFPLS_A>;
#[doc = "Rx FIFO Payload Data Size Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFPLS_A {
    #[doc = "0: 8 bytes"]
    _000 = 0,
    #[doc = "1: 12 bytes"]
    _001 = 1,
    #[doc = "2: 16 bytes"]
    _010 = 2,
    #[doc = "3: 20 bytes"]
    _011 = 3,
    #[doc = "4: 24 bytes"]
    _100 = 4,
    #[doc = "5: 32 bytes"]
    _101 = 5,
    #[doc = "6: 48 bytes"]
    _110 = 6,
    #[doc = "7: 64 bytes"]
    _111 = 7,
}
impl From<RFPLS_A> for u8 {
    #[inline(always)]
    fn from(variant: RFPLS_A) -> Self {
        variant as _
    }
}
impl RFPLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFPLS_A {
        match self.bits {
            0 => RFPLS_A::_000,
            1 => RFPLS_A::_001,
            2 => RFPLS_A::_010,
            3 => RFPLS_A::_011,
            4 => RFPLS_A::_100,
            5 => RFPLS_A::_101,
            6 => RFPLS_A::_110,
            7 => RFPLS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFPLS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFPLS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFPLS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFPLS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFPLS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RFPLS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RFPLS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RFPLS_A::_111
    }
}
#[doc = "Field `RFPLS` writer - Rx FIFO Payload Data Size Configuration"]
pub type RFPLS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDRFCC_SPEC, u8, RFPLS_A, 3, O>;
impl<'a, const O: u8> RFPLS_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RFPLS_A::_000)
    }
    #[doc = "12 bytes"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RFPLS_A::_001)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RFPLS_A::_010)
    }
    #[doc = "20 bytes"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RFPLS_A::_011)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RFPLS_A::_100)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RFPLS_A::_101)
    }
    #[doc = "48 bytes"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RFPLS_A::_110)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RFPLS_A::_111)
    }
}
#[doc = "Field `RFDC` reader - RX FIFO Depth Configuration"]
pub type RFDC_R = crate::FieldReader<u8, RFDC_A>;
#[doc = "RX FIFO Depth Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFDC_A {
    #[doc = "0: FIFO Depth = 0 message"]
    _000 = 0,
    #[doc = "1: FIFO Depth = 4 messages"]
    _001 = 1,
    #[doc = "2: FIFO Depth = 8 messages"]
    _010 = 2,
    #[doc = "3: FIFO Depth = 16 messages"]
    _011 = 3,
    #[doc = "4: FIFO Depth = 32 essages"]
    _100 = 4,
    #[doc = "5: FIFO Depth = 48 messages"]
    _101 = 5,
    #[doc = "6: Reserved"]
    _110 = 6,
    #[doc = "7: Reserved"]
    _111 = 7,
}
impl From<RFDC_A> for u8 {
    #[inline(always)]
    fn from(variant: RFDC_A) -> Self {
        variant as _
    }
}
impl RFDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDC_A {
        match self.bits {
            0 => RFDC_A::_000,
            1 => RFDC_A::_001,
            2 => RFDC_A::_010,
            3 => RFDC_A::_011,
            4 => RFDC_A::_100,
            5 => RFDC_A::_101,
            6 => RFDC_A::_110,
            7 => RFDC_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFDC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFDC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFDC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFDC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFDC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RFDC_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RFDC_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RFDC_A::_111
    }
}
#[doc = "Field `RFDC` writer - RX FIFO Depth Configuration"]
pub type RFDC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFDRFCC_SPEC, u8, RFDC_A, 3, O>;
impl<'a, const O: u8> RFDC_W<'a, O> {
    #[doc = "FIFO Depth = 0 message"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RFDC_A::_000)
    }
    #[doc = "FIFO Depth = 4 messages"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RFDC_A::_001)
    }
    #[doc = "FIFO Depth = 8 messages"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RFDC_A::_010)
    }
    #[doc = "FIFO Depth = 16 messages"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RFDC_A::_011)
    }
    #[doc = "FIFO Depth = 32 essages"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RFDC_A::_100)
    }
    #[doc = "FIFO Depth = 48 messages"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RFDC_A::_101)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RFDC_A::_110)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RFDC_A::_111)
    }
}
#[doc = "Field `RFIM` reader - RX FIFO Interrupt Mode"]
pub type RFIM_R = crate::BitReader<RFIM_A>;
#[doc = "RX FIFO Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFIM_A {
    #[doc = "0: Interrupt generated when RX FIFO counter reaches RFIGCV value from values smaller than RFIGCV"]
    _0 = 0,
    #[doc = "1: Interrupt generated at the end of every received message storage"]
    _1 = 1,
}
impl From<RFIM_A> for bool {
    #[inline(always)]
    fn from(variant: RFIM_A) -> Self {
        variant as u8 != 0
    }
}
impl RFIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFIM_A {
        match self.bits {
            false => RFIM_A::_0,
            true => RFIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFIM_A::_1
    }
}
#[doc = "Field `RFIM` writer - RX FIFO Interrupt Mode"]
pub type RFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDRFCC_SPEC, RFIM_A, O>;
impl<'a, const O: u8> RFIM_W<'a, O> {
    #[doc = "Interrupt generated when RX FIFO counter reaches RFIGCV value from values smaller than RFIGCV"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFIM_A::_0)
    }
    #[doc = "Interrupt generated at the end of every received message storage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFIM_A::_1)
    }
}
#[doc = "Field `RFIGCV` reader - RX FIFO Interrupt Generation Counter Value"]
pub type RFIGCV_R = crate::FieldReader<u8, RFIGCV_A>;
#[doc = "RX FIFO Interrupt Generation Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFIGCV_A {
    #[doc = "0: Interrupt generated when FIFO is 1/8th full"]
    _000 = 0,
    #[doc = "1: Interrupt generated when FIFO is 1/4th full"]
    _001 = 1,
    #[doc = "2: Interrupt generated when FIFO is 3/8th full"]
    _010 = 2,
    #[doc = "3: Interrupt generated when FIFO is 1/2 full"]
    _011 = 3,
    #[doc = "4: Interrupt generated when FIFO is 5/8th full"]
    _100 = 4,
    #[doc = "5: Interrupt generated when FIFO is 3/4th full"]
    _101 = 5,
    #[doc = "6: Interrupt generated when FIFO is 7/8th full"]
    _110 = 6,
    #[doc = "7: Interrupt generated when FIFO is full"]
    _111 = 7,
}
impl From<RFIGCV_A> for u8 {
    #[inline(always)]
    fn from(variant: RFIGCV_A) -> Self {
        variant as _
    }
}
impl RFIGCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFIGCV_A {
        match self.bits {
            0 => RFIGCV_A::_000,
            1 => RFIGCV_A::_001,
            2 => RFIGCV_A::_010,
            3 => RFIGCV_A::_011,
            4 => RFIGCV_A::_100,
            5 => RFIGCV_A::_101,
            6 => RFIGCV_A::_110,
            7 => RFIGCV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFIGCV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFIGCV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFIGCV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFIGCV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFIGCV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RFIGCV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RFIGCV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RFIGCV_A::_111
    }
}
#[doc = "Field `RFIGCV` writer - RX FIFO Interrupt Generation Counter Value"]
pub type RFIGCV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDRFCC_SPEC, u8, RFIGCV_A, 3, O>;
impl<'a, const O: u8> RFIGCV_W<'a, O> {
    #[doc = "Interrupt generated when FIFO is 1/8th full"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RFIGCV_A::_000)
    }
    #[doc = "Interrupt generated when FIFO is 1/4th full"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RFIGCV_A::_001)
    }
    #[doc = "Interrupt generated when FIFO is 3/8th full"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RFIGCV_A::_010)
    }
    #[doc = "Interrupt generated when FIFO is 1/2 full"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RFIGCV_A::_011)
    }
    #[doc = "Interrupt generated when FIFO is 5/8th full"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RFIGCV_A::_100)
    }
    #[doc = "Interrupt generated when FIFO is 3/4th full"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RFIGCV_A::_101)
    }
    #[doc = "Interrupt generated when FIFO is 7/8th full"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RFIGCV_A::_110)
    }
    #[doc = "Interrupt generated when FIFO is full"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RFIGCV_A::_111)
    }
}
impl R {
    #[doc = "Bit 0 - RX FIFO Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn rfie(&self) -> RFIE_R {
        RFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Rx FIFO Payload Data Size Configuration"]
    #[inline(always)]
    pub fn rfpls(&self) -> RFPLS_R {
        RFPLS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - RX FIFO Depth Configuration"]
    #[inline(always)]
    pub fn rfdc(&self) -> RFDC_R {
        RFDC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - RX FIFO Interrupt Mode"]
    #[inline(always)]
    pub fn rfim(&self) -> RFIM_R {
        RFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RX FIFO Interrupt Generation Counter Value"]
    #[inline(always)]
    pub fn rfigcv(&self) -> RFIGCV_R {
        RFIGCV_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<0> {
        RFE_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfie(&mut self) -> RFIE_W<1> {
        RFIE_W::new(self)
    }
    #[doc = "Bits 4:6 - Rx FIFO Payload Data Size Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rfpls(&mut self) -> RFPLS_W<4> {
        RFPLS_W::new(self)
    }
    #[doc = "Bits 8:10 - RX FIFO Depth Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rfdc(&mut self) -> RFDC_W<8> {
        RFDC_W::new(self)
    }
    #[doc = "Bit 12 - RX FIFO Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rfim(&mut self) -> RFIM_W<12> {
        RFIM_W::new(self)
    }
    #[doc = "Bits 13:15 - RX FIFO Interrupt Generation Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn rfigcv(&mut self) -> RFIGCV_W<13> {
        RFIGCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX FIFO Configuration/Control Registers %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrfcc](index.html) module"]
pub struct CFDRFCC_SPEC;
impl crate::RegisterSpec for CFDRFCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrfcc::R](R) reader structure"]
impl crate::Readable for CFDRFCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdrfcc::W](W) writer structure"]
impl crate::Writable for CFDRFCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDRFCC%s to value 0"]
impl crate::Resettable for CFDRFCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
