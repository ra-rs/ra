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
#[doc = "Field `DRES` reader - Receive data ready error select bit"]
pub type DRES_R = crate::BitReader<DRES_A>;
#[doc = "Receive data ready error select bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRES_A {
    #[doc = "0: reception data full interrupt (SCIn_RXI)"]
    _0 = 0,
    #[doc = "1: receive error interrupt (SCIn_ERI)"]
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
#[doc = "Field `DRES` writer - Receive data ready error select bit"]
pub type DRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, DRES_A, O>;
impl<'a, const O: u8> DRES_W<'a, O> {
    #[doc = "reception data full interrupt (SCIn_RXI)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRES_A::_0)
    }
    #[doc = "receive error interrupt (SCIn_ERI)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRES_A::_1)
    }
}
#[doc = "Field `TTRG` reader - Transmit FIFO data trigger number"]
pub type TTRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTRG` writer - Transmit FIFO data trigger number"]
pub type TTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 5, O>;
#[doc = "Transmit FIFO Data Register Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_AW {
    #[doc = "0: It is invalid. It does not affect the operation."]
    _0 = 0,
    #[doc = "1: The number of data stored in Transmit-FIFO (TDR register) are made 0"]
    _1 = 1,
}
impl From<TFRST_AW> for bool {
    #[inline(always)]
    fn from(variant: TFRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFRST` writer - Transmit FIFO Data Register Reset"]
pub type TFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, TFRST_AW, O>;
impl<'a, const O: u8> TFRST_W<'a, O> {
    #[doc = "It is invalid. It does not affect the operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFRST_AW::_0)
    }
    #[doc = "The number of data stored in Transmit-FIFO (TDR register) are made 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFRST_AW::_1)
    }
}
#[doc = "Field `RTRG` reader - Receive FIFO data trigger number"]
pub type RTRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTRG` writer - Receive FIFO data trigger number"]
pub type RTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 5, O>;
#[doc = "Receive FIFO Data Register Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_AW {
    #[doc = "0: It is invalid. It does not affect the operation."]
    _0 = 0,
    #[doc = "1: The number of data stored in Receive-FIFO(RDR register) are made 0"]
    _1 = 1,
}
impl From<RFRST_AW> for bool {
    #[inline(always)]
    fn from(variant: RFRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRST` writer - Receive FIFO Data Register Reset"]
pub type RFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, RFRST_AW, O>;
impl<'a, const O: u8> RFRST_W<'a, O> {
    #[doc = "It is invalid. It does not affect the operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFRST_AW::_0)
    }
    #[doc = "The number of data stored in Receive-FIFO(RDR register) are made 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFRST_AW::_1)
    }
}
#[doc = "Field `RSTRG` reader - RTS Output Active Trigger Number Select"]
pub type RSTRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTRG` writer - RTS Output Active Trigger Number Select"]
pub type RSTRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Receive data ready error select bit"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - Transmit FIFO data trigger number"]
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Receive FIFO data trigger number"]
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - RTS Output Active Trigger Number Select"]
    #[inline(always)]
    pub fn rstrg(&self) -> RSTRG_R {
        RSTRG_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive data ready error select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DRES_W<0> {
        DRES_W::new(self)
    }
    #[doc = "Bits 8:12 - Transmit FIFO data trigger number"]
    #[inline(always)]
    #[must_use]
    pub fn ttrg(&mut self) -> TTRG_W<8> {
        TTRG_W::new(self)
    }
    #[doc = "Bit 15 - Transmit FIFO Data Register Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tfrst(&mut self) -> TFRST_W<15> {
        TFRST_W::new(self)
    }
    #[doc = "Bits 16:20 - Receive FIFO data trigger number"]
    #[inline(always)]
    #[must_use]
    pub fn rtrg(&mut self) -> RTRG_W<16> {
        RTRG_W::new(self)
    }
    #[doc = "Bit 23 - Receive FIFO Data Register Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<23> {
        RFRST_W::new(self)
    }
    #[doc = "Bits 24:28 - RTS Output Active Trigger Number Select"]
    #[inline(always)]
    #[must_use]
    pub fn rstrg(&mut self) -> RSTRG_W<24> {
        RSTRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
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
#[doc = "`reset()` method sets FCR to value 0x1f1f_0000"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f_0000;
}
