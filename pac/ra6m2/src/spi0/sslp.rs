#[doc = "Register `SSLP` reader"]
pub struct R(crate::R<SSLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSLP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSLP` writer"]
pub struct W(crate::W<SSLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSLP_SPEC>;
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
impl From<crate::W<SSLP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSL0P` reader - SSL0 Signal Polarity Setting"]
pub type SSL0P_R = crate::BitReader<SSL0P_A>;
#[doc = "SSL0 Signal Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL0P_A {
    #[doc = "0: SSL0 signal is active low"]
    _0 = 0,
    #[doc = "1: SSL0 signal is active high"]
    _1 = 1,
}
impl From<SSL0P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL0P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL0P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL0P_A {
        match self.bits {
            false => SSL0P_A::_0,
            true => SSL0P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL0P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL0P_A::_1
    }
}
#[doc = "Field `SSL0P` writer - SSL0 Signal Polarity Setting"]
pub type SSL0P_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSLP_SPEC, SSL0P_A, O>;
impl<'a, const O: u8> SSL0P_W<'a, O> {
    #[doc = "SSL0 signal is active low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL0P_A::_0)
    }
    #[doc = "SSL0 signal is active high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL0P_A::_1)
    }
}
#[doc = "Field `SSL1P` reader - SSL1 Signal Polarity Setting"]
pub type SSL1P_R = crate::BitReader<SSL1P_A>;
#[doc = "SSL1 Signal Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL1P_A {
    #[doc = "0: SSL1 signal is active low"]
    _0 = 0,
    #[doc = "1: SSL1 signal is active high"]
    _1 = 1,
}
impl From<SSL1P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL1P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL1P_A {
        match self.bits {
            false => SSL1P_A::_0,
            true => SSL1P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL1P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL1P_A::_1
    }
}
#[doc = "Field `SSL1P` writer - SSL1 Signal Polarity Setting"]
pub type SSL1P_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSLP_SPEC, SSL1P_A, O>;
impl<'a, const O: u8> SSL1P_W<'a, O> {
    #[doc = "SSL1 signal is active low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL1P_A::_0)
    }
    #[doc = "SSL1 signal is active high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL1P_A::_1)
    }
}
#[doc = "Field `SSL2P` reader - SSL2 Signal Polarity Setting"]
pub type SSL2P_R = crate::BitReader<SSL2P_A>;
#[doc = "SSL2 Signal Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL2P_A {
    #[doc = "0: SSL2 signal is active low"]
    _0 = 0,
    #[doc = "1: SSL2 signal is active high"]
    _1 = 1,
}
impl From<SSL2P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL2P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL2P_A {
        match self.bits {
            false => SSL2P_A::_0,
            true => SSL2P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL2P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL2P_A::_1
    }
}
#[doc = "Field `SSL2P` writer - SSL2 Signal Polarity Setting"]
pub type SSL2P_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSLP_SPEC, SSL2P_A, O>;
impl<'a, const O: u8> SSL2P_W<'a, O> {
    #[doc = "SSL2 signal is active low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL2P_A::_0)
    }
    #[doc = "SSL2 signal is active high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL2P_A::_1)
    }
}
#[doc = "Field `SSL3P` reader - SSL3 Signal Polarity Setting"]
pub type SSL3P_R = crate::BitReader<SSL3P_A>;
#[doc = "SSL3 Signal Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL3P_A {
    #[doc = "0: SSL3 signal is active low"]
    _0 = 0,
    #[doc = "1: SSL3 signal is active high"]
    _1 = 1,
}
impl From<SSL3P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL3P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL3P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL3P_A {
        match self.bits {
            false => SSL3P_A::_0,
            true => SSL3P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL3P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL3P_A::_1
    }
}
#[doc = "Field `SSL3P` writer - SSL3 Signal Polarity Setting"]
pub type SSL3P_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSLP_SPEC, SSL3P_A, O>;
impl<'a, const O: u8> SSL3P_W<'a, O> {
    #[doc = "SSL3 signal is active low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL3P_A::_0)
    }
    #[doc = "SSL3 signal is active high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL3P_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SSL0 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl0p(&self) -> SSL0P_R {
        SSL0P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSL1 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl1p(&self) -> SSL1P_R {
        SSL1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSL2 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl2p(&self) -> SSL2P_R {
        SSL2P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSL3 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl3p(&self) -> SSL3P_R {
        SSL3P_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSL0 Signal Polarity Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ssl0p(&mut self) -> SSL0P_W<0> {
        SSL0P_W::new(self)
    }
    #[doc = "Bit 1 - SSL1 Signal Polarity Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ssl1p(&mut self) -> SSL1P_W<1> {
        SSL1P_W::new(self)
    }
    #[doc = "Bit 2 - SSL2 Signal Polarity Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ssl2p(&mut self) -> SSL2P_W<2> {
        SSL2P_W::new(self)
    }
    #[doc = "Bit 3 - SSL3 Signal Polarity Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ssl3p(&mut self) -> SSL3P_W<3> {
        SSL3P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Slave Select Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sslp](index.html) module"]
pub struct SSLP_SPEC;
impl crate::RegisterSpec for SSLP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sslp::R](R) reader structure"]
impl crate::Readable for SSLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sslp::W](W) writer structure"]
impl crate::Writable for SSLP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSLP to value 0"]
impl crate::Resettable for SSLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
