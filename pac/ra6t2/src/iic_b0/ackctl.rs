#[doc = "Register `ACKCTL` reader"]
pub struct R(crate::R<ACKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACKCTL` writer"]
pub struct W(crate::W<ACKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACKCTL_SPEC>;
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
impl From<crate::W<ACKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKR` reader - Acknowledge Reception"]
pub type ACKR_R = crate::BitReader<ACKR_A>;
#[doc = "Acknowledge Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKR_A {
    #[doc = "0: A 0 is received as the acknowledge bit (ACK reception)."]
    _0 = 0,
    #[doc = "1: A 1 is received as the acknowledge bit (NACK reception)."]
    _1 = 1,
}
impl From<ACKR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKR_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKR_A {
        match self.bits {
            false => ACKR_A::_0,
            true => ACKR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKR_A::_1
    }
}
#[doc = "Field `ACKT` reader - Acknowledge Transmission"]
pub type ACKT_R = crate::BitReader<ACKT_A>;
#[doc = "Acknowledge Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKT_A {
    #[doc = "0: A 0 is sent as the acknowledge bit (ACK transmission)."]
    _0 = 0,
    #[doc = "1: A 1 is sent as the acknowledge bit (NACK transmission)."]
    _1 = 1,
}
impl From<ACKT_A> for bool {
    #[inline(always)]
    fn from(variant: ACKT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKT_A {
        match self.bits {
            false => ACKT_A::_0,
            true => ACKT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKT_A::_1
    }
}
#[doc = "Field `ACKT` writer - Acknowledge Transmission"]
pub type ACKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACKCTL_SPEC, ACKT_A, O>;
impl<'a, const O: u8> ACKT_W<'a, O> {
    #[doc = "A 0 is sent as the acknowledge bit (ACK transmission)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKT_A::_0)
    }
    #[doc = "A 1 is sent as the acknowledge bit (NACK transmission)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKT_A::_1)
    }
}
#[doc = "ACKT Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKTWP_AW {
    #[doc = "0: The ACKT bit are protected."]
    _0 = 0,
    #[doc = "1: The ACKT bit can be written (when writing simultaneously with the value of the target bit). This bit is read as 0."]
    _1 = 1,
}
impl From<ACKTWP_AW> for bool {
    #[inline(always)]
    fn from(variant: ACKTWP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKTWP` writer - ACKT Write Protect"]
pub type ACKTWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACKCTL_SPEC, ACKTWP_AW, O>;
impl<'a, const O: u8> ACKTWP_W<'a, O> {
    #[doc = "The ACKT bit are protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKTWP_AW::_0)
    }
    #[doc = "The ACKT bit can be written (when writing simultaneously with the value of the target bit). This bit is read as 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKTWP_AW::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge Reception"]
    #[inline(always)]
    pub fn ackr(&self) -> ACKR_R {
        ACKR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Transmission"]
    #[inline(always)]
    pub fn ackt(&self) -> ACKT_R {
        ACKT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Acknowledge Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ackt(&mut self) -> ACKT_W<1> {
        ACKT_W::new(self)
    }
    #[doc = "Bit 2 - ACKT Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn acktwp(&mut self) -> ACKTWP_W<2> {
        ACKTWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Acknowledge Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ackctl](index.html) module"]
pub struct ACKCTL_SPEC;
impl crate::RegisterSpec for ACKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ackctl::R](R) reader structure"]
impl crate::Readable for ACKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ackctl::W](W) writer structure"]
impl crate::Writable for ACKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACKCTL to value 0"]
impl crate::Resettable for ACKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
