#[doc = "Register `AGTCMSR` reader"]
pub struct R(crate::R<AGTCMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTCMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTCMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTCMSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTCMSR` writer"]
pub struct W(crate::W<AGTCMSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTCMSR_SPEC>;
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
impl From<crate::W<AGTCMSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTCMSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCMEA` reader - AGT Compare Match A Register Enable"]
pub type TCMEA_R = crate::BitReader<TCMEA_A>;
#[doc = "AGT Compare Match A Register Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMEA_A {
    #[doc = "0: AGT Compare match A register disabled"]
    _0 = 0,
    #[doc = "1: AGT Compare match A register enabled"]
    _1 = 1,
}
impl From<TCMEA_A> for bool {
    #[inline(always)]
    fn from(variant: TCMEA_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCMEA_A {
        match self.bits {
            false => TCMEA_A::_0,
            true => TCMEA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMEA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMEA_A::_1
    }
}
#[doc = "Field `TCMEA` writer - AGT Compare Match A Register Enable"]
pub type TCMEA_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCMSR_SPEC, TCMEA_A, O>;
impl<'a, const O: u8> TCMEA_W<'a, O> {
    #[doc = "AGT Compare match A register disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCMEA_A::_0)
    }
    #[doc = "AGT Compare match A register enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCMEA_A::_1)
    }
}
#[doc = "Field `TOEA` reader - AGTOAn Pin Output Enable"]
pub type TOEA_R = crate::BitReader<TOEA_A>;
#[doc = "AGTOAn Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOEA_A {
    #[doc = "0: AGTOAn pin output disabled"]
    _0 = 0,
    #[doc = "1: AGTOAn pin output enabled"]
    _1 = 1,
}
impl From<TOEA_A> for bool {
    #[inline(always)]
    fn from(variant: TOEA_A) -> Self {
        variant as u8 != 0
    }
}
impl TOEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOEA_A {
        match self.bits {
            false => TOEA_A::_0,
            true => TOEA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOEA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOEA_A::_1
    }
}
#[doc = "Field `TOEA` writer - AGTOAn Pin Output Enable"]
pub type TOEA_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCMSR_SPEC, TOEA_A, O>;
impl<'a, const O: u8> TOEA_W<'a, O> {
    #[doc = "AGTOAn pin output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOEA_A::_0)
    }
    #[doc = "AGTOAn pin output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOEA_A::_1)
    }
}
#[doc = "Field `TOPOLA` reader - AGTOAn Pin Polarity Select"]
pub type TOPOLA_R = crate::BitReader<TOPOLA_A>;
#[doc = "AGTOAn Pin Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOPOLA_A {
    #[doc = "0: AGTOAn pin output is started on low. i.e. normal output"]
    _0 = 0,
    #[doc = "1: AGTOAn pin output is started on high. i.e. inverted output"]
    _1 = 1,
}
impl From<TOPOLA_A> for bool {
    #[inline(always)]
    fn from(variant: TOPOLA_A) -> Self {
        variant as u8 != 0
    }
}
impl TOPOLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOPOLA_A {
        match self.bits {
            false => TOPOLA_A::_0,
            true => TOPOLA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOPOLA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOPOLA_A::_1
    }
}
#[doc = "Field `TOPOLA` writer - AGTOAn Pin Polarity Select"]
pub type TOPOLA_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCMSR_SPEC, TOPOLA_A, O>;
impl<'a, const O: u8> TOPOLA_W<'a, O> {
    #[doc = "AGTOAn pin output is started on low. i.e. normal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOPOLA_A::_0)
    }
    #[doc = "AGTOAn pin output is started on high. i.e. inverted output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOPOLA_A::_1)
    }
}
#[doc = "Field `TCMEB` reader - AGT Compare Match B Register Enable"]
pub type TCMEB_R = crate::BitReader<TCMEB_A>;
#[doc = "AGT Compare Match B Register Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMEB_A {
    #[doc = "0: Compare match B register disabled"]
    _0 = 0,
    #[doc = "1: Compare match B register enabled"]
    _1 = 1,
}
impl From<TCMEB_A> for bool {
    #[inline(always)]
    fn from(variant: TCMEB_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMEB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCMEB_A {
        match self.bits {
            false => TCMEB_A::_0,
            true => TCMEB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMEB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMEB_A::_1
    }
}
#[doc = "Field `TCMEB` writer - AGT Compare Match B Register Enable"]
pub type TCMEB_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCMSR_SPEC, TCMEB_A, O>;
impl<'a, const O: u8> TCMEB_W<'a, O> {
    #[doc = "Compare match B register disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCMEB_A::_0)
    }
    #[doc = "Compare match B register enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCMEB_A::_1)
    }
}
#[doc = "Field `TOEB` reader - AGTOBn Pin Output Enable"]
pub type TOEB_R = crate::BitReader<TOEB_A>;
#[doc = "AGTOBn Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOEB_A {
    #[doc = "0: AGTOBn pin output disabled"]
    _0 = 0,
    #[doc = "1: AGTOBn pin output enabled"]
    _1 = 1,
}
impl From<TOEB_A> for bool {
    #[inline(always)]
    fn from(variant: TOEB_A) -> Self {
        variant as u8 != 0
    }
}
impl TOEB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOEB_A {
        match self.bits {
            false => TOEB_A::_0,
            true => TOEB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOEB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOEB_A::_1
    }
}
#[doc = "Field `TOEB` writer - AGTOBn Pin Output Enable"]
pub type TOEB_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCMSR_SPEC, TOEB_A, O>;
impl<'a, const O: u8> TOEB_W<'a, O> {
    #[doc = "AGTOBn pin output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOEB_A::_0)
    }
    #[doc = "AGTOBn pin output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOEB_A::_1)
    }
}
#[doc = "Field `TOPOLB` reader - AGTOBn Pin Polarity Select"]
pub type TOPOLB_R = crate::BitReader<TOPOLB_A>;
#[doc = "AGTOBn Pin Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOPOLB_A {
    #[doc = "0: AGTOBn pin output is started on low. i.e. normal output"]
    _0 = 0,
    #[doc = "1: AGTOBn pin output is started on high. i.e. inverted output"]
    _1 = 1,
}
impl From<TOPOLB_A> for bool {
    #[inline(always)]
    fn from(variant: TOPOLB_A) -> Self {
        variant as u8 != 0
    }
}
impl TOPOLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOPOLB_A {
        match self.bits {
            false => TOPOLB_A::_0,
            true => TOPOLB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOPOLB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOPOLB_A::_1
    }
}
#[doc = "Field `TOPOLB` writer - AGTOBn Pin Polarity Select"]
pub type TOPOLB_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTCMSR_SPEC, TOPOLB_A, O>;
impl<'a, const O: u8> TOPOLB_W<'a, O> {
    #[doc = "AGTOBn pin output is started on low. i.e. normal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOPOLB_A::_0)
    }
    #[doc = "AGTOBn pin output is started on high. i.e. inverted output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOPOLB_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT Compare Match A Register Enable"]
    #[inline(always)]
    pub fn tcmea(&self) -> TCMEA_R {
        TCMEA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AGTOAn Pin Output Enable"]
    #[inline(always)]
    pub fn toea(&self) -> TOEA_R {
        TOEA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AGTOAn Pin Polarity Select"]
    #[inline(always)]
    pub fn topola(&self) -> TOPOLA_R {
        TOPOLA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - AGT Compare Match B Register Enable"]
    #[inline(always)]
    pub fn tcmeb(&self) -> TCMEB_R {
        TCMEB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AGTOBn Pin Output Enable"]
    #[inline(always)]
    pub fn toeb(&self) -> TOEB_R {
        TOEB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AGTOBn Pin Polarity Select"]
    #[inline(always)]
    pub fn topolb(&self) -> TOPOLB_R {
        TOPOLB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT Compare Match A Register Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcmea(&mut self) -> TCMEA_W<0> {
        TCMEA_W::new(self)
    }
    #[doc = "Bit 1 - AGTOAn Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toea(&mut self) -> TOEA_W<1> {
        TOEA_W::new(self)
    }
    #[doc = "Bit 2 - AGTOAn Pin Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn topola(&mut self) -> TOPOLA_W<2> {
        TOPOLA_W::new(self)
    }
    #[doc = "Bit 4 - AGT Compare Match B Register Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcmeb(&mut self) -> TCMEB_W<4> {
        TCMEB_W::new(self)
    }
    #[doc = "Bit 5 - AGTOBn Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toeb(&mut self) -> TOEB_W<5> {
        TOEB_W::new(self)
    }
    #[doc = "Bit 6 - AGTOBn Pin Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn topolb(&mut self) -> TOPOLB_W<6> {
        TOPOLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Compare Match Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtcmsr](index.html) module"]
pub struct AGTCMSR_SPEC;
impl crate::RegisterSpec for AGTCMSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [agtcmsr::R](R) reader structure"]
impl crate::Readable for AGTCMSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtcmsr::W](W) writer structure"]
impl crate::Writable for AGTCMSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTCMSR to value 0"]
impl crate::Resettable for AGTCMSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
