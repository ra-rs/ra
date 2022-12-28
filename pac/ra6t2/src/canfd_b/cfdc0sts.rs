#[doc = "Register `CFDC0STS` reader"]
pub struct R(crate::R<CFDC0STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0STS` writer"]
pub struct W(crate::W<CFDC0STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0STS_SPEC>;
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
impl From<crate::W<CFDC0STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSTSTS` reader - Channel Reset Status"]
pub type CRSTSTS_R = crate::BitReader<CRSTSTS_A>;
#[doc = "Channel Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSTSTS_A {
    #[doc = "0: Channel not in Reset mode"]
    _0 = 0,
    #[doc = "1: Channel in Reset mode"]
    _1 = 1,
}
impl From<CRSTSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CRSTSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSTSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSTSTS_A {
        match self.bits {
            false => CRSTSTS_A::_0,
            true => CRSTSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRSTSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRSTSTS_A::_1
    }
}
#[doc = "Field `CHLTSTS` reader - Channel Halt Status"]
pub type CHLTSTS_R = crate::BitReader<CHLTSTS_A>;
#[doc = "Channel Halt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLTSTS_A {
    #[doc = "0: Channel not in Halt mode"]
    _0 = 0,
    #[doc = "1: Channel in Halt mode"]
    _1 = 1,
}
impl From<CHLTSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CHLTSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CHLTSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHLTSTS_A {
        match self.bits {
            false => CHLTSTS_A::_0,
            true => CHLTSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHLTSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHLTSTS_A::_1
    }
}
#[doc = "Field `CSLPSTS` reader - Channel Sleep Status"]
pub type CSLPSTS_R = crate::BitReader<CSLPSTS_A>;
#[doc = "Channel Sleep Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSLPSTS_A {
    #[doc = "0: Channel not in Sleep mode"]
    _0 = 0,
    #[doc = "1: Channel in Sleep mode"]
    _1 = 1,
}
impl From<CSLPSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CSLPSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CSLPSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSLPSTS_A {
        match self.bits {
            false => CSLPSTS_A::_0,
            true => CSLPSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSLPSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSLPSTS_A::_1
    }
}
#[doc = "Field `EPSTS` reader - Channel Error Passive Status"]
pub type EPSTS_R = crate::BitReader<EPSTS_A>;
#[doc = "Channel Error Passive Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPSTS_A {
    #[doc = "0: Channel not in error passive state"]
    _0 = 0,
    #[doc = "1: Channel in error passive state"]
    _1 = 1,
}
impl From<EPSTS_A> for bool {
    #[inline(always)]
    fn from(variant: EPSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl EPSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPSTS_A {
        match self.bits {
            false => EPSTS_A::_0,
            true => EPSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPSTS_A::_1
    }
}
#[doc = "Field `BOSTS` reader - Channel Bus-Off Status"]
pub type BOSTS_R = crate::BitReader<BOSTS_A>;
#[doc = "Channel Bus-Off Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOSTS_A {
    #[doc = "0: Channel not in bus-off state"]
    _0 = 0,
    #[doc = "1: Channel in bus-off state"]
    _1 = 1,
}
impl From<BOSTS_A> for bool {
    #[inline(always)]
    fn from(variant: BOSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl BOSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOSTS_A {
        match self.bits {
            false => BOSTS_A::_0,
            true => BOSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOSTS_A::_1
    }
}
#[doc = "Field `TRMSTS` reader - Channel Transmit Status"]
pub type TRMSTS_R = crate::BitReader<TRMSTS_A>;
#[doc = "Channel Transmit Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMSTS_A {
    #[doc = "0: Channel is not transmitting"]
    _0 = 0,
    #[doc = "1: Channel is transmitting"]
    _1 = 1,
}
impl From<TRMSTS_A> for bool {
    #[inline(always)]
    fn from(variant: TRMSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl TRMSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRMSTS_A {
        match self.bits {
            false => TRMSTS_A::_0,
            true => TRMSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMSTS_A::_1
    }
}
#[doc = "Field `RECSTS` reader - Channel Receive Status"]
pub type RECSTS_R = crate::BitReader<RECSTS_A>;
#[doc = "Channel Receive Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECSTS_A {
    #[doc = "0: Channel is not receiving"]
    _0 = 0,
    #[doc = "1: Channel is receiving"]
    _1 = 1,
}
impl From<RECSTS_A> for bool {
    #[inline(always)]
    fn from(variant: RECSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl RECSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECSTS_A {
        match self.bits {
            false => RECSTS_A::_0,
            true => RECSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECSTS_A::_1
    }
}
#[doc = "Field `COMSTS` reader - Channel Communication Status"]
pub type COMSTS_R = crate::BitReader<COMSTS_A>;
#[doc = "Channel Communication Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMSTS_A {
    #[doc = "0: Channel is not ready for communication"]
    _0 = 0,
    #[doc = "1: Channel is ready for communication"]
    _1 = 1,
}
impl From<COMSTS_A> for bool {
    #[inline(always)]
    fn from(variant: COMSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl COMSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMSTS_A {
        match self.bits {
            false => COMSTS_A::_0,
            true => COMSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMSTS_A::_1
    }
}
#[doc = "Field `ESIF` reader - Error State Indication Flag"]
pub type ESIF_R = crate::BitReader<ESIF_A>;
#[doc = "Error State Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESIF_A {
    #[doc = "0: No CANFD message has been received when the ESI flag was set"]
    _0 = 0,
    #[doc = "1: At least one CANFD message was received when the ESI flag was set"]
    _1 = 1,
}
impl From<ESIF_A> for bool {
    #[inline(always)]
    fn from(variant: ESIF_A) -> Self {
        variant as u8 != 0
    }
}
impl ESIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESIF_A {
        match self.bits {
            false => ESIF_A::_0,
            true => ESIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESIF_A::_1
    }
}
#[doc = "Field `ESIF` writer - Error State Indication Flag"]
pub type ESIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0STS_SPEC, ESIF_A, O>;
impl<'a, const O: u8> ESIF_W<'a, O> {
    #[doc = "No CANFD message has been received when the ESI flag was set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESIF_A::_0)
    }
    #[doc = "At least one CANFD message was received when the ESI flag was set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESIF_A::_1)
    }
}
#[doc = "Field `REC` reader - Reception Error Count"]
pub type REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEC` reader - Transmission Error Count"]
pub type TEC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Channel Reset Status"]
    #[inline(always)]
    pub fn crststs(&self) -> CRSTSTS_R {
        CRSTSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halt Status"]
    #[inline(always)]
    pub fn chltsts(&self) -> CHLTSTS_R {
        CHLTSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Sleep Status"]
    #[inline(always)]
    pub fn cslpsts(&self) -> CSLPSTS_R {
        CSLPSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Error Passive Status"]
    #[inline(always)]
    pub fn epsts(&self) -> EPSTS_R {
        EPSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Bus-Off Status"]
    #[inline(always)]
    pub fn bosts(&self) -> BOSTS_R {
        BOSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Transmit Status"]
    #[inline(always)]
    pub fn trmsts(&self) -> TRMSTS_R {
        TRMSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Receive Status"]
    #[inline(always)]
    pub fn recsts(&self) -> RECSTS_R {
        RECSTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Communication Status"]
    #[inline(always)]
    pub fn comsts(&self) -> COMSTS_R {
        COMSTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error State Indication Flag"]
    #[inline(always)]
    pub fn esif(&self) -> ESIF_R {
        ESIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Reception Error Count"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmission Error Count"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Error State Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn esif(&mut self) -> ESIF_W<8> {
        ESIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0sts](index.html) module"]
pub struct CFDC0STS_SPEC;
impl crate::RegisterSpec for CFDC0STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0sts::R](R) reader structure"]
impl crate::Readable for CFDC0STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0sts::W](W) writer structure"]
impl crate::Writable for CFDC0STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0STS to value 0x05"]
impl crate::Resettable for CFDC0STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
