#[doc = "Register `DAAMPCR` reader"]
pub struct R(crate::R<DAAMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAAMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAAMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAAMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAAMPCR` writer"]
pub struct W(crate::W<DAAMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAAMPCR_SPEC>;
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
impl From<crate::W<DAAMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAAMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAAMP0` reader - Amplifier Control 0"]
pub type DAAMP0_R = crate::BitReader<DAAMP0_A>;
#[doc = "Amplifier Control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAAMP0_A {
    #[doc = "0: Do not use channel 0 output amplifier"]
    _0 = 0,
    #[doc = "1: Use channel 0 output amplifier"]
    _1 = 1,
}
impl From<DAAMP0_A> for bool {
    #[inline(always)]
    fn from(variant: DAAMP0_A) -> Self {
        variant as u8 != 0
    }
}
impl DAAMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAAMP0_A {
        match self.bits {
            false => DAAMP0_A::_0,
            true => DAAMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAAMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAAMP0_A::_1
    }
}
#[doc = "Field `DAAMP0` writer - Amplifier Control 0"]
pub type DAAMP0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAAMPCR_SPEC, DAAMP0_A, O>;
impl<'a, const O: u8> DAAMP0_W<'a, O> {
    #[doc = "Do not use channel 0 output amplifier"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAAMP0_A::_0)
    }
    #[doc = "Use channel 0 output amplifier"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAAMP0_A::_1)
    }
}
#[doc = "Field `DAAMP1` reader - Amplifier Control 1"]
pub type DAAMP1_R = crate::BitReader<DAAMP1_A>;
#[doc = "Amplifier Control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAAMP1_A {
    #[doc = "0: Do not use channel 1 output amplifier"]
    _0 = 0,
    #[doc = "1: Use channel 1 output amplifier"]
    _1 = 1,
}
impl From<DAAMP1_A> for bool {
    #[inline(always)]
    fn from(variant: DAAMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl DAAMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAAMP1_A {
        match self.bits {
            false => DAAMP1_A::_0,
            true => DAAMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAAMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAAMP1_A::_1
    }
}
#[doc = "Field `DAAMP1` writer - Amplifier Control 1"]
pub type DAAMP1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAAMPCR_SPEC, DAAMP1_A, O>;
impl<'a, const O: u8> DAAMP1_W<'a, O> {
    #[doc = "Do not use channel 1 output amplifier"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAAMP1_A::_0)
    }
    #[doc = "Use channel 1 output amplifier"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAAMP1_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - Amplifier Control 0"]
    #[inline(always)]
    pub fn daamp0(&self) -> DAAMP0_R {
        DAAMP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Amplifier Control 1"]
    #[inline(always)]
    pub fn daamp1(&self) -> DAAMP1_R {
        DAAMP1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Amplifier Control 0"]
    #[inline(always)]
    #[must_use]
    pub fn daamp0(&mut self) -> DAAMP0_W<6> {
        DAAMP0_W::new(self)
    }
    #[doc = "Bit 7 - Amplifier Control 1"]
    #[inline(always)]
    #[must_use]
    pub fn daamp1(&mut self) -> DAAMP1_W<7> {
        DAAMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A Output Amplifier Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daampcr](index.html) module"]
pub struct DAAMPCR_SPEC;
impl crate::RegisterSpec for DAAMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [daampcr::R](R) reader structure"]
impl crate::Readable for DAAMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daampcr::W](W) writer structure"]
impl crate::Writable for DAAMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAAMPCR to value 0"]
impl crate::Resettable for DAAMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
