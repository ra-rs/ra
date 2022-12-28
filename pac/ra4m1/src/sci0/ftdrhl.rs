#[doc = "Register `FTDRHL` writer"]
pub struct W(crate::W<FTDRHL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTDRHL_SPEC>;
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
impl From<crate::W<FTDRHL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTDRHL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDAT` writer - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FTDRHL_SPEC, u16, u16, 9, O>;
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
pub type MPBT_W<'a, const O: u8> = crate::BitWriter<'a, u16, FTDRHL_SPEC, MPBT_AW, O>;
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
    #[doc = "Bits 0:8 - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn tdat(&mut self) -> TDAT_W<0> {
        TDAT_W::new(self)
    }
    #[doc = "Bit 9 - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn mpbt(&mut self) -> MPBT_W<9> {
        MPBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Data Register HL\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftdrhl](index.html) module"]
pub struct FTDRHL_SPEC;
impl crate::RegisterSpec for FTDRHL_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [ftdrhl::W](W) writer structure"]
impl crate::Writable for FTDRHL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTDRHL to value 0xffff"]
impl crate::Resettable for FTDRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
