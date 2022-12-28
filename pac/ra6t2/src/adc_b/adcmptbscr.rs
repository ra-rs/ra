#[doc = "Register `ADCMPTBSCR` writer"]
pub struct W(crate::W<ADCMPTBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPTBSCR_SPEC>;
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
impl From<crate::W<ADCMPTBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPTBSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC0_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC0` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC0_AW, O>;
impl<'a, const O: u8> CMPTBC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC0_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC0_AW::_1)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC1_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC1` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC1_AW, O>;
impl<'a, const O: u8> CMPTBC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC1_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC1_AW::_1)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC2_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC2` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC2_AW, O>;
impl<'a, const O: u8> CMPTBC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC2_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC2_AW::_1)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC3_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC3` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC3_AW, O>;
impl<'a, const O: u8> CMPTBC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC3_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC3_AW::_1)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC4_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC4` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC4_AW, O>;
impl<'a, const O: u8> CMPTBC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC4_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC4_AW::_1)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC5_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC5` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC5_AW, O>;
impl<'a, const O: u8> CMPTBC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC5_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC5_AW::_1)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC6_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC6` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC6_AW, O>;
impl<'a, const O: u8> CMPTBC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC6_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC6_AW::_1)
    }
}
#[doc = "Compare Match Table n: Match Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTBC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADCMPTBSR.CMPTBFn is cleared"]
    _1 = 1,
}
impl From<CMPTBC7_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPTBC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTBC7` writer - Compare Match Table n: Match Flag Clear"]
pub type CMPTBC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCMPTBSCR_SPEC, CMPTBC7_AW, O>;
impl<'a, const O: u8> CMPTBC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPTBC7_AW::_0)
    }
    #[doc = "ADCMPTBSR.CMPTBFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPTBC7_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc0(&mut self) -> CMPTBC0_W<0> {
        CMPTBC0_W::new(self)
    }
    #[doc = "Bit 1 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc1(&mut self) -> CMPTBC1_W<1> {
        CMPTBC1_W::new(self)
    }
    #[doc = "Bit 2 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc2(&mut self) -> CMPTBC2_W<2> {
        CMPTBC2_W::new(self)
    }
    #[doc = "Bit 3 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc3(&mut self) -> CMPTBC3_W<3> {
        CMPTBC3_W::new(self)
    }
    #[doc = "Bit 4 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc4(&mut self) -> CMPTBC4_W<4> {
        CMPTBC4_W::new(self)
    }
    #[doc = "Bit 5 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc5(&mut self) -> CMPTBC5_W<5> {
        CMPTBC5_W::new(self)
    }
    #[doc = "Bit 6 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc6(&mut self) -> CMPTBC6_W<6> {
        CMPTBC6_W::new(self)
    }
    #[doc = "Bit 7 - Compare Match Table n: Match Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbc7(&mut self) -> CMPTBC7_W<7> {
        CMPTBC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Table Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmptbscr](index.html) module"]
pub struct ADCMPTBSCR_SPEC;
impl crate::RegisterSpec for ADCMPTBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adcmptbscr::W](W) writer structure"]
impl crate::Writable for ADCMPTBSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPTBSCR to value 0"]
impl crate::Resettable for ADCMPTBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
