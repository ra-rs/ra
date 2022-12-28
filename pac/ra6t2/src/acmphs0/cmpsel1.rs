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
#[doc = "Field `CRVS` reader - Reference Voltage Selection"]
pub type CRVS_R = crate::FieldReader<u8, CRVS_A>;
#[doc = "Reference Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS_A {
    #[doc = "0: Do not input"]
    _0X0 = 0,
    #[doc = "1: Select IVREF0"]
    _0X1 = 1,
    #[doc = "2: Select IVREF1"]
    _0X2 = 2,
    #[doc = "4: Select IVREF2"]
    _0X4 = 4,
    #[doc = "8: Select IVREF3"]
    _0X8 = 8,
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
            0 => Some(CRVS_A::_0X0),
            1 => Some(CRVS_A::_0X1),
            2 => Some(CRVS_A::_0X2),
            4 => Some(CRVS_A::_0X4),
            8 => Some(CRVS_A::_0X8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CRVS_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == CRVS_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == CRVS_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == CRVS_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == CRVS_A::_0X8
    }
}
#[doc = "Field `CRVS` writer - Reference Voltage Selection"]
pub type CRVS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMPSEL1_SPEC, u8, CRVS_A, 4, O>;
impl<'a, const O: u8> CRVS_W<'a, O> {
    #[doc = "Do not input"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CRVS_A::_0X0)
    }
    #[doc = "Select IVREF0"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(CRVS_A::_0X1)
    }
    #[doc = "Select IVREF1"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(CRVS_A::_0X2)
    }
    #[doc = "Select IVREF2"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(CRVS_A::_0X4)
    }
    #[doc = "Select IVREF3"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(CRVS_A::_0X8)
    }
}
impl R {
    #[doc = "Bits 0:3 - Reference Voltage Selection"]
    #[inline(always)]
    pub fn crvs(&self) -> CRVS_R {
        CRVS_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference Voltage Selection"]
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
