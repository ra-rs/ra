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
#[doc = "Field `ADNDIS` reader - The charging time"]
pub type ADNDIS_R = crate::FieldReader<u8, ADNDIS_A>;
#[doc = "The charging time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADNDIS_A {
    #[doc = "0: Disconnection detection is disabled"]
    _0000 = 0,
    #[doc = "1: Setting prohibited"]
    _0001 = 1,
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
            0 => Some(ADNDIS_A::_0000),
            1 => Some(ADNDIS_A::_0001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADNDIS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADNDIS_A::_0001
    }
}
#[doc = "Field `ADNDIS` writer - The charging time"]
pub type ADNDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADDISCR_SPEC, u8, ADNDIS_A, 4, O>;
impl<'a, const O: u8> ADNDIS_W<'a, O> {
    #[doc = "Disconnection detection is disabled"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADNDIS_A::_0000)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADNDIS_A::_0001)
    }
}
#[doc = "Field `CHARGE` reader - Selection of Precharge or Discharge"]
pub type CHARGE_R = crate::BitReader<CHARGE_A>;
#[doc = "Selection of Precharge or Discharge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHARGE_A {
    #[doc = "0: Discharge"]
    _0 = 0,
    #[doc = "1: Precharge"]
    _1 = 1,
}
impl From<CHARGE_A> for bool {
    #[inline(always)]
    fn from(variant: CHARGE_A) -> Self {
        variant as u8 != 0
    }
}
impl CHARGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHARGE_A {
        match self.bits {
            false => CHARGE_A::_0,
            true => CHARGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHARGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHARGE_A::_1
    }
}
#[doc = "Field `CHARGE` writer - Selection of Precharge or Discharge"]
pub type CHARGE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADDISCR_SPEC, CHARGE_A, O>;
impl<'a, const O: u8> CHARGE_W<'a, O> {
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHARGE_A::_0)
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHARGE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - The charging time"]
    #[inline(always)]
    pub fn adndis(&self) -> ADNDIS_R {
        ADNDIS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Selection of Precharge or Discharge"]
    #[inline(always)]
    pub fn charge(&self) -> CHARGE_R {
        CHARGE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The charging time"]
    #[inline(always)]
    #[must_use]
    pub fn adndis(&mut self) -> ADNDIS_W<0> {
        ADNDIS_W::new(self)
    }
    #[doc = "Bit 4 - Selection of Precharge or Discharge"]
    #[inline(always)]
    #[must_use]
    pub fn charge(&mut self) -> CHARGE_W<4> {
        CHARGE_W::new(self)
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
