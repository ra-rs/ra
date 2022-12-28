#[doc = "Register `SD_CLK_CTRL` reader"]
pub struct R(crate::R<SD_CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_CLK_CTRL` writer"]
pub struct W(crate::W<SD_CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CLK_CTRL_SPEC>;
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
impl From<crate::W<SD_CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - SDHI Clock Frequency Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "SDHI Clock Frequency Select\n\nValue on reset: 32"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: PCLKA divided by 2"]
    _0X00 = 0,
    #[doc = "1: PCLKA divided by 4"]
    _0X01 = 1,
    #[doc = "2: PCLKA divided by 8"]
    _0X02 = 2,
    #[doc = "4: PCLKA divided by 16"]
    _0X04 = 4,
    #[doc = "8: PCLKA divided by 32"]
    _0X08 = 8,
    #[doc = "16: PCLKA divided by 64"]
    _0X10 = 16,
    #[doc = "32: PCLKA divided by 128"]
    _0X20 = 32,
    #[doc = "64: PCLKA divided by 256"]
    _0X40 = 64,
    #[doc = "128: PCLKA divided by 512"]
    _0X80 = 128,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::_0X00),
            1 => Some(CLKSEL_A::_0X01),
            2 => Some(CLKSEL_A::_0X02),
            4 => Some(CLKSEL_A::_0X04),
            8 => Some(CLKSEL_A::_0X08),
            16 => Some(CLKSEL_A::_0X10),
            32 => Some(CLKSEL_A::_0X20),
            64 => Some(CLKSEL_A::_0X40),
            128 => Some(CLKSEL_A::_0X80),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CLKSEL_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == CLKSEL_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == CLKSEL_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == CLKSEL_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == CLKSEL_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == CLKSEL_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X20`"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == CLKSEL_A::_0X20
    }
    #[doc = "Checks if the value of the field is `_0X40`"]
    #[inline(always)]
    pub fn is_0x40(&self) -> bool {
        *self == CLKSEL_A::_0X40
    }
    #[doc = "Checks if the value of the field is `_0X80`"]
    #[inline(always)]
    pub fn is_0x80(&self) -> bool {
        *self == CLKSEL_A::_0X80
    }
}
#[doc = "Field `CLKSEL` writer - SDHI Clock Frequency Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SD_CLK_CTRL_SPEC, u8, CLKSEL_A, 8, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "PCLKA divided by 2"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X00)
    }
    #[doc = "PCLKA divided by 4"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X01)
    }
    #[doc = "PCLKA divided by 8"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X02)
    }
    #[doc = "PCLKA divided by 16"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X04)
    }
    #[doc = "PCLKA divided by 32"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X08)
    }
    #[doc = "PCLKA divided by 64"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X10)
    }
    #[doc = "PCLKA divided by 128"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X20)
    }
    #[doc = "PCLKA divided by 256"]
    #[inline(always)]
    pub fn _0x40(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X40)
    }
    #[doc = "PCLKA divided by 512"]
    #[inline(always)]
    pub fn _0x80(self) -> &'a mut W {
        self.variant(CLKSEL_A::_0X80)
    }
}
#[doc = "Field `CLKEN` reader - SD/MMC Clock Output Control Enable"]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
#[doc = "SD/MMC Clock Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    #[doc = "0: SD/MMC Clock output is disabled. The SDCLK signal is fixed 0."]
    _0 = 0,
    #[doc = "1: SD/MMC Clock output is enabled."]
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
#[doc = "Field `CLKEN` writer - SD/MMC Clock Output Control Enable"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CLK_CTRL_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    #[doc = "SD/MMC Clock output is disabled. The SDCLK signal is fixed 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKEN_A::_0)
    }
    #[doc = "SD/MMC Clock output is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKEN_A::_1)
    }
}
#[doc = "Field `CLKCTRLEN` reader - SD/MMC Clock Output Automatic Control Enable"]
pub type CLKCTRLEN_R = crate::BitReader<CLKCTRLEN_A>;
#[doc = "SD/MMC Clock Output Automatic Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKCTRLEN_A {
    #[doc = "0: Automatic control for SD/MMC Clock output is disabled."]
    _0 = 0,
    #[doc = "1: Automatic control for SD/MMC Clock output is enabled."]
    _1 = 1,
}
impl From<CLKCTRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKCTRLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKCTRLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKCTRLEN_A {
        match self.bits {
            false => CLKCTRLEN_A::_0,
            true => CLKCTRLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKCTRLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKCTRLEN_A::_1
    }
}
#[doc = "Field `CLKCTRLEN` writer - SD/MMC Clock Output Automatic Control Enable"]
pub type CLKCTRLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CLK_CTRL_SPEC, CLKCTRLEN_A, O>;
impl<'a, const O: u8> CLKCTRLEN_W<'a, O> {
    #[doc = "Automatic control for SD/MMC Clock output is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKCTRLEN_A::_0)
    }
    #[doc = "Automatic control for SD/MMC Clock output is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKCTRLEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - SDHI Clock Frequency Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - SD/MMC Clock Output Control Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SD/MMC Clock Output Automatic Control Enable"]
    #[inline(always)]
    pub fn clkctrlen(&self) -> CLKCTRLEN_R {
        CLKCTRLEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDHI Clock Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 8 - SD/MMC Clock Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<8> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 9 - SD/MMC Clock Output Automatic Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkctrlen(&mut self) -> CLKCTRLEN_W<9> {
        CLKCTRLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_clk_ctrl](index.html) module"]
pub struct SD_CLK_CTRL_SPEC;
impl crate::RegisterSpec for SD_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_clk_ctrl::R](R) reader structure"]
impl crate::Readable for SD_CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_clk_ctrl::W](W) writer structure"]
impl crate::Writable for SD_CLK_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_CLK_CTRL to value 0x20"]
impl crate::Resettable for SD_CLK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
