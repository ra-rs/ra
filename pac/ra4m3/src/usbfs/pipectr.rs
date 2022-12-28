#[doc = "Register `PIPE%sCTR` reader"]
pub struct R(crate::R<PIPECTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPECTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPECTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPECTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPE%sCTR` writer"]
pub struct W(crate::W<PIPECTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPECTR_SPEC>;
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
impl From<crate::W<PIPECTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPECTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - Response PID"]
pub type PID_R = crate::FieldReader<u8, PID_A>;
#[doc = "Response PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: NAK response"]
    _00 = 0,
    #[doc = "1: BUF response (depends buffer state)"]
    _01 = 1,
    #[doc = "2: STALL response"]
    _10 = 2,
    #[doc = "3: STALL response"]
    _11 = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
impl PID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::_00,
            1 => PID_A::_01,
            2 => PID_A::_10,
            3 => PID_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PID_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PID_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PID_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PID_A::_11
    }
}
#[doc = "Field `PID` writer - Response PID"]
pub type PID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, PIPECTR_SPEC, u8, PID_A, 2, O>;
impl<'a, const O: u8> PID_W<'a, O> {
    #[doc = "NAK response"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PID_A::_00)
    }
    #[doc = "BUF response (depends buffer state)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PID_A::_01)
    }
    #[doc = "STALL response"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PID_A::_10)
    }
    #[doc = "STALL response"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PID_A::_11)
    }
}
#[doc = "Field `PBUSY` reader - Pipe Busy"]
pub type PBUSY_R = crate::BitReader<PBUSY_A>;
#[doc = "Pipe Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBUSY_A {
    #[doc = "0: Pipe n not in use for the transaction"]
    _0 = 0,
    #[doc = "1: Pipe n in use for the transaction"]
    _1 = 1,
}
impl From<PBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl PBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBUSY_A {
        match self.bits {
            false => PBUSY_A::_0,
            true => PBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PBUSY_A::_1
    }
}
#[doc = "Field `SQMON` reader - Sequence Toggle Bit Confirmation"]
pub type SQMON_R = crate::BitReader<SQMON_A>;
#[doc = "Sequence Toggle Bit Confirmation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQMON_A {
    #[doc = "0: DATA0"]
    _0 = 0,
    #[doc = "1: DATA1"]
    _1 = 1,
}
impl From<SQMON_A> for bool {
    #[inline(always)]
    fn from(variant: SQMON_A) -> Self {
        variant as u8 != 0
    }
}
impl SQMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQMON_A {
        match self.bits {
            false => SQMON_A::_0,
            true => SQMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SQMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SQMON_A::_1
    }
}
#[doc = "Field `SQSET` reader - Sequence Toggle Bit Set"]
pub type SQSET_R = crate::BitReader<SQSET_A>;
#[doc = "Sequence Toggle Bit Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQSET_A {
    #[doc = "0: Invalid (writing 0 has no effect)"]
    _0 = 0,
    #[doc = "1: Set the expected value for the next transaction to DATA1"]
    _1 = 1,
}
impl From<SQSET_A> for bool {
    #[inline(always)]
    fn from(variant: SQSET_A) -> Self {
        variant as u8 != 0
    }
}
impl SQSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQSET_A {
        match self.bits {
            false => SQSET_A::_0,
            true => SQSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SQSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SQSET_A::_1
    }
}
#[doc = "Field `SQSET` writer - Sequence Toggle Bit Set"]
pub type SQSET_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECTR_SPEC, SQSET_A, O>;
impl<'a, const O: u8> SQSET_W<'a, O> {
    #[doc = "Invalid (writing 0 has no effect)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SQSET_A::_0)
    }
    #[doc = "Set the expected value for the next transaction to DATA1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SQSET_A::_1)
    }
}
#[doc = "Field `SQCLR` reader - Sequence Toggle Bit Clear"]
pub type SQCLR_R = crate::BitReader<SQCLR_A>;
#[doc = "Sequence Toggle Bit Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQCLR_A {
    #[doc = "0: Invalid (writing 0 has no effect)"]
    _0 = 0,
    #[doc = "1: Clear the expected value for the next transaction to DATA0"]
    _1 = 1,
}
impl From<SQCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SQCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl SQCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQCLR_A {
        match self.bits {
            false => SQCLR_A::_0,
            true => SQCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SQCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SQCLR_A::_1
    }
}
#[doc = "Field `SQCLR` writer - Sequence Toggle Bit Clear"]
pub type SQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECTR_SPEC, SQCLR_A, O>;
impl<'a, const O: u8> SQCLR_W<'a, O> {
    #[doc = "Invalid (writing 0 has no effect)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SQCLR_A::_0)
    }
    #[doc = "Clear the expected value for the next transaction to DATA0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SQCLR_A::_1)
    }
}
#[doc = "Field `ACLRM` reader - Auto Buffer Clear Mode"]
pub type ACLRM_R = crate::BitReader<ACLRM_A>;
#[doc = "Auto Buffer Clear Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACLRM_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable (initialize all buffers)"]
    _1 = 1,
}
impl From<ACLRM_A> for bool {
    #[inline(always)]
    fn from(variant: ACLRM_A) -> Self {
        variant as u8 != 0
    }
}
impl ACLRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACLRM_A {
        match self.bits {
            false => ACLRM_A::_0,
            true => ACLRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACLRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACLRM_A::_1
    }
}
#[doc = "Field `ACLRM` writer - Auto Buffer Clear Mode"]
pub type ACLRM_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECTR_SPEC, ACLRM_A, O>;
impl<'a, const O: u8> ACLRM_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACLRM_A::_0)
    }
    #[doc = "Enable (initialize all buffers)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACLRM_A::_1)
    }
}
#[doc = "Field `ATREPM` reader - Auto Response Mode"]
pub type ATREPM_R = crate::BitReader<ATREPM_A>;
#[doc = "Auto Response Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATREPM_A {
    #[doc = "0: Disable auto response mode"]
    _0 = 0,
    #[doc = "1: Enable auto response mode"]
    _1 = 1,
}
impl From<ATREPM_A> for bool {
    #[inline(always)]
    fn from(variant: ATREPM_A) -> Self {
        variant as u8 != 0
    }
}
impl ATREPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATREPM_A {
        match self.bits {
            false => ATREPM_A::_0,
            true => ATREPM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATREPM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATREPM_A::_1
    }
}
#[doc = "Field `ATREPM` writer - Auto Response Mode"]
pub type ATREPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECTR_SPEC, ATREPM_A, O>;
impl<'a, const O: u8> ATREPM_W<'a, O> {
    #[doc = "Disable auto response mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATREPM_A::_0)
    }
    #[doc = "Enable auto response mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATREPM_A::_1)
    }
}
#[doc = "Field `INBUFM` reader - Transmit Buffer Monitor"]
pub type INBUFM_R = crate::BitReader<INBUFM_A>;
#[doc = "Transmit Buffer Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INBUFM_A {
    #[doc = "0: No data to be transmitted is in the FIFO buffer"]
    _0 = 0,
    #[doc = "1: Data to be transmitted is in the FIFO buffer"]
    _1 = 1,
}
impl From<INBUFM_A> for bool {
    #[inline(always)]
    fn from(variant: INBUFM_A) -> Self {
        variant as u8 != 0
    }
}
impl INBUFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INBUFM_A {
        match self.bits {
            false => INBUFM_A::_0,
            true => INBUFM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INBUFM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INBUFM_A::_1
    }
}
#[doc = "Field `BSTS` reader - Buffer Status"]
pub type BSTS_R = crate::BitReader<BSTS_A>;
#[doc = "Buffer Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSTS_A {
    #[doc = "0: Buffer access by the CPU disabled"]
    _0 = 0,
    #[doc = "1: Buffer access by the CPU enabled"]
    _1 = 1,
}
impl From<BSTS_A> for bool {
    #[inline(always)]
    fn from(variant: BSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl BSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSTS_A {
        match self.bits {
            false => BSTS_A::_0,
            true => BSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSTS_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Response PID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Pipe Busy"]
    #[inline(always)]
    pub fn pbusy(&self) -> PBUSY_R {
        PBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequence Toggle Bit Confirmation"]
    #[inline(always)]
    pub fn sqmon(&self) -> SQMON_R {
        SQMON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sequence Toggle Bit Set"]
    #[inline(always)]
    pub fn sqset(&self) -> SQSET_R {
        SQSET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub fn sqclr(&self) -> SQCLR_R {
        SQCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto Buffer Clear Mode"]
    #[inline(always)]
    pub fn aclrm(&self) -> ACLRM_R {
        ACLRM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Auto Response Mode"]
    #[inline(always)]
    pub fn atrepm(&self) -> ATREPM_R {
        ATREPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Buffer Monitor"]
    #[inline(always)]
    pub fn inbufm(&self) -> INBUFM_R {
        INBUFM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer Status"]
    #[inline(always)]
    pub fn bsts(&self) -> BSTS_R {
        BSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response PID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<0> {
        PID_W::new(self)
    }
    #[doc = "Bit 7 - Sequence Toggle Bit Set"]
    #[inline(always)]
    #[must_use]
    pub fn sqset(&mut self) -> SQSET_W<7> {
        SQSET_W::new(self)
    }
    #[doc = "Bit 8 - Sequence Toggle Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sqclr(&mut self) -> SQCLR_W<8> {
        SQCLR_W::new(self)
    }
    #[doc = "Bit 9 - Auto Buffer Clear Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aclrm(&mut self) -> ACLRM_W<9> {
        ACLRM_W::new(self)
    }
    #[doc = "Bit 10 - Auto Response Mode"]
    #[inline(always)]
    #[must_use]
    pub fn atrepm(&mut self) -> ATREPM_W<10> {
        ATREPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIPE%s Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipectr](index.html) module"]
pub struct PIPECTR_SPEC;
impl crate::RegisterSpec for PIPECTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipectr::R](R) reader structure"]
impl crate::Readable for PIPECTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipectr::W](W) writer structure"]
impl crate::Writable for PIPECTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPE%sCTR to value 0"]
impl crate::Resettable for PIPECTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
