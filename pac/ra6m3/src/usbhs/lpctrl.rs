#[doc = "Register `LPCTRL` reader"]
pub struct R(crate::R<LPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCTRL` writer"]
pub struct W(crate::W<LPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCTRL_SPEC>;
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
impl From<crate::W<LPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWUPM` reader - Resume Return Mode Setting"]
pub type HWUPM_R = crate::BitReader<HWUPM_A>;
#[doc = "Resume Return Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWUPM_A {
    #[doc = "0: Hardware does not recover while CPU clock inactive"]
    _0 = 0,
    #[doc = "1: Hardware recovers while CPU clock inactive."]
    _1 = 1,
}
impl From<HWUPM_A> for bool {
    #[inline(always)]
    fn from(variant: HWUPM_A) -> Self {
        variant as u8 != 0
    }
}
impl HWUPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWUPM_A {
        match self.bits {
            false => HWUPM_A::_0,
            true => HWUPM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWUPM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWUPM_A::_1
    }
}
#[doc = "Field `HWUPM` writer - Resume Return Mode Setting"]
pub type HWUPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, LPCTRL_SPEC, HWUPM_A, O>;
impl<'a, const O: u8> HWUPM_W<'a, O> {
    #[doc = "Hardware does not recover while CPU clock inactive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWUPM_A::_0)
    }
    #[doc = "Hardware recovers while CPU clock inactive."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWUPM_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Resume Return Mode Setting"]
    #[inline(always)]
    pub fn hwupm(&self) -> HWUPM_R {
        HWUPM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Resume Return Mode Setting"]
    #[inline(always)]
    #[must_use]
    pub fn hwupm(&mut self) -> HWUPM_W<7> {
        HWUPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpctrl](index.html) module"]
pub struct LPCTRL_SPEC;
impl crate::RegisterSpec for LPCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lpctrl::R](R) reader structure"]
impl crate::Readable for LPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpctrl::W](W) writer structure"]
impl crate::Writable for LPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPCTRL to value 0"]
impl crate::Resettable for LPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
