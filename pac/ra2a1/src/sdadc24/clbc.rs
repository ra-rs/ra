#[doc = "Register `CLBC` reader"]
pub struct R(crate::R<CLBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLBC` writer"]
pub struct W(crate::W<CLBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLBC_SPEC>;
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
impl From<crate::W<CLBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLBMD` reader - These bits are read as 0. The write value should be 0."]
pub type CLBMD_R = crate::FieldReader<u8, CLBMD_A>;
#[doc = "These bits are read as 0. The write value should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLBMD_A {
    #[doc = "0: Internal calibration mode"]
    _00 = 0,
    #[doc = "1: External offset calibration mode"]
    _01 = 1,
    #[doc = "2: External gain calibration mode"]
    _10 = 2,
    #[doc = "3: Settings are prohibited"]
    _11 = 3,
}
impl From<CLBMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CLBMD_A) -> Self {
        variant as _
    }
}
impl CLBMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLBMD_A {
        match self.bits {
            0 => CLBMD_A::_00,
            1 => CLBMD_A::_01,
            2 => CLBMD_A::_10,
            3 => CLBMD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLBMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLBMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLBMD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLBMD_A::_11
    }
}
#[doc = "Field `CLBMD` writer - These bits are read as 0. The write value should be 0."]
pub type CLBMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLBC_SPEC, u8, CLBMD_A, 2, O>;
impl<'a, const O: u8> CLBMD_W<'a, O> {
    #[doc = "Internal calibration mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLBMD_A::_00)
    }
    #[doc = "External offset calibration mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLBMD_A::_01)
    }
    #[doc = "External gain calibration mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLBMD_A::_10)
    }
    #[doc = "Settings are prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLBMD_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - These bits are read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn clbmd(&self) -> CLBMD_R {
        CLBMD_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits are read as 0. The write value should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn clbmd(&mut self) -> CLBMD_W<0> {
        CLBMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clbc](index.html) module"]
pub struct CLBC_SPEC;
impl crate::RegisterSpec for CLBC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clbc::R](R) reader structure"]
impl crate::Readable for CLBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clbc::W](W) writer structure"]
impl crate::Writable for CLBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLBC to value 0"]
impl crate::Resettable for CLBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
