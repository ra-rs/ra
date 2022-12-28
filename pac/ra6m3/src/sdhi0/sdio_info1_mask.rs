#[doc = "Register `SDIO_INFO1_MASK` reader"]
pub struct R(crate::R<SDIO_INFO1_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_INFO1_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_INFO1_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_INFO1_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_INFO1_MASK` writer"]
pub struct W(crate::W<SDIO_INFO1_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_INFO1_MASK_SPEC>;
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
impl From<crate::W<SDIO_INFO1_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_INFO1_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOIRQM` reader - IOIRQ Interrupt Mask Control"]
pub type IOIRQM_R = crate::BitReader<IOIRQM_A>;
#[doc = "IOIRQ Interrupt Mask Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOIRQM_A {
    #[doc = "0: IOIRQ interrupt not masked"]
    _0 = 0,
    #[doc = "1: IOIRQ interrupt masked"]
    _1 = 1,
}
impl From<IOIRQM_A> for bool {
    #[inline(always)]
    fn from(variant: IOIRQM_A) -> Self {
        variant as u8 != 0
    }
}
impl IOIRQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOIRQM_A {
        match self.bits {
            false => IOIRQM_A::_0,
            true => IOIRQM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOIRQM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOIRQM_A::_1
    }
}
#[doc = "Field `IOIRQM` writer - IOIRQ Interrupt Mask Control"]
pub type IOIRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_INFO1_MASK_SPEC, IOIRQM_A, O>;
impl<'a, const O: u8> IOIRQM_W<'a, O> {
    #[doc = "IOIRQ interrupt not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOIRQM_A::_0)
    }
    #[doc = "IOIRQ interrupt masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOIRQM_A::_1)
    }
}
#[doc = "Field `EXPUB52M` reader - EXPUB52 Interrupt Request Mask Control"]
pub type EXPUB52M_R = crate::BitReader<EXPUB52M_A>;
#[doc = "EXPUB52 Interrupt Request Mask Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXPUB52M_A {
    #[doc = "0: EXPUB52 interrupt request not masked"]
    _0 = 0,
    #[doc = "1: EXPUB52 interrupt request masked"]
    _1 = 1,
}
impl From<EXPUB52M_A> for bool {
    #[inline(always)]
    fn from(variant: EXPUB52M_A) -> Self {
        variant as u8 != 0
    }
}
impl EXPUB52M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXPUB52M_A {
        match self.bits {
            false => EXPUB52M_A::_0,
            true => EXPUB52M_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXPUB52M_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXPUB52M_A::_1
    }
}
#[doc = "Field `EXPUB52M` writer - EXPUB52 Interrupt Request Mask Control"]
pub type EXPUB52M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIO_INFO1_MASK_SPEC, EXPUB52M_A, O>;
impl<'a, const O: u8> EXPUB52M_W<'a, O> {
    #[doc = "EXPUB52 interrupt request not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXPUB52M_A::_0)
    }
    #[doc = "EXPUB52 interrupt request masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXPUB52M_A::_1)
    }
}
#[doc = "Field `EXWTM` reader - EXWT Interrupt Request Mask Control"]
pub type EXWTM_R = crate::BitReader<EXWTM_A>;
#[doc = "EXWT Interrupt Request Mask Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXWTM_A {
    #[doc = "0: EXWT interrupt request not masked"]
    _0 = 0,
    #[doc = "1: EXWT interrupt request masked"]
    _1 = 1,
}
impl From<EXWTM_A> for bool {
    #[inline(always)]
    fn from(variant: EXWTM_A) -> Self {
        variant as u8 != 0
    }
}
impl EXWTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXWTM_A {
        match self.bits {
            false => EXWTM_A::_0,
            true => EXWTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXWTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXWTM_A::_1
    }
}
#[doc = "Field `EXWTM` writer - EXWT Interrupt Request Mask Control"]
pub type EXWTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_INFO1_MASK_SPEC, EXWTM_A, O>;
impl<'a, const O: u8> EXWTM_W<'a, O> {
    #[doc = "EXWT interrupt request not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXWTM_A::_0)
    }
    #[doc = "EXWT interrupt request masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXWTM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IOIRQ Interrupt Mask Control"]
    #[inline(always)]
    pub fn ioirqm(&self) -> IOIRQM_R {
        IOIRQM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 14 - EXPUB52 Interrupt Request Mask Control"]
    #[inline(always)]
    pub fn expub52m(&self) -> EXPUB52M_R {
        EXPUB52M_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXWT Interrupt Request Mask Control"]
    #[inline(always)]
    pub fn exwtm(&self) -> EXWTM_R {
        EXWTM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IOIRQ Interrupt Mask Control"]
    #[inline(always)]
    #[must_use]
    pub fn ioirqm(&mut self) -> IOIRQM_W<0> {
        IOIRQM_W::new(self)
    }
    #[doc = "Bit 14 - EXPUB52 Interrupt Request Mask Control"]
    #[inline(always)]
    #[must_use]
    pub fn expub52m(&mut self) -> EXPUB52M_W<14> {
        EXPUB52M_W::new(self)
    }
    #[doc = "Bit 15 - EXWT Interrupt Request Mask Control"]
    #[inline(always)]
    #[must_use]
    pub fn exwtm(&mut self) -> EXWTM_W<15> {
        EXWTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO_INFO1 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_info1_mask](index.html) module"]
pub struct SDIO_INFO1_MASK_SPEC;
impl crate::RegisterSpec for SDIO_INFO1_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_info1_mask::R](R) reader structure"]
impl crate::Readable for SDIO_INFO1_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_info1_mask::W](W) writer structure"]
impl crate::Writable for SDIO_INFO1_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_INFO1_MASK to value 0xc007"]
impl crate::Resettable for SDIO_INFO1_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xc007;
}
