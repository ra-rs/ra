#[doc = "Register `CTSUCHTRCB` reader"]
pub struct R(crate::R<CTSUCHTRCB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHTRCB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHTRCB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHTRCB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHTRCB` writer"]
pub struct W(crate::W<CTSUCHTRCB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHTRCB_SPEC>;
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
impl From<crate::W<CTSUCHTRCB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHTRCB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHTRC32` reader - CTSU Channel Transmit/Receive Control B"]
pub type CHTRC32_R = crate::BitReader<CHTRC32_A>;
#[doc = "CTSU Channel Transmit/Receive Control B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC32_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC32_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC32_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC32_A {
        match self.bits {
            false => CHTRC32_A::_0,
            true => CHTRC32_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC32_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC32_A::_1
    }
}
#[doc = "Field `CHTRC32` writer - CTSU Channel Transmit/Receive Control B"]
pub type CHTRC32_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCB_SPEC, CHTRC32_A, O>;
impl<'a, const O: u8> CHTRC32_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC32_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC32_A::_1)
    }
}
#[doc = "Field `CHTRC33` reader - CTSU Channel Transmit/Receive Control B"]
pub type CHTRC33_R = crate::BitReader<CHTRC33_A>;
#[doc = "CTSU Channel Transmit/Receive Control B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC33_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC33_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC33_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC33_A {
        match self.bits {
            false => CHTRC33_A::_0,
            true => CHTRC33_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC33_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC33_A::_1
    }
}
#[doc = "Field `CHTRC33` writer - CTSU Channel Transmit/Receive Control B"]
pub type CHTRC33_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCB_SPEC, CHTRC33_A, O>;
impl<'a, const O: u8> CHTRC33_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC33_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC33_A::_1)
    }
}
#[doc = "Field `CHTRC34` reader - CTSU Channel Transmit/Receive Control B"]
pub type CHTRC34_R = crate::BitReader<CHTRC34_A>;
#[doc = "CTSU Channel Transmit/Receive Control B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTRC34_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CHTRC34_A> for bool {
    #[inline(always)]
    fn from(variant: CHTRC34_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTRC34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHTRC34_A {
        match self.bits {
            false => CHTRC34_A::_0,
            true => CHTRC34_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHTRC34_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHTRC34_A::_1
    }
}
#[doc = "Field `CHTRC34` writer - CTSU Channel Transmit/Receive Control B"]
pub type CHTRC34_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSUCHTRCB_SPEC, CHTRC34_A, O>;
impl<'a, const O: u8> CHTRC34_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHTRC34_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHTRC34_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Channel Transmit/Receive Control B"]
    #[inline(always)]
    pub fn chtrc32(&self) -> CHTRC32_R {
        CHTRC32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU Channel Transmit/Receive Control B"]
    #[inline(always)]
    pub fn chtrc33(&self) -> CHTRC33_R {
        CHTRC33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Channel Transmit/Receive Control B"]
    #[inline(always)]
    pub fn chtrc34(&self) -> CHTRC34_R {
        CHTRC34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Channel Transmit/Receive Control B"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc32(&mut self) -> CHTRC32_W<0> {
        CHTRC32_W::new(self)
    }
    #[doc = "Bit 1 - CTSU Channel Transmit/Receive Control B"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc33(&mut self) -> CHTRC33_W<1> {
        CHTRC33_W::new(self)
    }
    #[doc = "Bit 2 - CTSU Channel Transmit/Receive Control B"]
    #[inline(always)]
    #[must_use]
    pub fn chtrc34(&mut self) -> CHTRC34_W<2> {
        CHTRC34_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchtrcb](index.html) module"]
pub struct CTSUCHTRCB_SPEC;
impl crate::RegisterSpec for CTSUCHTRCB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsuchtrcb::R](R) reader structure"]
impl crate::Readable for CTSUCHTRCB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchtrcb::W](W) writer structure"]
impl crate::Writable for CTSUCHTRCB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHTRCB to value 0"]
impl crate::Resettable for CTSUCHTRCB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
