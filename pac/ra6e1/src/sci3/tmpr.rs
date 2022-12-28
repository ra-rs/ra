#[doc = "Register `TMPR` reader"]
pub struct R(crate::R<TMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMPR` writer"]
pub struct W(crate::W<TMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMPR_SPEC>;
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
impl From<crate::W<TMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPLEN` reader - Transmit preface length"]
pub type TPLEN_R = crate::FieldReader<u8, TPLEN_A>;
#[doc = "Transmit preface length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPLEN_A {
    #[doc = "0: Disables the transmit preface generation"]
    _0X0 = 0,
}
impl From<TPLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TPLEN_A) -> Self {
        variant as _
    }
}
impl TPLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPLEN_A> {
        match self.bits {
            0 => Some(TPLEN_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == TPLEN_A::_0X0
    }
}
#[doc = "Field `TPLEN` writer - Transmit preface length"]
pub type TPLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TMPR_SPEC, u8, TPLEN_A, 4, O>;
impl<'a, const O: u8> TPLEN_W<'a, O> {
    #[doc = "Disables the transmit preface generation"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(TPLEN_A::_0X0)
    }
}
#[doc = "Field `TPPAT` reader - Transmit preface pattern"]
pub type TPPAT_R = crate::FieldReader<u8, TPPAT_A>;
#[doc = "Transmit preface pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPPAT_A {
    #[doc = "0: ALL ZERO"]
    _00 = 0,
    #[doc = "1: ZERO ONE"]
    _01 = 1,
    #[doc = "2: ONE ZERO"]
    _10 = 2,
    #[doc = "3: ALL ONE"]
    _11 = 3,
}
impl From<TPPAT_A> for u8 {
    #[inline(always)]
    fn from(variant: TPPAT_A) -> Self {
        variant as _
    }
}
impl TPPAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPPAT_A {
        match self.bits {
            0 => TPPAT_A::_00,
            1 => TPPAT_A::_01,
            2 => TPPAT_A::_10,
            3 => TPPAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPPAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPPAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPPAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TPPAT_A::_11
    }
}
#[doc = "Field `TPPAT` writer - Transmit preface pattern"]
pub type TPPAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TMPR_SPEC, u8, TPPAT_A, 2, O>;
impl<'a, const O: u8> TPPAT_W<'a, O> {
    #[doc = "ALL ZERO"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPPAT_A::_00)
    }
    #[doc = "ZERO ONE"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPPAT_A::_01)
    }
    #[doc = "ONE ZERO"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPPAT_A::_10)
    }
    #[doc = "ALL ONE"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPPAT_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmit preface length"]
    #[inline(always)]
    pub fn tplen(&self) -> TPLEN_R {
        TPLEN_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Transmit preface pattern"]
    #[inline(always)]
    pub fn tppat(&self) -> TPPAT_R {
        TPPAT_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit preface length"]
    #[inline(always)]
    #[must_use]
    pub fn tplen(&mut self) -> TPLEN_W<0> {
        TPLEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Transmit preface pattern"]
    #[inline(always)]
    #[must_use]
    pub fn tppat(&mut self) -> TPPAT_W<4> {
        TPPAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Manchester Preface Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmpr](index.html) module"]
pub struct TMPR_SPEC;
impl crate::RegisterSpec for TMPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tmpr::R](R) reader structure"]
impl crate::Readable for TMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmpr::W](W) writer structure"]
impl crate::Writable for TMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMPR to value 0"]
impl crate::Resettable for TMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
