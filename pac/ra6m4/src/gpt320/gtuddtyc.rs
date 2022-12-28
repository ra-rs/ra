#[doc = "Register `GTUDDTYC` reader"]
pub struct R(crate::R<GTUDDTYC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUDDTYC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUDDTYC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUDDTYC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUDDTYC` writer"]
pub struct W(crate::W<GTUDDTYC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUDDTYC_SPEC>;
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
impl From<crate::W<GTUDDTYC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUDDTYC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UD` reader - Count Direction Setting"]
pub type UD_R = crate::BitReader<UD_A>;
#[doc = "Count Direction Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UD_A {
    #[doc = "0: GTCNT counts down"]
    _0 = 0,
    #[doc = "1: GTCNT counts up"]
    _1 = 1,
}
impl From<UD_A> for bool {
    #[inline(always)]
    fn from(variant: UD_A) -> Self {
        variant as u8 != 0
    }
}
impl UD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UD_A {
        match self.bits {
            false => UD_A::_0,
            true => UD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UD_A::_1
    }
}
#[doc = "Field `UD` writer - Count Direction Setting"]
pub type UD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUDDTYC_SPEC, UD_A, O>;
impl<'a, const O: u8> UD_W<'a, O> {
    #[doc = "GTCNT counts down"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UD_A::_0)
    }
    #[doc = "GTCNT counts up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UD_A::_1)
    }
}
#[doc = "Field `UDF` reader - Forcible Count Direction Setting"]
pub type UDF_R = crate::BitReader<UDF_A>;
#[doc = "Forcible Count Direction Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDF_A {
    #[doc = "0: Not forcibly set"]
    _0 = 0,
    #[doc = "1: Forcibly set"]
    _1 = 1,
}
impl From<UDF_A> for bool {
    #[inline(always)]
    fn from(variant: UDF_A) -> Self {
        variant as u8 != 0
    }
}
impl UDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDF_A {
        match self.bits {
            false => UDF_A::_0,
            true => UDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDF_A::_1
    }
}
#[doc = "Field `UDF` writer - Forcible Count Direction Setting"]
pub type UDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUDDTYC_SPEC, UDF_A, O>;
impl<'a, const O: u8> UDF_W<'a, O> {
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UDF_A::_0)
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UDF_A::_1)
    }
}
#[doc = "Field `OADTY` reader - GTIOCnA Output Duty Setting"]
pub type OADTY_R = crate::FieldReader<u8, OADTY_A>;
#[doc = "GTIOCnA Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OADTY_A {
    #[doc = "0: GTIOCnA pin duty depends on the compare match"]
    _00 = 0,
    #[doc = "1: GTIOCnA pin duty depends on the compare match"]
    _01 = 1,
    #[doc = "2: GTIOCnA pin duty 0%"]
    _10 = 2,
    #[doc = "3: GTIOCnA pin duty 100%"]
    _11 = 3,
}
impl From<OADTY_A> for u8 {
    #[inline(always)]
    fn from(variant: OADTY_A) -> Self {
        variant as _
    }
}
impl OADTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OADTY_A {
        match self.bits {
            0 => OADTY_A::_00,
            1 => OADTY_A::_01,
            2 => OADTY_A::_10,
            3 => OADTY_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OADTY_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OADTY_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OADTY_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OADTY_A::_11
    }
}
#[doc = "Field `OADTY` writer - GTIOCnA Output Duty Setting"]
pub type OADTY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTUDDTYC_SPEC, u8, OADTY_A, 2, O>;
impl<'a, const O: u8> OADTY_W<'a, O> {
    #[doc = "GTIOCnA pin duty depends on the compare match"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OADTY_A::_00)
    }
    #[doc = "GTIOCnA pin duty depends on the compare match"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OADTY_A::_01)
    }
    #[doc = "GTIOCnA pin duty 0%"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OADTY_A::_10)
    }
    #[doc = "GTIOCnA pin duty 100%"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OADTY_A::_11)
    }
}
#[doc = "Field `OADTYF` reader - Forcible GTIOCnA Output Duty Setting"]
pub type OADTYF_R = crate::BitReader<OADTYF_A>;
#[doc = "Forcible GTIOCnA Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OADTYF_A {
    #[doc = "0: Not forcibly set"]
    _0 = 0,
    #[doc = "1: Forcibly set"]
    _1 = 1,
}
impl From<OADTYF_A> for bool {
    #[inline(always)]
    fn from(variant: OADTYF_A) -> Self {
        variant as u8 != 0
    }
}
impl OADTYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OADTYF_A {
        match self.bits {
            false => OADTYF_A::_0,
            true => OADTYF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OADTYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OADTYF_A::_1
    }
}
#[doc = "Field `OADTYF` writer - Forcible GTIOCnA Output Duty Setting"]
pub type OADTYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUDDTYC_SPEC, OADTYF_A, O>;
impl<'a, const O: u8> OADTYF_W<'a, O> {
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OADTYF_A::_0)
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OADTYF_A::_1)
    }
}
#[doc = "Field `OADTYR` reader - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting"]
pub type OADTYR_R = crate::BitReader<OADTYR_A>;
#[doc = "GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OADTYR_A {
    #[doc = "0: The function selected by the GTIOA\\[3:2\\]
bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting."]
    _0 = 0,
    #[doc = "1: The function selected by the GTIOA\\[3:2\\]
bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting."]
    _1 = 1,
}
impl From<OADTYR_A> for bool {
    #[inline(always)]
    fn from(variant: OADTYR_A) -> Self {
        variant as u8 != 0
    }
}
impl OADTYR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OADTYR_A {
        match self.bits {
            false => OADTYR_A::_0,
            true => OADTYR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OADTYR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OADTYR_A::_1
    }
}
#[doc = "Field `OADTYR` writer - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting"]
pub type OADTYR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUDDTYC_SPEC, OADTYR_A, O>;
impl<'a, const O: u8> OADTYR_W<'a, O> {
    #[doc = "The function selected by the GTIOA\\[3:2\\]
bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OADTYR_A::_0)
    }
    #[doc = "The function selected by the GTIOA\\[3:2\\]
bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OADTYR_A::_1)
    }
}
#[doc = "Field `OBDTY` reader - GTIOCnB Output Duty Setting"]
pub type OBDTY_R = crate::FieldReader<u8, OBDTY_A>;
#[doc = "GTIOCnB Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OBDTY_A {
    #[doc = "0: GTIOCnB pin duty depends on the compare match"]
    _00 = 0,
    #[doc = "1: GTIOCnB pin duty depends on the compare match"]
    _01 = 1,
    #[doc = "2: GTIOCnB pin duty 0%"]
    _10 = 2,
    #[doc = "3: GTIOCnB pin duty 100%"]
    _11 = 3,
}
impl From<OBDTY_A> for u8 {
    #[inline(always)]
    fn from(variant: OBDTY_A) -> Self {
        variant as _
    }
}
impl OBDTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBDTY_A {
        match self.bits {
            0 => OBDTY_A::_00,
            1 => OBDTY_A::_01,
            2 => OBDTY_A::_10,
            3 => OBDTY_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OBDTY_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OBDTY_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OBDTY_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OBDTY_A::_11
    }
}
#[doc = "Field `OBDTY` writer - GTIOCnB Output Duty Setting"]
pub type OBDTY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTUDDTYC_SPEC, u8, OBDTY_A, 2, O>;
impl<'a, const O: u8> OBDTY_W<'a, O> {
    #[doc = "GTIOCnB pin duty depends on the compare match"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OBDTY_A::_00)
    }
    #[doc = "GTIOCnB pin duty depends on the compare match"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OBDTY_A::_01)
    }
    #[doc = "GTIOCnB pin duty 0%"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OBDTY_A::_10)
    }
    #[doc = "GTIOCnB pin duty 100%"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OBDTY_A::_11)
    }
}
#[doc = "Field `OBDTYF` reader - Forcible GTIOCnB Output Duty Setting"]
pub type OBDTYF_R = crate::BitReader<OBDTYF_A>;
#[doc = "Forcible GTIOCnB Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBDTYF_A {
    #[doc = "0: Not forcibly set"]
    _0 = 0,
    #[doc = "1: Forcibly set"]
    _1 = 1,
}
impl From<OBDTYF_A> for bool {
    #[inline(always)]
    fn from(variant: OBDTYF_A) -> Self {
        variant as u8 != 0
    }
}
impl OBDTYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBDTYF_A {
        match self.bits {
            false => OBDTYF_A::_0,
            true => OBDTYF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBDTYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBDTYF_A::_1
    }
}
#[doc = "Field `OBDTYF` writer - Forcible GTIOCnB Output Duty Setting"]
pub type OBDTYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUDDTYC_SPEC, OBDTYF_A, O>;
impl<'a, const O: u8> OBDTYF_W<'a, O> {
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBDTYF_A::_0)
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBDTYF_A::_1)
    }
}
#[doc = "Field `OBDTYR` reader - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting"]
pub type OBDTYR_R = crate::BitReader<OBDTYR_A>;
#[doc = "GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBDTYR_A {
    #[doc = "0: The function selected by the GTIOB\\[3:2\\]
bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting."]
    _0 = 0,
    #[doc = "1: The function selected by the GTIOB\\[3:2\\]
bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting."]
    _1 = 1,
}
impl From<OBDTYR_A> for bool {
    #[inline(always)]
    fn from(variant: OBDTYR_A) -> Self {
        variant as u8 != 0
    }
}
impl OBDTYR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBDTYR_A {
        match self.bits {
            false => OBDTYR_A::_0,
            true => OBDTYR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBDTYR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBDTYR_A::_1
    }
}
#[doc = "Field `OBDTYR` writer - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting"]
pub type OBDTYR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUDDTYC_SPEC, OBDTYR_A, O>;
impl<'a, const O: u8> OBDTYR_W<'a, O> {
    #[doc = "The function selected by the GTIOB\\[3:2\\]
bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OBDTYR_A::_0)
    }
    #[doc = "The function selected by the GTIOB\\[3:2\\]
bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OBDTYR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Count Direction Setting"]
    #[inline(always)]
    pub fn ud(&self) -> UD_R {
        UD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Forcible Count Direction Setting"]
    #[inline(always)]
    pub fn udf(&self) -> UDF_R {
        UDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:17 - GTIOCnA Output Duty Setting"]
    #[inline(always)]
    pub fn oadty(&self) -> OADTY_R {
        OADTY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Forcible GTIOCnA Output Duty Setting"]
    #[inline(always)]
    pub fn oadtyf(&self) -> OADTYF_R {
        OADTYF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting"]
    #[inline(always)]
    pub fn oadtyr(&self) -> OADTYR_R {
        OADTYR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:25 - GTIOCnB Output Duty Setting"]
    #[inline(always)]
    pub fn obdty(&self) -> OBDTY_R {
        OBDTY_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Forcible GTIOCnB Output Duty Setting"]
    #[inline(always)]
    pub fn obdtyf(&self) -> OBDTYF_R {
        OBDTYF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting"]
    #[inline(always)]
    pub fn obdtyr(&self) -> OBDTYR_R {
        OBDTYR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Count Direction Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ud(&mut self) -> UD_W<0> {
        UD_W::new(self)
    }
    #[doc = "Bit 1 - Forcible Count Direction Setting"]
    #[inline(always)]
    #[must_use]
    pub fn udf(&mut self) -> UDF_W<1> {
        UDF_W::new(self)
    }
    #[doc = "Bits 16:17 - GTIOCnA Output Duty Setting"]
    #[inline(always)]
    #[must_use]
    pub fn oadty(&mut self) -> OADTY_W<16> {
        OADTY_W::new(self)
    }
    #[doc = "Bit 18 - Forcible GTIOCnA Output Duty Setting"]
    #[inline(always)]
    #[must_use]
    pub fn oadtyf(&mut self) -> OADTYF_W<18> {
        OADTYF_W::new(self)
    }
    #[doc = "Bit 19 - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting"]
    #[inline(always)]
    #[must_use]
    pub fn oadtyr(&mut self) -> OADTYR_W<19> {
        OADTYR_W::new(self)
    }
    #[doc = "Bits 24:25 - GTIOCnB Output Duty Setting"]
    #[inline(always)]
    #[must_use]
    pub fn obdty(&mut self) -> OBDTY_W<24> {
        OBDTY_W::new(self)
    }
    #[doc = "Bit 26 - Forcible GTIOCnB Output Duty Setting"]
    #[inline(always)]
    #[must_use]
    pub fn obdtyf(&mut self) -> OBDTYF_W<26> {
        OBDTYF_W::new(self)
    }
    #[doc = "Bit 27 - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting"]
    #[inline(always)]
    #[must_use]
    pub fn obdtyr(&mut self) -> OBDTYR_W<27> {
        OBDTYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Count Direction and Duty Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuddtyc](index.html) module"]
pub struct GTUDDTYC_SPEC;
impl crate::RegisterSpec for GTUDDTYC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuddtyc::R](R) reader structure"]
impl crate::Readable for GTUDDTYC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuddtyc::W](W) writer structure"]
impl crate::Writable for GTUDDTYC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUDDTYC to value 0x01"]
impl crate::Resettable for GTUDDTYC_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
