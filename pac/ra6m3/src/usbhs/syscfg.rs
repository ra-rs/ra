#[doc = "Register `SYSCFG` reader"]
pub struct R(crate::R<SYSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG` writer"]
pub struct W(crate::W<SYSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_SPEC>;
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
impl From<crate::W<SYSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBE` reader - USB Operation Enable"]
pub type USBE_R = crate::BitReader<USBE_A>;
#[doc = "USB Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBE_A {
    #[doc = "0: USB operation is disabled."]
    _0 = 0,
    #[doc = "1: USB operation is enabled."]
    _1 = 1,
}
impl From<USBE_A> for bool {
    #[inline(always)]
    fn from(variant: USBE_A) -> Self {
        variant as u8 != 0
    }
}
impl USBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBE_A {
        match self.bits {
            false => USBE_A::_0,
            true => USBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBE_A::_1
    }
}
#[doc = "Field `USBE` writer - USB Operation Enable"]
pub type USBE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, USBE_A, O>;
impl<'a, const O: u8> USBE_W<'a, O> {
    #[doc = "USB operation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBE_A::_0)
    }
    #[doc = "USB operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBE_A::_1)
    }
}
#[doc = "Field `DPRPU` reader - D+ Line Resistor Control"]
pub type DPRPU_R = crate::BitReader<DPRPU_A>;
#[doc = "D+ Line Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPRPU_A {
    #[doc = "0: Pulling up the line is disabled."]
    _0 = 0,
    #[doc = "1: Pulling up the line is enabled."]
    _1 = 1,
}
impl From<DPRPU_A> for bool {
    #[inline(always)]
    fn from(variant: DPRPU_A) -> Self {
        variant as u8 != 0
    }
}
impl DPRPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPRPU_A {
        match self.bits {
            false => DPRPU_A::_0,
            true => DPRPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPRPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPRPU_A::_1
    }
}
#[doc = "Field `DPRPU` writer - D+ Line Resistor Control"]
pub type DPRPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, DPRPU_A, O>;
impl<'a, const O: u8> DPRPU_W<'a, O> {
    #[doc = "Pulling up the line is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPRPU_A::_0)
    }
    #[doc = "Pulling up the line is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPRPU_A::_1)
    }
}
#[doc = "Field `DRPD` reader - D+/D- Line Resistor Control"]
pub type DRPD_R = crate::BitReader<DRPD_A>;
#[doc = "D+/D- Line Resistor Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPD_A {
    #[doc = "0: Pulling down the line is disabled."]
    _0 = 0,
    #[doc = "1: Pulling down the line is enabled."]
    _1 = 1,
}
impl From<DRPD_A> for bool {
    #[inline(always)]
    fn from(variant: DRPD_A) -> Self {
        variant as u8 != 0
    }
}
impl DRPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRPD_A {
        match self.bits {
            false => DRPD_A::_0,
            true => DRPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRPD_A::_1
    }
}
#[doc = "Field `DRPD` writer - D+/D- Line Resistor Control"]
pub type DRPD_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, DRPD_A, O>;
impl<'a, const O: u8> DRPD_W<'a, O> {
    #[doc = "Pulling down the line is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRPD_A::_0)
    }
    #[doc = "Pulling down the line is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRPD_A::_1)
    }
}
#[doc = "Field `DCFM` reader - Controller Function Select"]
pub type DCFM_R = crate::BitReader<DCFM_A>;
#[doc = "Controller Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCFM_A {
    #[doc = "0: Function controller function is selected."]
    _0 = 0,
    #[doc = "1: Host controller function is selected."]
    _1 = 1,
}
impl From<DCFM_A> for bool {
    #[inline(always)]
    fn from(variant: DCFM_A) -> Self {
        variant as u8 != 0
    }
}
impl DCFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCFM_A {
        match self.bits {
            false => DCFM_A::_0,
            true => DCFM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCFM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCFM_A::_1
    }
}
#[doc = "Field `DCFM` writer - Controller Function Select"]
pub type DCFM_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, DCFM_A, O>;
impl<'a, const O: u8> DCFM_W<'a, O> {
    #[doc = "Function controller function is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCFM_A::_0)
    }
    #[doc = "Host controller function is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCFM_A::_1)
    }
}
#[doc = "Field `HSE` reader - High-Speed Operation Enable"]
pub type HSE_R = crate::BitReader<HSE_A>;
#[doc = "High-Speed Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSE_A {
    #[doc = "0: High-speed operation is disabled.(When the function controller function is selected: Full speed, When the host controller function is selected: Full/low speed)"]
    _0 = 0,
    #[doc = "1: High-speed operation is enabled (the controller detects the communication speed)."]
    _1 = 1,
}
impl From<HSE_A> for bool {
    #[inline(always)]
    fn from(variant: HSE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSE_A {
        match self.bits {
            false => HSE_A::_0,
            true => HSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSE_A::_1
    }
}
#[doc = "Field `HSE` writer - High-Speed Operation Enable"]
pub type HSE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, HSE_A, O>;
impl<'a, const O: u8> HSE_W<'a, O> {
    #[doc = "High-speed operation is disabled.(When the function controller function is selected: Full speed, When the host controller function is selected: Full/low speed)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSE_A::_0)
    }
    #[doc = "High-speed operation is enabled (the controller detects the communication speed)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSE_A::_1)
    }
}
#[doc = "Field `CNEN` reader - Single End Receiver Enable"]
pub type CNEN_R = crate::BitReader<CNEN_A>;
#[doc = "Single End Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNEN_A {
    #[doc = "0: Single end receiver operation is disabled."]
    _0 = 0,
    #[doc = "1: Single end receiver operation is enabled."]
    _1 = 1,
}
impl From<CNEN_A> for bool {
    #[inline(always)]
    fn from(variant: CNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNEN_A {
        match self.bits {
            false => CNEN_A::_0,
            true => CNEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNEN_A::_1
    }
}
#[doc = "Field `CNEN` writer - Single End Receiver Enable"]
pub type CNEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, CNEN_A, O>;
impl<'a, const O: u8> CNEN_W<'a, O> {
    #[doc = "Single end receiver operation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNEN_A::_0)
    }
    #[doc = "Single end receiver operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Operation Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - D+ Line Resistor Control"]
    #[inline(always)]
    pub fn dprpu(&self) -> DPRPU_R {
        DPRPU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D+/D- Line Resistor Control"]
    #[inline(always)]
    pub fn drpd(&self) -> DRPD_R {
        DRPD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Controller Function Select"]
    #[inline(always)]
    pub fn dcfm(&self) -> DCFM_R {
        DCFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - High-Speed Operation Enable"]
    #[inline(always)]
    pub fn hse(&self) -> HSE_R {
        HSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Single End Receiver Enable"]
    #[inline(always)]
    pub fn cnen(&self) -> CNEN_R {
        CNEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbe(&mut self) -> USBE_W<0> {
        USBE_W::new(self)
    }
    #[doc = "Bit 4 - D+ Line Resistor Control"]
    #[inline(always)]
    #[must_use]
    pub fn dprpu(&mut self) -> DPRPU_W<4> {
        DPRPU_W::new(self)
    }
    #[doc = "Bit 5 - D+/D- Line Resistor Control"]
    #[inline(always)]
    #[must_use]
    pub fn drpd(&mut self) -> DRPD_W<5> {
        DRPD_W::new(self)
    }
    #[doc = "Bit 6 - Controller Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn dcfm(&mut self) -> DCFM_W<6> {
        DCFM_W::new(self)
    }
    #[doc = "Bit 7 - High-Speed Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hse(&mut self) -> HSE_W<7> {
        HSE_W::new(self)
    }
    #[doc = "Bit 8 - Single End Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnen(&mut self) -> CNEN_W<8> {
        CNEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg](index.html) module"]
pub struct SYSCFG_SPEC;
impl crate::RegisterSpec for SYSCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg::R](R) reader structure"]
impl crate::Readable for SYSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg::W](W) writer structure"]
impl crate::Writable for SYSCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG to value 0x20"]
impl crate::Resettable for SYSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
