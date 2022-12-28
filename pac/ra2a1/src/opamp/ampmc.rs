#[doc = "Register `AMPMC` reader"]
pub struct R(crate::R<AMPMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPMC` writer"]
pub struct W(crate::W<AMPMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPMC_SPEC>;
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
impl From<crate::W<AMPMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPSP` reader - OPAMP Operation mode selection"]
pub type AMPSP_R = crate::FieldReader<u8, AMPSP_A>;
#[doc = "OPAMP Operation mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMPSP_A {
    #[doc = "0: Low-power mode (Low-speed)"]
    _00 = 0,
    #[doc = "2: Low-power mode (Low-speed)"]
    _10 = 2,
    #[doc = "1: Middle-speed mode"]
    _01 = 1,
    #[doc = "3: High-speed mode"]
    _11 = 3,
}
impl From<AMPSP_A> for u8 {
    #[inline(always)]
    fn from(variant: AMPSP_A) -> Self {
        variant as _
    }
}
impl AMPSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPSP_A {
        match self.bits {
            0 => AMPSP_A::_00,
            2 => AMPSP_A::_10,
            1 => AMPSP_A::_01,
            3 => AMPSP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AMPSP_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AMPSP_A::_10
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AMPSP_A::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AMPSP_A::_11
    }
}
#[doc = "Field `AMPSP` writer - OPAMP Operation mode selection"]
pub type AMPSP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, AMPMC_SPEC, u8, AMPSP_A, 2, O>;
impl<'a, const O: u8> AMPSP_W<'a, O> {
    #[doc = "Low-power mode (Low-speed)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AMPSP_A::_00)
    }
    #[doc = "Low-power mode (Low-speed)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AMPSP_A::_10)
    }
    #[doc = "Middle-speed mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AMPSP_A::_01)
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AMPSP_A::_11)
    }
}
impl R {
    #[doc = "Bits 6:7 - OPAMP Operation mode selection"]
    #[inline(always)]
    pub fn ampsp(&self) -> AMPSP_R {
        AMPSP_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 6:7 - OPAMP Operation mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ampsp(&mut self) -> AMPSP_W<6> {
        AMPSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational amplifier mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampmc](index.html) module"]
pub struct AMPMC_SPEC;
impl crate::RegisterSpec for AMPMC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ampmc::R](R) reader structure"]
impl crate::Readable for AMPMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampmc::W](W) writer structure"]
impl crate::Writable for AMPMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPMC to value 0"]
impl crate::Resettable for AMPMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
