#[doc = "Register `SCSTLCTL` reader"]
pub struct R(crate::R<SCSTLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSTLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSTLCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSTLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSTLCTL` writer"]
pub struct W(crate::W<SCSTLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSTLCTL_SPEC>;
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
impl From<crate::W<SCSTLCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSTLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STLCYC` reader - Stalling Cycle"]
pub type STLCYC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STLCYC` writer - Stalling Cycle"]
pub type STLCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCSTLCTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `AAPE` reader - Assigned Address Phase Enable"]
pub type AAPE_R = crate::BitReader<AAPE_A>;
#[doc = "Assigned Address Phase Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAPE_A {
    #[doc = "0: Does not stall the SCL clock during the address assignment phase."]
    _0 = 0,
    #[doc = "1: Stall the SCL clock during address assignment phase."]
    _1 = 1,
}
impl From<AAPE_A> for bool {
    #[inline(always)]
    fn from(variant: AAPE_A) -> Self {
        variant as u8 != 0
    }
}
impl AAPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAPE_A {
        match self.bits {
            false => AAPE_A::_0,
            true => AAPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAPE_A::_1
    }
}
#[doc = "Field `AAPE` writer - Assigned Address Phase Enable"]
pub type AAPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSTLCTL_SPEC, AAPE_A, O>;
impl<'a, const O: u8> AAPE_W<'a, O> {
    #[doc = "Does not stall the SCL clock during the address assignment phase."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAPE_A::_0)
    }
    #[doc = "Stall the SCL clock during address assignment phase."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAPE_A::_1)
    }
}
#[doc = "Field `TRAPE` reader - Transition Phase Enable"]
pub type TRAPE_R = crate::BitReader<TRAPE_A>;
#[doc = "Transition Phase Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRAPE_A {
    #[doc = "0: Does not stall the SCL clock during the transition bit in read transfer."]
    _0 = 0,
    #[doc = "1: Stall the SCL clock during the transition bit in read transfer."]
    _1 = 1,
}
impl From<TRAPE_A> for bool {
    #[inline(always)]
    fn from(variant: TRAPE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRAPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRAPE_A {
        match self.bits {
            false => TRAPE_A::_0,
            true => TRAPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRAPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRAPE_A::_1
    }
}
#[doc = "Field `TRAPE` writer - Transition Phase Enable"]
pub type TRAPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSTLCTL_SPEC, TRAPE_A, O>;
impl<'a, const O: u8> TRAPE_W<'a, O> {
    #[doc = "Does not stall the SCL clock during the transition bit in read transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRAPE_A::_0)
    }
    #[doc = "Stall the SCL clock during the transition bit in read transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRAPE_A::_1)
    }
}
#[doc = "Field `PARPE` reader - Parity Phase Enable"]
pub type PARPE_R = crate::BitReader<PARPE_A>;
#[doc = "Parity Phase Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARPE_A {
    #[doc = "0: Does not stall the SCL clock during the parity bit period."]
    _0 = 0,
    #[doc = "1: Stall the SCL clock during the parity bit period."]
    _1 = 1,
}
impl From<PARPE_A> for bool {
    #[inline(always)]
    fn from(variant: PARPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PARPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARPE_A {
        match self.bits {
            false => PARPE_A::_0,
            true => PARPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PARPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PARPE_A::_1
    }
}
#[doc = "Field `PARPE` writer - Parity Phase Enable"]
pub type PARPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSTLCTL_SPEC, PARPE_A, O>;
impl<'a, const O: u8> PARPE_W<'a, O> {
    #[doc = "Does not stall the SCL clock during the parity bit period."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PARPE_A::_0)
    }
    #[doc = "Stall the SCL clock during the parity bit period."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PARPE_A::_1)
    }
}
#[doc = "Field `ACKPE` reader - ACK phase Enable"]
pub type ACKPE_R = crate::BitReader<ACKPE_A>;
#[doc = "ACK phase Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKPE_A {
    #[doc = "0: Does not stall the SCL clock during the ACK/NACK phase."]
    _0 = 0,
    #[doc = "1: Stall the SCL clock during the ACK/NACK phase."]
    _1 = 1,
}
impl From<ACKPE_A> for bool {
    #[inline(always)]
    fn from(variant: ACKPE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKPE_A {
        match self.bits {
            false => ACKPE_A::_0,
            true => ACKPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKPE_A::_1
    }
}
#[doc = "Field `ACKPE` writer - ACK phase Enable"]
pub type ACKPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSTLCTL_SPEC, ACKPE_A, O>;
impl<'a, const O: u8> ACKPE_W<'a, O> {
    #[doc = "Does not stall the SCL clock during the ACK/NACK phase."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKPE_A::_0)
    }
    #[doc = "Stall the SCL clock during the ACK/NACK phase."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKPE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Stalling Cycle"]
    #[inline(always)]
    pub fn stlcyc(&self) -> STLCYC_R {
        STLCYC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 28 - Assigned Address Phase Enable"]
    #[inline(always)]
    pub fn aape(&self) -> AAPE_R {
        AAPE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transition Phase Enable"]
    #[inline(always)]
    pub fn trape(&self) -> TRAPE_R {
        TRAPE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Parity Phase Enable"]
    #[inline(always)]
    pub fn parpe(&self) -> PARPE_R {
        PARPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ACK phase Enable"]
    #[inline(always)]
    pub fn ackpe(&self) -> ACKPE_R {
        ACKPE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Stalling Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn stlcyc(&mut self) -> STLCYC_W<0> {
        STLCYC_W::new(self)
    }
    #[doc = "Bit 28 - Assigned Address Phase Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aape(&mut self) -> AAPE_W<28> {
        AAPE_W::new(self)
    }
    #[doc = "Bit 29 - Transition Phase Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trape(&mut self) -> TRAPE_W<29> {
        TRAPE_W::new(self)
    }
    #[doc = "Bit 30 - Parity Phase Enable"]
    #[inline(always)]
    #[must_use]
    pub fn parpe(&mut self) -> PARPE_W<30> {
        PARPE_W::new(self)
    }
    #[doc = "Bit 31 - ACK phase Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackpe(&mut self) -> ACKPE_W<31> {
        ACKPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCL Stalling Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scstlctl](index.html) module"]
pub struct SCSTLCTL_SPEC;
impl crate::RegisterSpec for SCSTLCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scstlctl::R](R) reader structure"]
impl crate::Readable for SCSTLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scstlctl::W](W) writer structure"]
impl crate::Writable for SCSTLCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCSTLCTL to value 0"]
impl crate::Resettable for SCSTLCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
