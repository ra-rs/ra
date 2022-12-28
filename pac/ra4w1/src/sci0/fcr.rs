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
    #[doc = "0: Non-FIFO mode(Selects o TDR/RDR for communication)"]
    _0 = 0,
    #[doc = "1: FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
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
    #[doc = "Non-FIFO mode(Selects o TDR/RDR for communication)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FM_A::_0)
    }
    #[doc = "FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
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
    #[doc = "0: The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    _0 = 0,
    #[doc = "1: The number of data stored in FRDRH and FRDRL register are made 0"]
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
    #[doc = "The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFRST_A::_0)
    }
    #[doc = "The number of data stored in FRDRH and FRDRL register are made 0"]
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
    #[doc = "0: The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    _0 = 0,
    #[doc = "1: The number of data stored in FTDRH and FTDRL register are made 0"]
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
    #[doc = "The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFRST_A::_0)
    }
    #[doc = "The number of data stored in FTDRH and FTDRL register are made 0"]
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
    #[doc = "0: reception data full interrupt (RXI)"]
    _0 = 0,
    #[doc = "1: receive error interrupt (ERI)"]
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
    #[doc = "reception data full interrupt (RXI)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRES_A::_0)
    }
    #[doc = "receive error interrupt (ERI)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRES_A::_1)
    }
}
#[doc = "Field `TTRG` reader - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TTRG_R = crate::FieldReader<u8, TTRG_A>;
#[doc = "Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTRG_A {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
}
impl From<TTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: TTRG_A) -> Self {
        variant as _
    }
}
impl TTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TTRG_A> {
        match self.bits {
            0 => Some(TTRG_A::_0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TTRG_A::_0000
    }
}
#[doc = "Field `TTRG` writer - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCR_SPEC, u8, TTRG_A, 4, O>;
impl<'a, const O: u8> TTRG_W<'a, O> {
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TTRG_A::_0000)
    }
}
#[doc = "Field `RTRG` reader - Receive FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RTRG_R = crate::FieldReader<u8, RTRG_A>;
#[doc = "Receive FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTRG_A {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
}
impl From<RTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RTRG_A) -> Self {
        variant as _
    }
}
impl RTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTRG_A> {
        match self.bits {
            0 => Some(RTRG_A::_0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RTRG_A::_0000
    }
}
#[doc = "Field `RTRG` writer - Receive FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCR_SPEC, u8, RTRG_A, 4, O>;
impl<'a, const O: u8> RTRG_W<'a, O> {
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(RTRG_A::_0000)
    }
}
#[doc = "Field `RSTRG` reader - RTS Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RSTRG_R = crate::FieldReader<u8, RSTRG_A>;
#[doc = "RTS Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTRG_A {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
}
impl From<RSTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTRG_A) -> Self {
        variant as _
    }
}
impl RSTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTRG_A> {
        match self.bits {
            0 => Some(RSTRG_A::_0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RSTRG_A::_0000
    }
}
#[doc = "Field `RSTRG` writer - RTS Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RSTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCR_SPEC, u8, RSTRG_A, 4, O>;
impl<'a, const O: u8> RSTRG_W<'a, O> {
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(RSTRG_A::_0000)
    }
}
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
    #[doc = "Bits 8:11 - Receive FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RTS Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
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
    #[doc = "Bits 8:11 - Receive FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rtrg(&mut self) -> RTRG_W<8> {
        RTRG_W::new(self)
    }
    #[doc = "Bits 12:15 - RTS Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
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
