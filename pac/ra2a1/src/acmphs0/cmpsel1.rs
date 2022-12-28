#[doc = "Register `CMPSEL1` reader"]
pub struct R(crate::R<CMPSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPSEL1` writer"]
pub struct W(crate::W<CMPSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPSEL1_SPEC>;
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
impl From<crate::W<CMPSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRVS` reader - Reference voltage selection"]
pub type CRVS_R = crate::FieldReader<u8, CRVS_A>;
#[doc = "Reference voltage selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS_A {
    #[doc = "0: No reference voltage"]
    _000000 = 0,
    #[doc = "1: IVREF0 selected"]
    _000001 = 1,
    #[doc = "2: IVREF1 selected"]
    _000010 = 2,
    #[doc = "4: IVREF2 selected"]
    _000100 = 4,
    #[doc = "8: IVREF3 selected"]
    _001000 = 8,
    #[doc = "16: IVREF4 selected"]
    _010000 = 16,
    #[doc = "32: IVREF5 selected"]
    _100000 = 32,
}
impl From<CRVS_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS_A) -> Self {
        variant as _
    }
}
impl CRVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRVS_A> {
        match self.bits {
            0 => Some(CRVS_A::_000000),
            1 => Some(CRVS_A::_000001),
            2 => Some(CRVS_A::_000010),
            4 => Some(CRVS_A::_000100),
            8 => Some(CRVS_A::_001000),
            16 => Some(CRVS_A::_010000),
            32 => Some(CRVS_A::_100000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        *self == CRVS_A::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        *self == CRVS_A::_000001
    }
    #[doc = "Checks if the value of the field is `_000010`"]
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        *self == CRVS_A::_000010
    }
    #[doc = "Checks if the value of the field is `_000100`"]
    #[inline(always)]
    pub fn is_000100(&self) -> bool {
        *self == CRVS_A::_000100
    }
    #[doc = "Checks if the value of the field is `_001000`"]
    #[inline(always)]
    pub fn is_001000(&self) -> bool {
        *self == CRVS_A::_001000
    }
    #[doc = "Checks if the value of the field is `_010000`"]
    #[inline(always)]
    pub fn is_010000(&self) -> bool {
        *self == CRVS_A::_010000
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline(always)]
    pub fn is_100000(&self) -> bool {
        *self == CRVS_A::_100000
    }
}
#[doc = "Field `CRVS` writer - Reference voltage selection"]
pub type CRVS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMPSEL1_SPEC, u8, CRVS_A, 6, O>;
impl<'a, const O: u8> CRVS_W<'a, O> {
    #[doc = "No reference voltage"]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(CRVS_A::_000000)
    }
    #[doc = "IVREF0 selected"]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(CRVS_A::_000001)
    }
    #[doc = "IVREF1 selected"]
    #[inline(always)]
    pub fn _000010(self) -> &'a mut W {
        self.variant(CRVS_A::_000010)
    }
    #[doc = "IVREF2 selected"]
    #[inline(always)]
    pub fn _000100(self) -> &'a mut W {
        self.variant(CRVS_A::_000100)
    }
    #[doc = "IVREF3 selected"]
    #[inline(always)]
    pub fn _001000(self) -> &'a mut W {
        self.variant(CRVS_A::_001000)
    }
    #[doc = "IVREF4 selected"]
    #[inline(always)]
    pub fn _010000(self) -> &'a mut W {
        self.variant(CRVS_A::_010000)
    }
    #[doc = "IVREF5 selected"]
    #[inline(always)]
    pub fn _100000(self) -> &'a mut W {
        self.variant(CRVS_A::_100000)
    }
}
impl R {
    #[doc = "Bits 0:5 - Reference voltage selection"]
    #[inline(always)]
    pub fn crvs(&self) -> CRVS_R {
        CRVS_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Reference voltage selection"]
    #[inline(always)]
    #[must_use]
    pub fn crvs(&mut self) -> CRVS_W<0> {
        CRVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Reference Voltage Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpsel1](index.html) module"]
pub struct CMPSEL1_SPEC;
impl crate::RegisterSpec for CMPSEL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmpsel1::R](R) reader structure"]
impl crate::Readable for CMPSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpsel1::W](W) writer structure"]
impl crate::Writable for CMPSEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPSEL1 to value 0"]
impl crate::Resettable for CMPSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
