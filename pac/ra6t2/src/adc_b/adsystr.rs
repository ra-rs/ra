#[doc = "Register `ADSYSTR` writer"]
pub struct W(crate::W<ADSYSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSYSTR_SPEC>;
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
impl From<crate::W<ADSYSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSYSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST0_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST0` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST0_AW, O>;
impl<'a, const O: u8> ADSYST0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST0_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST0_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST1_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST1` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST1_AW, O>;
impl<'a, const O: u8> ADSYST1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST1_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST1_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST2_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST2` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST2_AW, O>;
impl<'a, const O: u8> ADSYST2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST2_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST2_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST3_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST3` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST3_AW, O>;
impl<'a, const O: u8> ADSYST3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST3_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST3_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST4_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST4` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST4_AW, O>;
impl<'a, const O: u8> ADSYST4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST4_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST4_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST5_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST5` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST5_AW, O>;
impl<'a, const O: u8> ADSYST5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST5_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST5_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST6_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST6` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST6_AW, O>;
impl<'a, const O: u8> ADSYST6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST6_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST6_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST7_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST7` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST7_AW, O>;
impl<'a, const O: u8> ADSYST7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST7_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST7_AW::_1)
    }
}
#[doc = "Scan Group n: A/D Conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSYST8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADSYST8_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSYST8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSYST8` writer - Scan Group n: A/D Conversion start"]
pub type ADSYST8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSYSTR_SPEC, ADSYST8_AW, O>;
impl<'a, const O: u8> ADSYST8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSYST8_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSYST8_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst0(&mut self) -> ADSYST0_W<0> {
        ADSYST0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst1(&mut self) -> ADSYST1_W<1> {
        ADSYST1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst2(&mut self) -> ADSYST2_W<2> {
        ADSYST2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst3(&mut self) -> ADSYST3_W<3> {
        ADSYST3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst4(&mut self) -> ADSYST4_W<4> {
        ADSYST4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst5(&mut self) -> ADSYST5_W<5> {
        ADSYST5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst6(&mut self) -> ADSYST6_W<6> {
        ADSYST6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst7(&mut self) -> ADSYST7_W<7> {
        ADSYST7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n: A/D Conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adsyst8(&mut self) -> ADSYST8_W<8> {
        ADSYST8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Synchronous Software Start Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsystr](index.html) module"]
pub struct ADSYSTR_SPEC;
impl crate::RegisterSpec for ADSYSTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adsystr::W](W) writer structure"]
impl crate::Writable for ADSYSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSYSTR to value 0"]
impl crate::Resettable for ADSYSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
