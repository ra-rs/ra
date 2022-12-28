#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFSF` reader - Start Frame Status Flag"]
pub type SFSF_R = crate::BitReader<SFSF_A>;
#[doc = "Start Frame Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFSF_A {
    #[doc = "0: Start Frame detection function is disabled."]
    _0 = 0,
    #[doc = "1: Start Frame detection function is enabled."]
    _1 = 1,
}
impl From<SFSF_A> for bool {
    #[inline(always)]
    fn from(variant: SFSF_A) -> Self {
        variant as u8 != 0
    }
}
impl SFSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFSF_A {
        match self.bits {
            false => SFSF_A::_0,
            true => SFSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFSF_A::_1
    }
}
#[doc = "Field `RXDSF` reader - RXDXn Input Status Flag"]
pub type RXDSF_R = crate::BitReader<RXDSF_A>;
#[doc = "RXDXn Input Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDSF_A {
    #[doc = "0: RXDXn input is enabled."]
    _0 = 0,
    #[doc = "1: RXDXn input is disabled."]
    _1 = 1,
}
impl From<RXDSF_A> for bool {
    #[inline(always)]
    fn from(variant: RXDSF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDSF_A {
        match self.bits {
            false => RXDSF_A::_0,
            true => RXDSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDSF_A::_1
    }
}
#[doc = "Field `BRME` reader - Bit Rate Measurement Enable"]
pub type BRME_R = crate::BitReader<BRME_A>;
#[doc = "Bit Rate Measurement Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRME_A {
    #[doc = "0: Measurement of bit rate is disabled."]
    _0 = 0,
    #[doc = "1: Measurement of bit rate is enabled."]
    _1 = 1,
}
impl From<BRME_A> for bool {
    #[inline(always)]
    fn from(variant: BRME_A) -> Self {
        variant as u8 != 0
    }
}
impl BRME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRME_A {
        match self.bits {
            false => BRME_A::_0,
            true => BRME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRME_A::_1
    }
}
#[doc = "Field `BRME` writer - Bit Rate Measurement Enable"]
pub type BRME_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR0_SPEC, BRME_A, O>;
impl<'a, const O: u8> BRME_W<'a, O> {
    #[doc = "Measurement of bit rate is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRME_A::_0)
    }
    #[doc = "Measurement of bit rate is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRME_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Start Frame Status Flag"]
    #[inline(always)]
    pub fn sfsf(&self) -> SFSF_R {
        SFSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXDXn Input Status Flag"]
    #[inline(always)]
    pub fn rxdsf(&self) -> RXDSF_R {
        RXDSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rate Measurement Enable"]
    #[inline(always)]
    pub fn brme(&self) -> BRME_R {
        BRME_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Bit Rate Measurement Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brme(&mut self) -> BRME_W<3> {
        BRME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
