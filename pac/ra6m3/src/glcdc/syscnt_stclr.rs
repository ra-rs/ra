#[doc = "Register `SYSCNT_STCLR` reader"]
pub struct R(crate::R<SYSCNT_STCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCNT_STCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCNT_STCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCNT_STCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCNT_STCLR` writer"]
pub struct W(crate::W<SYSCNT_STCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCNT_STCLR_SPEC>;
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
impl From<crate::W<SYSCNT_STCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCNT_STCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPOSCLR` reader - Graphics 2 specified line detection flag clear field"]
pub type VPOSCLR_R = crate::BitReader<VPOSCLR_A>;
#[doc = "Graphics 2 specified line detection flag clear field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOSCLR_A {
    #[doc = "1: Clears the specified line detection flag."]
    _1 = 1,
    #[doc = "0: No operation"]
    _0 = 0,
}
impl From<VPOSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: VPOSCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl VPOSCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPOSCLR_A {
        match self.bits {
            true => VPOSCLR_A::_1,
            false => VPOSCLR_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOSCLR_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOSCLR_A::_0
    }
}
#[doc = "Field `VPOSCLR` writer - Graphics 2 specified line detection flag clear field"]
pub type VPOSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCNT_STCLR_SPEC, VPOSCLR_A, O>;
impl<'a, const O: u8> VPOSCLR_W<'a, O> {
    #[doc = "Clears the specified line detection flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VPOSCLR_A::_1)
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VPOSCLR_A::_0)
    }
}
#[doc = "Field `L1UNDFCLR` reader - Graphics 1 underflow detection flag clear field"]
pub type L1UNDFCLR_R = crate::BitReader<L1UNDFCLR_A>;
#[doc = "Graphics 1 underflow detection flag clear field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDFCLR_A {
    #[doc = "1: Clears the graphics 1 underflow detection flag."]
    _1 = 1,
    #[doc = "0: No operation"]
    _0 = 0,
}
impl From<L1UNDFCLR_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDFCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl L1UNDFCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1UNDFCLR_A {
        match self.bits {
            true => L1UNDFCLR_A::_1,
            false => L1UNDFCLR_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDFCLR_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDFCLR_A::_0
    }
}
#[doc = "Field `L1UNDFCLR` writer - Graphics 1 underflow detection flag clear field"]
pub type L1UNDFCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCNT_STCLR_SPEC, L1UNDFCLR_A, O>;
impl<'a, const O: u8> L1UNDFCLR_W<'a, O> {
    #[doc = "Clears the graphics 1 underflow detection flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L1UNDFCLR_A::_1)
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L1UNDFCLR_A::_0)
    }
}
#[doc = "Field `L2UNDFCLR` reader - Graphics 2 underflow detection flag clear field"]
pub type L2UNDFCLR_R = crate::BitReader<L2UNDFCLR_A>;
#[doc = "Graphics 2 underflow detection flag clear field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDFCLR_A {
    #[doc = "1: Clears the graphics 2 underflow detection flag."]
    _1 = 1,
    #[doc = "0: No operation"]
    _0 = 0,
}
impl From<L2UNDFCLR_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDFCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl L2UNDFCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L2UNDFCLR_A {
        match self.bits {
            true => L2UNDFCLR_A::_1,
            false => L2UNDFCLR_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDFCLR_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDFCLR_A::_0
    }
}
#[doc = "Field `L2UNDFCLR` writer - Graphics 2 underflow detection flag clear field"]
pub type L2UNDFCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCNT_STCLR_SPEC, L2UNDFCLR_A, O>;
impl<'a, const O: u8> L2UNDFCLR_W<'a, O> {
    #[doc = "Clears the graphics 2 underflow detection flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L2UNDFCLR_A::_1)
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L2UNDFCLR_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Graphics 2 specified line detection flag clear field"]
    #[inline(always)]
    pub fn vposclr(&self) -> VPOSCLR_R {
        VPOSCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Graphics 1 underflow detection flag clear field"]
    #[inline(always)]
    pub fn l1undfclr(&self) -> L1UNDFCLR_R {
        L1UNDFCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Graphics 2 underflow detection flag clear field"]
    #[inline(always)]
    pub fn l2undfclr(&self) -> L2UNDFCLR_R {
        L2UNDFCLR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Graphics 2 specified line detection flag clear field"]
    #[inline(always)]
    #[must_use]
    pub fn vposclr(&mut self) -> VPOSCLR_W<0> {
        VPOSCLR_W::new(self)
    }
    #[doc = "Bit 1 - Graphics 1 underflow detection flag clear field"]
    #[inline(always)]
    #[must_use]
    pub fn l1undfclr(&mut self) -> L1UNDFCLR_W<1> {
        L1UNDFCLR_W::new(self)
    }
    #[doc = "Bit 2 - Graphics 2 underflow detection flag clear field"]
    #[inline(always)]
    #[must_use]
    pub fn l2undfclr(&mut self) -> L2UNDFCLR_W<2> {
        L2UNDFCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Block Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscnt_stclr](index.html) module"]
pub struct SYSCNT_STCLR_SPEC;
impl crate::RegisterSpec for SYSCNT_STCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscnt_stclr::R](R) reader structure"]
impl crate::Readable for SYSCNT_STCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscnt_stclr::W](W) writer structure"]
impl crate::Writable for SYSCNT_STCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCNT_STCLR to value 0"]
impl crate::Resettable for SYSCNT_STCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
