#[doc = "Register `PORR` writer"]
pub struct W(crate::W<PORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORR_SPEC>;
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
impl From<crate::W<PORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR00_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR00_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR00_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR00` writer - Pmn Output Reset"]
pub type PORR00_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR00_AW, O>;
impl<'a, const O: u8> PORR00_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR00_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR00_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR01_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR01_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR01_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR01` writer - Pmn Output Reset"]
pub type PORR01_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR01_AW, O>;
impl<'a, const O: u8> PORR01_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR01_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR01_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR02_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR02_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR02_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR02` writer - Pmn Output Reset"]
pub type PORR02_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR02_AW, O>;
impl<'a, const O: u8> PORR02_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR02_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR02_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR03_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR03_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR03_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR03` writer - Pmn Output Reset"]
pub type PORR03_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR03_AW, O>;
impl<'a, const O: u8> PORR03_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR03_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR03_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR04_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR04_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR04_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR04` writer - Pmn Output Reset"]
pub type PORR04_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR04_AW, O>;
impl<'a, const O: u8> PORR04_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR04_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR04_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR05_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR05_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR05_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR05` writer - Pmn Output Reset"]
pub type PORR05_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR05_AW, O>;
impl<'a, const O: u8> PORR05_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR05_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR05_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR06_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR06_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR06_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR06` writer - Pmn Output Reset"]
pub type PORR06_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR06_AW, O>;
impl<'a, const O: u8> PORR06_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR06_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR06_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR07_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR07_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR07_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR07` writer - Pmn Output Reset"]
pub type PORR07_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR07_AW, O>;
impl<'a, const O: u8> PORR07_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR07_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR07_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR08_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR08_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR08_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR08` writer - Pmn Output Reset"]
pub type PORR08_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR08_AW, O>;
impl<'a, const O: u8> PORR08_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR08_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR08_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR09_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR09_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR09_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR09` writer - Pmn Output Reset"]
pub type PORR09_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR09_AW, O>;
impl<'a, const O: u8> PORR09_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR09_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR09_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR10_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR10_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR10` writer - Pmn Output Reset"]
pub type PORR10_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR10_AW, O>;
impl<'a, const O: u8> PORR10_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR10_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR10_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR11_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR11_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR11` writer - Pmn Output Reset"]
pub type PORR11_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR11_AW, O>;
impl<'a, const O: u8> PORR11_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR11_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR11_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR12_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR12_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR12` writer - Pmn Output Reset"]
pub type PORR12_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR12_AW, O>;
impl<'a, const O: u8> PORR12_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR12_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR12_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR13_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR13_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR13` writer - Pmn Output Reset"]
pub type PORR13_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR13_AW, O>;
impl<'a, const O: u8> PORR13_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR13_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR13_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR14_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR14_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR14` writer - Pmn Output Reset"]
pub type PORR14_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR14_AW, O>;
impl<'a, const O: u8> PORR14_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR14_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR14_AW::_1)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR15_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<PORR15_AW> for bool {
    #[inline(always)]
    fn from(variant: PORR15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR15` writer - Pmn Output Reset"]
pub type PORR15_W<'a, const O: u8> = crate::BitWriter<'a, u16, PORR_SPEC, PORR15_AW, O>;
impl<'a, const O: u8> PORR15_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR15_AW::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR15_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr00(&mut self) -> PORR00_W<0> {
        PORR00_W::new(self)
    }
    #[doc = "Bit 1 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr01(&mut self) -> PORR01_W<1> {
        PORR01_W::new(self)
    }
    #[doc = "Bit 2 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr02(&mut self) -> PORR02_W<2> {
        PORR02_W::new(self)
    }
    #[doc = "Bit 3 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr03(&mut self) -> PORR03_W<3> {
        PORR03_W::new(self)
    }
    #[doc = "Bit 4 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr04(&mut self) -> PORR04_W<4> {
        PORR04_W::new(self)
    }
    #[doc = "Bit 5 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr05(&mut self) -> PORR05_W<5> {
        PORR05_W::new(self)
    }
    #[doc = "Bit 6 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr06(&mut self) -> PORR06_W<6> {
        PORR06_W::new(self)
    }
    #[doc = "Bit 7 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr07(&mut self) -> PORR07_W<7> {
        PORR07_W::new(self)
    }
    #[doc = "Bit 8 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr08(&mut self) -> PORR08_W<8> {
        PORR08_W::new(self)
    }
    #[doc = "Bit 9 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr09(&mut self) -> PORR09_W<9> {
        PORR09_W::new(self)
    }
    #[doc = "Bit 10 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr10(&mut self) -> PORR10_W<10> {
        PORR10_W::new(self)
    }
    #[doc = "Bit 11 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr11(&mut self) -> PORR11_W<11> {
        PORR11_W::new(self)
    }
    #[doc = "Bit 12 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr12(&mut self) -> PORR12_W<12> {
        PORR12_W::new(self)
    }
    #[doc = "Bit 13 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr13(&mut self) -> PORR13_W<13> {
        PORR13_W::new(self)
    }
    #[doc = "Bit 14 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr14(&mut self) -> PORR14_W<14> {
        PORR14_W::new(self)
    }
    #[doc = "Bit 15 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr15(&mut self) -> PORR15_W<15> {
        PORR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 3\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porr](index.html) module"]
pub struct PORR_SPEC;
impl crate::RegisterSpec for PORR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [porr::W](W) writer structure"]
impl crate::Writable for PORR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORR to value 0"]
impl crate::Resettable for PORR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
