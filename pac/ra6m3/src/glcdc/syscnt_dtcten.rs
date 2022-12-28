#[doc = "Register `SYSCNT_DTCTEN` reader"]
pub struct R(crate::R<SYSCNT_DTCTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCNT_DTCTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCNT_DTCTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCNT_DTCTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCNT_DTCTEN` writer"]
pub struct W(crate::W<SYSCNT_DTCTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCNT_DTCTEN_SPEC>;
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
impl From<crate::W<SYSCNT_DTCTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCNT_DTCTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPOSDTC` reader - Specified line detection control"]
pub type VPOSDTC_R = crate::BitReader<VPOSDTC_A>;
#[doc = "Specified line detection control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOSDTC_A {
    #[doc = "1: Enables detection."]
    _1 = 1,
    #[doc = "0: Disables detection."]
    _0 = 0,
}
impl From<VPOSDTC_A> for bool {
    #[inline(always)]
    fn from(variant: VPOSDTC_A) -> Self {
        variant as u8 != 0
    }
}
impl VPOSDTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPOSDTC_A {
        match self.bits {
            true => VPOSDTC_A::_1,
            false => VPOSDTC_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOSDTC_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOSDTC_A::_0
    }
}
#[doc = "Field `VPOSDTC` writer - Specified line detection control"]
pub type VPOSDTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCNT_DTCTEN_SPEC, VPOSDTC_A, O>;
impl<'a, const O: u8> VPOSDTC_W<'a, O> {
    #[doc = "Enables detection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VPOSDTC_A::_1)
    }
    #[doc = "Disables detection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VPOSDTC_A::_0)
    }
}
#[doc = "Field `L1UNDFDTC` reader - Graphics 1 underflow detection control"]
pub type L1UNDFDTC_R = crate::BitReader<L1UNDFDTC_A>;
#[doc = "Graphics 1 underflow detection control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDFDTC_A {
    #[doc = "1: Enables detection."]
    _1 = 1,
    #[doc = "0: Disables detection."]
    _0 = 0,
}
impl From<L1UNDFDTC_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDFDTC_A) -> Self {
        variant as u8 != 0
    }
}
impl L1UNDFDTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1UNDFDTC_A {
        match self.bits {
            true => L1UNDFDTC_A::_1,
            false => L1UNDFDTC_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDFDTC_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDFDTC_A::_0
    }
}
#[doc = "Field `L1UNDFDTC` writer - Graphics 1 underflow detection control"]
pub type L1UNDFDTC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCNT_DTCTEN_SPEC, L1UNDFDTC_A, O>;
impl<'a, const O: u8> L1UNDFDTC_W<'a, O> {
    #[doc = "Enables detection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L1UNDFDTC_A::_1)
    }
    #[doc = "Disables detection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L1UNDFDTC_A::_0)
    }
}
#[doc = "Field `L2UNDFDTC` reader - Graphics 2 underflow detection control"]
pub type L2UNDFDTC_R = crate::BitReader<L2UNDFDTC_A>;
#[doc = "Graphics 2 underflow detection control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDFDTC_A {
    #[doc = "1: Enables detection."]
    _1 = 1,
    #[doc = "0: Disables detection."]
    _0 = 0,
}
impl From<L2UNDFDTC_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDFDTC_A) -> Self {
        variant as u8 != 0
    }
}
impl L2UNDFDTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L2UNDFDTC_A {
        match self.bits {
            true => L2UNDFDTC_A::_1,
            false => L2UNDFDTC_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDFDTC_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDFDTC_A::_0
    }
}
#[doc = "Field `L2UNDFDTC` writer - Graphics 2 underflow detection control"]
pub type L2UNDFDTC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCNT_DTCTEN_SPEC, L2UNDFDTC_A, O>;
impl<'a, const O: u8> L2UNDFDTC_W<'a, O> {
    #[doc = "Enables detection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L2UNDFDTC_A::_1)
    }
    #[doc = "Disables detection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L2UNDFDTC_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Specified line detection control"]
    #[inline(always)]
    pub fn vposdtc(&self) -> VPOSDTC_R {
        VPOSDTC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Graphics 1 underflow detection control"]
    #[inline(always)]
    pub fn l1undfdtc(&self) -> L1UNDFDTC_R {
        L1UNDFDTC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Graphics 2 underflow detection control"]
    #[inline(always)]
    pub fn l2undfdtc(&self) -> L2UNDFDTC_R {
        L2UNDFDTC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specified line detection control"]
    #[inline(always)]
    #[must_use]
    pub fn vposdtc(&mut self) -> VPOSDTC_W<0> {
        VPOSDTC_W::new(self)
    }
    #[doc = "Bit 1 - Graphics 1 underflow detection control"]
    #[inline(always)]
    #[must_use]
    pub fn l1undfdtc(&mut self) -> L1UNDFDTC_W<1> {
        L1UNDFDTC_W::new(self)
    }
    #[doc = "Bit 2 - Graphics 2 underflow detection control"]
    #[inline(always)]
    #[must_use]
    pub fn l2undfdtc(&mut self) -> L2UNDFDTC_W<2> {
        L2UNDFDTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Block State Detection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscnt_dtcten](index.html) module"]
pub struct SYSCNT_DTCTEN_SPEC;
impl crate::RegisterSpec for SYSCNT_DTCTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscnt_dtcten::R](R) reader structure"]
impl crate::Readable for SYSCNT_DTCTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscnt_dtcten::W](W) writer structure"]
impl crate::Writable for SYSCNT_DTCTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCNT_DTCTEN to value 0"]
impl crate::Resettable for SYSCNT_DTCTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
