#[doc = "Register `ECCMODE` reader"]
pub struct R(crate::R<ECCMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCMODE` writer"]
pub struct W(crate::W<ECCMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCMODE_SPEC>;
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
impl From<crate::W<ECCMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCMOD` reader - ECC Operating Mode Select"]
pub type ECCMOD_R = crate::FieldReader<u8, ECCMOD_A>;
#[doc = "ECC Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECCMOD_A {
    #[doc = "0: Disable ECC function"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Enable ECC function without error checking"]
    _10 = 2,
    #[doc = "3: Enable ECC function with error checking."]
    _11 = 3,
}
impl From<ECCMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCMOD_A) -> Self {
        variant as _
    }
}
impl ECCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCMOD_A {
        match self.bits {
            0 => ECCMOD_A::_00,
            1 => ECCMOD_A::_01,
            2 => ECCMOD_A::_10,
            3 => ECCMOD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ECCMOD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ECCMOD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ECCMOD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ECCMOD_A::_11
    }
}
#[doc = "Field `ECCMOD` writer - ECC Operating Mode Select"]
pub type ECCMOD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, ECCMODE_SPEC, u8, ECCMOD_A, 2, O>;
impl<'a, const O: u8> ECCMOD_W<'a, O> {
    #[doc = "Disable ECC function"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ECCMOD_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ECCMOD_A::_01)
    }
    #[doc = "Enable ECC function without error checking"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ECCMOD_A::_10)
    }
    #[doc = "Enable ECC function with error checking."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ECCMOD_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(&self) -> ECCMOD_R {
        ECCMOD_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - ECC Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn eccmod(&mut self) -> ECCMOD_W<0> {
        ECCMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECCRAM Operating Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccmode](index.html) module"]
pub struct ECCMODE_SPEC;
impl crate::RegisterSpec for ECCMODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eccmode::R](R) reader structure"]
impl crate::Readable for ECCMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccmode::W](W) writer structure"]
impl crate::Writable for ECCMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCMODE to value 0"]
impl crate::Resettable for ECCMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
