#[doc = "Register `AMPTRS` reader"]
pub struct R(crate::R<AMPTRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPTRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPTRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPTRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPTRS` writer"]
pub struct W(crate::W<AMPTRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPTRS_SPEC>;
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
impl From<crate::W<AMPTRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPTRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPTRS` reader - Activation Trigger SelectionNote: Do not change the value of the AMPTRS register after setting the AMPTRM register."]
pub type AMPTRS_R = crate::FieldReader<u8, AMPTRS_A>;
#[doc = "Activation Trigger SelectionNote: Do not change the value of the AMPTRS register after setting the AMPTRM register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMPTRS_A {
    #[doc = "0: OPAMPn: OPAMP activation trigger n (n = 0 to 2)"]
    _00 = 0,
    #[doc = "1: OPAMPn: OPAMP activation trigger 0 (n = 0, 1), OPAMP2: OPAMP activation trigger 1"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: OPAMPn: OPAMP activation trigger 0 (n = 0 to 2)."]
    _11 = 3,
}
impl From<AMPTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: AMPTRS_A) -> Self {
        variant as _
    }
}
impl AMPTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPTRS_A {
        match self.bits {
            0 => AMPTRS_A::_00,
            1 => AMPTRS_A::_01,
            2 => AMPTRS_A::_10,
            3 => AMPTRS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AMPTRS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AMPTRS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AMPTRS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AMPTRS_A::_11
    }
}
#[doc = "Field `AMPTRS` writer - Activation Trigger SelectionNote: Do not change the value of the AMPTRS register after setting the AMPTRM register."]
pub type AMPTRS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, AMPTRS_SPEC, u8, AMPTRS_A, 2, O>;
impl<'a, const O: u8> AMPTRS_W<'a, O> {
    #[doc = "OPAMPn: OPAMP activation trigger n (n = 0 to 2)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AMPTRS_A::_00)
    }
    #[doc = "OPAMPn: OPAMP activation trigger 0 (n = 0, 1), OPAMP2: OPAMP activation trigger 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AMPTRS_A::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AMPTRS_A::_10)
    }
    #[doc = "OPAMPn: OPAMP activation trigger 0 (n = 0 to 2)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AMPTRS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Activation Trigger SelectionNote: Do not change the value of the AMPTRS register after setting the AMPTRM register."]
    #[inline(always)]
    pub fn amptrs(&self) -> AMPTRS_R {
        AMPTRS_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Activation Trigger SelectionNote: Do not change the value of the AMPTRS register after setting the AMPTRM register."]
    #[inline(always)]
    #[must_use]
    pub fn amptrs(&mut self) -> AMPTRS_W<0> {
        AMPTRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Activation Trigger Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amptrs](index.html) module"]
pub struct AMPTRS_SPEC;
impl crate::RegisterSpec for AMPTRS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amptrs::R](R) reader structure"]
impl crate::Readable for AMPTRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amptrs::W](W) writer structure"]
impl crate::Writable for AMPTRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPTRS to value 0"]
impl crate::Resettable for AMPTRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
