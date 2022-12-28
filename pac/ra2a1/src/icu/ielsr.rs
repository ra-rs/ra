#[doc = "Register `IELSR%s` reader"]
pub struct R(crate::R<IELSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IELSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IELSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IELSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IELSR%s` writer"]
pub struct W(crate::W<IELSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IELSR_SPEC>;
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
impl From<crate::W<IELSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IELSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IELS` reader - ICU Event selection to NVICSet the number for the event signal to be linked ."]
pub type IELS_R = crate::FieldReader<u8, IELS_A>;
#[doc = "ICU Event selection to NVICSet the number for the event signal to be linked .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IELS_A {
    #[doc = "0: Nothing is selected"]
    _0X000 = 0,
}
impl From<IELS_A> for u8 {
    #[inline(always)]
    fn from(variant: IELS_A) -> Self {
        variant as _
    }
}
impl IELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IELS_A> {
        match self.bits {
            0 => Some(IELS_A::_0X000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X000`"]
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == IELS_A::_0X000
    }
}
#[doc = "Field `IELS` writer - ICU Event selection to NVICSet the number for the event signal to be linked ."]
pub type IELS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IELSR_SPEC, u8, IELS_A, 8, O>;
impl<'a, const O: u8> IELS_W<'a, O> {
    #[doc = "Nothing is selected"]
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut W {
        self.variant(IELS_A::_0X000)
    }
}
#[doc = "Field `IR` reader - Interrupt Status Flag\n\nThe field is **modified** in some way after a read operation."]
pub type IR_R = crate::BitReader<IR_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_A {
    #[doc = "0: No interrupt request is generated"]
    _0 = 0,
    #[doc = "1: An interrupt request is generated ( \"1\" write to the IR bit is prohibited. )"]
    _1 = 1,
}
impl From<IR_A> for bool {
    #[inline(always)]
    fn from(variant: IR_A) -> Self {
        variant as u8 != 0
    }
}
impl IR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IR_A {
        match self.bits {
            false => IR_A::_0,
            true => IR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IR_A::_1
    }
}
#[doc = "Field `IR` writer - Interrupt Status Flag"]
pub type IR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, IELSR_SPEC, IR_A, O>;
impl<'a, const O: u8> IR_W<'a, O> {
    #[doc = "No interrupt request is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IR_A::_0)
    }
    #[doc = "An interrupt request is generated ( \"1\" write to the IR bit is prohibited. )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IR_A::_1)
    }
}
#[doc = "Field `DTCE` reader - DTC Activation Enable\n\nThe field is **modified** in some way after a read operation."]
pub type DTCE_R = crate::BitReader<DTCE_A>;
#[doc = "DTC Activation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCE_A {
    #[doc = "0: DTC activation is disabled"]
    _0 = 0,
    #[doc = "1: DTC activation is enabled"]
    _1 = 1,
}
impl From<DTCE_A> for bool {
    #[inline(always)]
    fn from(variant: DTCE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCE_A {
        match self.bits {
            false => DTCE_A::_0,
            true => DTCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCE_A::_1
    }
}
#[doc = "Field `DTCE` writer - DTC Activation Enable"]
pub type DTCE_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, IELSR_SPEC, DTCE_A, O>;
impl<'a, const O: u8> DTCE_W<'a, O> {
    #[doc = "DTC activation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCE_A::_0)
    }
    #[doc = "DTC activation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - ICU Event selection to NVICSet the number for the event signal to be linked ."]
    #[inline(always)]
    pub fn iels(&self) -> IELS_R {
        IELS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - DTC Activation Enable"]
    #[inline(always)]
    pub fn dtce(&self) -> DTCE_R {
        DTCE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ICU Event selection to NVICSet the number for the event signal to be linked ."]
    #[inline(always)]
    #[must_use]
    pub fn iels(&mut self) -> IELS_W<0> {
        IELS_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ir(&mut self) -> IR_W<16> {
        IR_W::new(self)
    }
    #[doc = "Bit 24 - DTC Activation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtce(&mut self) -> DTCE_W<24> {
        DTCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICU Event Link Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ielsr](index.html) module"]
pub struct IELSR_SPEC;
impl crate::RegisterSpec for IELSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ielsr::R](R) reader structure"]
impl crate::Readable for IELSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ielsr::W](W) writer structure"]
impl crate::Writable for IELSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0101_0000;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IELSR%s to value 0"]
impl crate::Resettable for IELSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
