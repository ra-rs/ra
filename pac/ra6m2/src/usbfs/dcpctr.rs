#[doc = "Register `DCPCTR` reader"]
pub struct R(crate::R<DCPCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCPCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCPCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCPCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCPCTR` writer"]
pub struct W(crate::W<DCPCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCPCTR_SPEC>;
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
impl From<crate::W<DCPCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCPCTR_SPEC>) -> Self {
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
    #[doc = "1: BUF response (depending on the buffer state)"]
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
pub type PID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DCPCTR_SPEC, u8, PID_A, 2, O>;
impl<'a, const O: u8> PID_W<'a, O> {
    #[doc = "NAK response"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PID_A::_00)
    }
    #[doc = "BUF response (depending on the buffer state)"]
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
#[doc = "Field `CCPL` reader - Control Transfer End Enable"]
pub type CCPL_R = crate::BitReader<CCPL_A>;
#[doc = "Control Transfer End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPL_A {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Completion of control transfer is enabled."]
    _1 = 1,
}
impl From<CCPL_A> for bool {
    #[inline(always)]
    fn from(variant: CCPL_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPL_A {
        match self.bits {
            false => CCPL_A::_0,
            true => CCPL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCPL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCPL_A::_1
    }
}
#[doc = "Field `CCPL` writer - Control Transfer End Enable"]
pub type CCPL_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCTR_SPEC, CCPL_A, O>;
impl<'a, const O: u8> CCPL_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCPL_A::_0)
    }
    #[doc = "Completion of control transfer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCPL_A::_1)
    }
}
#[doc = "Field `PBUSY` reader - Pipe Busy"]
pub type PBUSY_R = crate::BitReader<PBUSY_A>;
#[doc = "Pipe Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBUSY_A {
    #[doc = "0: DCP is not used for the transaction."]
    _0 = 0,
    #[doc = "1: DCP is used for the transaction."]
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
#[doc = "Field `SQMON` reader - Sequence Toggle Bit Monitor"]
pub type SQMON_R = crate::BitReader<SQMON_A>;
#[doc = "Sequence Toggle Bit Monitor\n\nValue on reset: 1"]
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
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Specifies DATA1."]
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
pub type SQSET_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCTR_SPEC, SQSET_A, O>;
impl<'a, const O: u8> SQSET_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SQSET_A::_0)
    }
    #[doc = "Specifies DATA1."]
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
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Specifies DATA0."]
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
pub type SQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCTR_SPEC, SQCLR_A, O>;
impl<'a, const O: u8> SQCLR_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SQCLR_A::_0)
    }
    #[doc = "Specifies DATA0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SQCLR_A::_1)
    }
}
#[doc = "Field `SUREQCLR` reader - SUREQ Bit Clear"]
pub type SUREQCLR_R = crate::BitReader<SUREQCLR_A>;
#[doc = "SUREQ Bit Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUREQCLR_A {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Clears the SUREQ bit to 0."]
    _1 = 1,
}
impl From<SUREQCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SUREQCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl SUREQCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUREQCLR_A {
        match self.bits {
            false => SUREQCLR_A::_0,
            true => SUREQCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUREQCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUREQCLR_A::_1
    }
}
#[doc = "Field `SUREQCLR` writer - SUREQ Bit Clear"]
pub type SUREQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCTR_SPEC, SUREQCLR_A, O>;
impl<'a, const O: u8> SUREQCLR_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUREQCLR_A::_0)
    }
    #[doc = "Clears the SUREQ bit to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUREQCLR_A::_1)
    }
}
#[doc = "Field `SUREQ` reader - Setup Token Transmission"]
pub type SUREQ_R = crate::BitReader<SUREQ_A>;
#[doc = "Setup Token Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUREQ_A {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Transmits the setup packet."]
    _1 = 1,
}
impl From<SUREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SUREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SUREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUREQ_A {
        match self.bits {
            false => SUREQ_A::_0,
            true => SUREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUREQ_A::_1
    }
}
#[doc = "Field `SUREQ` writer - Setup Token Transmission"]
pub type SUREQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCTR_SPEC, SUREQ_A, O>;
impl<'a, const O: u8> SUREQ_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUREQ_A::_0)
    }
    #[doc = "Transmits the setup packet."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUREQ_A::_1)
    }
}
#[doc = "Field `BSTS` reader - Buffer Status"]
pub type BSTS_R = crate::BitReader<BSTS_A>;
#[doc = "Buffer Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSTS_A {
    #[doc = "0: Buffer access is disabled."]
    _0 = 0,
    #[doc = "1: Buffer access is enabled."]
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
    #[doc = "Bit 2 - Control Transfer End Enable"]
    #[inline(always)]
    pub fn ccpl(&self) -> CCPL_R {
        CCPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe Busy"]
    #[inline(always)]
    pub fn pbusy(&self) -> PBUSY_R {
        PBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequence Toggle Bit Monitor"]
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
    #[doc = "Bit 11 - SUREQ Bit Clear"]
    #[inline(always)]
    pub fn sureqclr(&self) -> SUREQCLR_R {
        SUREQCLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Setup Token Transmission"]
    #[inline(always)]
    pub fn sureq(&self) -> SUREQ_R {
        SUREQ_R::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 2 - Control Transfer End Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccpl(&mut self) -> CCPL_W<2> {
        CCPL_W::new(self)
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
    #[doc = "Bit 11 - SUREQ Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sureqclr(&mut self) -> SUREQCLR_W<11> {
        SUREQCLR_W::new(self)
    }
    #[doc = "Bit 14 - Setup Token Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn sureq(&mut self) -> SUREQ_W<14> {
        SUREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcpctr](index.html) module"]
pub struct DCPCTR_SPEC;
impl crate::RegisterSpec for DCPCTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dcpctr::R](R) reader structure"]
impl crate::Readable for DCPCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcpctr::W](W) writer structure"]
impl crate::Writable for DCPCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCPCTR to value 0x40"]
impl crate::Resettable for DCPCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
