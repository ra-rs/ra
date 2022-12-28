#[doc = "Register `ADFIFODCR` writer"]
pub struct W(crate::W<ADFIFODCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFODCR_SPEC>;
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
impl From<crate::W<ADFIFODCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFODCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC0_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC0` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC0_AW, O>;
impl<'a, const O: u8> FIFODC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC0_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC0_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC1_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC1` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC1_AW, O>;
impl<'a, const O: u8> FIFODC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC1_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC1_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC2_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC2` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC2_AW, O>;
impl<'a, const O: u8> FIFODC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC2_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC2_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC3_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC3` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC3_AW, O>;
impl<'a, const O: u8> FIFODC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC3_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC3_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC4_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC4` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC4_AW, O>;
impl<'a, const O: u8> FIFODC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC4_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC4_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC5_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC5` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC5_AW, O>;
impl<'a, const O: u8> FIFODC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC5_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC5_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC6_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC6` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC6_AW, O>;
impl<'a, const O: u8> FIFODC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC6_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC6_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC7_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC7` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC7_AW, O>;
impl<'a, const O: u8> FIFODC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC7_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC7_AW::_1)
    }
}
#[doc = "Scan Group n FIFO Data Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFODC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the data of scan group n FIFO"]
    _1 = 1,
}
impl From<FIFODC8_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFODC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFODC8` writer - Scan Group n FIFO Data Clear"]
pub type FIFODC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFODCR_SPEC, FIFODC8_AW, O>;
impl<'a, const O: u8> FIFODC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFODC8_AW::_0)
    }
    #[doc = "Clear the data of scan group n FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFODC8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc0(&mut self) -> FIFODC0_W<0> {
        FIFODC0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc1(&mut self) -> FIFODC1_W<1> {
        FIFODC1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc2(&mut self) -> FIFODC2_W<2> {
        FIFODC2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc3(&mut self) -> FIFODC3_W<3> {
        FIFODC3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc4(&mut self) -> FIFODC4_W<4> {
        FIFODC4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc5(&mut self) -> FIFODC5_W<5> {
        FIFODC5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc6(&mut self) -> FIFODC6_W<6> {
        FIFODC6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc7(&mut self) -> FIFODC7_W<7> {
        FIFODC7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n FIFO Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifodc8(&mut self) -> FIFODC8_W<8> {
        FIFODC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Data Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifodcr](index.html) module"]
pub struct ADFIFODCR_SPEC;
impl crate::RegisterSpec for ADFIFODCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adfifodcr::W](W) writer structure"]
impl crate::Writable for ADFIFODCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFODCR to value 0"]
impl crate::Resettable for ADFIFODCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
