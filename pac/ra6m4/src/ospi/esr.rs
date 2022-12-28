#[doc = "Register `ESR` reader"]
pub struct R(crate::R<ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESR` writer"]
pub struct W(crate::W<ESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR_SPEC>;
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
impl From<crate::W<ESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRESR` reader - Memory map read error status"]
pub type MRESR_R = crate::FieldReader<u8, MRESR_A>;
#[doc = "Memory map read error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MRESR_A {
    #[doc = "1: ECC error"]
    _0X01 = 1,
    #[doc = "2: Preamble error"]
    _0X02 = 2,
    #[doc = "3: Wait OM_DQS timeout"]
    _0X03 = 3,
    #[doc = "128: Invalid command"]
    _0X80 = 128,
}
impl From<MRESR_A> for u8 {
    #[inline(always)]
    fn from(variant: MRESR_A) -> Self {
        variant as _
    }
}
impl MRESR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MRESR_A> {
        match self.bits {
            1 => Some(MRESR_A::_0X01),
            2 => Some(MRESR_A::_0X02),
            3 => Some(MRESR_A::_0X03),
            128 => Some(MRESR_A::_0X80),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == MRESR_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == MRESR_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == MRESR_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X80`"]
    #[inline(always)]
    pub fn is_0x80(&self) -> bool {
        *self == MRESR_A::_0X80
    }
}
#[doc = "Field `MRESR` writer - Memory map read error status"]
pub type MRESR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESR_SPEC, u8, MRESR_A, 8, O>;
impl<'a, const O: u8> MRESR_W<'a, O> {
    #[doc = "ECC error"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(MRESR_A::_0X01)
    }
    #[doc = "Preamble error"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(MRESR_A::_0X02)
    }
    #[doc = "Wait OM_DQS timeout"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(MRESR_A::_0X03)
    }
    #[doc = "Invalid command"]
    #[inline(always)]
    pub fn _0x80(self) -> &'a mut W {
        self.variant(MRESR_A::_0X80)
    }
}
#[doc = "Field `MWESR` reader - Memory map write error status"]
pub type MWESR_R = crate::FieldReader<u8, MWESR_A>;
#[doc = "Memory map write error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MWESR_A {
    #[doc = "128: Invalid command"]
    _0X80 = 128,
}
impl From<MWESR_A> for u8 {
    #[inline(always)]
    fn from(variant: MWESR_A) -> Self {
        variant as _
    }
}
impl MWESR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MWESR_A> {
        match self.bits {
            128 => Some(MWESR_A::_0X80),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X80`"]
    #[inline(always)]
    pub fn is_0x80(&self) -> bool {
        *self == MWESR_A::_0X80
    }
}
#[doc = "Field `MWESR` writer - Memory map write error status"]
pub type MWESR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESR_SPEC, u8, MWESR_A, 8, O>;
impl<'a, const O: u8> MWESR_W<'a, O> {
    #[doc = "Invalid command"]
    #[inline(always)]
    pub fn _0x80(self) -> &'a mut W {
        self.variant(MWESR_A::_0X80)
    }
}
impl R {
    #[doc = "Bits 0:7 - Memory map read error status"]
    #[inline(always)]
    pub fn mresr(&self) -> MRESR_R {
        MRESR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Memory map write error status"]
    #[inline(always)]
    pub fn mwesr(&self) -> MWESR_R {
        MWESR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Memory map read error status"]
    #[inline(always)]
    #[must_use]
    pub fn mresr(&mut self) -> MRESR_W<0> {
        MRESR_W::new(self)
    }
    #[doc = "Bits 8:15 - Memory map write error status"]
    #[inline(always)]
    #[must_use]
    pub fn mwesr(&mut self) -> MWESR_W<8> {
        MWESR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](index.html) module"]
pub struct ESR_SPEC;
impl crate::RegisterSpec for ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr::R](R) reader structure"]
impl crate::Readable for ESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esr::W](W) writer structure"]
impl crate::Writable for ESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
