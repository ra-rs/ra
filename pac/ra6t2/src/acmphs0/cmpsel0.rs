#[doc = "Register `CMPSEL0` reader"]
pub struct R(crate::R<CMPSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPSEL0` writer"]
pub struct W(crate::W<CMPSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPSEL0_SPEC>;
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
impl From<crate::W<CMPSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSEL` reader - Comparator Input Selection"]
pub type CMPSEL_R = crate::FieldReader<u8, CMPSEL_A>;
#[doc = "Comparator Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL_A {
    #[doc = "0: Do not input"]
    _0X0 = 0,
    #[doc = "1: Select IVCMP0"]
    _0X1 = 1,
    #[doc = "2: Setting prohibited"]
    _0X2 = 2,
    #[doc = "4: Select IVCMP2"]
    _0X4 = 4,
    #[doc = "8: Select IVCMP3"]
    _0X8 = 8,
}
impl From<CMPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL_A) -> Self {
        variant as _
    }
}
impl CMPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPSEL_A> {
        match self.bits {
            0 => Some(CMPSEL_A::_0X0),
            1 => Some(CMPSEL_A::_0X1),
            2 => Some(CMPSEL_A::_0X2),
            4 => Some(CMPSEL_A::_0X4),
            8 => Some(CMPSEL_A::_0X8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CMPSEL_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == CMPSEL_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == CMPSEL_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == CMPSEL_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == CMPSEL_A::_0X8
    }
}
#[doc = "Field `CMPSEL` writer - Comparator Input Selection"]
pub type CMPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMPSEL0_SPEC, u8, CMPSEL_A, 4, O>;
impl<'a, const O: u8> CMPSEL_W<'a, O> {
    #[doc = "Do not input"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CMPSEL_A::_0X0)
    }
    #[doc = "Select IVCMP0"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(CMPSEL_A::_0X1)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(CMPSEL_A::_0X2)
    }
    #[doc = "Select IVCMP2"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(CMPSEL_A::_0X4)
    }
    #[doc = "Select IVCMP3"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(CMPSEL_A::_0X8)
    }
}
impl R {
    #[doc = "Bits 0:3 - Comparator Input Selection"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comparator Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel(&mut self) -> CMPSEL_W<0> {
        CMPSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpsel0](index.html) module"]
pub struct CMPSEL0_SPEC;
impl crate::RegisterSpec for CMPSEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmpsel0::R](R) reader structure"]
impl crate::Readable for CMPSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpsel0::W](W) writer structure"]
impl crate::Writable for CMPSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPSEL0 to value 0"]
impl crate::Resettable for CMPSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
