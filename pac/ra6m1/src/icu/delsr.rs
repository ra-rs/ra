#[doc = "Register `DELSR%s` reader"]
pub struct R(crate::R<DELSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELSR%s` writer"]
pub struct W(crate::W<DELSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELSR_SPEC>;
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
impl From<crate::W<DELSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELS` reader - DMAC Event Link Select"]
pub type DELS_R = crate::FieldReader<u16, DELS_A>;
#[doc = "DMAC Event Link Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DELS_A {
    #[doc = "0: Nothing is selected."]
    _0X000 = 0,
}
impl From<DELS_A> for u16 {
    #[inline(always)]
    fn from(variant: DELS_A) -> Self {
        variant as _
    }
}
impl DELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DELS_A> {
        match self.bits {
            0 => Some(DELS_A::_0X000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X000`"]
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == DELS_A::_0X000
    }
}
#[doc = "Field `DELS` writer - DMAC Event Link Select"]
pub type DELS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELSR_SPEC, u16, DELS_A, 9, O>;
impl<'a, const O: u8> DELS_W<'a, O> {
    #[doc = "Nothing is selected."]
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut W {
        self.variant(DELS_A::_0X000)
    }
}
#[doc = "Field `IR` reader - Interrupt Status Flag for DMAC"]
pub type IR_R = crate::BitReader<IR_A>;
#[doc = "Interrupt Status Flag for DMAC\n\nValue on reset: 0"]
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
#[doc = "Field `IR` writer - Interrupt Status Flag for DMAC"]
pub type IR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DELSR_SPEC, IR_A, O>;
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
impl R {
    #[doc = "Bits 0:8 - DMAC Event Link Select"]
    #[inline(always)]
    pub fn dels(&self) -> DELS_R {
        DELS_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Interrupt Status Flag for DMAC"]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - DMAC Event Link Select"]
    #[inline(always)]
    #[must_use]
    pub fn dels(&mut self) -> DELS_W<0> {
        DELS_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Status Flag for DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn ir(&mut self) -> IR_W<16> {
        IR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Event Link Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delsr](index.html) module"]
pub struct DELSR_SPEC;
impl crate::RegisterSpec for DELSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delsr::R](R) reader structure"]
impl crate::Readable for DELSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delsr::W](W) writer structure"]
impl crate::Writable for DELSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DELSR%s to value 0"]
impl crate::Resettable for DELSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
