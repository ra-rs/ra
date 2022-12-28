#[doc = "Register `MCTL_TX[%s]` reader"]
pub struct R(crate::R<MCTL_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTL_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTL_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTL_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTL_TX[%s]` writer"]
pub struct W(crate::W<MCTL_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTL_TX_SPEC>;
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
impl From<crate::W<MCTL_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTL_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENTDATA` reader - Transmission Complete Flag"]
pub type SENTDATA_R = crate::BitReader<SENTDATA_A>;
#[doc = "Transmission Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENTDATA_A {
    #[doc = "0: Transmission is not completed"]
    _0 = 0,
    #[doc = "1: Transmission is completed"]
    _1 = 1,
}
impl From<SENTDATA_A> for bool {
    #[inline(always)]
    fn from(variant: SENTDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl SENTDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SENTDATA_A {
        match self.bits {
            false => SENTDATA_A::_0,
            true => SENTDATA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SENTDATA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SENTDATA_A::_1
    }
}
#[doc = "Field `SENTDATA` writer - Transmission Complete Flag"]
pub type SENTDATA_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_TX_SPEC, SENTDATA_A, O>;
impl<'a, const O: u8> SENTDATA_W<'a, O> {
    #[doc = "Transmission is not completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SENTDATA_A::_0)
    }
    #[doc = "Transmission is completed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SENTDATA_A::_1)
    }
}
#[doc = "Field `TRMACTIVE` reader - Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
pub type TRMACTIVE_R = crate::BitReader<TRMACTIVE_A>;
#[doc = "Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMACTIVE_A {
    #[doc = "0: Transmission is pending or transmission is not requested"]
    _0 = 0,
    #[doc = "1: From acceptance of transmission request to completion of transmission, or error/arbitration-lost"]
    _1 = 1,
}
impl From<TRMACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: TRMACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRMACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRMACTIVE_A {
        match self.bits {
            false => TRMACTIVE_A::_0,
            true => TRMACTIVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMACTIVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMACTIVE_A::_1
    }
}
#[doc = "Field `TRMABT` reader - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
pub type TRMABT_R = crate::BitReader<TRMABT_A>;
#[doc = "Transmission Abort Complete Flag (Transmit mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMABT_A {
    #[doc = "0: Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested"]
    _0 = 0,
    #[doc = "1: Transmission abort is completed"]
    _1 = 1,
}
impl From<TRMABT_A> for bool {
    #[inline(always)]
    fn from(variant: TRMABT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRMABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRMABT_A {
        match self.bits {
            false => TRMABT_A::_0,
            true => TRMABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMABT_A::_1
    }
}
#[doc = "Field `TRMABT` writer - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
pub type TRMABT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_TX_SPEC, TRMABT_A, O>;
impl<'a, const O: u8> TRMABT_W<'a, O> {
    #[doc = "Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRMABT_A::_0)
    }
    #[doc = "Transmission abort is completed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRMABT_A::_1)
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
pub type ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_TX_SPEC, ONESHOT_A, O>;
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
pub type RECREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_TX_SPEC, RECREQ_A, O>;
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
pub type TRMREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTL_TX_SPEC, TRMREQ_A, O>;
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
    #[doc = "Bit 0 - Transmission Complete Flag"]
    #[inline(always)]
    pub fn sentdata(&self) -> SENTDATA_R {
        SENTDATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub fn trmactive(&self) -> TRMACTIVE_R {
        TRMACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub fn trmabt(&self) -> TRMABT_R {
        TRMABT_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 0 - Transmission Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sentdata(&mut self) -> SENTDATA_W<0> {
        SENTDATA_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn trmabt(&mut self) -> TRMABT_W<2> {
        TRMABT_W::new(self)
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
#[doc = "Message Control Register for Transmit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl_tx](index.html) module"]
pub struct MCTL_TX_SPEC;
impl crate::RegisterSpec for MCTL_TX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mctl_tx::R](R) reader structure"]
impl crate::Readable for MCTL_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctl_tx::W](W) writer structure"]
impl crate::Writable for MCTL_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTL_TX[%s]
to value 0"]
impl crate::Resettable for MCTL_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
