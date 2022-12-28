#[doc = "Register `OUT_CLKPHASE` reader"]
pub struct R(crate::R<OUT_CLKPHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CLKPHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CLKPHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CLKPHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CLKPHASE` writer"]
pub struct W(crate::W<OUT_CLKPHASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CLKPHASE_SPEC>;
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
impl From<crate::W<OUT_CLKPHASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CLKPHASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCON3EDGE` reader - LCD_TCON3 Output Phase Control"]
pub type TCON3EDGE_R = crate::BitReader<TCON3EDGE_A>;
#[doc = "LCD_TCON3 Output Phase Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON3EDGE_A {
    #[doc = "1: In synchronization with the falling edge of LCD_CLK."]
    _1 = 1,
    #[doc = "0: In synchronization with the rising edge of LCD_CLK."]
    _0 = 0,
}
impl From<TCON3EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON3EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCON3EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCON3EDGE_A {
        match self.bits {
            true => TCON3EDGE_A::_1,
            false => TCON3EDGE_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON3EDGE_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON3EDGE_A::_0
    }
}
#[doc = "Field `TCON3EDGE` writer - LCD_TCON3 Output Phase Control"]
pub type TCON3EDGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CLKPHASE_SPEC, TCON3EDGE_A, O>;
impl<'a, const O: u8> TCON3EDGE_W<'a, O> {
    #[doc = "In synchronization with the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCON3EDGE_A::_1)
    }
    #[doc = "In synchronization with the rising edge of LCD_CLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCON3EDGE_A::_0)
    }
}
#[doc = "Field `TCON2EDGE` reader - LCD_TCON2 Output Phase Control"]
pub type TCON2EDGE_R = crate::BitReader<TCON2EDGE_A>;
#[doc = "LCD_TCON2 Output Phase Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON2EDGE_A {
    #[doc = "1: In synchronization with the falling edge of LCD_CLK."]
    _1 = 1,
    #[doc = "0: In synchronization with the rising edge of LCD_CLK."]
    _0 = 0,
}
impl From<TCON2EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON2EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCON2EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCON2EDGE_A {
        match self.bits {
            true => TCON2EDGE_A::_1,
            false => TCON2EDGE_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON2EDGE_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON2EDGE_A::_0
    }
}
#[doc = "Field `TCON2EDGE` writer - LCD_TCON2 Output Phase Control"]
pub type TCON2EDGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CLKPHASE_SPEC, TCON2EDGE_A, O>;
impl<'a, const O: u8> TCON2EDGE_W<'a, O> {
    #[doc = "In synchronization with the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCON2EDGE_A::_1)
    }
    #[doc = "In synchronization with the rising edge of LCD_CLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCON2EDGE_A::_0)
    }
}
#[doc = "Field `TCON1EDGE` reader - LCD_TCON1 Output Phase Control"]
pub type TCON1EDGE_R = crate::BitReader<TCON1EDGE_A>;
#[doc = "LCD_TCON1 Output Phase Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON1EDGE_A {
    #[doc = "1: In synchronization with the falling edge of LCD_CLK."]
    _1 = 1,
    #[doc = "0: In synchronization with the rising edge of LCD_CLK."]
    _0 = 0,
}
impl From<TCON1EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON1EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCON1EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCON1EDGE_A {
        match self.bits {
            true => TCON1EDGE_A::_1,
            false => TCON1EDGE_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON1EDGE_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON1EDGE_A::_0
    }
}
#[doc = "Field `TCON1EDGE` writer - LCD_TCON1 Output Phase Control"]
pub type TCON1EDGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CLKPHASE_SPEC, TCON1EDGE_A, O>;
impl<'a, const O: u8> TCON1EDGE_W<'a, O> {
    #[doc = "In synchronization with the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCON1EDGE_A::_1)
    }
    #[doc = "In synchronization with the rising edge of LCD_CLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCON1EDGE_A::_0)
    }
}
#[doc = "Field `TCON0EDGE` reader - LCD_TCON0 Output Phase Control"]
pub type TCON0EDGE_R = crate::BitReader<TCON0EDGE_A>;
#[doc = "LCD_TCON0 Output Phase Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON0EDGE_A {
    #[doc = "1: In synchronization with the falling edge of LCD_CLK."]
    _1 = 1,
    #[doc = "0: In synchronization with the rising edge of LCD_CLK."]
    _0 = 0,
}
impl From<TCON0EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON0EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCON0EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCON0EDGE_A {
        match self.bits {
            true => TCON0EDGE_A::_1,
            false => TCON0EDGE_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON0EDGE_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON0EDGE_A::_0
    }
}
#[doc = "Field `TCON0EDGE` writer - LCD_TCON0 Output Phase Control"]
pub type TCON0EDGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CLKPHASE_SPEC, TCON0EDGE_A, O>;
impl<'a, const O: u8> TCON0EDGE_W<'a, O> {
    #[doc = "In synchronization with the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCON0EDGE_A::_1)
    }
    #[doc = "In synchronization with the rising edge of LCD_CLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCON0EDGE_A::_0)
    }
}
#[doc = "Field `LCDEDGE` reader - LCD_DATA Output Phase Control"]
pub type LCDEDGE_R = crate::BitReader<LCDEDGE_A>;
#[doc = "LCD_DATA Output Phase Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDEDGE_A {
    #[doc = "0: In synchronization with the rising edge of LCD_CLK."]
    _0 = 0,
    #[doc = "1: In synchronization with the falling edge of LCD_CLK"]
    _1 = 1,
}
impl From<LCDEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDEDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDEDGE_A {
        match self.bits {
            false => LCDEDGE_A::_0,
            true => LCDEDGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDEDGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDEDGE_A::_1
    }
}
#[doc = "Field `LCDEDGE` writer - LCD_DATA Output Phase Control"]
pub type LCDEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUT_CLKPHASE_SPEC, LCDEDGE_A, O>;
impl<'a, const O: u8> LCDEDGE_W<'a, O> {
    #[doc = "In synchronization with the rising edge of LCD_CLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCDEDGE_A::_0)
    }
    #[doc = "In synchronization with the falling edge of LCD_CLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCDEDGE_A::_1)
    }
}
#[doc = "Field `FRONTGAM` reader - Correction control"]
pub type FRONTGAM_R = crate::BitReader<FRONTGAM_A>;
#[doc = "Correction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRONTGAM_A {
    #[doc = "1: Gamma correction is followed by brightness/contrast correction."]
    _1 = 1,
    #[doc = "0: Brightness/contrast correction is followed by gamma correction."]
    _0 = 0,
}
impl From<FRONTGAM_A> for bool {
    #[inline(always)]
    fn from(variant: FRONTGAM_A) -> Self {
        variant as u8 != 0
    }
}
impl FRONTGAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRONTGAM_A {
        match self.bits {
            true => FRONTGAM_A::_1,
            false => FRONTGAM_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRONTGAM_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRONTGAM_A::_0
    }
}
#[doc = "Field `FRONTGAM` writer - Correction control"]
pub type FRONTGAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUT_CLKPHASE_SPEC, FRONTGAM_A, O>;
impl<'a, const O: u8> FRONTGAM_W<'a, O> {
    #[doc = "Gamma correction is followed by brightness/contrast correction."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRONTGAM_A::_1)
    }
    #[doc = "Brightness/contrast correction is followed by gamma correction."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRONTGAM_A::_0)
    }
}
impl R {
    #[doc = "Bit 3 - LCD_TCON3 Output Phase Control"]
    #[inline(always)]
    pub fn tcon3edge(&self) -> TCON3EDGE_R {
        TCON3EDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD_TCON2 Output Phase Control"]
    #[inline(always)]
    pub fn tcon2edge(&self) -> TCON2EDGE_R {
        TCON2EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD_TCON1 Output Phase Control"]
    #[inline(always)]
    pub fn tcon1edge(&self) -> TCON1EDGE_R {
        TCON1EDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD_TCON0 Output Phase Control"]
    #[inline(always)]
    pub fn tcon0edge(&self) -> TCON0EDGE_R {
        TCON0EDGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD_DATA Output Phase Control"]
    #[inline(always)]
    pub fn lcdedge(&self) -> LCDEDGE_R {
        LCDEDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Correction control"]
    #[inline(always)]
    pub fn frontgam(&self) -> FRONTGAM_R {
        FRONTGAM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LCD_TCON3 Output Phase Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcon3edge(&mut self) -> TCON3EDGE_W<3> {
        TCON3EDGE_W::new(self)
    }
    #[doc = "Bit 4 - LCD_TCON2 Output Phase Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcon2edge(&mut self) -> TCON2EDGE_W<4> {
        TCON2EDGE_W::new(self)
    }
    #[doc = "Bit 5 - LCD_TCON1 Output Phase Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcon1edge(&mut self) -> TCON1EDGE_W<5> {
        TCON1EDGE_W::new(self)
    }
    #[doc = "Bit 6 - LCD_TCON0 Output Phase Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcon0edge(&mut self) -> TCON0EDGE_W<6> {
        TCON0EDGE_W::new(self)
    }
    #[doc = "Bit 8 - LCD_DATA Output Phase Control"]
    #[inline(always)]
    #[must_use]
    pub fn lcdedge(&mut self) -> LCDEDGE_W<8> {
        LCDEDGE_W::new(self)
    }
    #[doc = "Bit 12 - Correction control"]
    #[inline(always)]
    #[must_use]
    pub fn frontgam(&mut self) -> FRONTGAM_W<12> {
        FRONTGAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Control Block Output Phase Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_clkphase](index.html) module"]
pub struct OUT_CLKPHASE_SPEC;
impl crate::RegisterSpec for OUT_CLKPHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_clkphase::R](R) reader structure"]
impl crate::Readable for OUT_CLKPHASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_clkphase::W](W) writer structure"]
impl crate::Writable for OUT_CLKPHASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CLKPHASE to value 0"]
impl crate::Resettable for OUT_CLKPHASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
