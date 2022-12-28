#[doc = "Register `ADSCANENDSCR` writer"]
pub struct W(crate::W<ADSCANENDSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSCANENDSCR_SPEC>;
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
impl From<crate::W<ADSCANENDSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSCANENDSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC0_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC0` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC0_AW, O>;
impl<'a, const O: u8> SCENDC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC0_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC0_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC1_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC1` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC1_AW, O>;
impl<'a, const O: u8> SCENDC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC1_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC1_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC2_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC2` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC2_AW, O>;
impl<'a, const O: u8> SCENDC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC2_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC2_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC3_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC3` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC3_AW, O>;
impl<'a, const O: u8> SCENDC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC3_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC3_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC4_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC4` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC4_AW, O>;
impl<'a, const O: u8> SCENDC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC4_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC4_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC5_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC5` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC5_AW, O>;
impl<'a, const O: u8> SCENDC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC5_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC5_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC6_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC6` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC6_AW, O>;
impl<'a, const O: u8> SCENDC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC6_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC6_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC7_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC7` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC7_AW, O>;
impl<'a, const O: u8> SCENDC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC7_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC7_AW::_1)
    }
}
#[doc = "Scan Group n Scan End Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCENDC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADSCANENDSR.SCENDFn is cleared"]
    _1 = 1,
}
impl From<SCENDC8_AW> for bool {
    #[inline(always)]
    fn from(variant: SCENDC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCENDC8` writer - Scan Group n Scan End Flag Clear"]
pub type SCENDC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSCANENDSCR_SPEC, SCENDC8_AW, O>;
impl<'a, const O: u8> SCENDC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENDC8_AW::_0)
    }
    #[doc = "ADSCANENDSR.SCENDFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENDC8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc0(&mut self) -> SCENDC0_W<0> {
        SCENDC0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc1(&mut self) -> SCENDC1_W<1> {
        SCENDC1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc2(&mut self) -> SCENDC2_W<2> {
        SCENDC2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc3(&mut self) -> SCENDC3_W<3> {
        SCENDC3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc4(&mut self) -> SCENDC4_W<4> {
        SCENDC4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc5(&mut self) -> SCENDC5_W<5> {
        SCENDC5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc6(&mut self) -> SCENDC6_W<6> {
        SCENDC6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc7(&mut self) -> SCENDC7_W<7> {
        SCENDC7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n Scan End Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scendc8(&mut self) -> SCENDC8_W<8> {
        SCENDC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan End Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adscanendscr](index.html) module"]
pub struct ADSCANENDSCR_SPEC;
impl crate::RegisterSpec for ADSCANENDSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adscanendscr::W](W) writer structure"]
impl crate::Writable for ADSCANENDSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSCANENDSCR to value 0"]
impl crate::Resettable for ADSCANENDSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
