#[doc = "Register `PCNTR4` reader"]
pub struct R(crate::R<PCNTR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNTR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNTR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNTR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNTR4` writer"]
pub struct W(crate::W<PCNTR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNTR4_SPEC>;
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
impl From<crate::W<PCNTR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNTR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOSR` reader - Pmn Event Output Set"]
pub type EOSR_R = crate::FieldReader<u16, EOSR_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EOSR_A {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<EOSR_A> for u16 {
    #[inline(always)]
    fn from(variant: EOSR_A) -> Self {
        variant as _
    }
}
impl EOSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EOSR_A> {
        match self.bits {
            0 => Some(EOSR_A::_0),
            1 => Some(EOSR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR_A::_1
    }
}
#[doc = "Field `EOSR` writer - Pmn Event Output Set"]
pub type EOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNTR4_SPEC, u16, EOSR_A, 16, O>;
impl<'a, const O: u8> EOSR_W<'a, O> {
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSR_A::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSR_A::_1)
    }
}
#[doc = "Field `EORR` reader - Pmn Event Output Reset"]
pub type EORR_R = crate::FieldReader<u16, EORR_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EORR_A {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR_A> for u16 {
    #[inline(always)]
    fn from(variant: EORR_A) -> Self {
        variant as _
    }
}
impl EORR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EORR_A> {
        match self.bits {
            0 => Some(EORR_A::_0),
            1 => Some(EORR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR_A::_1
    }
}
#[doc = "Field `EORR` writer - Pmn Event Output Reset"]
pub type EORR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNTR4_SPEC, u16, EORR_A, 16, O>;
impl<'a, const O: u8> EORR_W<'a, O> {
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EORR_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EORR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr(&self) -> EOSR_R {
        EOSR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr(&self) -> EORR_R {
        EORR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr(&mut self) -> EOSR_W<0> {
        EOSR_W::new(self)
    }
    #[doc = "Bits 16:31 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr(&mut self) -> EORR_W<16> {
        EORR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntr4](index.html) module"]
pub struct PCNTR4_SPEC;
impl crate::RegisterSpec for PCNTR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcntr4::R](R) reader structure"]
impl crate::Readable for PCNTR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcntr4::W](W) writer structure"]
impl crate::Writable for PCNTR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNTR4 to value 0"]
impl crate::Resettable for PCNTR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
