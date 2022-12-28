#[doc = "Register `RMPR` reader"]
pub struct R(crate::R<RMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMPR` writer"]
pub struct W(crate::W<RMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMPR_SPEC>;
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
impl From<crate::W<RMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPLEN` reader - Receive Preface Length"]
pub type RPLEN_R = crate::FieldReader<u8, RPLEN_A>;
#[doc = "Receive Preface Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPLEN_A {
    #[doc = "0: Disables the receive preface generation"]
    _0 = 0,
}
impl From<RPLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RPLEN_A) -> Self {
        variant as _
    }
}
impl RPLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RPLEN_A> {
        match self.bits {
            0 => Some(RPLEN_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPLEN_A::_0
    }
}
#[doc = "Field `RPLEN` writer - Receive Preface Length"]
pub type RPLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RMPR_SPEC, u8, RPLEN_A, 4, O>;
impl<'a, const O: u8> RPLEN_W<'a, O> {
    #[doc = "Disables the receive preface generation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPLEN_A::_0)
    }
}
#[doc = "Field `RPPAT` reader - Receive Preface Pattern"]
pub type RPPAT_R = crate::FieldReader<u8, RPPAT_A>;
#[doc = "Receive Preface Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPPAT_A {
    #[doc = "0: ALL ZERO"]
    _00 = 0,
    #[doc = "1: ZERO ONE"]
    _01 = 1,
    #[doc = "2: ONE ZERO"]
    _10 = 2,
    #[doc = "3: ALL ONE"]
    _11 = 3,
}
impl From<RPPAT_A> for u8 {
    #[inline(always)]
    fn from(variant: RPPAT_A) -> Self {
        variant as _
    }
}
impl RPPAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPPAT_A {
        match self.bits {
            0 => RPPAT_A::_00,
            1 => RPPAT_A::_01,
            2 => RPPAT_A::_10,
            3 => RPPAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RPPAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RPPAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RPPAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RPPAT_A::_11
    }
}
#[doc = "Field `RPPAT` writer - Receive Preface Pattern"]
pub type RPPAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RMPR_SPEC, u8, RPPAT_A, 2, O>;
impl<'a, const O: u8> RPPAT_W<'a, O> {
    #[doc = "ALL ZERO"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RPPAT_A::_00)
    }
    #[doc = "ZERO ONE"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RPPAT_A::_01)
    }
    #[doc = "ONE ZERO"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RPPAT_A::_10)
    }
    #[doc = "ALL ONE"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RPPAT_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive Preface Length"]
    #[inline(always)]
    pub fn rplen(&self) -> RPLEN_R {
        RPLEN_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Receive Preface Pattern"]
    #[inline(always)]
    pub fn rppat(&self) -> RPPAT_R {
        RPPAT_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Preface Length"]
    #[inline(always)]
    #[must_use]
    pub fn rplen(&mut self) -> RPLEN_W<0> {
        RPLEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Receive Preface Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn rppat(&mut self) -> RPPAT_W<4> {
        RPPAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Manchester Preface Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmpr](index.html) module"]
pub struct RMPR_SPEC;
impl crate::RegisterSpec for RMPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rmpr::R](R) reader structure"]
impl crate::Readable for RMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmpr::W](W) writer structure"]
impl crate::Writable for RMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMPR to value 0"]
impl crate::Resettable for RMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
