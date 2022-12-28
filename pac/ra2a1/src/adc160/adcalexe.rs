#[doc = "Register `ADCALEXE` reader"]
pub struct R(crate::R<ADCALEXE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCALEXE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCALEXE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCALEXE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCALEXE` writer"]
pub struct W(crate::W<ADCALEXE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCALEXE_SPEC>;
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
impl From<crate::W<ADCALEXE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCALEXE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALMON` reader - Calibration Status Flag"]
pub type CALMON_R = crate::BitReader<CALMON_A>;
#[doc = "Calibration Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALMON_A {
    #[doc = "0: Calibration is not in progress"]
    _0 = 0,
    #[doc = "1: Calibration is in progress"]
    _1 = 1,
}
impl From<CALMON_A> for bool {
    #[inline(always)]
    fn from(variant: CALMON_A) -> Self {
        variant as u8 != 0
    }
}
impl CALMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALMON_A {
        match self.bits {
            false => CALMON_A::_0,
            true => CALMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALMON_A::_1
    }
}
#[doc = "Field `CALEXE` reader - Calibration Start"]
pub type CALEXE_R = crate::BitReader<CALEXE_A>;
#[doc = "Calibration Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALEXE_A {
    #[doc = "0: Calibration does not start"]
    _0 = 0,
    #[doc = "1: Calibration starts"]
    _1 = 1,
}
impl From<CALEXE_A> for bool {
    #[inline(always)]
    fn from(variant: CALEXE_A) -> Self {
        variant as u8 != 0
    }
}
impl CALEXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALEXE_A {
        match self.bits {
            false => CALEXE_A::_0,
            true => CALEXE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALEXE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALEXE_A::_1
    }
}
#[doc = "Field `CALEXE` writer - Calibration Start"]
pub type CALEXE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCALEXE_SPEC, CALEXE_A, O>;
impl<'a, const O: u8> CALEXE_W<'a, O> {
    #[doc = "Calibration does not start"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALEXE_A::_0)
    }
    #[doc = "Calibration starts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALEXE_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - Calibration Status Flag"]
    #[inline(always)]
    pub fn calmon(&self) -> CALMON_R {
        CALMON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration Start"]
    #[inline(always)]
    pub fn calexe(&self) -> CALEXE_R {
        CALEXE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn calexe(&mut self) -> CALEXE_W<7> {
        CALEXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Calibration Execution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalexe](index.html) module"]
pub struct ADCALEXE_SPEC;
impl crate::RegisterSpec for ADCALEXE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcalexe::R](R) reader structure"]
impl crate::Readable for ADCALEXE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcalexe::W](W) writer structure"]
impl crate::Writable for ADCALEXE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCALEXE to value 0"]
impl crate::Resettable for ADCALEXE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
