#[doc = "Register `SSIFCR` reader"]
pub struct R(crate::R<SSIFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSIFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSIFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSIFCR` writer"]
pub struct W(crate::W<SSIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIFCR_SPEC>;
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
impl From<crate::W<SSIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFRST` reader - Receive FIFO Data Register Reset"]
pub type RFRST_R = crate::BitReader<RFRST_A>;
#[doc = "Receive FIFO Data Register Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    #[doc = "0: Clears a receive data FIFO reset condition"]
    _0 = 0,
    #[doc = "1: Sets a receive data FIFO reset condition."]
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
#[doc = "Field `RFRST` writer - Receive FIFO Data Register Reset"]
pub type RFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, RFRST_A, O>;
impl<'a, const O: u8> RFRST_W<'a, O> {
    #[doc = "Clears a receive data FIFO reset condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFRST_A::_0)
    }
    #[doc = "Sets a receive data FIFO reset condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFRST_A::_1)
    }
}
#[doc = "Field `TFRST` reader - Transmit FIFO Data Register Reset"]
pub type TFRST_R = crate::BitReader<TFRST_A>;
#[doc = "Transmit FIFO Data Register Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_A {
    #[doc = "0: Clears a transmit data FIFO reset condition"]
    _0 = 0,
    #[doc = "1: Sets a transmit data FIFO reset condition."]
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
#[doc = "Field `TFRST` writer - Transmit FIFO Data Register Reset"]
pub type TFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, TFRST_A, O>;
impl<'a, const O: u8> TFRST_W<'a, O> {
    #[doc = "Clears a transmit data FIFO reset condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFRST_A::_0)
    }
    #[doc = "Sets a transmit data FIFO reset condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFRST_A::_1)
    }
}
#[doc = "Field `RIE` reader - Receive Data Full Interrupt Output Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receive Data Full Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: Disables receive data full interrupts"]
    _0 = 0,
    #[doc = "1: Enables receive data full interrupts."]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Receive Data Full Interrupt Output Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "Disables receive data full interrupts"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Enables receive data full interrupts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `TIE` reader - Transmit Data Empty Interrupt Output Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmit Data Empty Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Disables transmit data empty interrupts"]
    _0 = 0,
    #[doc = "1: Enables transmit data empty interrupts."]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Transmit Data Empty Interrupt Output Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Disables transmit data empty interrupts"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Enables transmit data empty interrupts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
#[doc = "Field `BSW` reader - Byte Swap Enable"]
pub type BSW_R = crate::BitReader<BSW_A>;
#[doc = "Byte Swap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSW_A {
    #[doc = "0: Disables byte swap"]
    _0 = 0,
    #[doc = "1: Enables byte swap"]
    _1 = 1,
}
impl From<BSW_A> for bool {
    #[inline(always)]
    fn from(variant: BSW_A) -> Self {
        variant as u8 != 0
    }
}
impl BSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSW_A {
        match self.bits {
            false => BSW_A::_0,
            true => BSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSW_A::_1
    }
}
#[doc = "Field `BSW` writer - Byte Swap Enable"]
pub type BSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, BSW_A, O>;
impl<'a, const O: u8> BSW_W<'a, O> {
    #[doc = "Disables byte swap"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSW_A::_0)
    }
    #[doc = "Enables byte swap"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSW_A::_1)
    }
}
#[doc = "Field `SSIRST` reader - Software Reset"]
pub type SSIRST_R = crate::BitReader<SSIRST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIRST_A {
    #[doc = "0: Clears a software reset condition"]
    _0 = 0,
    #[doc = "1: Sets a software reset condition."]
    _1 = 1,
}
impl From<SSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: SSIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SSIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIRST_A {
        match self.bits {
            false => SSIRST_A::_0,
            true => SSIRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIRST_A::_1
    }
}
#[doc = "Field `SSIRST` writer - Software Reset"]
pub type SSIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, SSIRST_A, O>;
impl<'a, const O: u8> SSIRST_W<'a, O> {
    #[doc = "Clears a software reset condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIRST_A::_0)
    }
    #[doc = "Sets a software reset condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIRST_A::_1)
    }
}
#[doc = "Field `AUCKE` reader - AUDIO_MCK Enable in Mastermode Communication"]
pub type AUCKE_R = crate::BitReader<AUCKE_A>;
#[doc = "AUDIO_MCK Enable in Mastermode Communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUCKE_A {
    #[doc = "0: Disables supply of AUDIO_MCK"]
    _0 = 0,
    #[doc = "1: Enables supply of AUDIO_MCK."]
    _1 = 1,
}
impl From<AUCKE_A> for bool {
    #[inline(always)]
    fn from(variant: AUCKE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUCKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUCKE_A {
        match self.bits {
            false => AUCKE_A::_0,
            true => AUCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AUCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AUCKE_A::_1
    }
}
#[doc = "Field `AUCKE` writer - AUDIO_MCK Enable in Mastermode Communication"]
pub type AUCKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIFCR_SPEC, AUCKE_A, O>;
impl<'a, const O: u8> AUCKE_W<'a, O> {
    #[doc = "Disables supply of AUDIO_MCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUCKE_A::_0)
    }
    #[doc = "Enables supply of AUDIO_MCK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUCKE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Data Full Interrupt Output Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Data Empty Interrupt Output Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Byte Swap Enable"]
    #[inline(always)]
    pub fn bsw(&self) -> BSW_R {
        BSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Reset"]
    #[inline(always)]
    pub fn ssirst(&self) -> SSIRST_R {
        SSIRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - AUDIO_MCK Enable in Mastermode Communication"]
    #[inline(always)]
    pub fn aucke(&self) -> AUCKE_R {
        AUCKE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Data Register Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<0> {
        RFRST_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO Data Register Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tfrst(&mut self) -> TFRST_W<1> {
        TFRST_W::new(self)
    }
    #[doc = "Bit 2 - Receive Data Full Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<2> {
        RIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Data Empty Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<3> {
        TIE_W::new(self)
    }
    #[doc = "Bit 11 - Byte Swap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsw(&mut self) -> BSW_W<11> {
        BSW_W::new(self)
    }
    #[doc = "Bit 16 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ssirst(&mut self) -> SSIRST_W<16> {
        SSIRST_W::new(self)
    }
    #[doc = "Bit 31 - AUDIO_MCK Enable in Mastermode Communication"]
    #[inline(always)]
    #[must_use]
    pub fn aucke(&mut self) -> AUCKE_W<31> {
        AUCKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssifcr](index.html) module"]
pub struct SSIFCR_SPEC;
impl crate::RegisterSpec for SSIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssifcr::R](R) reader structure"]
impl crate::Readable for SSIFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssifcr::W](W) writer structure"]
impl crate::Writable for SSIFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSIFCR to value 0"]
impl crate::Resettable for SSIFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
