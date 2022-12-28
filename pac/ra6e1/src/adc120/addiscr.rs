#[doc = "Register `ADDISCR` reader"]
pub struct R(crate::R<ADDISCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDISCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDISCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDISCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDISCR` writer"]
pub struct W(crate::W<ADDISCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDISCR_SPEC>;
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
impl From<crate::W<ADDISCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDISCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADNDIS` reader - Disconnection Detection Assist Setting"]
pub type ADNDIS_R = crate::FieldReader<u8, ADNDIS_A>;
#[doc = "Disconnection Detection Assist Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADNDIS_A {
    #[doc = "0: The disconnection detection assist function is disabled"]
    _0X0 = 0,
    #[doc = "1: Setting prohibited"]
    _0X1 = 1,
}
impl From<ADNDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADNDIS_A) -> Self {
        variant as _
    }
}
impl ADNDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADNDIS_A> {
        match self.bits {
            0 => Some(ADNDIS_A::_0X0),
            1 => Some(ADNDIS_A::_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == ADNDIS_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == ADNDIS_A::_0X1
    }
}
#[doc = "Field `ADNDIS` writer - Disconnection Detection Assist Setting"]
pub type ADNDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADDISCR_SPEC, u8, ADNDIS_A, 4, O>;
impl<'a, const O: u8> ADNDIS_W<'a, O> {
    #[doc = "The disconnection detection assist function is disabled"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(ADNDIS_A::_0X0)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(ADNDIS_A::_0X1)
    }
}
#[doc = "Field `PCHG` reader - Precharge/discharge select"]
pub type PCHG_R = crate::BitReader<PCHG_A>;
#[doc = "Precharge/discharge select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCHG_A {
    #[doc = "0: Discharge"]
    _0 = 0,
    #[doc = "1: Precharge"]
    _1 = 1,
}
impl From<PCHG_A> for bool {
    #[inline(always)]
    fn from(variant: PCHG_A) -> Self {
        variant as u8 != 0
    }
}
impl PCHG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCHG_A {
        match self.bits {
            false => PCHG_A::_0,
            true => PCHG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCHG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCHG_A::_1
    }
}
#[doc = "Field `PCHG` writer - Precharge/discharge select"]
pub type PCHG_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADDISCR_SPEC, PCHG_A, O>;
impl<'a, const O: u8> PCHG_W<'a, O> {
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCHG_A::_0)
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCHG_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Disconnection Detection Assist Setting"]
    #[inline(always)]
    pub fn adndis(&self) -> ADNDIS_R {
        ADNDIS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Precharge/discharge select"]
    #[inline(always)]
    pub fn pchg(&self) -> PCHG_R {
        PCHG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Disconnection Detection Assist Setting"]
    #[inline(always)]
    #[must_use]
    pub fn adndis(&mut self) -> ADNDIS_W<0> {
        ADNDIS_W::new(self)
    }
    #[doc = "Bit 4 - Precharge/discharge select"]
    #[inline(always)]
    #[must_use]
    pub fn pchg(&mut self) -> PCHG_W<4> {
        PCHG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Disconnection Detection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addiscr](index.html) module"]
pub struct ADDISCR_SPEC;
impl crate::RegisterSpec for ADDISCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [addiscr::R](R) reader structure"]
impl crate::Readable for ADDISCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addiscr::W](W) writer structure"]
impl crate::Writable for ADDISCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDISCR to value 0"]
impl crate::Resettable for ADDISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
