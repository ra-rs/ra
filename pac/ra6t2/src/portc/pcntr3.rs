#[doc = "Register `PCNTR3` writer"]
pub struct W(crate::W<PCNTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNTR3_SPEC>;
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
impl From<crate::W<PCNTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR00_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR01_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR02_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR03_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR04_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR05_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR06_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR07_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR08_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR09_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR10_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR11_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR12_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR13_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR14_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type POSR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, POSR15_AW, O>;
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
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR00_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR01_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR02_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR03_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR04_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR05_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR06_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR07_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR08_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR09_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR10_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR11_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR12_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR13_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR14_AW, O>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PORR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTR3_SPEC, PORR15_AW, O>;
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
    #[doc = "Bit 0 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr00(&mut self) -> POSR00_W<0> {
        POSR00_W::new(self)
    }
    #[doc = "Bit 1 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr01(&mut self) -> POSR01_W<1> {
        POSR01_W::new(self)
    }
    #[doc = "Bit 2 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr02(&mut self) -> POSR02_W<2> {
        POSR02_W::new(self)
    }
    #[doc = "Bit 3 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr03(&mut self) -> POSR03_W<3> {
        POSR03_W::new(self)
    }
    #[doc = "Bit 4 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr04(&mut self) -> POSR04_W<4> {
        POSR04_W::new(self)
    }
    #[doc = "Bit 5 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr05(&mut self) -> POSR05_W<5> {
        POSR05_W::new(self)
    }
    #[doc = "Bit 6 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr06(&mut self) -> POSR06_W<6> {
        POSR06_W::new(self)
    }
    #[doc = "Bit 7 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr07(&mut self) -> POSR07_W<7> {
        POSR07_W::new(self)
    }
    #[doc = "Bit 8 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr08(&mut self) -> POSR08_W<8> {
        POSR08_W::new(self)
    }
    #[doc = "Bit 9 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr09(&mut self) -> POSR09_W<9> {
        POSR09_W::new(self)
    }
    #[doc = "Bit 10 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr10(&mut self) -> POSR10_W<10> {
        POSR10_W::new(self)
    }
    #[doc = "Bit 11 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr11(&mut self) -> POSR11_W<11> {
        POSR11_W::new(self)
    }
    #[doc = "Bit 12 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr12(&mut self) -> POSR12_W<12> {
        POSR12_W::new(self)
    }
    #[doc = "Bit 13 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr13(&mut self) -> POSR13_W<13> {
        POSR13_W::new(self)
    }
    #[doc = "Bit 14 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr14(&mut self) -> POSR14_W<14> {
        POSR14_W::new(self)
    }
    #[doc = "Bit 15 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr15(&mut self) -> POSR15_W<15> {
        POSR15_W::new(self)
    }
    #[doc = "Bit 16 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr00(&mut self) -> PORR00_W<16> {
        PORR00_W::new(self)
    }
    #[doc = "Bit 17 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr01(&mut self) -> PORR01_W<17> {
        PORR01_W::new(self)
    }
    #[doc = "Bit 18 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr02(&mut self) -> PORR02_W<18> {
        PORR02_W::new(self)
    }
    #[doc = "Bit 19 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr03(&mut self) -> PORR03_W<19> {
        PORR03_W::new(self)
    }
    #[doc = "Bit 20 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr04(&mut self) -> PORR04_W<20> {
        PORR04_W::new(self)
    }
    #[doc = "Bit 21 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr05(&mut self) -> PORR05_W<21> {
        PORR05_W::new(self)
    }
    #[doc = "Bit 22 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr06(&mut self) -> PORR06_W<22> {
        PORR06_W::new(self)
    }
    #[doc = "Bit 23 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr07(&mut self) -> PORR07_W<23> {
        PORR07_W::new(self)
    }
    #[doc = "Bit 24 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr08(&mut self) -> PORR08_W<24> {
        PORR08_W::new(self)
    }
    #[doc = "Bit 25 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr09(&mut self) -> PORR09_W<25> {
        PORR09_W::new(self)
    }
    #[doc = "Bit 26 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr10(&mut self) -> PORR10_W<26> {
        PORR10_W::new(self)
    }
    #[doc = "Bit 27 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr11(&mut self) -> PORR11_W<27> {
        PORR11_W::new(self)
    }
    #[doc = "Bit 28 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr12(&mut self) -> PORR12_W<28> {
        PORR12_W::new(self)
    }
    #[doc = "Bit 29 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr13(&mut self) -> PORR13_W<29> {
        PORR13_W::new(self)
    }
    #[doc = "Bit 30 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr14(&mut self) -> PORR14_W<30> {
        PORR14_W::new(self)
    }
    #[doc = "Bit 31 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr15(&mut self) -> PORR15_W<31> {
        PORR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register 3\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntr3](index.html) module"]
pub struct PCNTR3_SPEC;
impl crate::RegisterSpec for PCNTR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcntr3::W](W) writer structure"]
impl crate::Writable for PCNTR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNTR3 to value 0"]
impl crate::Resettable for PCNTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
