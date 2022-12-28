#[doc = "Register `SOMRG` reader"]
pub struct R(crate::R<SOMRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOMRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOMRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOMRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOMRG` writer"]
pub struct W(crate::W<SOMRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOMRG_SPEC>;
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
impl From<crate::W<SOMRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOMRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOSCMRG` reader - Sub Clock Oscillator Margin check Switching"]
pub type SOSCMRG_R = crate::FieldReader<u8, SOSCMRG_A>;
#[doc = "Sub Clock Oscillator Margin check Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOSCMRG_A {
    #[doc = "0: Normal Current"]
    _00 = 0,
    #[doc = "1: Lower Margin check"]
    _01 = 1,
    #[doc = "2: Upper Margin check"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<SOSCMRG_A> for u8 {
    #[inline(always)]
    fn from(variant: SOSCMRG_A) -> Self {
        variant as _
    }
}
impl SOSCMRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCMRG_A {
        match self.bits {
            0 => SOSCMRG_A::_00,
            1 => SOSCMRG_A::_01,
            2 => SOSCMRG_A::_10,
            3 => SOSCMRG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SOSCMRG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SOSCMRG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SOSCMRG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SOSCMRG_A::_11
    }
}
#[doc = "Field `SOSCMRG` writer - Sub Clock Oscillator Margin check Switching"]
pub type SOSCMRG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SOMRG_SPEC, u8, SOSCMRG_A, 2, O>;
impl<'a, const O: u8> SOSCMRG_W<'a, O> {
    #[doc = "Normal Current"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SOSCMRG_A::_00)
    }
    #[doc = "Lower Margin check"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SOSCMRG_A::_01)
    }
    #[doc = "Upper Margin check"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SOSCMRG_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SOSCMRG_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sub Clock Oscillator Margin check Switching"]
    #[inline(always)]
    pub fn soscmrg(&self) -> SOSCMRG_R {
        SOSCMRG_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sub Clock Oscillator Margin check Switching"]
    #[inline(always)]
    #[must_use]
    pub fn soscmrg(&mut self) -> SOSCMRG_W<0> {
        SOSCMRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Clock Oscillator Margin Check Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [somrg](index.html) module"]
pub struct SOMRG_SPEC;
impl crate::RegisterSpec for SOMRG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [somrg::R](R) reader structure"]
impl crate::Readable for SOMRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [somrg::W](W) writer structure"]
impl crate::Writable for SOMRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOMRG to value 0"]
impl crate::Resettable for SOMRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
