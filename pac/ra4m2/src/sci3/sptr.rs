#[doc = "Register `SPTR` reader"]
pub struct R(crate::R<SPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPTR` writer"]
pub struct W(crate::W<SPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPTR_SPEC>;
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
impl From<crate::W<SPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDMON` reader - Serial Input Data Monitor"]
pub type RXDMON_R = crate::BitReader<bool>;
#[doc = "Field `SPB2DT` reader - Serial Port Break Data Select"]
pub type SPB2DT_R = crate::BitReader<bool>;
#[doc = "Field `SPB2DT` writer - Serial Port Break Data Select"]
pub type SPB2DT_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPTR_SPEC, bool, O>;
#[doc = "Field `SPB2IO` reader - Serial Port Break I/O"]
pub type SPB2IO_R = crate::BitReader<SPB2IO_A>;
#[doc = "Serial Port Break I/O\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPB2IO_A {
    #[doc = "0: Do not output value of SPB2DT bit on TXDn pin"]
    _0 = 0,
    #[doc = "1: Output value of SPB2DT bit on TXDn pin"]
    _1 = 1,
}
impl From<SPB2IO_A> for bool {
    #[inline(always)]
    fn from(variant: SPB2IO_A) -> Self {
        variant as u8 != 0
    }
}
impl SPB2IO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPB2IO_A {
        match self.bits {
            false => SPB2IO_A::_0,
            true => SPB2IO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPB2IO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPB2IO_A::_1
    }
}
#[doc = "Field `SPB2IO` writer - Serial Port Break I/O"]
pub type SPB2IO_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPTR_SPEC, SPB2IO_A, O>;
impl<'a, const O: u8> SPB2IO_W<'a, O> {
    #[doc = "Do not output value of SPB2DT bit on TXDn pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPB2IO_A::_0)
    }
    #[doc = "Output value of SPB2DT bit on TXDn pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPB2IO_A::_1)
    }
}
#[doc = "Field `RINV` reader - RXD invert bit"]
pub type RINV_R = crate::BitReader<RINV_A>;
#[doc = "RXD invert bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RINV_A {
    #[doc = "0: Received data from RXDn is not inverted and input."]
    _0 = 0,
    #[doc = "1: Received data from RXDn is inverted and input."]
    _1 = 1,
}
impl From<RINV_A> for bool {
    #[inline(always)]
    fn from(variant: RINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RINV_A {
        match self.bits {
            false => RINV_A::_0,
            true => RINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RINV_A::_1
    }
}
#[doc = "Field `RINV` writer - RXD invert bit"]
pub type RINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPTR_SPEC, RINV_A, O>;
impl<'a, const O: u8> RINV_W<'a, O> {
    #[doc = "Received data from RXDn is not inverted and input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RINV_A::_0)
    }
    #[doc = "Received data from RXDn is inverted and input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RINV_A::_1)
    }
}
#[doc = "Field `TINV` reader - TXD invert bit"]
pub type TINV_R = crate::BitReader<TINV_A>;
#[doc = "TXD invert bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TINV_A {
    #[doc = "0: Transmit data is not inverted and output to TXDn."]
    _0 = 0,
    #[doc = "1: Transmit data is inverted and output to TXDn."]
    _1 = 1,
}
impl From<TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TINV_A) -> Self {
        variant as u8 != 0
    }
}
impl TINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TINV_A {
        match self.bits {
            false => TINV_A::_0,
            true => TINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TINV_A::_1
    }
}
#[doc = "Field `TINV` writer - TXD invert bit"]
pub type TINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPTR_SPEC, TINV_A, O>;
impl<'a, const O: u8> TINV_W<'a, O> {
    #[doc = "Transmit data is not inverted and output to TXDn."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TINV_A::_0)
    }
    #[doc = "Transmit data is inverted and output to TXDn."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TINV_A::_1)
    }
}
#[doc = "Field `ASEN` reader - Adjust receive sampling timing enable"]
pub type ASEN_R = crate::BitReader<ASEN_A>;
#[doc = "Adjust receive sampling timing enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEN_A {
    #[doc = "0: Adjust sampling timing disable."]
    _0 = 0,
    #[doc = "1: Adjust sampling timing enable."]
    _1 = 1,
}
impl From<ASEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASEN_A {
        match self.bits {
            false => ASEN_A::_0,
            true => ASEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASEN_A::_1
    }
}
#[doc = "Field `ASEN` writer - Adjust receive sampling timing enable"]
pub type ASEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPTR_SPEC, ASEN_A, O>;
impl<'a, const O: u8> ASEN_W<'a, O> {
    #[doc = "Adjust sampling timing disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASEN_A::_0)
    }
    #[doc = "Adjust sampling timing enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASEN_A::_1)
    }
}
#[doc = "Field `ATEN` reader - Adjust transmit timing enable"]
pub type ATEN_R = crate::BitReader<ATEN_A>;
#[doc = "Adjust transmit timing enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATEN_A {
    #[doc = "0: Adjust transmit timing disable."]
    _0 = 0,
    #[doc = "1: Adjust transmit timing enable."]
    _1 = 1,
}
impl From<ATEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATEN_A {
        match self.bits {
            false => ATEN_A::_0,
            true => ATEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATEN_A::_1
    }
}
#[doc = "Field `ATEN` writer - Adjust transmit timing enable"]
pub type ATEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPTR_SPEC, ATEN_A, O>;
impl<'a, const O: u8> ATEN_W<'a, O> {
    #[doc = "Adjust transmit timing disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATEN_A::_0)
    }
    #[doc = "Adjust transmit timing enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Serial Input Data Monitor"]
    #[inline(always)]
    pub fn rxdmon(&self) -> RXDMON_R {
        RXDMON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Port Break Data Select"]
    #[inline(always)]
    pub fn spb2dt(&self) -> SPB2DT_R {
        SPB2DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Serial Port Break I/O"]
    #[inline(always)]
    pub fn spb2io(&self) -> SPB2IO_R {
        SPB2IO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RXD invert bit"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXD invert bit"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Adjust receive sampling timing enable"]
    #[inline(always)]
    pub fn asen(&self) -> ASEN_R {
        ASEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Adjust transmit timing enable"]
    #[inline(always)]
    pub fn aten(&self) -> ATEN_R {
        ATEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Serial Port Break Data Select"]
    #[inline(always)]
    #[must_use]
    pub fn spb2dt(&mut self) -> SPB2DT_W<1> {
        SPB2DT_W::new(self)
    }
    #[doc = "Bit 2 - Serial Port Break I/O"]
    #[inline(always)]
    #[must_use]
    pub fn spb2io(&mut self) -> SPB2IO_W<2> {
        SPB2IO_W::new(self)
    }
    #[doc = "Bit 4 - RXD invert bit"]
    #[inline(always)]
    #[must_use]
    pub fn rinv(&mut self) -> RINV_W<4> {
        RINV_W::new(self)
    }
    #[doc = "Bit 5 - TXD invert bit"]
    #[inline(always)]
    #[must_use]
    pub fn tinv(&mut self) -> TINV_W<5> {
        TINV_W::new(self)
    }
    #[doc = "Bit 6 - Adjust receive sampling timing enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen(&mut self) -> ASEN_W<6> {
        ASEN_W::new(self)
    }
    #[doc = "Bit 7 - Adjust transmit timing enable"]
    #[inline(always)]
    #[must_use]
    pub fn aten(&mut self) -> ATEN_W<7> {
        ATEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Port Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptr](index.html) module"]
pub struct SPTR_SPEC;
impl crate::RegisterSpec for SPTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sptr::R](R) reader structure"]
impl crate::Readable for SPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sptr::W](W) writer structure"]
impl crate::Writable for SPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPTR to value 0x03"]
impl crate::Resettable for SPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
