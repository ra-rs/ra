#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTE` reader - CAN Test Mode Enable"]
pub type TSTE_R = crate::BitReader<TSTE_A>;
#[doc = "CAN Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTE_A {
    #[doc = "0: CAN test mode disabled"]
    _0 = 0,
    #[doc = "1: CAN test mode enabled"]
    _1 = 1,
}
impl From<TSTE_A> for bool {
    #[inline(always)]
    fn from(variant: TSTE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTE_A {
        match self.bits {
            false => TSTE_A::_0,
            true => TSTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTE_A::_1
    }
}
#[doc = "Field `TSTE` writer - CAN Test Mode Enable"]
pub type TSTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCR_SPEC, TSTE_A, O>;
impl<'a, const O: u8> TSTE_W<'a, O> {
    #[doc = "CAN test mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTE_A::_0)
    }
    #[doc = "CAN test mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTE_A::_1)
    }
}
#[doc = "Field `TSTM` reader - CAN Test Mode Select"]
pub type TSTM_R = crate::FieldReader<u8, TSTM_A>;
#[doc = "CAN Test Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTM_A {
    #[doc = "0: Other than CAN test mode"]
    _00 = 0,
    #[doc = "1: Listen-only mode"]
    _01 = 1,
    #[doc = "2: Self-test mode 0 (external loopback)"]
    _10 = 2,
    #[doc = "3: Self-test mode 1 (internal loopback)"]
    _11 = 3,
}
impl From<TSTM_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTM_A) -> Self {
        variant as _
    }
}
impl TSTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTM_A {
        match self.bits {
            0 => TSTM_A::_00,
            1 => TSTM_A::_01,
            2 => TSTM_A::_10,
            3 => TSTM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TSTM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TSTM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TSTM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TSTM_A::_11
    }
}
#[doc = "Field `TSTM` writer - CAN Test Mode Select"]
pub type TSTM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCR_SPEC, u8, TSTM_A, 2, O>;
impl<'a, const O: u8> TSTM_W<'a, O> {
    #[doc = "Other than CAN test mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TSTM_A::_00)
    }
    #[doc = "Listen-only mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TSTM_A::_01)
    }
    #[doc = "Self-test mode 0 (external loopback)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TSTM_A::_10)
    }
    #[doc = "Self-test mode 1 (internal loopback)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TSTM_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Test Mode Enable"]
    #[inline(always)]
    pub fn tste(&self) -> TSTE_R {
        TSTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - CAN Test Mode Select"]
    #[inline(always)]
    pub fn tstm(&self) -> TSTM_R {
        TSTM_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tste(&mut self) -> TSTE_W<0> {
        TSTE_W::new(self)
    }
    #[doc = "Bits 1:2 - CAN Test Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tstm(&mut self) -> TSTM_W<1> {
        TSTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
