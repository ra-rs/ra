#[doc = "Register `CFDCDTCT` reader"]
pub struct R(crate::R<CFDCDTCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCDTCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCDTCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCDTCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCDTCT` writer"]
pub struct W(crate::W<CFDCDTCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCDTCT_SPEC>;
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
impl From<crate::W<CFDCDTCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCDTCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFDMAE0` reader - DMA Transfer Enable for RXFIFO 0"]
pub type RFDMAE0_R = crate::BitReader<RFDMAE0_A>;
#[doc = "DMA Transfer Enable for RXFIFO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDMAE0_A {
    #[doc = "0: DMA transfer request disabled"]
    _0 = 0,
    #[doc = "1: DMA transfer request enabled"]
    _1 = 1,
}
impl From<RFDMAE0_A> for bool {
    #[inline(always)]
    fn from(variant: RFDMAE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDMAE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDMAE0_A {
        match self.bits {
            false => RFDMAE0_A::_0,
            true => RFDMAE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDMAE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDMAE0_A::_1
    }
}
#[doc = "Field `RFDMAE0` writer - DMA Transfer Enable for RXFIFO 0"]
pub type RFDMAE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCDTCT_SPEC, RFDMAE0_A, O>;
impl<'a, const O: u8> RFDMAE0_W<'a, O> {
    #[doc = "DMA transfer request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDMAE0_A::_0)
    }
    #[doc = "DMA transfer request enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDMAE0_A::_1)
    }
}
#[doc = "Field `RFDMAE1` reader - DMA Transfer Enable for RXFIFO 1"]
pub type RFDMAE1_R = crate::BitReader<RFDMAE1_A>;
#[doc = "DMA Transfer Enable for RXFIFO 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDMAE1_A {
    #[doc = "0: DMA transfer request disabled"]
    _0 = 0,
    #[doc = "1: DMA transfer request enabled"]
    _1 = 1,
}
impl From<RFDMAE1_A> for bool {
    #[inline(always)]
    fn from(variant: RFDMAE1_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDMAE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDMAE1_A {
        match self.bits {
            false => RFDMAE1_A::_0,
            true => RFDMAE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDMAE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDMAE1_A::_1
    }
}
#[doc = "Field `RFDMAE1` writer - DMA Transfer Enable for RXFIFO 1"]
pub type RFDMAE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCDTCT_SPEC, RFDMAE1_A, O>;
impl<'a, const O: u8> RFDMAE1_W<'a, O> {
    #[doc = "DMA transfer request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDMAE1_A::_0)
    }
    #[doc = "DMA transfer request enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDMAE1_A::_1)
    }
}
#[doc = "Field `CFDMAE` reader - DMA Transfer Enable for Common FIFO 0"]
pub type CFDMAE_R = crate::BitReader<CFDMAE_A>;
#[doc = "DMA Transfer Enable for Common FIFO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFDMAE_A {
    #[doc = "0: DMA transfer request disabled"]
    _0 = 0,
    #[doc = "1: DMA transfer request enabled"]
    _1 = 1,
}
impl From<CFDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: CFDMAE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFDMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFDMAE_A {
        match self.bits {
            false => CFDMAE_A::_0,
            true => CFDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFDMAE_A::_1
    }
}
#[doc = "Field `CFDMAE` writer - DMA Transfer Enable for Common FIFO 0"]
pub type CFDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCDTCT_SPEC, CFDMAE_A, O>;
impl<'a, const O: u8> CFDMAE_W<'a, O> {
    #[doc = "DMA transfer request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFDMAE_A::_0)
    }
    #[doc = "DMA transfer request enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFDMAE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Transfer Enable for RXFIFO 0"]
    #[inline(always)]
    pub fn rfdmae0(&self) -> RFDMAE0_R {
        RFDMAE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Transfer Enable for RXFIFO 1"]
    #[inline(always)]
    pub fn rfdmae1(&self) -> RFDMAE1_R {
        RFDMAE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Transfer Enable for Common FIFO 0"]
    #[inline(always)]
    pub fn cfdmae(&self) -> CFDMAE_R {
        CFDMAE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Transfer Enable for RXFIFO 0"]
    #[inline(always)]
    #[must_use]
    pub fn rfdmae0(&mut self) -> RFDMAE0_W<0> {
        RFDMAE0_W::new(self)
    }
    #[doc = "Bit 1 - DMA Transfer Enable for RXFIFO 1"]
    #[inline(always)]
    #[must_use]
    pub fn rfdmae1(&mut self) -> RFDMAE1_W<1> {
        RFDMAE1_W::new(self)
    }
    #[doc = "Bit 8 - DMA Transfer Enable for Common FIFO 0"]
    #[inline(always)]
    #[must_use]
    pub fn cfdmae(&mut self) -> CFDMAE_W<8> {
        CFDMAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Transfer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcdtct](index.html) module"]
pub struct CFDCDTCT_SPEC;
impl crate::RegisterSpec for CFDCDTCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcdtct::R](R) reader structure"]
impl crate::Readable for CFDCDTCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdcdtct::W](W) writer structure"]
impl crate::Writable for CFDCDTCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCDTCT to value 0"]
impl crate::Resettable for CFDCDTCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
