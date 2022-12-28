#[doc = "Register `ADC1` reader"]
pub struct R(crate::R<ADC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1` writer"]
pub struct W(crate::W<ADC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1_SPEC>;
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
impl From<crate::W<ADC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDADSCM` reader - Selection of autoscan mode"]
pub type SDADSCM_R = crate::BitReader<SDADSCM_A>;
#[doc = "Selection of autoscan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADSCM_A {
    #[doc = "0: Continuous scan mode"]
    _0 = 0,
    #[doc = "1: Single scan mode"]
    _1 = 1,
}
impl From<SDADSCM_A> for bool {
    #[inline(always)]
    fn from(variant: SDADSCM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADSCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADSCM_A {
        match self.bits {
            false => SDADSCM_A::_0,
            true => SDADSCM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADSCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADSCM_A::_1
    }
}
#[doc = "Field `SDADSCM` writer - Selection of autoscan mode"]
pub type SDADSCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_SPEC, SDADSCM_A, O>;
impl<'a, const O: u8> SDADSCM_W<'a, O> {
    #[doc = "Continuous scan mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDADSCM_A::_0)
    }
    #[doc = "Single scan mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDADSCM_A::_1)
    }
}
#[doc = "Field `SDADTMD` reader - Selection of A/D conversion trigger signal"]
pub type SDADTMD_R = crate::BitReader<SDADTMD_A>;
#[doc = "Selection of A/D conversion trigger signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADTMD_A {
    #[doc = "0: Software trigger (conversion is started by a write to SFR)"]
    _0 = 0,
    #[doc = "1: Hardware trigger (conversion is started in synchronization with the event signal selected by ELC_SDADC24)."]
    _1 = 1,
}
impl From<SDADTMD_A> for bool {
    #[inline(always)]
    fn from(variant: SDADTMD_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADTMD_A {
        match self.bits {
            false => SDADTMD_A::_0,
            true => SDADTMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDADTMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDADTMD_A::_1
    }
}
#[doc = "Field `SDADTMD` writer - Selection of A/D conversion trigger signal"]
pub type SDADTMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_SPEC, SDADTMD_A, O>;
impl<'a, const O: u8> SDADTMD_W<'a, O> {
    #[doc = "Software trigger (conversion is started by a write to SFR)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDADTMD_A::_0)
    }
    #[doc = "Hardware trigger (conversion is started in synchronization with the event signal selected by ELC_SDADC24)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDADTMD_A::_1)
    }
}
#[doc = "Field `SDADBMP` reader - A/D conversion control of the signal from input multiplexer"]
pub type SDADBMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDADBMP` writer - A/D conversion control of the signal from input multiplexer"]
pub type SDADBMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC1_SPEC, u8, u8, 5, O>;
#[doc = "Field `PGADISA` reader - Control of disconnection detection"]
pub type PGADISA_R = crate::BitReader<PGADISA_A>;
#[doc = "Control of disconnection detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGADISA_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: State of disconnection detection"]
    _1 = 1,
}
impl From<PGADISA_A> for bool {
    #[inline(always)]
    fn from(variant: PGADISA_A) -> Self {
        variant as u8 != 0
    }
}
impl PGADISA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGADISA_A {
        match self.bits {
            false => PGADISA_A::_0,
            true => PGADISA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGADISA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGADISA_A::_1
    }
}
#[doc = "Field `PGADISA` writer - Control of disconnection detection"]
pub type PGADISA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_SPEC, PGADISA_A, O>;
impl<'a, const O: u8> PGADISA_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGADISA_A::_0)
    }
    #[doc = "State of disconnection detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGADISA_A::_1)
    }
}
#[doc = "Field `PGADISC` reader - Disconnection Detection Assist Setting"]
pub type PGADISC_R = crate::BitReader<PGADISC_A>;
#[doc = "Disconnection Detection Assist Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGADISC_A {
    #[doc = "0: Discharge"]
    _0 = 0,
    #[doc = "1: Pre-charge"]
    _1 = 1,
}
impl From<PGADISC_A> for bool {
    #[inline(always)]
    fn from(variant: PGADISC_A) -> Self {
        variant as u8 != 0
    }
}
impl PGADISC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGADISC_A {
        match self.bits {
            false => PGADISC_A::_0,
            true => PGADISC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGADISC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGADISC_A::_1
    }
}
#[doc = "Field `PGADISC` writer - Disconnection Detection Assist Setting"]
pub type PGADISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_SPEC, PGADISC_A, O>;
impl<'a, const O: u8> PGADISC_W<'a, O> {
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGADISC_A::_0)
    }
    #[doc = "Pre-charge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGADISC_A::_1)
    }
}
#[doc = "Field `PGASLFT` reader - PGA offset self-diagnosis enable"]
pub type PGASLFT_R = crate::BitReader<PGASLFT_A>;
#[doc = "PGA offset self-diagnosis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGASLFT_A {
    #[doc = "0: Disable PGA offset self-diagnosis"]
    _0 = 0,
    #[doc = "1: Enable PGA offset self-diagnosis"]
    _1 = 1,
}
impl From<PGASLFT_A> for bool {
    #[inline(always)]
    fn from(variant: PGASLFT_A) -> Self {
        variant as u8 != 0
    }
}
impl PGASLFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGASLFT_A {
        match self.bits {
            false => PGASLFT_A::_0,
            true => PGASLFT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGASLFT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGASLFT_A::_1
    }
}
#[doc = "Field `PGASLFT` writer - PGA offset self-diagnosis enable"]
pub type PGASLFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_SPEC, PGASLFT_A, O>;
impl<'a, const O: u8> PGASLFT_W<'a, O> {
    #[doc = "Disable PGA offset self-diagnosis"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGASLFT_A::_0)
    }
    #[doc = "Enable PGA offset self-diagnosis"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGASLFT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selection of autoscan mode"]
    #[inline(always)]
    pub fn sdadscm(&self) -> SDADSCM_R {
        SDADSCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Selection of A/D conversion trigger signal"]
    #[inline(always)]
    pub fn sdadtmd(&self) -> SDADTMD_R {
        SDADTMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - A/D conversion control of the signal from input multiplexer"]
    #[inline(always)]
    pub fn sdadbmp(&self) -> SDADBMP_R {
        SDADBMP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Control of disconnection detection"]
    #[inline(always)]
    pub fn pgadisa(&self) -> PGADISA_R {
        PGADISA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disconnection Detection Assist Setting"]
    #[inline(always)]
    pub fn pgadisc(&self) -> PGADISC_R {
        PGADISC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - PGA offset self-diagnosis enable"]
    #[inline(always)]
    pub fn pgaslft(&self) -> PGASLFT_R {
        PGASLFT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of autoscan mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdadscm(&mut self) -> SDADSCM_W<0> {
        SDADSCM_W::new(self)
    }
    #[doc = "Bit 4 - Selection of A/D conversion trigger signal"]
    #[inline(always)]
    #[must_use]
    pub fn sdadtmd(&mut self) -> SDADTMD_W<4> {
        SDADTMD_W::new(self)
    }
    #[doc = "Bits 8:12 - A/D conversion control of the signal from input multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn sdadbmp(&mut self) -> SDADBMP_W<8> {
        SDADBMP_W::new(self)
    }
    #[doc = "Bit 16 - Control of disconnection detection"]
    #[inline(always)]
    #[must_use]
    pub fn pgadisa(&mut self) -> PGADISA_W<16> {
        PGADISA_W::new(self)
    }
    #[doc = "Bit 17 - Disconnection Detection Assist Setting"]
    #[inline(always)]
    #[must_use]
    pub fn pgadisc(&mut self) -> PGADISC_W<17> {
        PGADISC_W::new(self)
    }
    #[doc = "Bit 20 - PGA offset self-diagnosis enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgaslft(&mut self) -> PGASLFT_W<20> {
        PGASLFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sigma-delta A/D Converter Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1](index.html) module"]
pub struct ADC1_SPEC;
impl crate::RegisterSpec for ADC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1::R](R) reader structure"]
impl crate::Readable for ADC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1::W](W) writer structure"]
impl crate::Writable for ADC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1 to value 0"]
impl crate::Resettable for ADC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
