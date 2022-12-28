#[doc = "Register `PIPE%sTRE` reader"]
pub struct R(crate::R<PIPETRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPETRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPETRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPETRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPE%sTRE` writer"]
pub struct W(crate::W<PIPETRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPETRE_SPEC>;
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
impl From<crate::W<PIPETRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPETRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRCLR` reader - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0."]
pub type TRCLR_R = crate::BitReader<TRCLR_A>;
#[doc = "Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRCLR_A {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: The current counter value is cleared."]
    _1 = 1,
}
impl From<TRCLR_A> for bool {
    #[inline(always)]
    fn from(variant: TRCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRCLR_A {
        match self.bits {
            false => TRCLR_A::_0,
            true => TRCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRCLR_A::_1
    }
}
#[doc = "Field `TRCLR` writer - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0."]
pub type TRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPETRE_SPEC, TRCLR_A, O>;
impl<'a, const O: u8> TRCLR_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRCLR_A::_0)
    }
    #[doc = "The current counter value is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRCLR_A::_1)
    }
}
#[doc = "Field `TRENB` reader - Transaction Counter EnableEnables or disables the transaction counter function."]
pub type TRENB_R = crate::BitReader<TRENB_A>;
#[doc = "Transaction Counter EnableEnables or disables the transaction counter function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRENB_A {
    #[doc = "0: The transaction counter function is disabled."]
    _0 = 0,
    #[doc = "1: The transaction counter function is enabled."]
    _1 = 1,
}
impl From<TRENB_A> for bool {
    #[inline(always)]
    fn from(variant: TRENB_A) -> Self {
        variant as u8 != 0
    }
}
impl TRENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRENB_A {
        match self.bits {
            false => TRENB_A::_0,
            true => TRENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRENB_A::_1
    }
}
#[doc = "Field `TRENB` writer - Transaction Counter EnableEnables or disables the transaction counter function."]
pub type TRENB_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPETRE_SPEC, TRENB_A, O>;
impl<'a, const O: u8> TRENB_W<'a, O> {
    #[doc = "The transaction counter function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRENB_A::_0)
    }
    #[doc = "The transaction counter function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRENB_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0."]
    #[inline(always)]
    pub fn trclr(&self) -> TRCLR_R {
        TRCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transaction Counter EnableEnables or disables the transaction counter function."]
    #[inline(always)]
    pub fn trenb(&self) -> TRENB_R {
        TRENB_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0."]
    #[inline(always)]
    #[must_use]
    pub fn trclr(&mut self) -> TRCLR_W<8> {
        TRCLR_W::new(self)
    }
    #[doc = "Bit 9 - Transaction Counter EnableEnables or disables the transaction counter function."]
    #[inline(always)]
    #[must_use]
    pub fn trenb(&mut self) -> TRENB_W<9> {
        TRENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIPE Transaction Counter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipetre](index.html) module"]
pub struct PIPETRE_SPEC;
impl crate::RegisterSpec for PIPETRE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipetre::R](R) reader structure"]
impl crate::Readable for PIPETRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipetre::W](W) writer structure"]
impl crate::Writable for PIPETRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPE%sTRE to value 0"]
impl crate::Resettable for PIPETRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
