#[doc = "Register `DATBAS%s` reader"]
pub struct R(crate::R<DATBAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATBAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATBAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATBAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATBAS%s` writer"]
pub struct W(crate::W<DATBAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATBAS_SPEC>;
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
impl From<crate::W<DATBAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATBAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVSTAD` reader - Device Static Address"]
pub type DVSTAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVSTAD` writer - Device Static Address"]
pub type DVSTAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATBAS_SPEC, u8, u8, 7, O>;
#[doc = "Field `DVIBIPL` reader - Device IBI Payload"]
pub type DVIBIPL_R = crate::BitReader<DVIBIPL_A>;
#[doc = "Device IBI Payload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVIBIPL_A {
    #[doc = "0: IBIs from this Device do not carry a Data Payload."]
    _0 = 0,
    #[doc = "1: IBIs from this Device do carry a Data Payload."]
    _1 = 1,
}
impl From<DVIBIPL_A> for bool {
    #[inline(always)]
    fn from(variant: DVIBIPL_A) -> Self {
        variant as u8 != 0
    }
}
impl DVIBIPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVIBIPL_A {
        match self.bits {
            false => DVIBIPL_A::_0,
            true => DVIBIPL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVIBIPL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVIBIPL_A::_1
    }
}
#[doc = "Field `DVIBIPL` writer - Device IBI Payload"]
pub type DVIBIPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATBAS_SPEC, DVIBIPL_A, O>;
impl<'a, const O: u8> DVIBIPL_W<'a, O> {
    #[doc = "IBIs from this Device do not carry a Data Payload."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVIBIPL_A::_0)
    }
    #[doc = "IBIs from this Device do carry a Data Payload."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVIBIPL_A::_1)
    }
}
#[doc = "Field `DVSIRRJ` reader - Device In-Band Slave Interrupt Request Reject"]
pub type DVSIRRJ_R = crate::BitReader<DVSIRRJ_A>;
#[doc = "Device In-Band Slave Interrupt Request Reject\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVSIRRJ_A {
    #[doc = "0: This Device shall ACK the SIR."]
    _0 = 0,
    #[doc = "1: This Device shall NACK the SIR and send the auto-disable CCC."]
    _1 = 1,
}
impl From<DVSIRRJ_A> for bool {
    #[inline(always)]
    fn from(variant: DVSIRRJ_A) -> Self {
        variant as u8 != 0
    }
}
impl DVSIRRJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVSIRRJ_A {
        match self.bits {
            false => DVSIRRJ_A::_0,
            true => DVSIRRJ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVSIRRJ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVSIRRJ_A::_1
    }
}
#[doc = "Field `DVSIRRJ` writer - Device In-Band Slave Interrupt Request Reject"]
pub type DVSIRRJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATBAS_SPEC, DVSIRRJ_A, O>;
impl<'a, const O: u8> DVSIRRJ_W<'a, O> {
    #[doc = "This Device shall ACK the SIR."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVSIRRJ_A::_0)
    }
    #[doc = "This Device shall NACK the SIR and send the auto-disable CCC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVSIRRJ_A::_1)
    }
}
#[doc = "Field `DVMRRJ` reader - Device In-Band Master Request Reject"]
pub type DVMRRJ_R = crate::BitReader<DVMRRJ_A>;
#[doc = "Device In-Band Master Request Reject\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVMRRJ_A {
    #[doc = "0: This Device shall ACK Master Requests."]
    _0 = 0,
    #[doc = "1: This Device shall NACK Master Requests and send the auto-disable command."]
    _1 = 1,
}
impl From<DVMRRJ_A> for bool {
    #[inline(always)]
    fn from(variant: DVMRRJ_A) -> Self {
        variant as u8 != 0
    }
}
impl DVMRRJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVMRRJ_A {
        match self.bits {
            false => DVMRRJ_A::_0,
            true => DVMRRJ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVMRRJ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVMRRJ_A::_1
    }
}
#[doc = "Field `DVMRRJ` writer - Device In-Band Master Request Reject"]
pub type DVMRRJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATBAS_SPEC, DVMRRJ_A, O>;
impl<'a, const O: u8> DVMRRJ_W<'a, O> {
    #[doc = "This Device shall ACK Master Requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVMRRJ_A::_0)
    }
    #[doc = "This Device shall NACK Master Requests and send the auto-disable command."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVMRRJ_A::_1)
    }
}
#[doc = "Field `DVIBITS` reader - Device IBI Time-stamp"]
pub type DVIBITS_R = crate::BitReader<DVIBITS_A>;
#[doc = "Device IBI Time-stamp\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVIBITS_A {
    #[doc = "0: The Master shall not time-stamp IBIs from this Device with Master Time-stamps."]
    _0 = 0,
    #[doc = "1: The Master shall time-stamp IBIs for this Device with Master Time-stamps."]
    _1 = 1,
}
impl From<DVIBITS_A> for bool {
    #[inline(always)]
    fn from(variant: DVIBITS_A) -> Self {
        variant as u8 != 0
    }
}
impl DVIBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVIBITS_A {
        match self.bits {
            false => DVIBITS_A::_0,
            true => DVIBITS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVIBITS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVIBITS_A::_1
    }
}
#[doc = "Field `DVIBITS` writer - Device IBI Time-stamp"]
pub type DVIBITS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATBAS_SPEC, DVIBITS_A, O>;
impl<'a, const O: u8> DVIBITS_W<'a, O> {
    #[doc = "The Master shall not time-stamp IBIs from this Device with Master Time-stamps."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVIBITS_A::_0)
    }
    #[doc = "The Master shall time-stamp IBIs for this Device with Master Time-stamps."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVIBITS_A::_1)
    }
}
#[doc = "Field `DVDYAD` reader - Device I3C Dynamic Address"]
pub type DVDYAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVDYAD` writer - Device I3C Dynamic Address"]
pub type DVDYAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATBAS_SPEC, u8, u8, 8, O>;
#[doc = "Field `DVNACK` reader - Device NACK Retry Count"]
pub type DVNACK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVNACK` writer - Device NACK Retry Count"]
pub type DVNACK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATBAS_SPEC, u8, u8, 2, O>;
#[doc = "Field `DVTYP` reader - Device Type"]
pub type DVTYP_R = crate::BitReader<DVTYP_A>;
#[doc = "Device Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVTYP_A {
    #[doc = "0: I3C Device"]
    _0 = 0,
    #[doc = "1: I2C Device"]
    _1 = 1,
}
impl From<DVTYP_A> for bool {
    #[inline(always)]
    fn from(variant: DVTYP_A) -> Self {
        variant as u8 != 0
    }
}
impl DVTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVTYP_A {
        match self.bits {
            false => DVTYP_A::_0,
            true => DVTYP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVTYP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVTYP_A::_1
    }
}
#[doc = "Field `DVTYP` writer - Device Type"]
pub type DVTYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATBAS_SPEC, DVTYP_A, O>;
impl<'a, const O: u8> DVTYP_W<'a, O> {
    #[doc = "I3C Device"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVTYP_A::_0)
    }
    #[doc = "I2C Device"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVTYP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - Device Static Address"]
    #[inline(always)]
    pub fn dvstad(&self) -> DVSTAD_R {
        DVSTAD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Device IBI Payload"]
    #[inline(always)]
    pub fn dvibipl(&self) -> DVIBIPL_R {
        DVIBIPL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Device In-Band Slave Interrupt Request Reject"]
    #[inline(always)]
    pub fn dvsirrj(&self) -> DVSIRRJ_R {
        DVSIRRJ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Device In-Band Master Request Reject"]
    #[inline(always)]
    pub fn dvmrrj(&self) -> DVMRRJ_R {
        DVMRRJ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Device IBI Time-stamp"]
    #[inline(always)]
    pub fn dvibits(&self) -> DVIBITS_R {
        DVIBITS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Device I3C Dynamic Address"]
    #[inline(always)]
    pub fn dvdyad(&self) -> DVDYAD_R {
        DVDYAD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - Device NACK Retry Count"]
    #[inline(always)]
    pub fn dvnack(&self) -> DVNACK_R {
        DVNACK_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Device Type"]
    #[inline(always)]
    pub fn dvtyp(&self) -> DVTYP_R {
        DVTYP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Static Address"]
    #[inline(always)]
    #[must_use]
    pub fn dvstad(&mut self) -> DVSTAD_W<0> {
        DVSTAD_W::new(self)
    }
    #[doc = "Bit 12 - Device IBI Payload"]
    #[inline(always)]
    #[must_use]
    pub fn dvibipl(&mut self) -> DVIBIPL_W<12> {
        DVIBIPL_W::new(self)
    }
    #[doc = "Bit 13 - Device In-Band Slave Interrupt Request Reject"]
    #[inline(always)]
    #[must_use]
    pub fn dvsirrj(&mut self) -> DVSIRRJ_W<13> {
        DVSIRRJ_W::new(self)
    }
    #[doc = "Bit 14 - Device In-Band Master Request Reject"]
    #[inline(always)]
    #[must_use]
    pub fn dvmrrj(&mut self) -> DVMRRJ_W<14> {
        DVMRRJ_W::new(self)
    }
    #[doc = "Bit 15 - Device IBI Time-stamp"]
    #[inline(always)]
    #[must_use]
    pub fn dvibits(&mut self) -> DVIBITS_W<15> {
        DVIBITS_W::new(self)
    }
    #[doc = "Bits 16:23 - Device I3C Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn dvdyad(&mut self) -> DVDYAD_W<16> {
        DVDYAD_W::new(self)
    }
    #[doc = "Bits 29:30 - Device NACK Retry Count"]
    #[inline(always)]
    #[must_use]
    pub fn dvnack(&mut self) -> DVNACK_W<29> {
        DVNACK_W::new(self)
    }
    #[doc = "Bit 31 - Device Type"]
    #[inline(always)]
    #[must_use]
    pub fn dvtyp(&mut self) -> DVTYP_W<31> {
        DVTYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Address Table Basic Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datbas](index.html) module"]
pub struct DATBAS_SPEC;
impl crate::RegisterSpec for DATBAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datbas::R](R) reader structure"]
impl crate::Readable for DATBAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datbas::W](W) writer structure"]
impl crate::Writable for DATBAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATBAS%s to value 0"]
impl crate::Resettable for DATBAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
