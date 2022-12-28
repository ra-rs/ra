#[doc = "Register `GTSECR` reader"]
pub struct R(crate::R<GTSECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTSECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTSECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTSECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTSECR` writer"]
pub struct W(crate::W<GTSECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTSECR_SPEC>;
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
impl From<crate::W<GTSECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTSECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBDCE` reader - GTCCR Register Buffer Operation Simultaneous Enable"]
pub type SBDCE_R = crate::BitReader<SBDCE_A>;
#[doc = "GTCCR Register Buffer Operation Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDCE_A {
    #[doc = "0: Disable simultaneous enabling GTCCR buffer operations"]
    _0 = 0,
    #[doc = "1: Enable GTCCR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDCE_A> for bool {
    #[inline(always)]
    fn from(variant: SBDCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDCE_A {
        match self.bits {
            false => SBDCE_A::_0,
            true => SBDCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDCE_A::_1
    }
}
#[doc = "Field `SBDCE` writer - GTCCR Register Buffer Operation Simultaneous Enable"]
pub type SBDCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDCE_A, O>;
impl<'a, const O: u8> SBDCE_W<'a, O> {
    #[doc = "Disable simultaneous enabling GTCCR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDCE_A::_0)
    }
    #[doc = "Enable GTCCR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDCE_A::_1)
    }
}
#[doc = "Field `SBDPE` reader - GTPR Register Buffer Operation Simultaneous Enable"]
pub type SBDPE_R = crate::BitReader<SBDPE_A>;
#[doc = "GTPR Register Buffer Operation Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDPE_A {
    #[doc = "0: Disable simultaneous enabling GTPR buffer operations"]
    _0 = 0,
    #[doc = "1: Enable GTPR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDPE_A> for bool {
    #[inline(always)]
    fn from(variant: SBDPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDPE_A {
        match self.bits {
            false => SBDPE_A::_0,
            true => SBDPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDPE_A::_1
    }
}
#[doc = "Field `SBDPE` writer - GTPR Register Buffer Operation Simultaneous Enable"]
pub type SBDPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDPE_A, O>;
impl<'a, const O: u8> SBDPE_W<'a, O> {
    #[doc = "Disable simultaneous enabling GTPR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDPE_A::_0)
    }
    #[doc = "Enable GTPR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDPE_A::_1)
    }
}
#[doc = "Field `SBDCD` reader - GTCCR Register Buffer Operation Simultaneous Disable"]
pub type SBDCD_R = crate::BitReader<SBDCD_A>;
#[doc = "GTCCR Register Buffer Operation Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDCD_A {
    #[doc = "0: Disable simultaneous disabling GTCCR buffer operations"]
    _0 = 0,
    #[doc = "1: Disable GTCCR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDCD_A> for bool {
    #[inline(always)]
    fn from(variant: SBDCD_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDCD_A {
        match self.bits {
            false => SBDCD_A::_0,
            true => SBDCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDCD_A::_1
    }
}
#[doc = "Field `SBDCD` writer - GTCCR Register Buffer Operation Simultaneous Disable"]
pub type SBDCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDCD_A, O>;
impl<'a, const O: u8> SBDCD_W<'a, O> {
    #[doc = "Disable simultaneous disabling GTCCR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDCD_A::_0)
    }
    #[doc = "Disable GTCCR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDCD_A::_1)
    }
}
#[doc = "Field `SBDPD` reader - GTPR Register Buffer Operation Simultaneous Disable"]
pub type SBDPD_R = crate::BitReader<SBDPD_A>;
#[doc = "GTPR Register Buffer Operation Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDPD_A {
    #[doc = "0: Disable simultaneous disabling GTPR buffer operations"]
    _0 = 0,
    #[doc = "1: Disable GTPR register buffer operations simultaneously"]
    _1 = 1,
}
impl From<SBDPD_A> for bool {
    #[inline(always)]
    fn from(variant: SBDPD_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDPD_A {
        match self.bits {
            false => SBDPD_A::_0,
            true => SBDPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDPD_A::_1
    }
}
#[doc = "Field `SBDPD` writer - GTPR Register Buffer Operation Simultaneous Disable"]
pub type SBDPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SBDPD_A, O>;
impl<'a, const O: u8> SBDPD_W<'a, O> {
    #[doc = "Disable simultaneous disabling GTPR buffer operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDPD_A::_0)
    }
    #[doc = "Disable GTPR register buffer operations simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDPD_A::_1)
    }
}
#[doc = "Field `SPCE` reader - Period Count Function Simultaneous Enable"]
pub type SPCE_R = crate::BitReader<SPCE_A>;
#[doc = "Period Count Function Simultaneous Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCE_A {
    #[doc = "0: Disable simultaneous enabling period count function"]
    _0 = 0,
    #[doc = "1: Enable period count function simultaneously"]
    _1 = 1,
}
impl From<SPCE_A> for bool {
    #[inline(always)]
    fn from(variant: SPCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCE_A {
        match self.bits {
            false => SPCE_A::_0,
            true => SPCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCE_A::_1
    }
}
#[doc = "Field `SPCE` writer - Period Count Function Simultaneous Enable"]
pub type SPCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SPCE_A, O>;
impl<'a, const O: u8> SPCE_W<'a, O> {
    #[doc = "Disable simultaneous enabling period count function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCE_A::_0)
    }
    #[doc = "Enable period count function simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCE_A::_1)
    }
}
#[doc = "Field `SPCD` reader - Period Count Function Simultaneous Disable"]
pub type SPCD_R = crate::BitReader<SPCD_A>;
#[doc = "Period Count Function Simultaneous Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCD_A {
    #[doc = "0: Disable simultaneous disabling period count function"]
    _0 = 0,
    #[doc = "1: Disable period count function simultaneously"]
    _1 = 1,
}
impl From<SPCD_A> for bool {
    #[inline(always)]
    fn from(variant: SPCD_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCD_A {
        match self.bits {
            false => SPCD_A::_0,
            true => SPCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCD_A::_1
    }
}
#[doc = "Field `SPCD` writer - Period Count Function Simultaneous Disable"]
pub type SPCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTSECR_SPEC, SPCD_A, O>;
impl<'a, const O: u8> SPCD_W<'a, O> {
    #[doc = "Disable simultaneous disabling period count function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCD_A::_0)
    }
    #[doc = "Disable period count function simultaneously"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTCCR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdce(&self) -> SBDCE_R {
        SBDCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTPR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdpe(&self) -> SBDPE_R {
        SBDPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - GTCCR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdcd(&self) -> SBDCD_R {
        SBDCD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTPR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdpd(&self) -> SBDPD_R {
        SBDPD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Period Count Function Simultaneous Enable"]
    #[inline(always)]
    pub fn spce(&self) -> SPCE_R {
        SPCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Period Count Function Simultaneous Disable"]
    #[inline(always)]
    pub fn spcd(&self) -> SPCD_R {
        SPCD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTCCR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdce(&mut self) -> SBDCE_W<0> {
        SBDCE_W::new(self)
    }
    #[doc = "Bit 1 - GTPR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdpe(&mut self) -> SBDPE_W<1> {
        SBDPE_W::new(self)
    }
    #[doc = "Bit 8 - GTCCR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdcd(&mut self) -> SBDCD_W<8> {
        SBDCD_W::new(self)
    }
    #[doc = "Bit 9 - GTPR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdpd(&mut self) -> SBDPD_W<9> {
        SBDPD_W::new(self)
    }
    #[doc = "Bit 16 - Period Count Function Simultaneous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spce(&mut self) -> SPCE_W<16> {
        SPCE_W::new(self)
    }
    #[doc = "Bit 24 - Period Count Function Simultaneous Disable"]
    #[inline(always)]
    #[must_use]
    pub fn spcd(&mut self) -> SPCD_W<24> {
        SPCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Operation Enable Bit Simultaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtsecr](index.html) module"]
pub struct GTSECR_SPEC;
impl crate::RegisterSpec for GTSECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtsecr::R](R) reader structure"]
impl crate::Readable for GTSECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtsecr::W](W) writer structure"]
impl crate::Writable for GTSECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSECR to value 0"]
impl crate::Resettable for GTSECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
