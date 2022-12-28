#[doc = "Register `SCSTRCTL` reader"]
pub struct R(crate::R<SCSTRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSTRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSTRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSTRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSTRCTL` writer"]
pub struct W(crate::W<SCSTRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSTRCTL_SPEC>;
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
impl From<crate::W<SCSTRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSTRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKTWE` reader - Acknowledge Transmission Wait Enable"]
pub type ACKTWE_R = crate::BitReader<ACKTWE_A>;
#[doc = "Acknowledge Transmission Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKTWE_A {
    #[doc = "0: NTST.RDBFF0 is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
    _0 = 0,
    #[doc = "1: NTST.RDBFF0 is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.) Low-hold is released by writing a value to the ACKCTL.ACKT bit."]
    _1 = 1,
}
impl From<ACKTWE_A> for bool {
    #[inline(always)]
    fn from(variant: ACKTWE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKTWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKTWE_A {
        match self.bits {
            false => ACKTWE_A::_0,
            true => ACKTWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKTWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKTWE_A::_1
    }
}
#[doc = "Field `ACKTWE` writer - Acknowledge Transmission Wait Enable"]
pub type ACKTWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSTRCTL_SPEC, ACKTWE_A, O>;
impl<'a, const O: u8> ACKTWE_W<'a, O> {
    #[doc = "NTST.RDBFF0 is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKTWE_A::_0)
    }
    #[doc = "NTST.RDBFF0 is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.) Low-hold is released by writing a value to the ACKCTL.ACKT bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKTWE_A::_1)
    }
}
#[doc = "Field `RWE` reader - Receive Wait Enable"]
pub type RWE_R = crate::BitReader<RWE_A>;
#[doc = "Receive Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWE_A {
    #[doc = "0: No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
    _0 = 0,
    #[doc = "1: WAIT (The period between ninth clock cycle and first clock cycle is held low.) Low-hold is released by reading NTDTBP0."]
    _1 = 1,
}
impl From<RWE_A> for bool {
    #[inline(always)]
    fn from(variant: RWE_A) -> Self {
        variant as u8 != 0
    }
}
impl RWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWE_A {
        match self.bits {
            false => RWE_A::_0,
            true => RWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWE_A::_1
    }
}
#[doc = "Field `RWE` writer - Receive Wait Enable"]
pub type RWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSTRCTL_SPEC, RWE_A, O>;
impl<'a, const O: u8> RWE_W<'a, O> {
    #[doc = "No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWE_A::_0)
    }
    #[doc = "WAIT (The period between ninth clock cycle and first clock cycle is held low.) Low-hold is released by reading NTDTBP0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge Transmission Wait Enable"]
    #[inline(always)]
    pub fn acktwe(&self) -> ACKTWE_R {
        ACKTWE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Wait Enable"]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge Transmission Wait Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acktwe(&mut self) -> ACKTWE_W<0> {
        ACKTWE_W::new(self)
    }
    #[doc = "Bit 1 - Receive Wait Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwe(&mut self) -> RWE_W<1> {
        RWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCL Stretch Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scstrctl](index.html) module"]
pub struct SCSTRCTL_SPEC;
impl crate::RegisterSpec for SCSTRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scstrctl::R](R) reader structure"]
impl crate::Readable for SCSTRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scstrctl::W](W) writer structure"]
impl crate::Writable for SCSTRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCSTRCTL to value 0"]
impl crate::Resettable for SCSTRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
