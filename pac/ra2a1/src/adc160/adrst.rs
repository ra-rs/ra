#[doc = "Register `ADRST` reader"]
pub struct R(crate::R<ADRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADRST` writer"]
pub struct W(crate::W<ADRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADRST_SPEC>;
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
impl From<crate::W<ADRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAGST` reader - Self-Diagnosis Status"]
pub type DIAGST_R = crate::FieldReader<u8, DIAGST_A>;
#[doc = "Self-Diagnosis Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGST_A {
    #[doc = "0: Self-diagnosis has not been executed since power-on"]
    _00 = 0,
    #[doc = "1: Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 8000h"]
    _01 = 1,
    #[doc = "2: Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 0000h"]
    _10 = 2,
    #[doc = "3: Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 7FFFh."]
    _11 = 3,
}
impl From<DIAGST_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGST_A) -> Self {
        variant as _
    }
}
impl DIAGST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAGST_A {
        match self.bits {
            0 => DIAGST_A::_00,
            1 => DIAGST_A::_01,
            2 => DIAGST_A::_10,
            3 => DIAGST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DIAGST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DIAGST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DIAGST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DIAGST_A::_11
    }
}
#[doc = "Field `DIAGST` writer - Self-Diagnosis Status"]
pub type DIAGST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADRST_SPEC, u8, DIAGST_A, 2, O>;
impl<'a, const O: u8> DIAGST_W<'a, O> {
    #[doc = "Self-diagnosis has not been executed since power-on"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DIAGST_A::_00)
    }
    #[doc = "Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 8000h"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DIAGST_A::_01)
    }
    #[doc = "Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 0000h"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DIAGST_A::_10)
    }
    #[doc = "Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 7FFFh."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DIAGST_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Self-Diagnosis Status"]
    #[inline(always)]
    pub fn diagst(&self) -> DIAGST_R {
        DIAGST_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Self-Diagnosis Status"]
    #[inline(always)]
    #[must_use]
    pub fn diagst(&mut self) -> DIAGST_W<0> {
        DIAGST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Self-Diagnostic Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adrst](index.html) module"]
pub struct ADRST_SPEC;
impl crate::RegisterSpec for ADRST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adrst::R](R) reader structure"]
impl crate::Readable for ADRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adrst::W](W) writer structure"]
impl crate::Writable for ADRST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADRST to value 0"]
impl crate::Resettable for ADRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
