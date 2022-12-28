#[doc = "Register `CFIFOCTR` reader"]
pub struct R(crate::R<CFIFOCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFIFOCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFIFOCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFIFOCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFIFOCTR` writer"]
pub struct W(crate::W<CFIFOCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFIFOCTR_SPEC>;
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
impl From<crate::W<CFIFOCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFIFOCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTLN` reader - Receive Data LengthIndicates the length of the receive data."]
pub type DTLN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRDY` reader - FIFO Port Ready"]
pub type FRDY_R = crate::BitReader<FRDY_A>;
#[doc = "FIFO Port Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDY_A {
    #[doc = "0: FIFO port access is disabled."]
    _0 = 0,
    #[doc = "1: FIFO port access is enabled."]
    _1 = 1,
}
impl From<FRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl FRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDY_A {
        match self.bits {
            false => FRDY_A::_0,
            true => FRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDY_A::_1
    }
}
#[doc = "CPU Buffer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCLR_AW {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Clears the buffer memory on the CPU side"]
    _1 = 1,
}
impl From<BCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR` writer - CPU Buffer Clear"]
pub type BCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOCTR_SPEC, BCLR_AW, O>;
impl<'a, const O: u8> BCLR_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCLR_AW::_0)
    }
    #[doc = "Clears the buffer memory on the CPU side"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCLR_AW::_1)
    }
}
#[doc = "Field `BVAL` reader - Buffer Memory Valid Flag"]
pub type BVAL_R = crate::BitReader<BVAL_A>;
#[doc = "Buffer Memory Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BVAL_A {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Writing ended"]
    _1 = 1,
}
impl From<BVAL_A> for bool {
    #[inline(always)]
    fn from(variant: BVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl BVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVAL_A {
        match self.bits {
            false => BVAL_A::_0,
            true => BVAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BVAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BVAL_A::_1
    }
}
#[doc = "Field `BVAL` writer - Buffer Memory Valid Flag"]
pub type BVAL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOCTR_SPEC, BVAL_A, O>;
impl<'a, const O: u8> BVAL_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BVAL_A::_0)
    }
    #[doc = "Writing ended"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BVAL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Receive Data LengthIndicates the length of the receive data."]
    #[inline(always)]
    pub fn dtln(&self) -> DTLN_R {
        DTLN_R::new(self.bits & 0x01ff)
    }
    #[doc = "Bit 13 - FIFO Port Ready"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer Memory Valid Flag"]
    #[inline(always)]
    pub fn bval(&self) -> BVAL_R {
        BVAL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - CPU Buffer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bclr(&mut self) -> BCLR_W<14> {
        BCLR_W::new(self)
    }
    #[doc = "Bit 15 - Buffer Memory Valid Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bval(&mut self) -> BVAL_W<15> {
        BVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFIFO Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfifoctr](index.html) module"]
pub struct CFIFOCTR_SPEC;
impl crate::RegisterSpec for CFIFOCTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cfifoctr::R](R) reader structure"]
impl crate::Readable for CFIFOCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfifoctr::W](W) writer structure"]
impl crate::Writable for CFIFOCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFIFOCTR to value 0"]
impl crate::Resettable for CFIFOCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
