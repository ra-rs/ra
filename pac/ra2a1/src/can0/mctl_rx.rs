#[doc = "Register `MCTL_RX[%s]` reader"]
pub struct R(crate::R<MCTL_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTL_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTL_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTL_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTL_RX[%s]` writer"]
pub struct W(crate::W<MCTL_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTL_RX_SPEC>;
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
impl From<crate::W<MCTL_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTL_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEWDATA` reader - Reception Complete Flag"]
pub type NEWDATA_R = crate::BitReader<NEWDATA_A>;
#[doc = "Reception Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEWDATA_A {
    #[doc = "0: No data has been received or 0 is written to the NEWDATA bit"]
    _0 = 0,
    #[doc = "1: A new message is being stored or has been stored to the mailbox"]
    _1 = 1,
}
impl From<NEWDATA_A> for bool {
    #[inline(always)]
    fn from(variant: NEWDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl NEWDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEWDATA_A {
        match self.bits {
            false => NEWDATA_A::_0,
            true => NEWDATA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NEWDATA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NEWDATA_A::_1
    }
}
#[doc = "Field `NEWDATA` writer - Reception Complete Flag"]
pub type NEWDATA_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_RX_SPEC, NEWDATA_A, O>;
impl<'a, const O: u8> NEWDATA_W<'a, O> {
    #[doc = "No data has been received or 0 is written to the NEWDATA bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEWDATA_A::_0)
    }
    #[doc = "A new message is being stored or has been stored to the mailbox"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEWDATA_A::_1)
    }
}
#[doc = "Field `INVALDATA` reader - Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
pub type INVALDATA_R = crate::BitReader<INVALDATA_A>;
#[doc = "Reception-in-Progress Status Flag (Receive mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVALDATA_A {
    #[doc = "0: Message valid"]
    _0 = 0,
    #[doc = "1: Message being updated"]
    _1 = 1,
}
impl From<INVALDATA_A> for bool {
    #[inline(always)]
    fn from(variant: INVALDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl INVALDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVALDATA_A {
        match self.bits {
            false => INVALDATA_A::_0,
            true => INVALDATA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVALDATA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVALDATA_A::_1
    }
}
#[doc = "Field `MSGLOST` reader - Message Lost Flag(Receive mailbox setting enabled)"]
pub type MSGLOST_R = crate::BitReader<MSGLOST_A>;
#[doc = "Message Lost Flag(Receive mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSGLOST_A {
    #[doc = "0: Message is not overwritten or overrun"]
    _0 = 0,
    #[doc = "1: Message is overwritten or overrun"]
    _1 = 1,
}
impl From<MSGLOST_A> for bool {
    #[inline(always)]
    fn from(variant: MSGLOST_A) -> Self {
        variant as u8 != 0
    }
}
impl MSGLOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGLOST_A {
        match self.bits {
            false => MSGLOST_A::_0,
            true => MSGLOST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSGLOST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSGLOST_A::_1
    }
}
#[doc = "Field `MSGLOST` writer - Message Lost Flag(Receive mailbox setting enabled)"]
pub type MSGLOST_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_RX_SPEC, MSGLOST_A, O>;
impl<'a, const O: u8> MSGLOST_W<'a, O> {
    #[doc = "Message is not overwritten or overrun"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSGLOST_A::_0)
    }
    #[doc = "Message is overwritten or overrun"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSGLOST_A::_1)
    }
}
#[doc = "Field `ONESHOT` reader - One-Shot Enable"]
pub type ONESHOT_R = crate::BitReader<ONESHOT_A>;
#[doc = "One-Shot Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONESHOT_A {
    #[doc = "0: One-shot reception or one-shot transmission disabled"]
    _0 = 0,
    #[doc = "1: One-shot reception or one-shot transmission enabled"]
    _1 = 1,
}
impl From<ONESHOT_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOT_A) -> Self {
        variant as u8 != 0
    }
}
impl ONESHOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOT_A {
        match self.bits {
            false => ONESHOT_A::_0,
            true => ONESHOT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONESHOT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONESHOT_A::_1
    }
}
#[doc = "Field `ONESHOT` writer - One-Shot Enable"]
pub type ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_RX_SPEC, ONESHOT_A, O>;
impl<'a, const O: u8> ONESHOT_W<'a, O> {
    #[doc = "One-shot reception or one-shot transmission disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONESHOT_A::_0)
    }
    #[doc = "One-shot reception or one-shot transmission enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONESHOT_A::_1)
    }
}
#[doc = "Field `RECREQ` reader - Receive Mailbox Request"]
pub type RECREQ_R = crate::BitReader<RECREQ_A>;
#[doc = "Receive Mailbox Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECREQ_A {
    #[doc = "0: Not configured for reception"]
    _0 = 0,
    #[doc = "1: Configured for reception"]
    _1 = 1,
}
impl From<RECREQ_A> for bool {
    #[inline(always)]
    fn from(variant: RECREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RECREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECREQ_A {
        match self.bits {
            false => RECREQ_A::_0,
            true => RECREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECREQ_A::_1
    }
}
#[doc = "Field `RECREQ` writer - Receive Mailbox Request"]
pub type RECREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_RX_SPEC, RECREQ_A, O>;
impl<'a, const O: u8> RECREQ_W<'a, O> {
    #[doc = "Not configured for reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECREQ_A::_0)
    }
    #[doc = "Configured for reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECREQ_A::_1)
    }
}
#[doc = "Field `TRMREQ` reader - Transmit Mailbox Request"]
pub type TRMREQ_R = crate::BitReader<TRMREQ_A>;
#[doc = "Transmit Mailbox Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMREQ_A {
    #[doc = "0: Not configured for transmission"]
    _0 = 0,
    #[doc = "1: Configured for transmission"]
    _1 = 1,
}
impl From<TRMREQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRMREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRMREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRMREQ_A {
        match self.bits {
            false => TRMREQ_A::_0,
            true => TRMREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMREQ_A::_1
    }
}
#[doc = "Field `TRMREQ` writer - Transmit Mailbox Request"]
pub type TRMREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_RX_SPEC, TRMREQ_A, O>;
impl<'a, const O: u8> TRMREQ_W<'a, O> {
    #[doc = "Not configured for transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRMREQ_A::_0)
    }
    #[doc = "Configured for transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRMREQ_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Reception Complete Flag"]
    #[inline(always)]
    pub fn newdata(&self) -> NEWDATA_R {
        NEWDATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub fn invaldata(&self) -> INVALDATA_R {
        INVALDATA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message Lost Flag(Receive mailbox setting enabled)"]
    #[inline(always)]
    pub fn msglost(&self) -> MSGLOST_R {
        MSGLOST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - One-Shot Enable"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Mailbox Request"]
    #[inline(always)]
    pub fn recreq(&self) -> RECREQ_R {
        RECREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Mailbox Request"]
    #[inline(always)]
    pub fn trmreq(&self) -> TRMREQ_R {
        TRMREQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn newdata(&mut self) -> NEWDATA_W<0> {
        NEWDATA_W::new(self)
    }
    #[doc = "Bit 2 - Message Lost Flag(Receive mailbox setting enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn msglost(&mut self) -> MSGLOST_W<2> {
        MSGLOST_W::new(self)
    }
    #[doc = "Bit 4 - One-Shot Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> ONESHOT_W<4> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bit 6 - Receive Mailbox Request"]
    #[inline(always)]
    #[must_use]
    pub fn recreq(&mut self) -> RECREQ_W<6> {
        RECREQ_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Mailbox Request"]
    #[inline(always)]
    #[must_use]
    pub fn trmreq(&mut self) -> TRMREQ_W<7> {
        TRMREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Control Register( Receive mode (when the TRMREQ bit is 0 and the RECREQ bit is 1))\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl_rx](index.html) module"]
pub struct MCTL_RX_SPEC;
impl crate::RegisterSpec for MCTL_RX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mctl_rx::R](R) reader structure"]
impl crate::Readable for MCTL_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctl_rx::W](W) writer structure"]
impl crate::Writable for MCTL_RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTL_RX[%s]
to value 0"]
impl crate::Resettable for MCTL_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
