#[doc = "Register `MMPUSARA` reader"]
pub struct R(crate::R<MMPUSARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUSARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUSARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUSARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUSARA` writer"]
pub struct W(crate::W<MMPUSARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUSARA_SPEC>;
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
impl From<crate::W<MMPUSARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUSARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUASAn` reader - MMPUA Security Attribution (n = 0 to 7)"]
pub type MMPUASAN_R = crate::FieldReader<u8, MMPUASAN_A>;
#[doc = "MMPUA Security Attribution (n = 0 to 7)\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMPUASAN_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<MMPUASAN_A> for u8 {
    #[inline(always)]
    fn from(variant: MMPUASAN_A) -> Self {
        variant as _
    }
}
impl MMPUASAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMPUASAN_A> {
        match self.bits {
            0 => Some(MMPUASAN_A::_0),
            1 => Some(MMPUASAN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MMPUASAN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MMPUASAN_A::_1
    }
}
#[doc = "Field `MMPUASAn` writer - MMPUA Security Attribution (n = 0 to 7)"]
pub type MMPUASAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MMPUSARA_SPEC, u8, MMPUASAN_A, 8, O>;
impl<'a, const O: u8> MMPUASAN_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MMPUASAN_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MMPUASAN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - MMPUA Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasan(&self) -> MMPUASAN_R {
        MMPUASAN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MMPUA Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn mmpuasan(&mut self) -> MMPUASAN_W<0> {
        MMPUASAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Memory Protection Unit Security Attribution Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpusara](index.html) module"]
pub struct MMPUSARA_SPEC;
impl crate::RegisterSpec for MMPUSARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpusara::R](R) reader structure"]
impl crate::Readable for MMPUSARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpusara::W](W) writer structure"]
impl crate::Writable for MMPUSARA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSARA to value 0xffff_ffff"]
impl crate::Resettable for MMPUSARA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
