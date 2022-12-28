#[doc = "Register `SYFORMR` reader"]
pub struct R(crate::R<SYFORMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYFORMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYFORMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYFORMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYFORMR` writer"]
pub struct W(crate::W<SYFORMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYFORMR_SPEC>;
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
impl From<crate::W<SYFORMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYFORMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORM0` reader - Ethernet/UDP Encapsulation"]
pub type FORM0_R = crate::BitReader<FORM0_A>;
#[doc = "Ethernet/UDP Encapsulation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORM0_A {
    #[doc = "0: PTP directly over Ethernet"]
    _0 = 0,
    #[doc = "1: PTP over UDP/IPv4"]
    _1 = 1,
}
impl From<FORM0_A> for bool {
    #[inline(always)]
    fn from(variant: FORM0_A) -> Self {
        variant as u8 != 0
    }
}
impl FORM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORM0_A {
        match self.bits {
            false => FORM0_A::_0,
            true => FORM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FORM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FORM0_A::_1
    }
}
#[doc = "Field `FORM0` writer - Ethernet/UDP Encapsulation"]
pub type FORM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYFORMR_SPEC, FORM0_A, O>;
impl<'a, const O: u8> FORM0_W<'a, O> {
    #[doc = "PTP directly over Ethernet"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORM0_A::_0)
    }
    #[doc = "PTP over UDP/IPv4"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORM0_A::_1)
    }
}
#[doc = "Field `FORM1` reader - Ethernet Frame Format Setting"]
pub type FORM1_R = crate::BitReader<FORM1_A>;
#[doc = "Ethernet Frame Format Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORM1_A {
    #[doc = "0: Set this bit to 0 (Ethernet II frame format)."]
    _0 = 0,
    #[doc = "1: Setting prohibited"]
    _1 = 1,
}
impl From<FORM1_A> for bool {
    #[inline(always)]
    fn from(variant: FORM1_A) -> Self {
        variant as u8 != 0
    }
}
impl FORM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORM1_A {
        match self.bits {
            false => FORM1_A::_0,
            true => FORM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FORM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FORM1_A::_1
    }
}
#[doc = "Field `FORM1` writer - Ethernet Frame Format Setting"]
pub type FORM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYFORMR_SPEC, FORM1_A, O>;
impl<'a, const O: u8> FORM1_W<'a, O> {
    #[doc = "Set this bit to 0 (Ethernet II frame format)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORM1_A::_0)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORM1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Ethernet/UDP Encapsulation"]
    #[inline(always)]
    pub fn form0(&self) -> FORM0_R {
        FORM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ethernet Frame Format Setting"]
    #[inline(always)]
    pub fn form1(&self) -> FORM1_R {
        FORM1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet/UDP Encapsulation"]
    #[inline(always)]
    #[must_use]
    pub fn form0(&mut self) -> FORM0_W<0> {
        FORM0_W::new(self)
    }
    #[doc = "Bit 1 - Ethernet Frame Format Setting"]
    #[inline(always)]
    #[must_use]
    pub fn form1(&mut self) -> FORM1_W<1> {
        FORM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Frame Format Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syformr](index.html) module"]
pub struct SYFORMR_SPEC;
impl crate::RegisterSpec for SYFORMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syformr::R](R) reader structure"]
impl crate::Readable for SYFORMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syformr::W](W) writer structure"]
impl crate::Writable for SYFORMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYFORMR to value 0"]
impl crate::Resettable for SYFORMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
