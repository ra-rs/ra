#[doc = "Register `ICCR2` reader"]
pub struct R(crate::R<ICCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCR2` writer"]
pub struct W(crate::W<ICCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCR2_SPEC>;
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
impl From<crate::W<ICCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
pub type ST_R = crate::BitReader<ST_A>;
#[doc = "Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    #[doc = "0: Does not request to issue a start condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a start condition."]
    _1 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
impl ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::_0,
            true => ST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ST_A::_1
    }
}
#[doc = "Field `ST` writer - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR2_SPEC, ST_A, O>;
impl<'a, const O: u8> ST_W<'a, O> {
    #[doc = "Does not request to issue a start condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ST_A::_0)
    }
    #[doc = "Requests to issue a start condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ST_A::_1)
    }
}
#[doc = "Field `RS` reader - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition."]
pub type RS_R = crate::BitReader<RS_A>;
#[doc = "Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS_A {
    #[doc = "0: Does not request to issue a restart condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a restart condition."]
    _1 = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::_0,
            true => RS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RS_A::_1
    }
}
#[doc = "Field `RS` writer - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition."]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR2_SPEC, RS_A, O>;
impl<'a, const O: u8> RS_W<'a, O> {
    #[doc = "Does not request to issue a restart condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RS_A::_0)
    }
    #[doc = "Requests to issue a restart condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RS_A::_1)
    }
}
#[doc = "Field `SP` reader - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued."]
pub type SP_R = crate::BitReader<SP_A>;
#[doc = "Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP_A {
    #[doc = "0: Does not request to issue a stop condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a stop condition."]
    _1 = 1,
}
impl From<SP_A> for bool {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as u8 != 0
    }
}
impl SP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP_A {
        match self.bits {
            false => SP_A::_0,
            true => SP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP_A::_1
    }
}
#[doc = "Field `SP` writer - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued."]
pub type SP_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR2_SPEC, SP_A, O>;
impl<'a, const O: u8> SP_W<'a, O> {
    #[doc = "Does not request to issue a stop condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP_A::_0)
    }
    #[doc = "Requests to issue a stop condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP_A::_1)
    }
}
#[doc = "Field `TRS` reader - Transmit/Receive Mode"]
pub type TRS_R = crate::BitReader<TRS_A>;
#[doc = "Transmit/Receive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRS_A {
    #[doc = "0: Receive mode"]
    _0 = 0,
    #[doc = "1: Transmit mode"]
    _1 = 1,
}
impl From<TRS_A> for bool {
    #[inline(always)]
    fn from(variant: TRS_A) -> Self {
        variant as u8 != 0
    }
}
impl TRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRS_A {
        match self.bits {
            false => TRS_A::_0,
            true => TRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRS_A::_1
    }
}
#[doc = "Field `TRS` writer - Transmit/Receive Mode"]
pub type TRS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR2_SPEC, TRS_A, O>;
impl<'a, const O: u8> TRS_W<'a, O> {
    #[doc = "Receive mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRS_A::_0)
    }
    #[doc = "Transmit mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRS_A::_1)
    }
}
#[doc = "Field `MST` reader - Master/Slave Mode"]
pub type MST_R = crate::BitReader<MST_A>;
#[doc = "Master/Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_A {
    #[doc = "0: Slave mode"]
    _0 = 0,
    #[doc = "1: Master mode"]
    _1 = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::_0,
            true => MST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MST_A::_1
    }
}
#[doc = "Field `MST` writer - Master/Slave Mode"]
pub type MST_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR2_SPEC, MST_A, O>;
impl<'a, const O: u8> MST_W<'a, O> {
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MST_A::_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MST_A::_1)
    }
}
#[doc = "Field `BBSY` reader - Bus Busy Detection Flag"]
pub type BBSY_R = crate::BitReader<BBSY_A>;
#[doc = "Bus Busy Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BBSY_A {
    #[doc = "0: The I2C bus is released (bus free state)."]
    _0 = 0,
    #[doc = "1: The I2C bus is occupied (bus busy state)."]
    _1 = 1,
}
impl From<BBSY_A> for bool {
    #[inline(always)]
    fn from(variant: BBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BBSY_A {
        match self.bits {
            false => BBSY_A::_0,
            true => BBSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BBSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BBSY_A::_1
    }
}
impl R {
    #[doc = "Bit 1 - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued."]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit/Receive Mode"]
    #[inline(always)]
    pub fn trs(&self) -> TRS_R {
        TRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Busy Detection Flag"]
    #[inline(always)]
    pub fn bbsy(&self) -> BBSY_R {
        BBSY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<1> {
        ST_W::new(self)
    }
    #[doc = "Bit 2 - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition."]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<2> {
        RS_W::new(self)
    }
    #[doc = "Bit 3 - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued."]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<3> {
        SP_W::new(self)
    }
    #[doc = "Bit 5 - Transmit/Receive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trs(&mut self) -> TRS_W<5> {
        TRS_W::new(self)
    }
    #[doc = "Bit 6 - Master/Slave Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mst(&mut self) -> MST_W<6> {
        MST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccr2](index.html) module"]
pub struct ICCR2_SPEC;
impl crate::RegisterSpec for ICCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iccr2::R](R) reader structure"]
impl crate::Readable for ICCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccr2::W](W) writer structure"]
impl crate::Writable for ICCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICCR2 to value 0"]
impl crate::Resettable for ICCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
