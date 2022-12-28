#[doc = "Register `CFDGAFLP1%s` reader"]
pub struct R(crate::R<CFDGAFLP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLP1%s` writer"]
pub struct W(crate::W<CFDGAFLP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLP1_SPEC>;
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
impl From<crate::W<CFDGAFLP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAFLFDP0` reader - Global Acceptance Filter List FIFO Direction Pointer"]
pub type GAFLFDP0_R = crate::BitReader<GAFLFDP0_A>;
#[doc = "Global Acceptance Filter List FIFO Direction Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLFDP0_A {
    #[doc = "0: Disable RX FIFO 0 as target for reception"]
    _0 = 0,
    #[doc = "1: Enable RX FIFO 0 as target for reception"]
    _1 = 1,
}
impl From<GAFLFDP0_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLFDP0_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLFDP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLFDP0_A {
        match self.bits {
            false => GAFLFDP0_A::_0,
            true => GAFLFDP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLFDP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLFDP0_A::_1
    }
}
#[doc = "Field `GAFLFDP0` writer - Global Acceptance Filter List FIFO Direction Pointer"]
pub type GAFLFDP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLP1_SPEC, GAFLFDP0_A, O>;
impl<'a, const O: u8> GAFLFDP0_W<'a, O> {
    #[doc = "Disable RX FIFO 0 as target for reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLFDP0_A::_0)
    }
    #[doc = "Enable RX FIFO 0 as target for reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLFDP0_A::_1)
    }
}
#[doc = "Field `GAFLFDP1` reader - Global Acceptance Filter List FIFO Direction Pointer"]
pub type GAFLFDP1_R = crate::BitReader<GAFLFDP1_A>;
#[doc = "Global Acceptance Filter List FIFO Direction Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLFDP1_A {
    #[doc = "0: Disable RX FIFO 1 as target for reception"]
    _0 = 0,
    #[doc = "1: Enable RX FIFO 1 as target for reception"]
    _1 = 1,
}
impl From<GAFLFDP1_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLFDP1_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLFDP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLFDP1_A {
        match self.bits {
            false => GAFLFDP1_A::_0,
            true => GAFLFDP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLFDP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLFDP1_A::_1
    }
}
#[doc = "Field `GAFLFDP1` writer - Global Acceptance Filter List FIFO Direction Pointer"]
pub type GAFLFDP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLP1_SPEC, GAFLFDP1_A, O>;
impl<'a, const O: u8> GAFLFDP1_W<'a, O> {
    #[doc = "Disable RX FIFO 1 as target for reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLFDP1_A::_0)
    }
    #[doc = "Enable RX FIFO 1 as target for reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLFDP1_A::_1)
    }
}
#[doc = "Field `GAFLFDP8` reader - Global Acceptance Filter List FIFO Direction Pointer"]
pub type GAFLFDP8_R = crate::BitReader<GAFLFDP8_A>;
#[doc = "Global Acceptance Filter List FIFO Direction Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLFDP8_A {
    #[doc = "0: Disable Common FIFO as target for reception"]
    _0 = 0,
    #[doc = "1: Enable Common FIFO as target for reception"]
    _1 = 1,
}
impl From<GAFLFDP8_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLFDP8_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLFDP8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLFDP8_A {
        match self.bits {
            false => GAFLFDP8_A::_0,
            true => GAFLFDP8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLFDP8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLFDP8_A::_1
    }
}
#[doc = "Field `GAFLFDP8` writer - Global Acceptance Filter List FIFO Direction Pointer"]
pub type GAFLFDP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLP1_SPEC, GAFLFDP8_A, O>;
impl<'a, const O: u8> GAFLFDP8_W<'a, O> {
    #[doc = "Disable Common FIFO as target for reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLFDP8_A::_0)
    }
    #[doc = "Enable Common FIFO as target for reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLFDP8_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    pub fn gaflfdp0(&self) -> GAFLFDP0_R {
        GAFLFDP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    pub fn gaflfdp1(&self) -> GAFLFDP1_R {
        GAFLFDP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    pub fn gaflfdp8(&self) -> GAFLFDP8_R {
        GAFLFDP8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn gaflfdp0(&mut self) -> GAFLFDP0_W<0> {
        GAFLFDP0_W::new(self)
    }
    #[doc = "Bit 1 - Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn gaflfdp1(&mut self) -> GAFLFDP1_W<1> {
        GAFLFDP1_W::new(self)
    }
    #[doc = "Bit 8 - Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn gaflfdp8(&mut self) -> GAFLFDP8_W<8> {
        GAFLFDP8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Acceptance Filter List Pointer 1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflp1](index.html) module"]
pub struct CFDGAFLP1_SPEC;
impl crate::RegisterSpec for CFDGAFLP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflp1::R](R) reader structure"]
impl crate::Readable for CFDGAFLP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflp1::W](W) writer structure"]
impl crate::Writable for CFDGAFLP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLP1%s to value 0"]
impl crate::Resettable for CFDGAFLP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
