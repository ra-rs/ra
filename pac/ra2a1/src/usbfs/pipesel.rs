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
    _0000 = 0,
    #[doc = "4: PIPE4"]
    _0100 = 4,
    #[doc = "5: PIPE5"]
    _0101 = 5,
    #[doc = "6: PIPE6"]
    _0110 = 6,
    #[doc = "7: PIPE7"]
    _0111 = 7,
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
            0 => Some(PIPESEL_A::_0000),
            4 => Some(PIPESEL_A::_0100),
            5 => Some(PIPESEL_A::_0101),
            6 => Some(PIPESEL_A::_0110),
            7 => Some(PIPESEL_A::_0111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PIPESEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PIPESEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PIPESEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PIPESEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PIPESEL_A::_0111
    }
}
#[doc = "Field `PIPESEL` writer - Pipe Window Select"]
pub type PIPESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, PIPESEL_SPEC, u8, PIPESEL_A, 4, O>;
impl<'a, const O: u8> PIPESEL_W<'a, O> {
    #[doc = "No pipe selected"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0000)
    }
    #[doc = "PIPE4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0100)
    }
    #[doc = "PIPE5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0101)
    }
    #[doc = "PIPE6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0110)
    }
    #[doc = "PIPE7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PIPESEL_A::_0111)
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
