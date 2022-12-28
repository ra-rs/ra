#[doc = "Register `CFDC0FDSTS` reader"]
pub struct R(crate::R<CFDC0FDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0FDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0FDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0FDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0FDSTS` writer"]
pub struct W(crate::W<CFDC0FDSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0FDSTS_SPEC>;
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
impl From<crate::W<CFDC0FDSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0FDSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDCR` reader - Transceiver Delay Compensation Result"]
pub type TDCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EOCO` reader - Error Occurrence Counter Overflow"]
pub type EOCO_R = crate::BitReader<EOCO_A>;
#[doc = "Error Occurrence Counter Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCO_A {
    #[doc = "0: Error occurrence counter has not overflowed"]
    _0 = 0,
    #[doc = "1: Error occurrence counter has overflowed"]
    _1 = 1,
}
impl From<EOCO_A> for bool {
    #[inline(always)]
    fn from(variant: EOCO_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCO_A {
        match self.bits {
            false => EOCO_A::_0,
            true => EOCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOCO_A::_1
    }
}
#[doc = "Field `EOCO` writer - Error Occurrence Counter Overflow"]
pub type EOCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDSTS_SPEC, EOCO_A, O>;
impl<'a, const O: u8> EOCO_W<'a, O> {
    #[doc = "Error occurrence counter has not overflowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOCO_A::_0)
    }
    #[doc = "Error occurrence counter has overflowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOCO_A::_1)
    }
}
#[doc = "Field `SOCO` reader - Successful Occurrence Counter Overflow"]
pub type SOCO_R = crate::BitReader<SOCO_A>;
#[doc = "Successful Occurrence Counter Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOCO_A {
    #[doc = "0: Successful occurrence counter has not overflowed"]
    _0 = 0,
    #[doc = "1: Successful occurrence counter has overflowed"]
    _1 = 1,
}
impl From<SOCO_A> for bool {
    #[inline(always)]
    fn from(variant: SOCO_A) -> Self {
        variant as u8 != 0
    }
}
impl SOCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOCO_A {
        match self.bits {
            false => SOCO_A::_0,
            true => SOCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOCO_A::_1
    }
}
#[doc = "Field `SOCO` writer - Successful Occurrence Counter Overflow"]
pub type SOCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDSTS_SPEC, SOCO_A, O>;
impl<'a, const O: u8> SOCO_W<'a, O> {
    #[doc = "Successful occurrence counter has not overflowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOCO_A::_0)
    }
    #[doc = "Successful occurrence counter has overflowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOCO_A::_1)
    }
}
#[doc = "Field `TDCVF` reader - Transceiver Delay Compensation Violation Flag"]
pub type TDCVF_R = crate::BitReader<TDCVF_A>;
#[doc = "Transceiver Delay Compensation Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCVF_A {
    #[doc = "0: Transceiver delay compensation violation has not occurred"]
    _0 = 0,
    #[doc = "1: Transceiver delay compensation violation has occurred"]
    _1 = 1,
}
impl From<TDCVF_A> for bool {
    #[inline(always)]
    fn from(variant: TDCVF_A) -> Self {
        variant as u8 != 0
    }
}
impl TDCVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCVF_A {
        match self.bits {
            false => TDCVF_A::_0,
            true => TDCVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDCVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDCVF_A::_1
    }
}
#[doc = "Field `TDCVF` writer - Transceiver Delay Compensation Violation Flag"]
pub type TDCVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0FDSTS_SPEC, TDCVF_A, O>;
impl<'a, const O: u8> TDCVF_W<'a, O> {
    #[doc = "Transceiver delay compensation violation has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCVF_A::_0)
    }
    #[doc = "Transceiver delay compensation violation has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCVF_A::_1)
    }
}
#[doc = "Field `EOC` reader - Error Occurrence Counter"]
pub type EOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOC` reader - Successful occurrence counter"]
pub type SOC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transceiver Delay Compensation Result"]
    #[inline(always)]
    pub fn tdcr(&self) -> TDCR_R {
        TDCR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Error Occurrence Counter Overflow"]
    #[inline(always)]
    pub fn eoco(&self) -> EOCO_R {
        EOCO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Successful Occurrence Counter Overflow"]
    #[inline(always)]
    pub fn soco(&self) -> SOCO_R {
        SOCO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Violation Flag"]
    #[inline(always)]
    pub fn tdcvf(&self) -> TDCVF_R {
        TDCVF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Error Occurrence Counter"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Successful occurrence counter"]
    #[inline(always)]
    pub fn soc(&self) -> SOC_R {
        SOC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Error Occurrence Counter Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn eoco(&mut self) -> EOCO_W<8> {
        EOCO_W::new(self)
    }
    #[doc = "Bit 9 - Successful Occurrence Counter Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn soco(&mut self) -> SOCO_W<9> {
        SOCO_W::new(self)
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Violation Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tdcvf(&mut self) -> TDCVF_W<15> {
        TDCVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 CANFD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0fdsts](index.html) module"]
pub struct CFDC0FDSTS_SPEC;
impl crate::RegisterSpec for CFDC0FDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0fdsts::R](R) reader structure"]
impl crate::Readable for CFDC0FDSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0fdsts::W](W) writer structure"]
impl crate::Writable for CFDC0FDSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0FDSTS to value 0"]
impl crate::Resettable for CFDC0FDSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
