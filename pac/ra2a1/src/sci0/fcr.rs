#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FM` reader - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type FM_R = crate::BitReader<FM_A>;
#[doc = "FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FM_A {
    #[doc = "0: Non-FIFO mode"]
    _0 = 0,
    #[doc = "1: FIFO mode"]
    _1 = 1,
}
impl From<FM_A> for bool {
    #[inline(always)]
    fn from(variant: FM_A) -> Self {
        variant as u8 != 0
    }
}
impl FM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FM_A {
        match self.bits {
            false => FM_A::_0,
            true => FM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FM_A::_1
    }
}
#[doc = "Field `FM` writer - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type FM_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCR_SPEC, FM_A, O>;
impl<'a, const O: u8> FM_W<'a, O> {
    #[doc = "Non-FIFO mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FM_A::_0)
    }
    #[doc = "FIFO mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FM_A::_1)
    }
}
#[doc = "Field `RFRST` reader - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)"]
pub type RFRST_R = crate::BitReader<RFRST_A>;
#[doc = "Receive FIFO Data Register Reset(Valid only in FCR.FM=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    #[doc = "0: Not reset to FRDRHL"]
    _0 = 0,
    #[doc = "1: Reset to FRDRHL"]
    _1 = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRST_A {
        match self.bits {
            false => RFRST_A::_0,
            true => RFRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFRST_A::_1
    }
}
#[doc = "Field `RFRST` writer - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)"]
pub type RFRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCR_SPEC, RFRST_A, O>;
impl<'a, const O: u8> RFRST_W<'a, O> {
    #[doc = "Not reset to FRDRHL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFRST_A::_0)
    }
    #[doc = "Reset to FRDRHL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFRST_A::_1)
    }
}
#[doc = "Field `TFRST` reader - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)"]
pub type TFRST_R = crate::BitReader<TFRST_A>;
#[doc = "Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_A {
    #[doc = "0: Not reset to FTDRHL"]
    _0 = 0,
    #[doc = "1: Reset to FTDRHL"]
    _1 = 1,
}
impl From<TFRST_A> for bool {
    #[inline(always)]
    fn from(variant: TFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl TFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFRST_A {
        match self.bits {
            false => TFRST_A::_0,
            true => TFRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFRST_A::_1
    }
}
#[doc = "Field `TFRST` writer - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)"]
pub type TFRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCR_SPEC, TFRST_A, O>;
impl<'a, const O: u8> TFRST_W<'a, O> {
    #[doc = "Not reset to FTDRHL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFRST_A::_0)
    }
    #[doc = "Reset to FTDRHL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFRST_A::_1)
    }
}
#[doc = "Field `DRES` reader - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)"]
pub type DRES_R = crate::BitReader<DRES_A>;
#[doc = "Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRES_A {
    #[doc = "0: Reception data full interrupt (RXIn)"]
    _0 = 0,
    #[doc = "1: Receive error interrupt (ERIn)"]
    _1 = 1,
}
impl From<DRES_A> for bool {
    #[inline(always)]
    fn from(variant: DRES_A) -> Self {
        variant as u8 != 0
    }
}
impl DRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRES_A {
        match self.bits {
            false => DRES_A::_0,
            true => DRES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRES_A::_1
    }
}
#[doc = "Field `DRES` writer - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)"]
pub type DRES_W<'a, const O: u8> = crate::BitWriter<'a, u16, FCR_SPEC, DRES_A, O>;
impl<'a, const O: u8> DRES_W<'a, O> {
    #[doc = "Reception data full interrupt (RXIn)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRES_A::_0)
    }
    #[doc = "Receive error interrupt (ERIn)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRES_A::_1)
    }
}
#[doc = "Field `TTRG` reader - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TTRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTRG` writer - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RTRG` reader - Receive FIFO data trigger number"]
pub type RTRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTRG` writer - Receive FIFO data trigger number"]
pub type RTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSTRG` reader - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RSTRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTRG` writer - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RSTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn fm(&self) -> FM_R {
        FM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO data trigger number"]
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rstrg(&self) -> RSTRG_R {
        RSTRG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn fm(&mut self) -> FM_W<0> {
        FM_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<1> {
        RFRST_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)"]
    #[inline(always)]
    #[must_use]
    pub fn tfrst(&mut self) -> TFRST_W<2> {
        TFRST_W::new(self)
    }
    #[doc = "Bit 3 - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DRES_W<3> {
        DRES_W::new(self)
    }
    #[doc = "Bits 4:7 - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ttrg(&mut self) -> TTRG_W<4> {
        TTRG_W::new(self)
    }
    #[doc = "Bits 8:11 - Receive FIFO data trigger number"]
    #[inline(always)]
    #[must_use]
    pub fn rtrg(&mut self) -> RTRG_W<8> {
        RTRG_W::new(self)
    }
    #[doc = "Bits 12:15 - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rstrg(&mut self) -> RSTRG_W<12> {
        RSTRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0xf800"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf800;
}
