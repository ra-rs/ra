#[doc = "Register `FTDRH` writer"]
pub struct W(crate::W<FTDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTDRH_SPEC>;
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
impl From<crate::W<FTDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDATH` writer - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TDATH_W<'a, const O: u8> = crate::BitWriter<'a, u8, FTDRH_SPEC, bool, O>;
#[doc = "Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPBT_AW {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<MPBT_AW> for bool {
    #[inline(always)]
    fn from(variant: MPBT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPBT` writer - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
pub type MPBT_W<'a, const O: u8> = crate::BitWriter<'a, u8, FTDRH_SPEC, MPBT_AW, O>;
impl<'a, const O: u8> MPBT_W<'a, O> {
    #[doc = "Data transmission cycles"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPBT_AW::_0)
    }
    #[doc = "ID transmission cycles"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPBT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn tdath(&mut self) -> TDATH_W<0> {
        TDATH_W::new(self)
    }
    #[doc = "Bit 1 - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn mpbt(&mut self) -> MPBT_W<1> {
        MPBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Data Register H\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftdrh](index.html) module"]
pub struct FTDRH_SPEC;
impl crate::RegisterSpec for FTDRH_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [ftdrh::W](W) writer structure"]
impl crate::Writable for FTDRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTDRH to value 0xff"]
impl crate::Resettable for FTDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
