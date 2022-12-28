#[doc = "Register `PERFTRIGGER` writer"]
pub struct W(crate::W<PERFTRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFTRIGGER_SPEC>;
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
impl From<crate::W<PERFTRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERFTRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the internal event that will increment PERFCOUNT1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PERFTRIGGER1_AW {
    #[doc = "0: disable performance counter"]
    _0X00 = 0,
    #[doc = "1: 2D Drawing Engine active cycles"]
    _0X01 = 1,
    #[doc = "2: framebuffer read access"]
    _0X02 = 2,
    #[doc = "3: framebuffer write access"]
    _0X03 = 3,
    #[doc = "4: texture read access"]
    _0X04 = 4,
    #[doc = "5: invisible pixels (enumerated but selected with alpha 0percent)"]
    _0X05 = 5,
    #[doc = "6: invisible pixels while internal FIFO is empty (lost cycles)"]
    _0X06 = 6,
    #[doc = "7: display list reader active cycles"]
    _0X07 = 7,
    #[doc = "8: framebuffer read hits"]
    _0X08 = 8,
    #[doc = "9: framebuffer read misses"]
    _0X09 = 9,
    #[doc = "10: framebuffer write hits"]
    _0X0A = 10,
    #[doc = "11: framebuffer write misses"]
    _0X0B = 11,
    #[doc = "12: texture read hits"]
    _0X0C = 12,
    #[doc = "13: texture read misses"]
    _0X0D = 13,
    #[doc = "31: every clock cycle (for use as timer)"]
    _0X1F = 31,
}
impl From<PERFTRIGGER1_AW> for u16 {
    #[inline(always)]
    fn from(variant: PERFTRIGGER1_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PERFTRIGGER1` writer - Selects the internal event that will increment PERFCOUNT1 register."]
pub type PERFTRIGGER1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERFTRIGGER_SPEC, u16, PERFTRIGGER1_AW, 16, O>;
impl<'a, const O: u8> PERFTRIGGER1_W<'a, O> {
    #[doc = "disable performance counter"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X00)
    }
    #[doc = "2D Drawing Engine active cycles"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X01)
    }
    #[doc = "framebuffer read access"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X02)
    }
    #[doc = "framebuffer write access"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X03)
    }
    #[doc = "texture read access"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X04)
    }
    #[doc = "invisible pixels (enumerated but selected with alpha 0percent)"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X05)
    }
    #[doc = "invisible pixels while internal FIFO is empty (lost cycles)"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X06)
    }
    #[doc = "display list reader active cycles"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X07)
    }
    #[doc = "framebuffer read hits"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X08)
    }
    #[doc = "framebuffer read misses"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X09)
    }
    #[doc = "framebuffer write hits"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X0A)
    }
    #[doc = "framebuffer write misses"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X0B)
    }
    #[doc = "texture read hits"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X0C)
    }
    #[doc = "texture read misses"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X0D)
    }
    #[doc = "every clock cycle (for use as timer)"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(PERFTRIGGER1_AW::_0X1F)
    }
}
#[doc = "Selects the internal event that will increment PERFCOUNT2 register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PERFTRIGGER2_AW {
    #[doc = "0: disable performance counter"]
    _0X00 = 0,
    #[doc = "1: 2D Drawing Engine active cycles"]
    _0X01 = 1,
    #[doc = "2: framebuffer read access"]
    _0X02 = 2,
    #[doc = "3: framebuffer write access"]
    _0X03 = 3,
    #[doc = "4: texture read access"]
    _0X04 = 4,
    #[doc = "5: invisible pixels (enumerated but selected with alpha 0percent)"]
    _0X05 = 5,
    #[doc = "6: invisible pixels while internal FIFO is empty (lost cycles)"]
    _0X06 = 6,
    #[doc = "7: display list reader active cycles"]
    _0X07 = 7,
    #[doc = "8: framebuffer read hits"]
    _0X08 = 8,
    #[doc = "9: framebuffer read misses"]
    _0X09 = 9,
    #[doc = "10: framebuffer write hits"]
    _0X0A = 10,
    #[doc = "11: framebuffer write misses"]
    _0X0B = 11,
    #[doc = "12: texture read hits"]
    _0X0C = 12,
    #[doc = "13: texture read misses"]
    _0X0D = 13,
    #[doc = "31: every clock cycle (for use as timer)"]
    _0X1F = 31,
}
impl From<PERFTRIGGER2_AW> for u16 {
    #[inline(always)]
    fn from(variant: PERFTRIGGER2_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PERFTRIGGER2` writer - Selects the internal event that will increment PERFCOUNT2 register"]
pub type PERFTRIGGER2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERFTRIGGER_SPEC, u16, PERFTRIGGER2_AW, 16, O>;
impl<'a, const O: u8> PERFTRIGGER2_W<'a, O> {
    #[doc = "disable performance counter"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X00)
    }
    #[doc = "2D Drawing Engine active cycles"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X01)
    }
    #[doc = "framebuffer read access"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X02)
    }
    #[doc = "framebuffer write access"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X03)
    }
    #[doc = "texture read access"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X04)
    }
    #[doc = "invisible pixels (enumerated but selected with alpha 0percent)"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X05)
    }
    #[doc = "invisible pixels while internal FIFO is empty (lost cycles)"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X06)
    }
    #[doc = "display list reader active cycles"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X07)
    }
    #[doc = "framebuffer read hits"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X08)
    }
    #[doc = "framebuffer read misses"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X09)
    }
    #[doc = "framebuffer write hits"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X0A)
    }
    #[doc = "framebuffer write misses"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X0B)
    }
    #[doc = "texture read hits"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X0C)
    }
    #[doc = "texture read misses"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X0D)
    }
    #[doc = "every clock cycle (for use as timer)"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(PERFTRIGGER2_AW::_0X1F)
    }
}
impl W {
    #[doc = "Bits 0:15 - Selects the internal event that will increment PERFCOUNT1 register."]
    #[inline(always)]
    #[must_use]
    pub fn perftrigger1(&mut self) -> PERFTRIGGER1_W<0> {
        PERFTRIGGER1_W::new(self)
    }
    #[doc = "Bits 16:31 - Selects the internal event that will increment PERFCOUNT2 register"]
    #[inline(always)]
    #[must_use]
    pub fn perftrigger2(&mut self) -> PERFTRIGGER2_W<16> {
        PERFTRIGGER2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Performance Counters Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perftrigger](index.html) module"]
pub struct PERFTRIGGER_SPEC;
impl crate::RegisterSpec for PERFTRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [perftrigger::W](W) writer structure"]
impl crate::Writable for PERFTRIGGER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERFTRIGGER to value 0"]
impl crate::Resettable for PERFTRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
