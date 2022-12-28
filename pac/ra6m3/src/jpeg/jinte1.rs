#[doc = "Register `JINTE1` reader"]
pub struct R(crate::R<JINTE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JINTE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JINTE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JINTE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JINTE1` writer"]
pub struct W(crate::W<JINTE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JINTE1_SPEC>;
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
impl From<crate::W<JINTE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JINTE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTLEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1"]
pub type DOUTLEN_R = crate::BitReader<DOUTLEN_A>;
#[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOUTLEN_A {
    #[doc = "0: Disables an interrupt request."]
    _0 = 0,
    #[doc = "1: Enables an interrupt request."]
    _1 = 1,
}
impl From<DOUTLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOUTLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOUTLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUTLEN_A {
        match self.bits {
            false => DOUTLEN_A::_0,
            true => DOUTLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOUTLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOUTLEN_A::_1
    }
}
#[doc = "Field `DOUTLEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1"]
pub type DOUTLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTE1_SPEC, DOUTLEN_A, O>;
impl<'a, const O: u8> DOUTLEN_W<'a, O> {
    #[doc = "Disables an interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUTLEN_A::_0)
    }
    #[doc = "Enables an interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUTLEN_A::_1)
    }
}
#[doc = "Field `JINEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1."]
pub type JINEN_R = crate::BitReader<JINEN_A>;
#[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JINEN_A {
    #[doc = "0: Disables an interrupt request."]
    _0 = 0,
    #[doc = "1: Enables an interrupt request."]
    _1 = 1,
}
impl From<JINEN_A> for bool {
    #[inline(always)]
    fn from(variant: JINEN_A) -> Self {
        variant as u8 != 0
    }
}
impl JINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JINEN_A {
        match self.bits {
            false => JINEN_A::_0,
            true => JINEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JINEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JINEN_A::_1
    }
}
#[doc = "Field `JINEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1."]
pub type JINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTE1_SPEC, JINEN_A, O>;
impl<'a, const O: u8> JINEN_W<'a, O> {
    #[doc = "Disables an interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(JINEN_A::_0)
    }
    #[doc = "Enables an interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(JINEN_A::_1)
    }
}
#[doc = "Field `DBTEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1."]
pub type DBTEN_R = crate::BitReader<DBTEN_A>;
#[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBTEN_A {
    #[doc = "0: Disables an interrupt request."]
    _0 = 0,
    #[doc = "1: Enables an interrupt request."]
    _1 = 1,
}
impl From<DBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBTEN_A {
        match self.bits {
            false => DBTEN_A::_0,
            true => DBTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBTEN_A::_1
    }
}
#[doc = "Field `DBTEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1."]
pub type DBTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTE1_SPEC, DBTEN_A, O>;
impl<'a, const O: u8> DBTEN_W<'a, O> {
    #[doc = "Disables an interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBTEN_A::_0)
    }
    #[doc = "Enables an interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBTEN_A::_1)
    }
}
#[doc = "Field `DINLEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1."]
pub type DINLEN_R = crate::BitReader<DINLEN_A>;
#[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINLEN_A {
    #[doc = "0: Disables an interrupt request."]
    _0 = 0,
    #[doc = "1: Enables an interrupt request."]
    _1 = 1,
}
impl From<DINLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DINLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINLEN_A {
        match self.bits {
            false => DINLEN_A::_0,
            true => DINLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINLEN_A::_1
    }
}
#[doc = "Field `DINLEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1."]
pub type DINLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTE1_SPEC, DINLEN_A, O>;
impl<'a, const O: u8> DINLEN_W<'a, O> {
    #[doc = "Disables an interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINLEN_A::_0)
    }
    #[doc = "Enables an interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINLEN_A::_1)
    }
}
#[doc = "Field `CBTEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1."]
pub type CBTEN_R = crate::BitReader<CBTEN_A>;
#[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBTEN_A {
    #[doc = "0: Disables an interrupt request."]
    _0 = 0,
    #[doc = "1: Enables an interrupt request."]
    _1 = 1,
}
impl From<CBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CBTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CBTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBTEN_A {
        match self.bits {
            false => CBTEN_A::_0,
            true => CBTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CBTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CBTEN_A::_1
    }
}
#[doc = "Field `CBTEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1."]
pub type CBTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, JINTE1_SPEC, CBTEN_A, O>;
impl<'a, const O: u8> CBTEN_W<'a, O> {
    #[doc = "Disables an interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CBTEN_A::_0)
    }
    #[doc = "Enables an interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CBTEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1"]
    #[inline(always)]
    pub fn doutlen(&self) -> DOUTLEN_R {
        DOUTLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn jinen(&self) -> JINEN_R {
        JINEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn dbten(&self) -> DBTEN_R {
        DBTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn dinlen(&self) -> DINLEN_R {
        DINLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn cbten(&self) -> CBTEN_R {
        CBTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn doutlen(&mut self) -> DOUTLEN_W<0> {
        DOUTLEN_W::new(self)
    }
    #[doc = "Bit 1 - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn jinen(&mut self) -> JINEN_W<1> {
        JINEN_W::new(self)
    }
    #[doc = "Bit 2 - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn dbten(&mut self) -> DBTEN_W<2> {
        DBTEN_W::new(self)
    }
    #[doc = "Bit 5 - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn dinlen(&mut self) -> DINLEN_W<5> {
        DINLEN_W::new(self)
    }
    #[doc = "Bit 6 - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn cbten(&mut self) -> CBTEN_W<6> {
        CBTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interrupt Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jinte1](index.html) module"]
pub struct JINTE1_SPEC;
impl crate::RegisterSpec for JINTE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jinte1::R](R) reader structure"]
impl crate::Readable for JINTE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jinte1::W](W) writer structure"]
impl crate::Writable for JINTE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JINTE1 to value 0"]
impl crate::Resettable for JINTE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
