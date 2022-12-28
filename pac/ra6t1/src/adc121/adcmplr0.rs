#[doc = "Register `ADCMPLR0` reader"]
pub struct R(crate::R<ADCMPLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPLR0` writer"]
pub struct W(crate::W<ADCMPLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPLR0_SPEC>;
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
impl From<crate::W<ADCMPLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPLCHA00` reader - Comparison condition of AN100"]
pub type CMPLCHA00_R = crate::BitReader<CMPLCHA00_A>;
#[doc = "Comparison condition of AN100\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA00_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA00_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA00_A {
        match self.bits {
            false => CMPLCHA00_A::_0,
            true => CMPLCHA00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA00_A::_1
    }
}
#[doc = "Field `CMPLCHA00` writer - Comparison condition of AN100"]
pub type CMPLCHA00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA00_A, O>;
impl<'a, const O: u8> CMPLCHA00_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA00_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA00_A::_1)
    }
}
#[doc = "Field `CMPLCHA01` reader - Comparison condition of AN101"]
pub type CMPLCHA01_R = crate::BitReader<CMPLCHA01_A>;
#[doc = "Comparison condition of AN101\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA01_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA01_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA01_A {
        match self.bits {
            false => CMPLCHA01_A::_0,
            true => CMPLCHA01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA01_A::_1
    }
}
#[doc = "Field `CMPLCHA01` writer - Comparison condition of AN101"]
pub type CMPLCHA01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA01_A, O>;
impl<'a, const O: u8> CMPLCHA01_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA01_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA01_A::_1)
    }
}
#[doc = "Field `CMPLCHA02` reader - Comparison condition of AN102"]
pub type CMPLCHA02_R = crate::BitReader<CMPLCHA02_A>;
#[doc = "Comparison condition of AN102\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA02_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA02_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA02_A {
        match self.bits {
            false => CMPLCHA02_A::_0,
            true => CMPLCHA02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA02_A::_1
    }
}
#[doc = "Field `CMPLCHA02` writer - Comparison condition of AN102"]
pub type CMPLCHA02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA02_A, O>;
impl<'a, const O: u8> CMPLCHA02_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA02_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA02_A::_1)
    }
}
#[doc = "Field `CMPLCHA05` reader - Comparison condition of AN105"]
pub type CMPLCHA05_R = crate::BitReader<CMPLCHA05_A>;
#[doc = "Comparison condition of AN105\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA05_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA05_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA05_A {
        match self.bits {
            false => CMPLCHA05_A::_0,
            true => CMPLCHA05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA05_A::_1
    }
}
#[doc = "Field `CMPLCHA05` writer - Comparison condition of AN105"]
pub type CMPLCHA05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA05_A, O>;
impl<'a, const O: u8> CMPLCHA05_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA05_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA05_A::_1)
    }
}
#[doc = "Field `CMPLCHA06` reader - Comparison condition of AN106"]
pub type CMPLCHA06_R = crate::BitReader<CMPLCHA06_A>;
#[doc = "Comparison condition of AN106\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA06_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA06_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA06_A {
        match self.bits {
            false => CMPLCHA06_A::_0,
            true => CMPLCHA06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA06_A::_1
    }
}
#[doc = "Field `CMPLCHA06` writer - Comparison condition of AN106"]
pub type CMPLCHA06_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA06_A, O>;
impl<'a, const O: u8> CMPLCHA06_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA06_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA06_A::_1)
    }
}
#[doc = "Field `CMPLCHA07` reader - Comparison condition of AN107"]
pub type CMPLCHA07_R = crate::BitReader<CMPLCHA07_A>;
#[doc = "Comparison condition of AN107\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA07_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA07_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLCHA07_A {
        match self.bits {
            false => CMPLCHA07_A::_0,
            true => CMPLCHA07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA07_A::_1
    }
}
#[doc = "Field `CMPLCHA07` writer - Comparison condition of AN107"]
pub type CMPLCHA07_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPLR0_SPEC, CMPLCHA07_A, O>;
impl<'a, const O: u8> CMPLCHA07_W<'a, O> {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPLCHA07_A::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPLCHA07_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparison condition of AN100"]
    #[inline(always)]
    pub fn cmplcha00(&self) -> CMPLCHA00_R {
        CMPLCHA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison condition of AN101"]
    #[inline(always)]
    pub fn cmplcha01(&self) -> CMPLCHA01_R {
        CMPLCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison condition of AN102"]
    #[inline(always)]
    pub fn cmplcha02(&self) -> CMPLCHA02_R {
        CMPLCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison condition of AN105"]
    #[inline(always)]
    pub fn cmplcha05(&self) -> CMPLCHA05_R {
        CMPLCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison condition of AN106"]
    #[inline(always)]
    pub fn cmplcha06(&self) -> CMPLCHA06_R {
        CMPLCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison condition of AN107"]
    #[inline(always)]
    pub fn cmplcha07(&self) -> CMPLCHA07_R {
        CMPLCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison condition of AN100"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha00(&mut self) -> CMPLCHA00_W<0> {
        CMPLCHA00_W::new(self)
    }
    #[doc = "Bit 1 - Comparison condition of AN101"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha01(&mut self) -> CMPLCHA01_W<1> {
        CMPLCHA01_W::new(self)
    }
    #[doc = "Bit 2 - Comparison condition of AN102"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha02(&mut self) -> CMPLCHA02_W<2> {
        CMPLCHA02_W::new(self)
    }
    #[doc = "Bit 5 - Comparison condition of AN105"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha05(&mut self) -> CMPLCHA05_W<5> {
        CMPLCHA05_W::new(self)
    }
    #[doc = "Bit 6 - Comparison condition of AN106"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha06(&mut self) -> CMPLCHA06_W<6> {
        CMPLCHA06_W::new(self)
    }
    #[doc = "Bit 7 - Comparison condition of AN107"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha07(&mut self) -> CMPLCHA07_W<7> {
        CMPLCHA07_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmplr0](index.html) module"]
pub struct ADCMPLR0_SPEC;
impl crate::RegisterSpec for ADCMPLR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmplr0::R](R) reader structure"]
impl crate::Readable for ADCMPLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmplr0::W](W) writer structure"]
impl crate::Writable for ADCMPLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPLR0 to value 0"]
impl crate::Resettable for ADCMPLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
