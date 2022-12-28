#[doc = "Register `AMPCPC` reader"]
pub struct R(crate::R<AMPCPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCPC` writer"]
pub struct W(crate::W<AMPCPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCPC_SPEC>;
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
impl From<crate::W<AMPCPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUMP0EN` reader - Charge Pump for AMP0 Enable"]
pub type PUMP0EN_R = crate::BitReader<PUMP0EN_A>;
#[doc = "Charge Pump for AMP0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUMP0EN_A {
    #[doc = "0: Charge Pump for the AMP0 disabled"]
    _0 = 0,
    #[doc = "1: Charge Pump for the AMP0 enable"]
    _1 = 1,
}
impl From<PUMP0EN_A> for bool {
    #[inline(always)]
    fn from(variant: PUMP0EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PUMP0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUMP0EN_A {
        match self.bits {
            false => PUMP0EN_A::_0,
            true => PUMP0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PUMP0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PUMP0EN_A::_1
    }
}
#[doc = "Field `PUMP0EN` writer - Charge Pump for AMP0 Enable"]
pub type PUMP0EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPCPC_SPEC, PUMP0EN_A, O>;
impl<'a, const O: u8> PUMP0EN_W<'a, O> {
    #[doc = "Charge Pump for the AMP0 disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUMP0EN_A::_0)
    }
    #[doc = "Charge Pump for the AMP0 enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUMP0EN_A::_1)
    }
}
#[doc = "Field `PUMP1EN` reader - Charge Pump for AMP1 Enable"]
pub type PUMP1EN_R = crate::BitReader<PUMP1EN_A>;
#[doc = "Charge Pump for AMP1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUMP1EN_A {
    #[doc = "0: Charge Pump for the AMP1 disabled"]
    _0 = 0,
    #[doc = "1: Charge Pump for the AMP1 enable"]
    _1 = 1,
}
impl From<PUMP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: PUMP1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PUMP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUMP1EN_A {
        match self.bits {
            false => PUMP1EN_A::_0,
            true => PUMP1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PUMP1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PUMP1EN_A::_1
    }
}
#[doc = "Field `PUMP1EN` writer - Charge Pump for AMP1 Enable"]
pub type PUMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPCPC_SPEC, PUMP1EN_A, O>;
impl<'a, const O: u8> PUMP1EN_W<'a, O> {
    #[doc = "Charge Pump for the AMP1 disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUMP1EN_A::_0)
    }
    #[doc = "Charge Pump for the AMP1 enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUMP1EN_A::_1)
    }
}
#[doc = "Field `PUMP2EN` reader - Charge Pump for AMP2 Enable"]
pub type PUMP2EN_R = crate::BitReader<PUMP2EN_A>;
#[doc = "Charge Pump for AMP2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUMP2EN_A {
    #[doc = "0: Charge Pump for the AMP2 disabled"]
    _0 = 0,
    #[doc = "1: Charge Pump for the AMP2 enable"]
    _1 = 1,
}
impl From<PUMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: PUMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PUMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUMP2EN_A {
        match self.bits {
            false => PUMP2EN_A::_0,
            true => PUMP2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PUMP2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PUMP2EN_A::_1
    }
}
#[doc = "Field `PUMP2EN` writer - Charge Pump for AMP2 Enable"]
pub type PUMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPCPC_SPEC, PUMP2EN_A, O>;
impl<'a, const O: u8> PUMP2EN_W<'a, O> {
    #[doc = "Charge Pump for the AMP2 disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUMP2EN_A::_0)
    }
    #[doc = "Charge Pump for the AMP2 enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUMP2EN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Charge Pump for AMP0 Enable"]
    #[inline(always)]
    pub fn pump0en(&self) -> PUMP0EN_R {
        PUMP0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Charge Pump for AMP1 Enable"]
    #[inline(always)]
    pub fn pump1en(&self) -> PUMP1EN_R {
        PUMP1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Charge Pump for AMP2 Enable"]
    #[inline(always)]
    pub fn pump2en(&self) -> PUMP2EN_R {
        PUMP2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Charge Pump for AMP0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pump0en(&mut self) -> PUMP0EN_W<0> {
        PUMP0EN_W::new(self)
    }
    #[doc = "Bit 1 - Charge Pump for AMP1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pump1en(&mut self) -> PUMP1EN_W<1> {
        PUMP1EN_W::new(self)
    }
    #[doc = "Bit 2 - Charge Pump for AMP2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pump2en(&mut self) -> PUMP2EN_W<2> {
        PUMP2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Switch Charge Pump Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcpc](index.html) module"]
pub struct AMPCPC_SPEC;
impl crate::RegisterSpec for AMPCPC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ampcpc::R](R) reader structure"]
impl crate::Readable for AMPCPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcpc::W](W) writer structure"]
impl crate::Writable for AMPCPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPCPC to value 0"]
impl crate::Resettable for AMPCPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
