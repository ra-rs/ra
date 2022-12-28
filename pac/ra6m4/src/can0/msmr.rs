#[doc = "Register `MSMR` reader"]
pub struct R(crate::R<MSMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSMR` writer"]
pub struct W(crate::W<MSMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSMR_SPEC>;
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
impl From<crate::W<MSMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MBSM` reader - Mailbox Search Mode Select"]
pub type MBSM_R = crate::FieldReader<u8, MBSM_A>;
#[doc = "Mailbox Search Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBSM_A {
    #[doc = "0: Receive mailbox search mode"]
    _00 = 0,
    #[doc = "1: Transmit mailbox search mode"]
    _01 = 1,
    #[doc = "2: Message lost search mode"]
    _10 = 2,
    #[doc = "3: Channel search mode"]
    _11 = 3,
}
impl From<MBSM_A> for u8 {
    #[inline(always)]
    fn from(variant: MBSM_A) -> Self {
        variant as _
    }
}
impl MBSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBSM_A {
        match self.bits {
            0 => MBSM_A::_00,
            1 => MBSM_A::_01,
            2 => MBSM_A::_10,
            3 => MBSM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MBSM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MBSM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MBSM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MBSM_A::_11
    }
}
#[doc = "Field `MBSM` writer - Mailbox Search Mode Select"]
pub type MBSM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MSMR_SPEC, u8, MBSM_A, 2, O>;
impl<'a, const O: u8> MBSM_W<'a, O> {
    #[doc = "Receive mailbox search mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MBSM_A::_00)
    }
    #[doc = "Transmit mailbox search mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MBSM_A::_01)
    }
    #[doc = "Message lost search mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MBSM_A::_10)
    }
    #[doc = "Channel search mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MBSM_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Mailbox Search Mode Select"]
    #[inline(always)]
    pub fn mbsm(&self) -> MBSM_R {
        MBSM_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mailbox Search Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mbsm(&mut self) -> MBSM_W<0> {
        MBSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Search Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msmr](index.html) module"]
pub struct MSMR_SPEC;
impl crate::RegisterSpec for MSMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [msmr::R](R) reader structure"]
impl crate::Readable for MSMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msmr::W](W) writer structure"]
impl crate::Writable for MSMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSMR to value 0"]
impl crate::Resettable for MSMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
