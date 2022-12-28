#[doc = "Register `CTSUCHACB` reader"]
pub struct R(crate::R<CTSUCHACB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHACB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHACB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHACB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHACB` writer"]
pub struct W(crate::W<CTSUCHACB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHACB_SPEC>;
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
impl From<crate::W<CTSUCHACB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHACB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHAC32` reader - CTSU Channel Enable Control B"]
pub type CHAC32_R = crate::BitReader<CHAC32_A>;
#[doc = "CTSU Channel Enable Control B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC32_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC32_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC32_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC32_A {
        match self.bits {
            false => CHAC32_A::_0,
            true => CHAC32_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC32_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC32_A::_1
    }
}
#[doc = "Field `CHAC32` writer - CTSU Channel Enable Control B"]
pub type CHAC32_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACB_SPEC, CHAC32_A, O>;
impl<'a, const O: u8> CHAC32_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC32_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC32_A::_1)
    }
}
#[doc = "Field `CHAC33` reader - CTSU Channel Enable Control B"]
pub type CHAC33_R = crate::BitReader<CHAC33_A>;
#[doc = "CTSU Channel Enable Control B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC33_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC33_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC33_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC33_A {
        match self.bits {
            false => CHAC33_A::_0,
            true => CHAC33_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC33_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC33_A::_1
    }
}
#[doc = "Field `CHAC33` writer - CTSU Channel Enable Control B"]
pub type CHAC33_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACB_SPEC, CHAC33_A, O>;
impl<'a, const O: u8> CHAC33_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC33_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC33_A::_1)
    }
}
#[doc = "Field `CHAC34` reader - CTSU Channel Enable Control B"]
pub type CHAC34_R = crate::BitReader<CHAC34_A>;
#[doc = "CTSU Channel Enable Control B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC34_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC34_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC34_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC34_A {
        match self.bits {
            false => CHAC34_A::_0,
            true => CHAC34_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC34_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC34_A::_1
    }
}
#[doc = "Field `CHAC34` writer - CTSU Channel Enable Control B"]
pub type CHAC34_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACB_SPEC, CHAC34_A, O>;
impl<'a, const O: u8> CHAC34_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC34_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC34_A::_1)
    }
}
#[doc = "Field `CHAC35` reader - CTSU Channel Enable Control B"]
pub type CHAC35_R = crate::BitReader<CHAC35_A>;
#[doc = "CTSU Channel Enable Control B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHAC35_A {
    #[doc = "0: Do not measure."]
    _0 = 0,
    #[doc = "1: Measure."]
    _1 = 1,
}
impl From<CHAC35_A> for bool {
    #[inline(always)]
    fn from(variant: CHAC35_A) -> Self {
        variant as u8 != 0
    }
}
impl CHAC35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAC35_A {
        match self.bits {
            false => CHAC35_A::_0,
            true => CHAC35_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHAC35_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHAC35_A::_1
    }
}
#[doc = "Field `CHAC35` writer - CTSU Channel Enable Control B"]
pub type CHAC35_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHACB_SPEC, CHAC35_A, O>;
impl<'a, const O: u8> CHAC35_W<'a, O> {
    #[doc = "Do not measure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAC35_A::_0)
    }
    #[doc = "Measure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAC35_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Channel Enable Control B"]
    #[inline(always)]
    pub fn chac32(&self) -> CHAC32_R {
        CHAC32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU Channel Enable Control B"]
    #[inline(always)]
    pub fn chac33(&self) -> CHAC33_R {
        CHAC33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Channel Enable Control B"]
    #[inline(always)]
    pub fn chac34(&self) -> CHAC34_R {
        CHAC34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTSU Channel Enable Control B"]
    #[inline(always)]
    pub fn chac35(&self) -> CHAC35_R {
        CHAC35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Channel Enable Control B"]
    #[inline(always)]
    #[must_use]
    pub fn chac32(&mut self) -> CHAC32_W<0> {
        CHAC32_W::new(self)
    }
    #[doc = "Bit 1 - CTSU Channel Enable Control B"]
    #[inline(always)]
    #[must_use]
    pub fn chac33(&mut self) -> CHAC33_W<1> {
        CHAC33_W::new(self)
    }
    #[doc = "Bit 2 - CTSU Channel Enable Control B"]
    #[inline(always)]
    #[must_use]
    pub fn chac34(&mut self) -> CHAC34_W<2> {
        CHAC34_W::new(self)
    }
    #[doc = "Bit 3 - CTSU Channel Enable Control B"]
    #[inline(always)]
    #[must_use]
    pub fn chac35(&mut self) -> CHAC35_W<3> {
        CHAC35_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Enable Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchacb](index.html) module"]
pub struct CTSUCHACB_SPEC;
impl crate::RegisterSpec for CTSUCHACB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsuchacb::R](R) reader structure"]
impl crate::Readable for CTSUCHACB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchacb::W](W) writer structure"]
impl crate::Writable for CTSUCHACB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHACB to value 0"]
impl crate::Resettable for CTSUCHACB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
