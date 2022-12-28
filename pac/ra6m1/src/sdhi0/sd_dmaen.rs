#[doc = "Register `SD_DMAEN` reader"]
pub struct R(crate::R<SD_DMAEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_DMAEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_DMAEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_DMAEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_DMAEN` writer"]
pub struct W(crate::W<SD_DMAEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_DMAEN_SPEC>;
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
impl From<crate::W<SD_DMAEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_DMAEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - SD_BUF Read/Write DMA Transfer"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "SD_BUF Read/Write DMA Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: The SD_BUF read/write DMA transfer is disabled."]
    _0 = 0,
    #[doc = "1: The SD_BUF read/write DMA transfer is enabled."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Field `DMAEN` writer - SD_BUF Read/Write DMA Transfer"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_DMAEN_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "The SD_BUF read/write DMA transfer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "The SD_BUF read/write DMA transfer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - SD_BUF Read/Write DMA Transfer"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SD_BUF Read/Write DMA Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<1> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Mode Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_dmaen](index.html) module"]
pub struct SD_DMAEN_SPEC;
impl crate::RegisterSpec for SD_DMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_dmaen::R](R) reader structure"]
impl crate::Readable for SD_DMAEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_dmaen::W](W) writer structure"]
impl crate::Writable for SD_DMAEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_DMAEN to value 0x1010"]
impl crate::Resettable for SD_DMAEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x1010;
}
