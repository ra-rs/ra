#[doc = "Register `ADEXICR` reader"]
pub struct R(crate::R<ADEXICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADEXICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADEXICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADEXICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADEXICR` writer"]
pub struct W(crate::W<ADEXICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADEXICR_SPEC>;
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
impl From<crate::W<ADEXICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADEXICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSAD` reader - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select"]
pub type TSSAD_R = crate::BitReader<TSSAD_A>;
#[doc = "Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSSAD_A {
    #[doc = "0: Do not select addition/average mode for temperature sensor output."]
    _0 = 0,
    #[doc = "1: Select addition/average mode for temperature sensor output."]
    _1 = 1,
}
impl From<TSSAD_A> for bool {
    #[inline(always)]
    fn from(variant: TSSAD_A) -> Self {
        variant as u8 != 0
    }
}
impl TSSAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSSAD_A {
        match self.bits {
            false => TSSAD_A::_0,
            true => TSSAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSAD_A::_1
    }
}
#[doc = "Field `TSSAD` writer - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select"]
pub type TSSAD_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADEXICR_SPEC, TSSAD_A, O>;
impl<'a, const O: u8> TSSAD_W<'a, O> {
    #[doc = "Do not select addition/average mode for temperature sensor output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSSAD_A::_0)
    }
    #[doc = "Select addition/average mode for temperature sensor output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSSAD_A::_1)
    }
}
#[doc = "Field `OCSAD` reader - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select"]
pub type OCSAD_R = crate::BitReader<OCSAD_A>;
#[doc = "Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCSAD_A {
    #[doc = "0: Do not select addition/average mode for internal reference voltage."]
    _0 = 0,
    #[doc = "1: Select addition/average mode for internal reference voltage."]
    _1 = 1,
}
impl From<OCSAD_A> for bool {
    #[inline(always)]
    fn from(variant: OCSAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OCSAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCSAD_A {
        match self.bits {
            false => OCSAD_A::_0,
            true => OCSAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCSAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCSAD_A::_1
    }
}
#[doc = "Field `OCSAD` writer - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select"]
pub type OCSAD_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADEXICR_SPEC, OCSAD_A, O>;
impl<'a, const O: u8> OCSAD_W<'a, O> {
    #[doc = "Do not select addition/average mode for internal reference voltage."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCSAD_A::_0)
    }
    #[doc = "Select addition/average mode for internal reference voltage."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCSAD_A::_1)
    }
}
#[doc = "Field `TSSA` reader - Temperature Sensor Output A/D Conversion Select"]
pub type TSSA_R = crate::BitReader<TSSA_A>;
#[doc = "Temperature Sensor Output A/D Conversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSSA_A {
    #[doc = "0: Disable A/D conversion of temperature sensor output"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion of temperature sensor output"]
    _1 = 1,
}
impl From<TSSA_A> for bool {
    #[inline(always)]
    fn from(variant: TSSA_A) -> Self {
        variant as u8 != 0
    }
}
impl TSSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSSA_A {
        match self.bits {
            false => TSSA_A::_0,
            true => TSSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSA_A::_1
    }
}
#[doc = "Field `TSSA` writer - Temperature Sensor Output A/D Conversion Select"]
pub type TSSA_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADEXICR_SPEC, TSSA_A, O>;
impl<'a, const O: u8> TSSA_W<'a, O> {
    #[doc = "Disable A/D conversion of temperature sensor output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSSA_A::_0)
    }
    #[doc = "Enable A/D conversion of temperature sensor output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSSA_A::_1)
    }
}
#[doc = "Field `OCSA` reader - Internal Reference Voltage A/D Conversion Select"]
pub type OCSA_R = crate::BitReader<OCSA_A>;
#[doc = "Internal Reference Voltage A/D Conversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCSA_A {
    #[doc = "0: Disable A/D conversion of internal reference voltage"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion of internal reference voltage"]
    _1 = 1,
}
impl From<OCSA_A> for bool {
    #[inline(always)]
    fn from(variant: OCSA_A) -> Self {
        variant as u8 != 0
    }
}
impl OCSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCSA_A {
        match self.bits {
            false => OCSA_A::_0,
            true => OCSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCSA_A::_1
    }
}
#[doc = "Field `OCSA` writer - Internal Reference Voltage A/D Conversion Select"]
pub type OCSA_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADEXICR_SPEC, OCSA_A, O>;
impl<'a, const O: u8> OCSA_W<'a, O> {
    #[doc = "Disable A/D conversion of internal reference voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCSA_A::_0)
    }
    #[doc = "Enable A/D conversion of internal reference voltage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCSA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn tssad(&self) -> TSSAD_R {
        TSSAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn ocsad(&self) -> OCSAD_R {
        OCSAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    pub fn tssa(&self) -> TSSA_R {
        TSSA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub fn ocsa(&self) -> OCSA_R {
        OCSA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn tssad(&mut self) -> TSSAD_W<0> {
        TSSAD_W::new(self)
    }
    #[doc = "Bit 1 - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn ocsad(&mut self) -> OCSAD_W<1> {
        OCSAD_W::new(self)
    }
    #[doc = "Bit 8 - Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    pub fn tssa(&mut self) -> TSSA_W<8> {
        TSSA_W::new(self)
    }
    #[doc = "Bit 9 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub fn ocsa(&mut self) -> OCSA_W<9> {
        OCSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Extended Input Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adexicr](index.html) module"]
pub struct ADEXICR_SPEC;
impl crate::RegisterSpec for ADEXICR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adexicr::R](R) reader structure"]
impl crate::Readable for ADEXICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adexicr::W](W) writer structure"]
impl crate::Writable for ADEXICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADEXICR to value 0"]
impl crate::Resettable for ADEXICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
