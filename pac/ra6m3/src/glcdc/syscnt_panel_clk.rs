#[doc = "Register `SYSCNT_PANEL_CLK` reader"]
pub struct R(crate::R<SYSCNT_PANEL_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCNT_PANEL_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCNT_PANEL_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCNT_PANEL_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCNT_PANEL_CLK` writer"]
pub struct W(crate::W<SYSCNT_PANEL_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCNT_PANEL_CLK_SPEC>;
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
impl From<crate::W<SYSCNT_PANEL_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCNT_PANEL_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDR` reader - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited."]
pub type DCDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCDR` writer - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited."]
pub type DCDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCNT_PANEL_CLK_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLKEN` reader - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0."]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
#[doc = "Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    #[doc = "0: Disable panel clock output"]
    _0 = 0,
    #[doc = "1: Enable panel clock output"]
    _1 = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::_0,
            true => CLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKEN_A::_1
    }
}
#[doc = "Field `CLKEN` writer - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0."]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCNT_PANEL_CLK_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    #[doc = "Disable panel clock output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKEN_A::_0)
    }
    #[doc = "Enable panel clock output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKEN_A::_1)
    }
}
#[doc = "Field `CLKSEL` reader - Panel clock supply source select"]
pub type CLKSEL_R = crate::BitReader<CLKSEL_A>;
#[doc = "Panel clock supply source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSEL_A {
    #[doc = "0: External clock select"]
    _0 = 0,
    #[doc = "1: PLL output select"]
    _1 = 1,
}
impl From<CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            false => CLKSEL_A::_0,
            true => CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSEL_A::_1
    }
}
#[doc = "Field `CLKSEL` writer - Panel clock supply source select"]
pub type CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCNT_PANEL_CLK_SPEC, CLKSEL_A, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "External clock select"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0)
    }
    #[doc = "PLL output select"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKSEL_A::_1)
    }
}
#[doc = "Field `PIXSEL` reader - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\\[1\\]."]
pub type PIXSEL_R = crate::BitReader<PIXSEL_A>;
#[doc = "Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\\[1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIXSEL_A {
    #[doc = "0: No frequency division, parallel RGB"]
    _0 = 0,
    #[doc = "1: Quarter frequency,serial RGB"]
    _1 = 1,
}
impl From<PIXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PIXSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PIXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXSEL_A {
        match self.bits {
            false => PIXSEL_A::_0,
            true => PIXSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIXSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIXSEL_A::_1
    }
}
#[doc = "Field `PIXSEL` writer - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\\[1\\]."]
pub type PIXSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCNT_PANEL_CLK_SPEC, PIXSEL_A, O>;
impl<'a, const O: u8> PIXSEL_W<'a, O> {
    #[doc = "No frequency division, parallel RGB"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIXSEL_A::_0)
    }
    #[doc = "Quarter frequency,serial RGB"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIXSEL_A::_1)
    }
}
#[doc = "Field `VER` reader - Version informationVersion information of the GLCD"]
pub type VER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited."]
    #[inline(always)]
    pub fn dcdr(&self) -> DCDR_R {
        DCDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Panel clock supply source select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\\[1\\]."]
    #[inline(always)]
    pub fn pixsel(&self) -> PIXSEL_R {
        PIXSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Version informationVersion information of the GLCD"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited."]
    #[inline(always)]
    #[must_use]
    pub fn dcdr(&mut self) -> DCDR_W<0> {
        DCDR_W::new(self)
    }
    #[doc = "Bit 6 - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<6> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 8 - Panel clock supply source select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<8> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 12 - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn pixsel(&mut self) -> PIXSEL_W<12> {
        PIXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Block Version and Panel Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscnt_panel_clk](index.html) module"]
pub struct SYSCNT_PANEL_CLK_SPEC;
impl crate::RegisterSpec for SYSCNT_PANEL_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscnt_panel_clk::R](R) reader structure"]
impl crate::Readable for SYSCNT_PANEL_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscnt_panel_clk::W](W) writer structure"]
impl crate::Writable for SYSCNT_PANEL_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCNT_PANEL_CLK to value 0x0110_0000"]
impl crate::Resettable for SYSCNT_PANEL_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110_0000;
}
