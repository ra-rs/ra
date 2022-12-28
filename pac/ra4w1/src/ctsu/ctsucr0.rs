#[doc = "Register `CTSUCR0` reader"]
pub struct R(crate::R<CTSUCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCR0` writer"]
pub struct W(crate::W<CTSUCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCR0_SPEC>;
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
impl From<crate::W<CTSUCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUSTRT` reader - CTSU Measurement Operation Start"]
pub type CTSUSTRT_R = crate::BitReader<CTSUSTRT_A>;
#[doc = "CTSU Measurement Operation Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSTRT_A {
    #[doc = "0: Measurement operation stops."]
    _0 = 0,
    #[doc = "1: Measurement operation starts."]
    _1 = 1,
}
impl From<CTSUSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSTRT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUSTRT_A {
        match self.bits {
            false => CTSUSTRT_A::_0,
            true => CTSUSTRT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSTRT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSTRT_A::_1
    }
}
#[doc = "Field `CTSUSTRT` writer - CTSU Measurement Operation Start"]
pub type CTSUSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR0_SPEC, CTSUSTRT_A, O>;
impl<'a, const O: u8> CTSUSTRT_W<'a, O> {
    #[doc = "Measurement operation stops."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUSTRT_A::_0)
    }
    #[doc = "Measurement operation starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUSTRT_A::_1)
    }
}
#[doc = "Field `CTSUCAP` reader - CTSU Measurement Operation Start Trigger Select"]
pub type CTSUCAP_R = crate::BitReader<CTSUCAP_A>;
#[doc = "CTSU Measurement Operation Start Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUCAP_A {
    #[doc = "0: Software trigger."]
    _0 = 0,
    #[doc = "1: External trigger."]
    _1 = 1,
}
impl From<CTSUCAP_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUCAP_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUCAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUCAP_A {
        match self.bits {
            false => CTSUCAP_A::_0,
            true => CTSUCAP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCAP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCAP_A::_1
    }
}
#[doc = "Field `CTSUCAP` writer - CTSU Measurement Operation Start Trigger Select"]
pub type CTSUCAP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR0_SPEC, CTSUCAP_A, O>;
impl<'a, const O: u8> CTSUCAP_W<'a, O> {
    #[doc = "Software trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUCAP_A::_0)
    }
    #[doc = "External trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUCAP_A::_1)
    }
}
#[doc = "Field `CTSUSNZ` reader - CTSU Wait State Power-Saving Enable"]
pub type CTSUSNZ_R = crate::BitReader<CTSUSNZ_A>;
#[doc = "CTSU Wait State Power-Saving Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSNZ_A {
    #[doc = "0: Power-saving function during wait state is disabled."]
    _0 = 0,
    #[doc = "1: Power-saving function during wait state is enabled."]
    _1 = 1,
}
impl From<CTSUSNZ_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSNZ_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUSNZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUSNZ_A {
        match self.bits {
            false => CTSUSNZ_A::_0,
            true => CTSUSNZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSNZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSNZ_A::_1
    }
}
#[doc = "Field `CTSUSNZ` writer - CTSU Wait State Power-Saving Enable"]
pub type CTSUSNZ_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR0_SPEC, CTSUSNZ_A, O>;
impl<'a, const O: u8> CTSUSNZ_W<'a, O> {
    #[doc = "Power-saving function during wait state is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUSNZ_A::_0)
    }
    #[doc = "Power-saving function during wait state is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUSNZ_A::_1)
    }
}
#[doc = "Field `CTSUIOC` reader - CTSU Transmit Pin Control"]
pub type CTSUIOC_R = crate::BitReader<CTSUIOC_A>;
#[doc = "CTSU Transmit Pin Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUIOC_A {
    #[doc = "0: Low-level output from transmit channel non-measurement pin."]
    _0 = 0,
    #[doc = "1: High-level output from transmit channel non-measurement pin."]
    _1 = 1,
}
impl From<CTSUIOC_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUIOC_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUIOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUIOC_A {
        match self.bits {
            false => CTSUIOC_A::_0,
            true => CTSUIOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUIOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUIOC_A::_1
    }
}
#[doc = "Field `CTSUIOC` writer - CTSU Transmit Pin Control"]
pub type CTSUIOC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR0_SPEC, CTSUIOC_A, O>;
impl<'a, const O: u8> CTSUIOC_W<'a, O> {
    #[doc = "Low-level output from transmit channel non-measurement pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUIOC_A::_0)
    }
    #[doc = "High-level output from transmit channel non-measurement pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUIOC_A::_1)
    }
}
#[doc = "Field `CTSUINIT` reader - CTSU Control Block Initialization"]
pub type CTSUINIT_R = crate::BitReader<CTSUINIT_A>;
#[doc = "CTSU Control Block Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUINIT_A {
    #[doc = "0: Writing a 0 has no effect, this bit is read as 0."]
    _0 = 0,
    #[doc = "1: initializes the CTSU control block and registers."]
    _1 = 1,
}
impl From<CTSUINIT_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUINIT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUINIT_A {
        match self.bits {
            false => CTSUINIT_A::_0,
            true => CTSUINIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUINIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUINIT_A::_1
    }
}
#[doc = "Field `CTSUINIT` writer - CTSU Control Block Initialization"]
pub type CTSUINIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUCR0_SPEC, CTSUINIT_A, O>;
impl<'a, const O: u8> CTSUINIT_W<'a, O> {
    #[doc = "Writing a 0 has no effect, this bit is read as 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUINIT_A::_0)
    }
    #[doc = "initializes the CTSU control block and registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUINIT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    pub fn ctsustrt(&self) -> CTSUSTRT_R {
        CTSUSTRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    pub fn ctsucap(&self) -> CTSUCAP_R {
        CTSUCAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    pub fn ctsusnz(&self) -> CTSUSNZ_R {
        CTSUSNZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTSU Transmit Pin Control"]
    #[inline(always)]
    pub fn ctsuioc(&self) -> CTSUIOC_R {
        CTSUIOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CTSU Control Block Initialization"]
    #[inline(always)]
    pub fn ctsuinit(&self) -> CTSUINIT_R {
        CTSUINIT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    #[must_use]
    pub fn ctsustrt(&mut self) -> CTSUSTRT_W<0> {
        CTSUSTRT_W::new(self)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn ctsucap(&mut self) -> CTSUCAP_W<1> {
        CTSUCAP_W::new(self)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsusnz(&mut self) -> CTSUSNZ_W<2> {
        CTSUSNZ_W::new(self)
    }
    #[doc = "Bit 3 - CTSU Transmit Pin Control"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuioc(&mut self) -> CTSUIOC_W<3> {
        CTSUIOC_W::new(self)
    }
    #[doc = "Bit 4 - CTSU Control Block Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuinit(&mut self) -> CTSUINIT_W<4> {
        CTSUINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsucr0](index.html) module"]
pub struct CTSUCR0_SPEC;
impl crate::RegisterSpec for CTSUCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsucr0::R](R) reader structure"]
impl crate::Readable for CTSUCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsucr0::W](W) writer structure"]
impl crate::Writable for CTSUCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCR0 to value 0"]
impl crate::Resettable for CTSUCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
