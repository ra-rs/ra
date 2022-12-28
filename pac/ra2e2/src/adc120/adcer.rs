#[doc = "Register `ADCER` reader"]
pub struct R(crate::R<ADCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCER` writer"]
pub struct W(crate::W<ADCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCER_SPEC>;
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
impl From<crate::W<ADCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACE` reader - A/D Data Register Automatic Clearing Enable"]
pub type ACE_R = crate::BitReader<ACE_A>;
#[doc = "A/D Data Register Automatic Clearing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACE_A {
    #[doc = "0: Disable automatic clearing"]
    _0 = 0,
    #[doc = "1: Enable automatic clearing"]
    _1 = 1,
}
impl From<ACE_A> for bool {
    #[inline(always)]
    fn from(variant: ACE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACE_A {
        match self.bits {
            false => ACE_A::_0,
            true => ACE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACE_A::_1
    }
}
#[doc = "Field `ACE` writer - A/D Data Register Automatic Clearing Enable"]
pub type ACE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCER_SPEC, ACE_A, O>;
impl<'a, const O: u8> ACE_W<'a, O> {
    #[doc = "Disable automatic clearing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACE_A::_0)
    }
    #[doc = "Enable automatic clearing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACE_A::_1)
    }
}
#[doc = "Field `DIAGVAL` reader - Self-Diagnosis Conversion Voltage Select"]
pub type DIAGVAL_R = crate::FieldReader<u8, DIAGVAL_A>;
#[doc = "Self-Diagnosis Conversion Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGVAL_A {
    #[doc = "0: Setting prohibited when self-diagnosis is enabled"]
    _00 = 0,
    #[doc = "1: 0 volts"]
    _01 = 1,
    #[doc = "2: Reference voltage × 1/2"]
    _10 = 2,
    #[doc = "3: Reference voltage"]
    _11 = 3,
}
impl From<DIAGVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGVAL_A) -> Self {
        variant as _
    }
}
impl DIAGVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAGVAL_A {
        match self.bits {
            0 => DIAGVAL_A::_00,
            1 => DIAGVAL_A::_01,
            2 => DIAGVAL_A::_10,
            3 => DIAGVAL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DIAGVAL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DIAGVAL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DIAGVAL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DIAGVAL_A::_11
    }
}
#[doc = "Field `DIAGVAL` writer - Self-Diagnosis Conversion Voltage Select"]
pub type DIAGVAL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCER_SPEC, u8, DIAGVAL_A, 2, O>;
impl<'a, const O: u8> DIAGVAL_W<'a, O> {
    #[doc = "Setting prohibited when self-diagnosis is enabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_00)
    }
    #[doc = "0 volts"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_01)
    }
    #[doc = "Reference voltage × 1/2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_10)
    }
    #[doc = "Reference voltage"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DIAGVAL_A::_11)
    }
}
#[doc = "Field `DIAGLD` reader - Self-Diagnosis Mode Select"]
pub type DIAGLD_R = crate::BitReader<DIAGLD_A>;
#[doc = "Self-Diagnosis Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIAGLD_A {
    #[doc = "0: Select rotation mode for self-diagnosis voltage"]
    _0 = 0,
    #[doc = "1: Select mixed mode for self-diagnosis voltage"]
    _1 = 1,
}
impl From<DIAGLD_A> for bool {
    #[inline(always)]
    fn from(variant: DIAGLD_A) -> Self {
        variant as u8 != 0
    }
}
impl DIAGLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAGLD_A {
        match self.bits {
            false => DIAGLD_A::_0,
            true => DIAGLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIAGLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIAGLD_A::_1
    }
}
#[doc = "Field `DIAGLD` writer - Self-Diagnosis Mode Select"]
pub type DIAGLD_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCER_SPEC, DIAGLD_A, O>;
impl<'a, const O: u8> DIAGLD_W<'a, O> {
    #[doc = "Select rotation mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIAGLD_A::_0)
    }
    #[doc = "Select mixed mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIAGLD_A::_1)
    }
}
#[doc = "Field `DIAGM` reader - Self-Diagnosis Enable"]
pub type DIAGM_R = crate::BitReader<DIAGM_A>;
#[doc = "Self-Diagnosis Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIAGM_A {
    #[doc = "0: Disable ADC12 self-diagnosis"]
    _0 = 0,
    #[doc = "1: Enable ADC12 self-diagnosis"]
    _1 = 1,
}
impl From<DIAGM_A> for bool {
    #[inline(always)]
    fn from(variant: DIAGM_A) -> Self {
        variant as u8 != 0
    }
}
impl DIAGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAGM_A {
        match self.bits {
            false => DIAGM_A::_0,
            true => DIAGM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIAGM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIAGM_A::_1
    }
}
#[doc = "Field `DIAGM` writer - Self-Diagnosis Enable"]
pub type DIAGM_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCER_SPEC, DIAGM_A, O>;
impl<'a, const O: u8> DIAGM_W<'a, O> {
    #[doc = "Disable ADC12 self-diagnosis"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIAGM_A::_0)
    }
    #[doc = "Enable ADC12 self-diagnosis"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIAGM_A::_1)
    }
}
#[doc = "Field `ADRFMT` reader - A/D Data Register Format Select"]
pub type ADRFMT_R = crate::BitReader<ADRFMT_A>;
#[doc = "A/D Data Register Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRFMT_A {
    #[doc = "0: Select right-justified for the A/D data register format"]
    _0 = 0,
    #[doc = "1: Select left-justified for the A/D data register format"]
    _1 = 1,
}
impl From<ADRFMT_A> for bool {
    #[inline(always)]
    fn from(variant: ADRFMT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRFMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRFMT_A {
        match self.bits {
            false => ADRFMT_A::_0,
            true => ADRFMT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADRFMT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADRFMT_A::_1
    }
}
#[doc = "Field `ADRFMT` writer - A/D Data Register Format Select"]
pub type ADRFMT_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCER_SPEC, ADRFMT_A, O>;
impl<'a, const O: u8> ADRFMT_W<'a, O> {
    #[doc = "Select right-justified for the A/D data register format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADRFMT_A::_0)
    }
    #[doc = "Select left-justified for the A/D data register format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADRFMT_A::_1)
    }
}
impl R {
    #[doc = "Bit 5 - A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    pub fn ace(&self) -> ACE_R {
        ACE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    pub fn diagval(&self) -> DIAGVAL_R {
        DIAGVAL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Self-Diagnosis Mode Select"]
    #[inline(always)]
    pub fn diagld(&self) -> DIAGLD_R {
        DIAGLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Self-Diagnosis Enable"]
    #[inline(always)]
    pub fn diagm(&self) -> DIAGM_R {
        DIAGM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - A/D Data Register Format Select"]
    #[inline(always)]
    pub fn adrfmt(&self) -> ADRFMT_R {
        ADRFMT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ace(&mut self) -> ACE_W<5> {
        ACE_W::new(self)
    }
    #[doc = "Bits 8:9 - Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn diagval(&mut self) -> DIAGVAL_W<8> {
        DIAGVAL_W::new(self)
    }
    #[doc = "Bit 10 - Self-Diagnosis Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn diagld(&mut self) -> DIAGLD_W<10> {
        DIAGLD_W::new(self)
    }
    #[doc = "Bit 11 - Self-Diagnosis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn diagm(&mut self) -> DIAGM_W<11> {
        DIAGM_W::new(self)
    }
    #[doc = "Bit 15 - A/D Data Register Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn adrfmt(&mut self) -> ADRFMT_W<15> {
        ADRFMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Control Extended Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcer](index.html) module"]
pub struct ADCER_SPEC;
impl crate::RegisterSpec for ADCER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcer::R](R) reader structure"]
impl crate::Readable for ADCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcer::W](W) writer structure"]
impl crate::Writable for ADCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCER to value 0"]
impl crate::Resettable for ADCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
