#[doc = "Register `ADANSB0` reader"]
pub struct R(crate::R<ADANSB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSB0` writer"]
pub struct W(crate::W<ADANSB0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSB0_SPEC>;
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
impl From<crate::W<ADANSB0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSB0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSB00` reader - AN000 Select"]
pub type ANSB00_R = crate::BitReader<ANSB00_A>;
#[doc = "AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB00_A {
    #[doc = "0: AN000 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN000 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB00_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB00_A {
        match self.bits {
            false => ANSB00_A::_0,
            true => ANSB00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB00_A::_1
    }
}
#[doc = "Field `ANSB00` writer - AN000 Select"]
pub type ANSB00_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB00_A, O>;
impl<'a, const O: u8> ANSB00_W<'a, O> {
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB00_A::_0)
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB00_A::_1)
    }
}
#[doc = "Field `ANSB01` reader - AN001 Select"]
pub type ANSB01_R = crate::BitReader<ANSB01_A>;
#[doc = "AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB01_A {
    #[doc = "0: AN001 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN001 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB01_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB01_A {
        match self.bits {
            false => ANSB01_A::_0,
            true => ANSB01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB01_A::_1
    }
}
#[doc = "Field `ANSB01` writer - AN001 Select"]
pub type ANSB01_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB01_A, O>;
impl<'a, const O: u8> ANSB01_W<'a, O> {
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB01_A::_0)
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB01_A::_1)
    }
}
#[doc = "Field `ANSB02` reader - AN002 Select"]
pub type ANSB02_R = crate::BitReader<ANSB02_A>;
#[doc = "AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB02_A {
    #[doc = "0: AN002 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN002 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB02_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB02_A {
        match self.bits {
            false => ANSB02_A::_0,
            true => ANSB02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB02_A::_1
    }
}
#[doc = "Field `ANSB02` writer - AN002 Select"]
pub type ANSB02_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB02_A, O>;
impl<'a, const O: u8> ANSB02_W<'a, O> {
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB02_A::_0)
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB02_A::_1)
    }
}
#[doc = "Field `ANSB03` reader - AN003 Select"]
pub type ANSB03_R = crate::BitReader<ANSB03_A>;
#[doc = "AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB03_A {
    #[doc = "0: AN003 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN003 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB03_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB03_A {
        match self.bits {
            false => ANSB03_A::_0,
            true => ANSB03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB03_A::_1
    }
}
#[doc = "Field `ANSB03` writer - AN003 Select"]
pub type ANSB03_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB03_A, O>;
impl<'a, const O: u8> ANSB03_W<'a, O> {
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB03_A::_0)
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB03_A::_1)
    }
}
#[doc = "Field `ANSB05` reader - AN005 Select"]
pub type ANSB05_R = crate::BitReader<ANSB05_A>;
#[doc = "AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB05_A {
    #[doc = "0: AN005 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN005 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB05_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB05_A {
        match self.bits {
            false => ANSB05_A::_0,
            true => ANSB05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB05_A::_1
    }
}
#[doc = "Field `ANSB05` writer - AN005 Select"]
pub type ANSB05_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB05_A, O>;
impl<'a, const O: u8> ANSB05_W<'a, O> {
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB05_A::_0)
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB05_A::_1)
    }
}
#[doc = "Field `ANSB06` reader - AN006 Select"]
pub type ANSB06_R = crate::BitReader<ANSB06_A>;
#[doc = "AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB06_A {
    #[doc = "0: AN006 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN006 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB06_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB06_A {
        match self.bits {
            false => ANSB06_A::_0,
            true => ANSB06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB06_A::_1
    }
}
#[doc = "Field `ANSB06` writer - AN006 Select"]
pub type ANSB06_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB06_A, O>;
impl<'a, const O: u8> ANSB06_W<'a, O> {
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB06_A::_0)
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB06_A::_1)
    }
}
#[doc = "Field `ANSB07` reader - AN007 Select"]
pub type ANSB07_R = crate::BitReader<ANSB07_A>;
#[doc = "AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB07_A {
    #[doc = "0: AN007 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN007 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB07_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANSB07_A {
        match self.bits {
            false => ANSB07_A::_0,
            true => ANSB07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB07_A::_1
    }
}
#[doc = "Field `ANSB07` writer - AN007 Select"]
pub type ANSB07_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADANSB0_SPEC, ANSB07_A, O>;
impl<'a, const O: u8> ANSB07_W<'a, O> {
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSB07_A::_0)
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSB07_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn ansb00(&self) -> ANSB00_R {
        ANSB00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn ansb01(&self) -> ANSB01_R {
        ANSB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn ansb02(&self) -> ANSB02_R {
        ANSB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn ansb03(&self) -> ANSB03_R {
        ANSB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn ansb05(&self) -> ANSB05_R {
        ANSB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn ansb06(&self) -> ANSB06_R {
        ANSB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn ansb07(&self) -> ANSB07_R {
        ANSB07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb00(&mut self) -> ANSB00_W<0> {
        ANSB00_W::new(self)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb01(&mut self) -> ANSB01_W<1> {
        ANSB01_W::new(self)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb02(&mut self) -> ANSB02_W<2> {
        ANSB02_W::new(self)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb03(&mut self) -> ANSB03_W<3> {
        ANSB03_W::new(self)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb05(&mut self) -> ANSB05_W<5> {
        ANSB05_W::new(self)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb06(&mut self) -> ANSB06_W<6> {
        ANSB06_W::new(self)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb07(&mut self) -> ANSB07_W<7> {
        ANSB07_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register B0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansb0](index.html) module"]
pub struct ADANSB0_SPEC;
impl crate::RegisterSpec for ADANSB0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansb0::R](R) reader structure"]
impl crate::Readable for ADANSB0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansb0::W](W) writer structure"]
impl crate::Writable for ADANSB0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSB0 to value 0"]
impl crate::Resettable for ADANSB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
