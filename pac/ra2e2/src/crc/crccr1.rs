#[doc = "Register `CRCCR1` reader"]
pub struct R(crate::R<CRCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCR1` writer"]
pub struct W(crate::W<CRCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCR1_SPEC>;
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
impl From<crate::W<CRCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCSWR` reader - Snoop-On-Write/Read Switch"]
pub type CRCSWR_R = crate::BitReader<CRCSWR_A>;
#[doc = "Snoop-On-Write/Read Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSWR_A {
    #[doc = "0: Snoop-on-read"]
    _0 = 0,
    #[doc = "1: Snoop-on-write"]
    _1 = 1,
}
impl From<CRCSWR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSWR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCSWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCSWR_A {
        match self.bits {
            false => CRCSWR_A::_0,
            true => CRCSWR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCSWR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCSWR_A::_1
    }
}
#[doc = "Field `CRCSWR` writer - Snoop-On-Write/Read Switch"]
pub type CRCSWR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CRCCR1_SPEC, CRCSWR_A, O>;
impl<'a, const O: u8> CRCSWR_W<'a, O> {
    #[doc = "Snoop-on-read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCSWR_A::_0)
    }
    #[doc = "Snoop-on-write"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCSWR_A::_1)
    }
}
#[doc = "Field `CRCSEN` reader - Snoop Enable"]
pub type CRCSEN_R = crate::BitReader<CRCSEN_A>;
#[doc = "Snoop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CRCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCSEN_A {
        match self.bits {
            false => CRCSEN_A::_0,
            true => CRCSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCSEN_A::_1
    }
}
#[doc = "Field `CRCSEN` writer - Snoop Enable"]
pub type CRCSEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CRCCR1_SPEC, CRCSEN_A, O>;
impl<'a, const O: u8> CRCSEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCSEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - Snoop-On-Write/Read Switch"]
    #[inline(always)]
    pub fn crcswr(&self) -> CRCSWR_R {
        CRCSWR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Snoop Enable"]
    #[inline(always)]
    pub fn crcsen(&self) -> CRCSEN_R {
        CRCSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Snoop-On-Write/Read Switch"]
    #[inline(always)]
    #[must_use]
    pub fn crcswr(&mut self) -> CRCSWR_W<6> {
        CRCSWR_W::new(self)
    }
    #[doc = "Bit 7 - Snoop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcsen(&mut self) -> CRCSEN_W<7> {
        CRCSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccr1](index.html) module"]
pub struct CRCCR1_SPEC;
impl crate::RegisterSpec for CRCCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crccr1::R](R) reader structure"]
impl crate::Readable for CRCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crccr1::W](W) writer structure"]
impl crate::Writable for CRCCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCR1 to value 0"]
impl crate::Resettable for CRCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
