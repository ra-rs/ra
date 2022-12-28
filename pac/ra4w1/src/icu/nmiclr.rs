#[doc = "Register `NMICLR` reader"]
pub struct R(crate::R<NMICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMICLR` writer"]
pub struct W(crate::W<NMICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMICLR_SPEC>;
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
impl From<crate::W<NMICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IWDT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.IWDTST flag."]
    _1 = 1,
}
impl From<IWDTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: IWDTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTCLR` writer - IWDT Clear"]
pub type IWDTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, IWDTCLR_AW, O>;
impl<'a, const O: u8> IWDTCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IWDTCLR_AW::_0)
    }
    #[doc = "Clear the NMISR.IWDTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IWDTCLR_AW::_1)
    }
}
#[doc = "WDT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.WDTST flag."]
    _1 = 1,
}
impl From<WDTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCLR` writer - WDT Clear"]
pub type WDTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, WDTCLR_AW, O>;
impl<'a, const O: u8> WDTCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTCLR_AW::_0)
    }
    #[doc = "Clear the NMISR.WDTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTCLR_AW::_1)
    }
}
#[doc = "LVD1 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1CLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD1ST flag."]
    _1 = 1,
}
impl From<LVD1CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: LVD1CLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1CLR` writer - LVD1 Clear"]
pub type LVD1CLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, LVD1CLR_AW, O>;
impl<'a, const O: u8> LVD1CLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1CLR_AW::_0)
    }
    #[doc = "Clear the NMISR.LVD1ST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1CLR_AW::_1)
    }
}
#[doc = "LVD2 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2CLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.LVD2ST flag."]
    _1 = 1,
}
impl From<LVD2CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: LVD2CLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2CLR` writer - LVD2 Clear"]
pub type LVD2CLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, LVD2CLR_AW, O>;
impl<'a, const O: u8> LVD2CLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2CLR_AW::_0)
    }
    #[doc = "Clear the NMISR.LVD2ST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2CLR_AW::_1)
    }
}
#[doc = "VBATT Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATTCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.VBATTST flag."]
    _1 = 1,
}
impl From<VBATTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATTCLR` writer - VBATT Clear"]
pub type VBATTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, VBATTCLR_AW, O>;
impl<'a, const O: u8> VBATTCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBATTCLR_AW::_0)
    }
    #[doc = "Clear the NMISR.VBATTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBATTCLR_AW::_1)
    }
}
#[doc = "OST Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.OSTST flag."]
    _1 = 1,
}
impl From<OSTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: OSTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTCLR` writer - OST Clear"]
pub type OSTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, OSTCLR_AW, O>;
impl<'a, const O: u8> OSTCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSTCLR_AW::_0)
    }
    #[doc = "Clear the NMISR.OSTST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSTCLR_AW::_1)
    }
}
#[doc = "NMI Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMICLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.NMIST flag."]
    _1 = 1,
}
impl From<NMICLR_AW> for bool {
    #[inline(always)]
    fn from(variant: NMICLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMICLR` writer - NMI Clear"]
pub type NMICLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, NMICLR_AW, O>;
impl<'a, const O: u8> NMICLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMICLR_AW::_0)
    }
    #[doc = "Clear the NMISR.NMIST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMICLR_AW::_1)
    }
}
#[doc = "SRAM Parity Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPECLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.RPEST flag."]
    _1 = 1,
}
impl From<RPECLR_AW> for bool {
    #[inline(always)]
    fn from(variant: RPECLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPECLR` writer - SRAM Parity Error Clear"]
pub type RPECLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, RPECLR_AW, O>;
impl<'a, const O: u8> RPECLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPECLR_AW::_0)
    }
    #[doc = "Clear the NMISR.RPEST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPECLR_AW::_1)
    }
}
#[doc = "SRAM ECC Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECCCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.RECCST flag."]
    _1 = 1,
}
impl From<RECCCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: RECCCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECCCLR` writer - SRAM ECC Error Clear"]
pub type RECCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, RECCCLR_AW, O>;
impl<'a, const O: u8> RECCCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RECCCLR_AW::_0)
    }
    #[doc = "Clear the NMISR.RECCST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RECCCLR_AW::_1)
    }
}
#[doc = "Bus Slave Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.BUSSST flag."]
    _1 = 1,
}
impl From<BUSSCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: BUSSCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSSCLR` writer - Bus Slave Error Clear"]
pub type BUSSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, BUSSCLR_AW, O>;
impl<'a, const O: u8> BUSSCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSSCLR_AW::_0)
    }
    #[doc = "Clear the NMISR.BUSSST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSSCLR_AW::_1)
    }
}
#[doc = "Bus Master Error Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMCLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.BUSMST flag."]
    _1 = 1,
}
impl From<BUSMCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: BUSMCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSMCLR` writer - Bus Master Error Clear"]
pub type BUSMCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, BUSMCLR_AW, O>;
impl<'a, const O: u8> BUSMCLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSMCLR_AW::_0)
    }
    #[doc = "Clear the NMISR.BUSMST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSMCLR_AW::_1)
    }
}
#[doc = "CPU Stack Pointer Monitor Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPECLR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Clear the NMISR.SPEST flag."]
    _1 = 1,
}
impl From<SPECLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SPECLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECLR` writer - CPU Stack Pointer Monitor Interrupt Clear"]
pub type SPECLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NMICLR_SPEC, SPECLR_AW, O>;
impl<'a, const O: u8> SPECLR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPECLR_AW::_0)
    }
    #[doc = "Clear the NMISR.SPEST flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPECLR_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - IWDT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn iwdtclr(&mut self) -> IWDTCLR_W<0> {
        IWDTCLR_W::new(self)
    }
    #[doc = "Bit 1 - WDT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdtclr(&mut self) -> WDTCLR_W<1> {
        WDTCLR_W::new(self)
    }
    #[doc = "Bit 2 - LVD1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1clr(&mut self) -> LVD1CLR_W<2> {
        LVD1CLR_W::new(self)
    }
    #[doc = "Bit 3 - LVD2 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2clr(&mut self) -> LVD2CLR_W<3> {
        LVD2CLR_W::new(self)
    }
    #[doc = "Bit 4 - VBATT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbattclr(&mut self) -> VBATTCLR_W<4> {
        VBATTCLR_W::new(self)
    }
    #[doc = "Bit 6 - OST Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ostclr(&mut self) -> OSTCLR_W<6> {
        OSTCLR_W::new(self)
    }
    #[doc = "Bit 7 - NMI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nmiclr(&mut self) -> NMICLR_W<7> {
        NMICLR_W::new(self)
    }
    #[doc = "Bit 8 - SRAM Parity Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpeclr(&mut self) -> RPECLR_W<8> {
        RPECLR_W::new(self)
    }
    #[doc = "Bit 9 - SRAM ECC Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn reccclr(&mut self) -> RECCCLR_W<9> {
        RECCCLR_W::new(self)
    }
    #[doc = "Bit 10 - Bus Slave Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bussclr(&mut self) -> BUSSCLR_W<10> {
        BUSSCLR_W::new(self)
    }
    #[doc = "Bit 11 - Bus Master Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn busmclr(&mut self) -> BUSMCLR_W<11> {
        BUSMCLR_W::new(self)
    }
    #[doc = "Bit 12 - CPU Stack Pointer Monitor Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn speclr(&mut self) -> SPECLR_W<12> {
        SPECLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Maskable Interrupt Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmiclr](index.html) module"]
pub struct NMICLR_SPEC;
impl crate::RegisterSpec for NMICLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nmiclr::R](R) reader structure"]
impl crate::Readable for NMICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmiclr::W](W) writer structure"]
impl crate::Writable for NMICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMICLR to value 0"]
impl crate::Resettable for NMICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
