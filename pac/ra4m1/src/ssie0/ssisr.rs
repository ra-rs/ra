#[doc = "Register `SSISR` reader"]
pub struct R(crate::R<SSISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSISR` writer"]
pub struct W(crate::W<SSISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSISR_SPEC>;
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
impl From<crate::W<SSISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IIRQ` reader - Idle Mode Status Flag"]
pub type IIRQ_R = crate::BitReader<IIRQ_A>;
#[doc = "Idle Mode Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIRQ_A {
    #[doc = "0: In the communication state"]
    _0 = 0,
    #[doc = "1: In the idle state"]
    _1 = 1,
}
impl From<IIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIRQ_A {
        match self.bits {
            false => IIRQ_A::_0,
            true => IIRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIRQ_A::_1
    }
}
#[doc = "Field `ROIRQ` reader - Receive Overflow Error Status Flag"]
pub type ROIRQ_R = crate::BitReader<ROIRQ_A>;
#[doc = "Receive Overflow Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIRQ_A {
    #[doc = "0: No receive overflow error is generated"]
    _0 = 0,
    #[doc = "1: A receive overflow error is generated."]
    _1 = 1,
}
impl From<ROIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ROIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ROIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROIRQ_A {
        match self.bits {
            false => ROIRQ_A::_0,
            true => ROIRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROIRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROIRQ_A::_1
    }
}
#[doc = "Field `ROIRQ` writer - Receive Overflow Error Status Flag"]
pub type ROIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSISR_SPEC, ROIRQ_A, O>;
impl<'a, const O: u8> ROIRQ_W<'a, O> {
    #[doc = "No receive overflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROIRQ_A::_0)
    }
    #[doc = "A receive overflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROIRQ_A::_1)
    }
}
#[doc = "Field `RUIRQ` reader - Receive Underflow Error Status Flag"]
pub type RUIRQ_R = crate::BitReader<RUIRQ_A>;
#[doc = "Receive Underflow Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIRQ_A {
    #[doc = "0: No receive underflow error is generated"]
    _0 = 0,
    #[doc = "1: A receive underflow error is generated."]
    _1 = 1,
}
impl From<RUIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RUIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RUIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUIRQ_A {
        match self.bits {
            false => RUIRQ_A::_0,
            true => RUIRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RUIRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RUIRQ_A::_1
    }
}
#[doc = "Field `RUIRQ` writer - Receive Underflow Error Status Flag"]
pub type RUIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSISR_SPEC, RUIRQ_A, O>;
impl<'a, const O: u8> RUIRQ_W<'a, O> {
    #[doc = "No receive underflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RUIRQ_A::_0)
    }
    #[doc = "A receive underflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RUIRQ_A::_1)
    }
}
#[doc = "Field `TOIRQ` reader - Transmit Overflow Error Status Flag"]
pub type TOIRQ_R = crate::BitReader<TOIRQ_A>;
#[doc = "Transmit Overflow Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIRQ_A {
    #[doc = "0: No transmit overflow error is generated"]
    _0 = 0,
    #[doc = "1: A transmit overflow error is generated."]
    _1 = 1,
}
impl From<TOIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TOIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TOIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIRQ_A {
        match self.bits {
            false => TOIRQ_A::_0,
            true => TOIRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIRQ_A::_1
    }
}
#[doc = "Field `TOIRQ` writer - Transmit Overflow Error Status Flag"]
pub type TOIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSISR_SPEC, TOIRQ_A, O>;
impl<'a, const O: u8> TOIRQ_W<'a, O> {
    #[doc = "No transmit overflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIRQ_A::_0)
    }
    #[doc = "A transmit overflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIRQ_A::_1)
    }
}
#[doc = "Field `TUIRQ` reader - Transmit Underflow Error Status flag"]
pub type TUIRQ_R = crate::BitReader<TUIRQ_A>;
#[doc = "Transmit Underflow Error Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUIRQ_A {
    #[doc = "0: No transmit underflow error is generated"]
    _0 = 0,
    #[doc = "1: A transmit underflow error is generated."]
    _1 = 1,
}
impl From<TUIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TUIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TUIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUIRQ_A {
        match self.bits {
            false => TUIRQ_A::_0,
            true => TUIRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUIRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUIRQ_A::_1
    }
}
#[doc = "Field `TUIRQ` writer - Transmit Underflow Error Status flag"]
pub type TUIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSISR_SPEC, TUIRQ_A, O>;
impl<'a, const O: u8> TUIRQ_W<'a, O> {
    #[doc = "No transmit underflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TUIRQ_A::_0)
    }
    #[doc = "A transmit underflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TUIRQ_A::_1)
    }
}
impl R {
    #[doc = "Bit 25 - Idle Mode Status Flag"]
    #[inline(always)]
    pub fn iirq(&self) -> IIRQ_R {
        IIRQ_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receive Overflow Error Status Flag"]
    #[inline(always)]
    pub fn roirq(&self) -> ROIRQ_R {
        ROIRQ_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Underflow Error Status Flag"]
    #[inline(always)]
    pub fn ruirq(&self) -> RUIRQ_R {
        RUIRQ_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Overflow Error Status Flag"]
    #[inline(always)]
    pub fn toirq(&self) -> TOIRQ_R {
        TOIRQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Underflow Error Status flag"]
    #[inline(always)]
    pub fn tuirq(&self) -> TUIRQ_R {
        TUIRQ_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - Receive Overflow Error Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn roirq(&mut self) -> ROIRQ_W<26> {
        ROIRQ_W::new(self)
    }
    #[doc = "Bit 27 - Receive Underflow Error Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ruirq(&mut self) -> RUIRQ_W<27> {
        RUIRQ_W::new(self)
    }
    #[doc = "Bit 28 - Transmit Overflow Error Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn toirq(&mut self) -> TOIRQ_W<28> {
        TOIRQ_W::new(self)
    }
    #[doc = "Bit 29 - Transmit Underflow Error Status flag"]
    #[inline(always)]
    #[must_use]
    pub fn tuirq(&mut self) -> TUIRQ_W<29> {
        TUIRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssisr](index.html) module"]
pub struct SSISR_SPEC;
impl crate::RegisterSpec for SSISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssisr::R](R) reader structure"]
impl crate::Readable for SSISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssisr::W](W) writer structure"]
impl crate::Writable for SSISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSISR to value 0x0200_0000"]
impl crate::Resettable for SSISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0000;
}
