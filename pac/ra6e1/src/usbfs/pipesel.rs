#[doc = "Register `PIPESEL` reader"]
pub struct R(crate::R<PIPESEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPESEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPESEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPESEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPESEL` writer"]
pub struct W(crate::W<PIPESEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPESEL_SPEC>;
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
impl From<crate::W<PIPESEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPESEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIPESEL` reader - Pipe Window Select"]
pub type PIPESEL_R = crate::FieldReader<u8, PIPESEL_A>;
#[doc = "Pipe Window Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIPESEL_A {
    #[doc = "0: No pipe selected"]
    _0X0 = 0,
    #[doc = "1: Pipe 1"]
    _0X1 = 1,
    #[doc = "2: Pipe 2"]
    _0X2 = 2,
    #[doc = "3: Pipe 3"]
    _0X3 = 3,
    #[doc = "4: Pipe 4"]
    _0X4 = 4,
    #[doc = "5: Pipe 5"]
    _0X5 = 5,
    #[doc = "6: Pipe 6"]
    _0X6 = 6,
    #[doc = "7: Pipe 7"]
    _0X7 = 7,
    #[doc = "8: Pipe 8"]
    _0X8 = 8,
    #[doc = "9: Pipe 9"]
    _0X9 = 9,
}
impl From<PIPESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PIPESEL_A) -> Self {
        variant as _
    }
}
impl PIPESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIPESEL_A> {
        match self.bits {
            0 => Some(PIPESEL_A::_0X0),
            1 => Some(PIPESEL_A::_0X1),
            2 => Some(PIPESEL_A::_0X2),
            3 => Some(PIPESEL_A::_0X3),
            4 => Some(PIPESEL_A::_0X4),
            5 => Some(PIPESEL_A::_0X5),
            6 => Some(PIPESEL_A::_0X6),
            7 => Some(PIPESEL_A::_0X7),
            8 => Some(PIPESEL_A::_0X8),
            9 => Some(PIPESEL_A::_0X9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == PIPESEL_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == PIPESEL_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == PIPESEL_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == PIPESEL_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == PIPESEL_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == PIPESEL_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == PIPESEL_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == PIPESEL_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == PIPESEL_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == PIPESEL_A::_0X9
    }
}
#[doc = "Field `PIPESEL` writer - Pipe Window Select"]
pub type PIPESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, PIPESEL_SPEC, u8, PIPESEL_A, 4, O>;
impl<'a, const O: u8> PIPESEL_W<'a, O> {
    #[doc = "No pipe selected"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X0)
    }
    #[doc = "Pipe 1"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X1)
    }
    #[doc = "Pipe 2"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X2)
    }
    #[doc = "Pipe 3"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X3)
    }
    #[doc = "Pipe 4"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X4)
    }
    #[doc = "Pipe 5"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X5)
    }
    #[doc = "Pipe 6"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X6)
    }
    #[doc = "Pipe 7"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X7)
    }
    #[doc = "Pipe 8"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X8)
    }
    #[doc = "Pipe 9"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0X9)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pipe Window Select"]
    #[inline(always)]
    pub fn pipesel(&self) -> PIPESEL_R {
        PIPESEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pipe Window Select"]
    #[inline(always)]
    #[must_use]
    pub fn pipesel(&mut self) -> PIPESEL_W<0> {
        PIPESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Window Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipesel](index.html) module"]
pub struct PIPESEL_SPEC;
impl crate::RegisterSpec for PIPESEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipesel::R](R) reader structure"]
impl crate::Readable for PIPESEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipesel::W](W) writer structure"]
impl crate::Writable for PIPESEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPESEL to value 0"]
impl crate::Resettable for PIPESEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
