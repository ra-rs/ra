#[doc = "Register `ADLIMGRSCR` writer"]
pub struct W(crate::W<ADLIMGRSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADLIMGRSCR_SPEC>;
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
impl From<crate::W<ADLIMGRSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADLIMGRSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC0_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC0` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC0_AW, O>;
impl<'a, const O: u8> LIMGRC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC0_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC0_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC1_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC1` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC1_AW, O>;
impl<'a, const O: u8> LIMGRC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC1_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC1_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC2_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC2` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC2_AW, O>;
impl<'a, const O: u8> LIMGRC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC2_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC2_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC3_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC3` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC3_AW, O>;
impl<'a, const O: u8> LIMGRC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC3_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC3_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC4_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC4` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC4_AW, O>;
impl<'a, const O: u8> LIMGRC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC4_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC4_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC5_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC5` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC5_AW, O>;
impl<'a, const O: u8> LIMGRC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC5_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC5_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC6_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC6` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC6_AW, O>;
impl<'a, const O: u8> LIMGRC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC6_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC6_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC7_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC7` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC7_AW, O>;
impl<'a, const O: u8> LIMGRC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC7_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC7_AW::_1)
    }
}
#[doc = "Scan Group n Limiter Clip Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMGRC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADLIMGRSR.LIMGRFn is cleared"]
    _1 = 1,
}
impl From<LIMGRC8_AW> for bool {
    #[inline(always)]
    fn from(variant: LIMGRC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMGRC8` writer - Scan Group n Limiter Clip Flag Clear"]
pub type LIMGRC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADLIMGRSCR_SPEC, LIMGRC8_AW, O>;
impl<'a, const O: u8> LIMGRC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIMGRC8_AW::_0)
    }
    #[doc = "ADLIMGRSR.LIMGRFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIMGRC8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc0(&mut self) -> LIMGRC0_W<0> {
        LIMGRC0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc1(&mut self) -> LIMGRC1_W<1> {
        LIMGRC1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc2(&mut self) -> LIMGRC2_W<2> {
        LIMGRC2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc3(&mut self) -> LIMGRC3_W<3> {
        LIMGRC3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc4(&mut self) -> LIMGRC4_W<4> {
        LIMGRC4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc5(&mut self) -> LIMGRC5_W<5> {
        LIMGRC5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc6(&mut self) -> LIMGRC6_W<6> {
        LIMGRC6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc7(&mut self) -> LIMGRC7_W<7> {
        LIMGRC7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n Limiter Clip Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn limgrc8(&mut self) -> LIMGRC8_W<8> {
        LIMGRC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limiter Clip Scan Group Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adlimgrscr](index.html) module"]
pub struct ADLIMGRSCR_SPEC;
impl crate::RegisterSpec for ADLIMGRSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adlimgrscr::W](W) writer structure"]
impl crate::Writable for ADLIMGRSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADLIMGRSCR to value 0"]
impl crate::Resettable for ADLIMGRSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
