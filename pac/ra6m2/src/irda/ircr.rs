#[doc = "Register `IRCR` reader"]
pub struct R(crate::R<IRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRCR` writer"]
pub struct W(crate::W<IRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRCR_SPEC>;
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
impl From<crate::W<IRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRRXINV` reader - IRRXD Polarity Switching"]
pub type IRRXINV_R = crate::BitReader<IRRXINV_A>;
#[doc = "IRRXD Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRRXINV_A {
    #[doc = "0: IRRXD input is used as received data as is."]
    _0 = 0,
    #[doc = "1: IRRXD input is used as received data after the polarity is inverted."]
    _1 = 1,
}
impl From<IRRXINV_A> for bool {
    #[inline(always)]
    fn from(variant: IRRXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl IRRXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRRXINV_A {
        match self.bits {
            false => IRRXINV_A::_0,
            true => IRRXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRRXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRRXINV_A::_1
    }
}
#[doc = "Field `IRRXINV` writer - IRRXD Polarity Switching"]
pub type IRRXINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRCR_SPEC, IRRXINV_A, O>;
impl<'a, const O: u8> IRRXINV_W<'a, O> {
    #[doc = "IRRXD input is used as received data as is."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRRXINV_A::_0)
    }
    #[doc = "IRRXD input is used as received data after the polarity is inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRRXINV_A::_1)
    }
}
#[doc = "Field `IRTXINV` reader - IRTXD Polarity Switching"]
pub type IRTXINV_R = crate::BitReader<IRTXINV_A>;
#[doc = "IRTXD Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRTXINV_A {
    #[doc = "0: Data to be transmitted is output to IRTXD as is."]
    _0 = 0,
    #[doc = "1: Data to be transmitted is output to IRTXD after the polarity is inverted."]
    _1 = 1,
}
impl From<IRTXINV_A> for bool {
    #[inline(always)]
    fn from(variant: IRTXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl IRTXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRTXINV_A {
        match self.bits {
            false => IRTXINV_A::_0,
            true => IRTXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRTXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRTXINV_A::_1
    }
}
#[doc = "Field `IRTXINV` writer - IRTXD Polarity Switching"]
pub type IRTXINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRCR_SPEC, IRTXINV_A, O>;
impl<'a, const O: u8> IRTXINV_W<'a, O> {
    #[doc = "Data to be transmitted is output to IRTXD as is."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRTXINV_A::_0)
    }
    #[doc = "Data to be transmitted is output to IRTXD after the polarity is inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRTXINV_A::_1)
    }
}
#[doc = "Field `IRE` reader - IrDA Enable"]
pub type IRE_R = crate::BitReader<IRE_A>;
#[doc = "IrDA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRE_A {
    #[doc = "0: Serial I/O pins are used for normal serial communication."]
    _0 = 0,
    #[doc = "1: Serial I/O pins are used for IrDA data communication."]
    _1 = 1,
}
impl From<IRE_A> for bool {
    #[inline(always)]
    fn from(variant: IRE_A) -> Self {
        variant as u8 != 0
    }
}
impl IRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRE_A {
        match self.bits {
            false => IRE_A::_0,
            true => IRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRE_A::_1
    }
}
#[doc = "Field `IRE` writer - IrDA Enable"]
pub type IRE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IRCR_SPEC, IRE_A, O>;
impl<'a, const O: u8> IRE_W<'a, O> {
    #[doc = "Serial I/O pins are used for normal serial communication."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRE_A::_0)
    }
    #[doc = "Serial I/O pins are used for IrDA data communication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRE_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - IRRXD Polarity Switching"]
    #[inline(always)]
    pub fn irrxinv(&self) -> IRRXINV_R {
        IRRXINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRTXD Polarity Switching"]
    #[inline(always)]
    pub fn irtxinv(&self) -> IRTXINV_R {
        IRTXINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - IrDA Enable"]
    #[inline(always)]
    pub fn ire(&self) -> IRE_R {
        IRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - IRRXD Polarity Switching"]
    #[inline(always)]
    #[must_use]
    pub fn irrxinv(&mut self) -> IRRXINV_W<2> {
        IRRXINV_W::new(self)
    }
    #[doc = "Bit 3 - IRTXD Polarity Switching"]
    #[inline(always)]
    #[must_use]
    pub fn irtxinv(&mut self) -> IRTXINV_W<3> {
        IRTXINV_W::new(self)
    }
    #[doc = "Bit 7 - IrDA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ire(&mut self) -> IRE_W<7> {
        IRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ircr](index.html) module"]
pub struct IRCR_SPEC;
impl crate::RegisterSpec for IRCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ircr::R](R) reader structure"]
impl crate::Readable for IRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ircr::W](W) writer structure"]
impl crate::Writable for IRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRCR to value 0"]
impl crate::Resettable for IRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
