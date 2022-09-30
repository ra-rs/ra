#[doc = "Register `POSR` writer"]
pub struct W(crate::W<POSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POSR_SPEC>;
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
impl From<crate::W<POSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR00_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR00_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR00_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR00` writer - Pmn Output Set"]
pub type POSR00_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR00_AW, O>;
impl<'a, const O: u8> POSR00_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR00_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR00_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR01_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR01_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR01_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR01` writer - Pmn Output Set"]
pub type POSR01_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR01_AW, O>;
impl<'a, const O: u8> POSR01_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR01_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR01_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR02_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR02_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR02_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR02` writer - Pmn Output Set"]
pub type POSR02_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR02_AW, O>;
impl<'a, const O: u8> POSR02_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR02_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR02_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR03_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR03_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR03_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR03` writer - Pmn Output Set"]
pub type POSR03_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR03_AW, O>;
impl<'a, const O: u8> POSR03_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR03_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR03_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR04_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR04_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR04_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR04` writer - Pmn Output Set"]
pub type POSR04_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR04_AW, O>;
impl<'a, const O: u8> POSR04_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR04_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR04_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR05_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR05_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR05_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR05` writer - Pmn Output Set"]
pub type POSR05_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR05_AW, O>;
impl<'a, const O: u8> POSR05_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR05_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR05_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR06_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR06_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR06_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR06` writer - Pmn Output Set"]
pub type POSR06_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR06_AW, O>;
impl<'a, const O: u8> POSR06_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR06_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR06_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR07_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR07_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR07_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR07` writer - Pmn Output Set"]
pub type POSR07_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR07_AW, O>;
impl<'a, const O: u8> POSR07_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR07_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR07_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR08_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR08_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR08_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR08` writer - Pmn Output Set"]
pub type POSR08_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR08_AW, O>;
impl<'a, const O: u8> POSR08_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR08_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR08_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR09_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR09_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR09_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR09` writer - Pmn Output Set"]
pub type POSR09_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR09_AW, O>;
impl<'a, const O: u8> POSR09_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR09_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR09_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR10_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR10_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR10` writer - Pmn Output Set"]
pub type POSR10_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR10_AW, O>;
impl<'a, const O: u8> POSR10_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR10_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR10_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR11_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR11_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR11` writer - Pmn Output Set"]
pub type POSR11_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR11_AW, O>;
impl<'a, const O: u8> POSR11_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR11_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR11_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR12_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR12_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR12` writer - Pmn Output Set"]
pub type POSR12_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR12_AW, O>;
impl<'a, const O: u8> POSR12_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR12_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR12_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR13_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR13_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR13` writer - Pmn Output Set"]
pub type POSR13_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR13_AW, O>;
impl<'a, const O: u8> POSR13_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR13_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR13_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR14_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR14_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR14` writer - Pmn Output Set"]
pub type POSR14_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR14_AW, O>;
impl<'a, const O: u8> POSR14_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR14_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR14_AW::_1)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR15_AW {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<POSR15_AW> for bool {
    #[inline(always)]
    fn from(variant: POSR15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR15` writer - Pmn Output Set"]
pub type POSR15_W<'a, const O: u8> = crate::BitWriter<'a, u16, POSR_SPEC, POSR15_AW, O>;
impl<'a, const O: u8> POSR15_W<'a, O> {
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR15_AW::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR15_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr00(&mut self) -> POSR00_W<0> {
        POSR00_W::new(self)
    }
    #[doc = "Bit 1 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr01(&mut self) -> POSR01_W<1> {
        POSR01_W::new(self)
    }
    #[doc = "Bit 2 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr02(&mut self) -> POSR02_W<2> {
        POSR02_W::new(self)
    }
    #[doc = "Bit 3 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr03(&mut self) -> POSR03_W<3> {
        POSR03_W::new(self)
    }
    #[doc = "Bit 4 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr04(&mut self) -> POSR04_W<4> {
        POSR04_W::new(self)
    }
    #[doc = "Bit 5 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr05(&mut self) -> POSR05_W<5> {
        POSR05_W::new(self)
    }
    #[doc = "Bit 6 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr06(&mut self) -> POSR06_W<6> {
        POSR06_W::new(self)
    }
    #[doc = "Bit 7 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr07(&mut self) -> POSR07_W<7> {
        POSR07_W::new(self)
    }
    #[doc = "Bit 8 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr08(&mut self) -> POSR08_W<8> {
        POSR08_W::new(self)
    }
    #[doc = "Bit 9 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr09(&mut self) -> POSR09_W<9> {
        POSR09_W::new(self)
    }
    #[doc = "Bit 10 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr10(&mut self) -> POSR10_W<10> {
        POSR10_W::new(self)
    }
    #[doc = "Bit 11 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr11(&mut self) -> POSR11_W<11> {
        POSR11_W::new(self)
    }
    #[doc = "Bit 12 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr12(&mut self) -> POSR12_W<12> {
        POSR12_W::new(self)
    }
    #[doc = "Bit 13 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr13(&mut self) -> POSR13_W<13> {
        POSR13_W::new(self)
    }
    #[doc = "Bit 14 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr14(&mut self) -> POSR14_W<14> {
        POSR14_W::new(self)
    }
    #[doc = "Bit 15 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr15(&mut self) -> POSR15_W<15> {
        POSR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 3\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [posr](index.html) module"]
pub struct POSR_SPEC;
impl crate::RegisterSpec for POSR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [posr::W](W) writer structure"]
impl crate::Writable for POSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POSR to value 0"]
impl crate::Resettable for POSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
