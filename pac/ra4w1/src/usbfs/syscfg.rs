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
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBE_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBE_A::_1)
    }
}
#[doc = "Field `DMRPU` reader - D- Line Resistor Control"]
pub type DMRPU_R = crate::BitReader<DMRPU_A>;
#[doc = "D- Line Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMRPU_A {
    #[doc = "0: Line pull-up disabled"]
    _0 = 0,
    #[doc = "1: Line pull-up enabled."]
    _1 = 1,
}
impl From<DMRPU_A> for bool {
    #[inline(always)]
    fn from(variant: DMRPU_A) -> Self {
        variant as u8 != 0
    }
}
impl DMRPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMRPU_A {
        match self.bits {
            false => DMRPU_A::_0,
            true => DMRPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMRPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMRPU_A::_1
    }
}
#[doc = "Field `DMRPU` writer - D- Line Resistor Control"]
pub type DMRPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, DMRPU_A, O>;
impl<'a, const O: u8> DMRPU_W<'a, O> {
    #[doc = "Line pull-up disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMRPU_A::_0)
    }
    #[doc = "Line pull-up enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMRPU_A::_1)
    }
}
#[doc = "Field `DPRPU` reader - D+ Line Resistor Control"]
pub type DPRPU_R = crate::BitReader<DPRPU_A>;
#[doc = "D+ Line Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPRPU_A {
    #[doc = "0: Line pull-down disabled"]
    _0 = 0,
    #[doc = "1: Line pull-down enabled."]
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
    #[doc = "Line pull-down disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPRPU_A::_0)
    }
    #[doc = "Line pull-down enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPRPU_A::_1)
    }
}
#[doc = "Field `DRPD` reader - D+/D- Line Resistor Control"]
pub type DRPD_R = crate::BitReader<DRPD_A>;
#[doc = "D+/D- Line Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPD_A {
    #[doc = "0: Line pull-down disabled"]
    _0 = 0,
    #[doc = "1: Line pull-down enabled."]
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
    #[doc = "Line pull-down disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRPD_A::_0)
    }
    #[doc = "Line pull-down enabled."]
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
    #[doc = "0: Device controller selected"]
    _0 = 0,
    #[doc = "1: Host controller selected."]
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
    #[doc = "Device controller selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCFM_A::_0)
    }
    #[doc = "Host controller selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCFM_A::_1)
    }
}
#[doc = "Field `CNEN` reader - CNEN Single End Receiver Enable"]
pub type CNEN_R = crate::BitReader<CNEN_A>;
#[doc = "CNEN Single End Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNEN_A {
    #[doc = "0: Single end receiver disabled"]
    _0 = 0,
    #[doc = "1: Single end receiver enabled"]
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
#[doc = "Field `CNEN` writer - CNEN Single End Receiver Enable"]
pub type CNEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, CNEN_A, O>;
impl<'a, const O: u8> CNEN_W<'a, O> {
    #[doc = "Single end receiver disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNEN_A::_0)
    }
    #[doc = "Single end receiver enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNEN_A::_1)
    }
}
#[doc = "Field `SCKE` reader - USB Clock Enable"]
pub type SCKE_R = crate::BitReader<SCKE_A>;
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKE_A {
    #[doc = "0: Clock supply to the USBFS stopped"]
    _0 = 0,
    #[doc = "1: Clock supply to the USBFS enabled."]
    _1 = 1,
}
impl From<SCKE_A> for bool {
    #[inline(always)]
    fn from(variant: SCKE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKE_A {
        match self.bits {
            false => SCKE_A::_0,
            true => SCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKE_A::_1
    }
}
#[doc = "Field `SCKE` writer - USB Clock Enable"]
pub type SCKE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG_SPEC, SCKE_A, O>;
impl<'a, const O: u8> SCKE_W<'a, O> {
    #[doc = "Clock supply to the USBFS stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCKE_A::_0)
    }
    #[doc = "Clock supply to the USBFS enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCKE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Operation Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - D- Line Resistor Control"]
    #[inline(always)]
    pub fn dmrpu(&self) -> DMRPU_R {
        DMRPU_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 8 - CNEN Single End Receiver Enable"]
    #[inline(always)]
    pub fn cnen(&self) -> CNEN_R {
        CNEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - USB Clock Enable"]
    #[inline(always)]
    pub fn scke(&self) -> SCKE_R {
        SCKE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbe(&mut self) -> USBE_W<0> {
        USBE_W::new(self)
    }
    #[doc = "Bit 3 - D- Line Resistor Control"]
    #[inline(always)]
    #[must_use]
    pub fn dmrpu(&mut self) -> DMRPU_W<3> {
        DMRPU_W::new(self)
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
    #[doc = "Bit 8 - CNEN Single End Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnen(&mut self) -> CNEN_W<8> {
        CNEN_W::new(self)
    }
    #[doc = "Bit 10 - USB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scke(&mut self) -> SCKE_W<10> {
        SCKE_W::new(self)
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
#[doc = "`reset()` method sets SYSCFG to value 0"]
impl crate::Resettable for SYSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
