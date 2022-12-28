#[doc = "Register `CFDTHLCC` reader"]
pub struct R(crate::R<CFDTHLCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTHLCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTHLCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTHLCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTHLCC` writer"]
pub struct W(crate::W<CFDTHLCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTHLCC_SPEC>;
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
impl From<crate::W<CFDTHLCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTHLCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THLE` reader - TX History List Enable"]
pub type THLE_R = crate::BitReader<THLE_A>;
#[doc = "TX History List Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLE_A {
    #[doc = "0: TX History List disabled"]
    _0 = 0,
    #[doc = "1: TX History List enabled"]
    _1 = 1,
}
impl From<THLE_A> for bool {
    #[inline(always)]
    fn from(variant: THLE_A) -> Self {
        variant as u8 != 0
    }
}
impl THLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLE_A {
        match self.bits {
            false => THLE_A::_0,
            true => THLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLE_A::_1
    }
}
#[doc = "Field `THLE` writer - TX History List Enable"]
pub type THLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTHLCC_SPEC, THLE_A, O>;
impl<'a, const O: u8> THLE_W<'a, O> {
    #[doc = "TX History List disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLE_A::_0)
    }
    #[doc = "TX History List enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLE_A::_1)
    }
}
#[doc = "Field `THLIE` reader - TX History List Interrupt Enable"]
pub type THLIE_R = crate::BitReader<THLIE_A>;
#[doc = "TX History List Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLIE_A {
    #[doc = "0: TX History List Interrupt disabled"]
    _0 = 0,
    #[doc = "1: TX History List Interrupt enabled"]
    _1 = 1,
}
impl From<THLIE_A> for bool {
    #[inline(always)]
    fn from(variant: THLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl THLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLIE_A {
        match self.bits {
            false => THLIE_A::_0,
            true => THLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLIE_A::_1
    }
}
#[doc = "Field `THLIE` writer - TX History List Interrupt Enable"]
pub type THLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTHLCC_SPEC, THLIE_A, O>;
impl<'a, const O: u8> THLIE_W<'a, O> {
    #[doc = "TX History List Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLIE_A::_0)
    }
    #[doc = "TX History List Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLIE_A::_1)
    }
}
#[doc = "Field `THLIM` reader - TX History List Interrupt Mode"]
pub type THLIM_R = crate::BitReader<THLIM_A>;
#[doc = "TX History List Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLIM_A {
    #[doc = "0: Interrupt generated if TX History List level reaches Â¾ of the TX History List depth"]
    _0 = 0,
    #[doc = "1: Interrupt generated for every successfully stored entry"]
    _1 = 1,
}
impl From<THLIM_A> for bool {
    #[inline(always)]
    fn from(variant: THLIM_A) -> Self {
        variant as u8 != 0
    }
}
impl THLIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLIM_A {
        match self.bits {
            false => THLIM_A::_0,
            true => THLIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLIM_A::_1
    }
}
#[doc = "Field `THLIM` writer - TX History List Interrupt Mode"]
pub type THLIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTHLCC_SPEC, THLIM_A, O>;
impl<'a, const O: u8> THLIM_W<'a, O> {
    #[doc = "Interrupt generated if TX History List level reaches Â¾ of the TX History List depth"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLIM_A::_0)
    }
    #[doc = "Interrupt generated for every successfully stored entry"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLIM_A::_1)
    }
}
#[doc = "Field `THLDTE` reader - TX History List Dedicated TX Enable"]
pub type THLDTE_R = crate::BitReader<THLDTE_A>;
#[doc = "TX History List Dedicated TX Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLDTE_A {
    #[doc = "0: TX FIFO + TX Queue"]
    _0 = 0,
    #[doc = "1: Flat TX MB + TX FIFO + TX Queue"]
    _1 = 1,
}
impl From<THLDTE_A> for bool {
    #[inline(always)]
    fn from(variant: THLDTE_A) -> Self {
        variant as u8 != 0
    }
}
impl THLDTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLDTE_A {
        match self.bits {
            false => THLDTE_A::_0,
            true => THLDTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLDTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLDTE_A::_1
    }
}
#[doc = "Field `THLDTE` writer - TX History List Dedicated TX Enable"]
pub type THLDTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTHLCC_SPEC, THLDTE_A, O>;
impl<'a, const O: u8> THLDTE_W<'a, O> {
    #[doc = "TX FIFO + TX Queue"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLDTE_A::_0)
    }
    #[doc = "Flat TX MB + TX FIFO + TX Queue"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLDTE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - TX History List Enable"]
    #[inline(always)]
    pub fn thle(&self) -> THLE_R {
        THLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - TX History List Interrupt Enable"]
    #[inline(always)]
    pub fn thlie(&self) -> THLIE_R {
        THLIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX History List Interrupt Mode"]
    #[inline(always)]
    pub fn thlim(&self) -> THLIM_R {
        THLIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX History List Dedicated TX Enable"]
    #[inline(always)]
    pub fn thldte(&self) -> THLDTE_R {
        THLDTE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX History List Enable"]
    #[inline(always)]
    #[must_use]
    pub fn thle(&mut self) -> THLE_W<0> {
        THLE_W::new(self)
    }
    #[doc = "Bit 8 - TX History List Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn thlie(&mut self) -> THLIE_W<8> {
        THLIE_W::new(self)
    }
    #[doc = "Bit 9 - TX History List Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn thlim(&mut self) -> THLIM_W<9> {
        THLIM_W::new(self)
    }
    #[doc = "Bit 10 - TX History List Dedicated TX Enable"]
    #[inline(always)]
    #[must_use]
    pub fn thldte(&mut self) -> THLDTE_W<10> {
        THLDTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX History List Configuration/Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdthlcc](index.html) module"]
pub struct CFDTHLCC_SPEC;
impl crate::RegisterSpec for CFDTHLCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdthlcc::R](R) reader structure"]
impl crate::Readable for CFDTHLCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdthlcc::W](W) writer structure"]
impl crate::Writable for CFDTHLCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTHLCC to value 0"]
impl crate::Resettable for CFDTHLCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
