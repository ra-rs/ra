#[doc = "Register `CFDCFCC` reader"]
pub struct R(crate::R<CFDCFCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCFCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCFCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCFCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCFCC` writer"]
pub struct W(crate::W<CFDCFCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCFCC_SPEC>;
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
impl From<crate::W<CFDCFCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCFCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFE` reader - Common FIFO Enable"]
pub type CFE_R = crate::BitReader<CFE_A>;
#[doc = "Common FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFE_A {
    #[doc = "0: FIFO disabled"]
    _0 = 0,
    #[doc = "1: FIFO enabled"]
    _1 = 1,
}
impl From<CFE_A> for bool {
    #[inline(always)]
    fn from(variant: CFE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFE_A {
        match self.bits {
            false => CFE_A::_0,
            true => CFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFE_A::_1
    }
}
#[doc = "Field `CFE` writer - Common FIFO Enable"]
pub type CFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFCC_SPEC, CFE_A, O>;
impl<'a, const O: u8> CFE_W<'a, O> {
    #[doc = "FIFO disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFE_A::_0)
    }
    #[doc = "FIFO enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFE_A::_1)
    }
}
#[doc = "Field `CFRXIE` reader - Common FIFO RX Interrupt Enable"]
pub type CFRXIE_R = crate::BitReader<CFRXIE_A>;
#[doc = "Common FIFO RX Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFRXIE_A {
    #[doc = "0: FIFO interrupt generation disabled for Frame RX"]
    _0 = 0,
    #[doc = "1: FIFO interrupt generation enabled for Frame RX"]
    _1 = 1,
}
impl From<CFRXIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFRXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFRXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFRXIE_A {
        match self.bits {
            false => CFRXIE_A::_0,
            true => CFRXIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFRXIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFRXIE_A::_1
    }
}
#[doc = "Field `CFRXIE` writer - Common FIFO RX Interrupt Enable"]
pub type CFRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFCC_SPEC, CFRXIE_A, O>;
impl<'a, const O: u8> CFRXIE_W<'a, O> {
    #[doc = "FIFO interrupt generation disabled for Frame RX"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFRXIE_A::_0)
    }
    #[doc = "FIFO interrupt generation enabled for Frame RX"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFRXIE_A::_1)
    }
}
#[doc = "Field `CFTXIE` reader - Common FIFO TX Interrupt Enable"]
pub type CFTXIE_R = crate::BitReader<CFTXIE_A>;
#[doc = "Common FIFO TX Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFTXIE_A {
    #[doc = "0: FIFO interrupt generation disabled for Frame TX"]
    _0 = 0,
    #[doc = "1: FIFO interrupt generation enabled for Frame TX"]
    _1 = 1,
}
impl From<CFTXIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFTXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFTXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFTXIE_A {
        match self.bits {
            false => CFTXIE_A::_0,
            true => CFTXIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFTXIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFTXIE_A::_1
    }
}
#[doc = "Field `CFTXIE` writer - Common FIFO TX Interrupt Enable"]
pub type CFTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFCC_SPEC, CFTXIE_A, O>;
impl<'a, const O: u8> CFTXIE_W<'a, O> {
    #[doc = "FIFO interrupt generation disabled for Frame TX"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFTXIE_A::_0)
    }
    #[doc = "FIFO interrupt generation enabled for Frame TX"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFTXIE_A::_1)
    }
}
#[doc = "Field `CFPLS` reader - Common FIFO Payload Data Size Configuration"]
pub type CFPLS_R = crate::FieldReader<u8, CFPLS_A>;
#[doc = "Common FIFO Payload Data Size Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFPLS_A {
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
impl From<CFPLS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFPLS_A) -> Self {
        variant as _
    }
}
impl CFPLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFPLS_A {
        match self.bits {
            0 => CFPLS_A::_000,
            1 => CFPLS_A::_001,
            2 => CFPLS_A::_010,
            3 => CFPLS_A::_011,
            4 => CFPLS_A::_100,
            5 => CFPLS_A::_101,
            6 => CFPLS_A::_110,
            7 => CFPLS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CFPLS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CFPLS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CFPLS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CFPLS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CFPLS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CFPLS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CFPLS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CFPLS_A::_111
    }
}
#[doc = "Field `CFPLS` writer - Common FIFO Payload Data Size Configuration"]
pub type CFPLS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDCFCC_SPEC, u8, CFPLS_A, 3, O>;
impl<'a, const O: u8> CFPLS_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CFPLS_A::_000)
    }
    #[doc = "12 bytes"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CFPLS_A::_001)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CFPLS_A::_010)
    }
    #[doc = "20 bytes"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CFPLS_A::_011)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CFPLS_A::_100)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CFPLS_A::_101)
    }
    #[doc = "48 bytes"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CFPLS_A::_110)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CFPLS_A::_111)
    }
}
#[doc = "Field `CFM` reader - Common FIFO Mode"]
pub type CFM_R = crate::BitReader<CFM_A>;
#[doc = "Common FIFO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFM_A {
    #[doc = "0: RX FIFO mode"]
    _0 = 0,
    #[doc = "1: TX FIFO mode"]
    _1 = 1,
}
impl From<CFM_A> for bool {
    #[inline(always)]
    fn from(variant: CFM_A) -> Self {
        variant as u8 != 0
    }
}
impl CFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFM_A {
        match self.bits {
            false => CFM_A::_0,
            true => CFM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFM_A::_1
    }
}
#[doc = "Field `CFM` writer - Common FIFO Mode"]
pub type CFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFCC_SPEC, CFM_A, O>;
impl<'a, const O: u8> CFM_W<'a, O> {
    #[doc = "RX FIFO mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFM_A::_0)
    }
    #[doc = "TX FIFO mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFM_A::_1)
    }
}
#[doc = "Field `CFITSS` reader - Common FIFO Interval Timer Source Select"]
pub type CFITSS_R = crate::BitReader<CFITSS_A>;
#[doc = "Common FIFO Interval Timer Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFITSS_A {
    #[doc = "0: Reference clock (Ã\u{97} 1 / Ã\u{97} 10 period)"]
    _0 = 0,
    #[doc = "1: Bit time clock of related channel (FIFO is linked to fixed channel)"]
    _1 = 1,
}
impl From<CFITSS_A> for bool {
    #[inline(always)]
    fn from(variant: CFITSS_A) -> Self {
        variant as u8 != 0
    }
}
impl CFITSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFITSS_A {
        match self.bits {
            false => CFITSS_A::_0,
            true => CFITSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFITSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFITSS_A::_1
    }
}
#[doc = "Field `CFITSS` writer - Common FIFO Interval Timer Source Select"]
pub type CFITSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFCC_SPEC, CFITSS_A, O>;
impl<'a, const O: u8> CFITSS_W<'a, O> {
    #[doc = "Reference clock (Ã\u{97} 1 / Ã\u{97} 10 period)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFITSS_A::_0)
    }
    #[doc = "Bit time clock of related channel (FIFO is linked to fixed channel)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFITSS_A::_1)
    }
}
#[doc = "Field `CFITR` reader - Common FIFO Interval Timer Resolution"]
pub type CFITR_R = crate::BitReader<CFITR_A>;
#[doc = "Common FIFO Interval Timer Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFITR_A {
    #[doc = "0: Reference clock period Ã\u{97} 1"]
    _0 = 0,
    #[doc = "1: Reference clock period Ã\u{97} 10"]
    _1 = 1,
}
impl From<CFITR_A> for bool {
    #[inline(always)]
    fn from(variant: CFITR_A) -> Self {
        variant as u8 != 0
    }
}
impl CFITR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFITR_A {
        match self.bits {
            false => CFITR_A::_0,
            true => CFITR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFITR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFITR_A::_1
    }
}
#[doc = "Field `CFITR` writer - Common FIFO Interval Timer Resolution"]
pub type CFITR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFCC_SPEC, CFITR_A, O>;
impl<'a, const O: u8> CFITR_W<'a, O> {
    #[doc = "Reference clock period Ã\u{97} 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFITR_A::_0)
    }
    #[doc = "Reference clock period Ã\u{97} 10"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFITR_A::_1)
    }
}
#[doc = "Field `CFIM` reader - Common FIFO Interrupt Mode"]
pub type CFIM_R = crate::BitReader<CFIM_A>;
#[doc = "Common FIFO Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFIM_A {
    #[doc = "0: RX FIFO mode: RX interrupt generated when Common FIFO counter reaches CFIGCV value from a lower value TX FIFO mode: TX interrupt generated when Common FIFO transmits the last message successfully"]
    _0 = 0,
    #[doc = "1: RX FIFO mode: RX interrupt generated at the end of every received message storage TX FIFO mode: interrupt generated for every successfully transmitted message"]
    _1 = 1,
}
impl From<CFIM_A> for bool {
    #[inline(always)]
    fn from(variant: CFIM_A) -> Self {
        variant as u8 != 0
    }
}
impl CFIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFIM_A {
        match self.bits {
            false => CFIM_A::_0,
            true => CFIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFIM_A::_1
    }
}
#[doc = "Field `CFIM` writer - Common FIFO Interrupt Mode"]
pub type CFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFCC_SPEC, CFIM_A, O>;
impl<'a, const O: u8> CFIM_W<'a, O> {
    #[doc = "RX FIFO mode: RX interrupt generated when Common FIFO counter reaches CFIGCV value from a lower value TX FIFO mode: TX interrupt generated when Common FIFO transmits the last message successfully"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFIM_A::_0)
    }
    #[doc = "RX FIFO mode: RX interrupt generated at the end of every received message storage TX FIFO mode: interrupt generated for every successfully transmitted message"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFIM_A::_1)
    }
}
#[doc = "Field `CFIGCV` reader - Common FIFO Interrupt Generation Counter Value"]
pub type CFIGCV_R = crate::FieldReader<u8, CFIGCV_A>;
#[doc = "Common FIFO Interrupt Generation Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFIGCV_A {
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
impl From<CFIGCV_A> for u8 {
    #[inline(always)]
    fn from(variant: CFIGCV_A) -> Self {
        variant as _
    }
}
impl CFIGCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFIGCV_A {
        match self.bits {
            0 => CFIGCV_A::_000,
            1 => CFIGCV_A::_001,
            2 => CFIGCV_A::_010,
            3 => CFIGCV_A::_011,
            4 => CFIGCV_A::_100,
            5 => CFIGCV_A::_101,
            6 => CFIGCV_A::_110,
            7 => CFIGCV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CFIGCV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CFIGCV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CFIGCV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CFIGCV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CFIGCV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CFIGCV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CFIGCV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CFIGCV_A::_111
    }
}
#[doc = "Field `CFIGCV` writer - Common FIFO Interrupt Generation Counter Value"]
pub type CFIGCV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDCFCC_SPEC, u8, CFIGCV_A, 3, O>;
impl<'a, const O: u8> CFIGCV_W<'a, O> {
    #[doc = "Interrupt generated when FIFO is 1/8th full"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CFIGCV_A::_000)
    }
    #[doc = "Interrupt generated when FIFO is 1/4th full"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CFIGCV_A::_001)
    }
    #[doc = "Interrupt generated when FIFO is 3/8th full"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CFIGCV_A::_010)
    }
    #[doc = "Interrupt generated when FIFO is 1/2 full"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CFIGCV_A::_011)
    }
    #[doc = "Interrupt generated when FIFO is 5/8th full"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CFIGCV_A::_100)
    }
    #[doc = "Interrupt generated when FIFO is 3/4th full"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CFIGCV_A::_101)
    }
    #[doc = "Interrupt generated when FIFO is 7/8th full"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CFIGCV_A::_110)
    }
    #[doc = "Interrupt generated when FIFO is full"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CFIGCV_A::_111)
    }
}
#[doc = "Field `CFTML` reader - Common FIFO TX Message Buffer Link"]
pub type CFTML_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFTML` writer - Common FIFO TX Message Buffer Link"]
pub type CFTML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFCC_SPEC, u8, u8, 2, O>;
#[doc = "Field `CFDC` reader - Common FIFO Depth Configuration"]
pub type CFDC_R = crate::FieldReader<u8, CFDC_A>;
#[doc = "Common FIFO Depth Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDC_A {
    #[doc = "0: FIFO Depth = 0 message"]
    _000 = 0,
    #[doc = "1: FIFO Depth = 4 messages"]
    _001 = 1,
    #[doc = "2: FIFO Depth = 8 messages"]
    _010 = 2,
    #[doc = "3: FIFO Depth = 16 messages"]
    _011 = 3,
    #[doc = "4: FIFO Depth = 32 messages"]
    _100 = 4,
    #[doc = "5: FIFO Depth = 48 messages"]
    _101 = 5,
    #[doc = "6: FIFO Depth = Reserved"]
    _110 = 6,
    #[doc = "7: FIFO Depth = Reserved"]
    _111 = 7,
}
impl From<CFDC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDC_A) -> Self {
        variant as _
    }
}
impl CFDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFDC_A {
        match self.bits {
            0 => CFDC_A::_000,
            1 => CFDC_A::_001,
            2 => CFDC_A::_010,
            3 => CFDC_A::_011,
            4 => CFDC_A::_100,
            5 => CFDC_A::_101,
            6 => CFDC_A::_110,
            7 => CFDC_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CFDC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CFDC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CFDC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CFDC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CFDC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CFDC_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CFDC_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CFDC_A::_111
    }
}
#[doc = "Field `CFDC` writer - Common FIFO Depth Configuration"]
pub type CFDC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFDCFCC_SPEC, u8, CFDC_A, 3, O>;
impl<'a, const O: u8> CFDC_W<'a, O> {
    #[doc = "FIFO Depth = 0 message"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CFDC_A::_000)
    }
    #[doc = "FIFO Depth = 4 messages"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CFDC_A::_001)
    }
    #[doc = "FIFO Depth = 8 messages"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CFDC_A::_010)
    }
    #[doc = "FIFO Depth = 16 messages"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CFDC_A::_011)
    }
    #[doc = "FIFO Depth = 32 messages"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CFDC_A::_100)
    }
    #[doc = "FIFO Depth = 48 messages"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CFDC_A::_101)
    }
    #[doc = "FIFO Depth = Reserved"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CFDC_A::_110)
    }
    #[doc = "FIFO Depth = Reserved"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CFDC_A::_111)
    }
}
#[doc = "Field `CFITT` reader - Common FIFO Interval Transmission Time"]
pub type CFITT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFITT` writer - Common FIFO Interval Transmission Time"]
pub type CFITT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFCC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Common FIFO Enable"]
    #[inline(always)]
    pub fn cfe(&self) -> CFE_R {
        CFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Common FIFO RX Interrupt Enable"]
    #[inline(always)]
    pub fn cfrxie(&self) -> CFRXIE_R {
        CFRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Common FIFO TX Interrupt Enable"]
    #[inline(always)]
    pub fn cftxie(&self) -> CFTXIE_R {
        CFTXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Common FIFO Payload Data Size Configuration"]
    #[inline(always)]
    pub fn cfpls(&self) -> CFPLS_R {
        CFPLS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Common FIFO Mode"]
    #[inline(always)]
    pub fn cfm(&self) -> CFM_R {
        CFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Common FIFO Interval Timer Source Select"]
    #[inline(always)]
    pub fn cfitss(&self) -> CFITSS_R {
        CFITSS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Common FIFO Interval Timer Resolution"]
    #[inline(always)]
    pub fn cfitr(&self) -> CFITR_R {
        CFITR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Common FIFO Interrupt Mode"]
    #[inline(always)]
    pub fn cfim(&self) -> CFIM_R {
        CFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Common FIFO Interrupt Generation Counter Value"]
    #[inline(always)]
    pub fn cfigcv(&self) -> CFIGCV_R {
        CFIGCV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Common FIFO TX Message Buffer Link"]
    #[inline(always)]
    pub fn cftml(&self) -> CFTML_R {
        CFTML_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:23 - Common FIFO Depth Configuration"]
    #[inline(always)]
    pub fn cfdc(&self) -> CFDC_R {
        CFDC_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Common FIFO Interval Transmission Time"]
    #[inline(always)]
    pub fn cfitt(&self) -> CFITT_R {
        CFITT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Common FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfe(&mut self) -> CFE_W<0> {
        CFE_W::new(self)
    }
    #[doc = "Bit 1 - Common FIFO RX Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfrxie(&mut self) -> CFRXIE_W<1> {
        CFRXIE_W::new(self)
    }
    #[doc = "Bit 2 - Common FIFO TX Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cftxie(&mut self) -> CFTXIE_W<2> {
        CFTXIE_W::new(self)
    }
    #[doc = "Bits 4:6 - Common FIFO Payload Data Size Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cfpls(&mut self) -> CFPLS_W<4> {
        CFPLS_W::new(self)
    }
    #[doc = "Bit 8 - Common FIFO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfm(&mut self) -> CFM_W<8> {
        CFM_W::new(self)
    }
    #[doc = "Bit 10 - Common FIFO Interval Timer Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn cfitss(&mut self) -> CFITSS_W<10> {
        CFITSS_W::new(self)
    }
    #[doc = "Bit 11 - Common FIFO Interval Timer Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn cfitr(&mut self) -> CFITR_W<11> {
        CFITR_W::new(self)
    }
    #[doc = "Bit 12 - Common FIFO Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfim(&mut self) -> CFIM_W<12> {
        CFIM_W::new(self)
    }
    #[doc = "Bits 13:15 - Common FIFO Interrupt Generation Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn cfigcv(&mut self) -> CFIGCV_W<13> {
        CFIGCV_W::new(self)
    }
    #[doc = "Bits 16:17 - Common FIFO TX Message Buffer Link"]
    #[inline(always)]
    #[must_use]
    pub fn cftml(&mut self) -> CFTML_W<16> {
        CFTML_W::new(self)
    }
    #[doc = "Bits 21:23 - Common FIFO Depth Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cfdc(&mut self) -> CFDC_W<21> {
        CFDC_W::new(self)
    }
    #[doc = "Bits 24:31 - Common FIFO Interval Transmission Time"]
    #[inline(always)]
    #[must_use]
    pub fn cfitt(&mut self) -> CFITT_W<24> {
        CFITT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common FIFO Configuration/Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcfcc](index.html) module"]
pub struct CFDCFCC_SPEC;
impl crate::RegisterSpec for CFDCFCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcfcc::R](R) reader structure"]
impl crate::Readable for CFDCFCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdcfcc::W](W) writer structure"]
impl crate::Writable for CFDCFCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCFCC to value 0"]
impl crate::Resettable for CFDCFCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
