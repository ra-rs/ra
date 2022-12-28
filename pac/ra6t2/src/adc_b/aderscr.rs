#[doc = "Register `ADERSCR` writer"]
pub struct W(crate::W<ADERSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADERSCR_SPEC>;
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
impl From<crate::W<ADERSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADERSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Converter Unit 0 Error Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADERCLR0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADERSR.ADERF0 is cleared"]
    _1 = 1,
}
impl From<ADERCLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: ADERCLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADERCLR0` writer - A/D Converter Unit 0 Error Flag Clear"]
pub type ADERCLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADERSCR_SPEC, ADERCLR0_AW, O>;
impl<'a, const O: u8> ADERCLR0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADERCLR0_AW::_0)
    }
    #[doc = "ADERSR.ADERF0 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADERCLR0_AW::_1)
    }
}
#[doc = "A/D Converter Unit 1 Error Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADERCLR1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADERSR.ADERF1 is cleared"]
    _1 = 1,
}
impl From<ADERCLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: ADERCLR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADERCLR1` writer - A/D Converter Unit 1 Error Flag Clear"]
pub type ADERCLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADERSCR_SPEC, ADERCLR1_AW, O>;
impl<'a, const O: u8> ADERCLR1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADERCLR1_AW::_0)
    }
    #[doc = "ADERSR.ADERF1 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADERCLR1_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter Unit 0 Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn aderclr0(&mut self) -> ADERCLR0_W<0> {
        ADERCLR0_W::new(self)
    }
    #[doc = "Bit 1 - A/D Converter Unit 1 Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn aderclr1(&mut self) -> ADERCLR1_W<1> {
        ADERCLR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Error Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aderscr](index.html) module"]
pub struct ADERSCR_SPEC;
impl crate::RegisterSpec for ADERSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aderscr::W](W) writer structure"]
impl crate::Writable for ADERSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADERSCR to value 0"]
impl crate::Resettable for ADERSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
