#[doc = "Register `ACTR` reader"]
pub struct R(crate::R<ACTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTR` writer"]
pub struct W(crate::W<ACTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTR_SPEC>;
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
impl From<crate::W<ACTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AST` reader - Adjustment value for receive Sampling Timing"]
pub type AST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AST` writer - Adjustment value for receive Sampling Timing"]
pub type AST_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ACTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AJD` reader - Adjustment Direction for receive sampling timing"]
pub type AJD_R = crate::BitReader<AJD_A>;
#[doc = "Adjustment Direction for receive sampling timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AJD_A {
    #[doc = "0: The sampling timing is adjusted backward to the middle of bit."]
    _0 = 0,
    #[doc = "1: The sampling timing is adjusted forward to the middle of bit."]
    _1 = 1,
}
impl From<AJD_A> for bool {
    #[inline(always)]
    fn from(variant: AJD_A) -> Self {
        variant as u8 != 0
    }
}
impl AJD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AJD_A {
        match self.bits {
            false => AJD_A::_0,
            true => AJD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AJD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AJD_A::_1
    }
}
#[doc = "Field `AJD` writer - Adjustment Direction for receive sampling timing"]
pub type AJD_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACTR_SPEC, AJD_A, O>;
impl<'a, const O: u8> AJD_W<'a, O> {
    #[doc = "The sampling timing is adjusted backward to the middle of bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AJD_A::_0)
    }
    #[doc = "The sampling timing is adjusted forward to the middle of bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AJD_A::_1)
    }
}
#[doc = "Field `ATT` reader - Adjustment value for Transmit timing"]
pub type ATT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATT` writer - Adjustment value for Transmit timing"]
pub type ATT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ACTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AET` reader - Adjustment edge for transmit timing"]
pub type AET_R = crate::BitReader<AET_A>;
#[doc = "Adjustment edge for transmit timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AET_A {
    #[doc = "0: Adjust the rising edge timing."]
    _0 = 0,
    #[doc = "1: Adjust the falling edge timing."]
    _1 = 1,
}
impl From<AET_A> for bool {
    #[inline(always)]
    fn from(variant: AET_A) -> Self {
        variant as u8 != 0
    }
}
impl AET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AET_A {
        match self.bits {
            false => AET_A::_0,
            true => AET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AET_A::_1
    }
}
#[doc = "Field `AET` writer - Adjustment edge for transmit timing"]
pub type AET_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACTR_SPEC, AET_A, O>;
impl<'a, const O: u8> AET_W<'a, O> {
    #[doc = "Adjust the rising edge timing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AET_A::_0)
    }
    #[doc = "Adjust the falling edge timing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AET_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Adjustment value for receive Sampling Timing"]
    #[inline(always)]
    pub fn ast(&self) -> AST_R {
        AST_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Adjustment Direction for receive sampling timing"]
    #[inline(always)]
    pub fn ajd(&self) -> AJD_R {
        AJD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Adjustment value for Transmit timing"]
    #[inline(always)]
    pub fn att(&self) -> ATT_R {
        ATT_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Adjustment edge for transmit timing"]
    #[inline(always)]
    pub fn aet(&self) -> AET_R {
        AET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Adjustment value for receive Sampling Timing"]
    #[inline(always)]
    #[must_use]
    pub fn ast(&mut self) -> AST_W<0> {
        AST_W::new(self)
    }
    #[doc = "Bit 3 - Adjustment Direction for receive sampling timing"]
    #[inline(always)]
    #[must_use]
    pub fn ajd(&mut self) -> AJD_W<3> {
        AJD_W::new(self)
    }
    #[doc = "Bits 4:6 - Adjustment value for Transmit timing"]
    #[inline(always)]
    #[must_use]
    pub fn att(&mut self) -> ATT_W<4> {
        ATT_W::new(self)
    }
    #[doc = "Bit 7 - Adjustment edge for transmit timing"]
    #[inline(always)]
    #[must_use]
    pub fn aet(&mut self) -> AET_W<7> {
        AET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Adjustment Communication Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actr](index.html) module"]
pub struct ACTR_SPEC;
impl crate::RegisterSpec for ACTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [actr::R](R) reader structure"]
impl crate::Readable for ACTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actr::W](W) writer structure"]
impl crate::Writable for ACTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTR to value 0"]
impl crate::Resettable for ACTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
