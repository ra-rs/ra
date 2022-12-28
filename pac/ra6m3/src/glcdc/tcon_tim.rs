#[doc = "Register `TCON_TIM` reader"]
pub struct R(crate::R<TCON_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCON_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCON_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCON_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCON_TIM` writer"]
pub struct W(crate::W<TCON_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCON_TIM_SPEC>;
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
impl From<crate::W<TCON_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCON_TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels."]
pub type OFFSET_R = crate::FieldReader<u16, OFFSET_A>;
#[doc = "Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OFFSET_A(u16);
impl From<OFFSET_A> for u16 {
    #[inline(always)]
    fn from(val: OFFSET_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `OFFSET` writer - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels."]
pub type OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TCON_TIM_SPEC, u16, OFFSET_A, 11, O>;
#[doc = "Field `HALF` reader - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels."]
pub type HALF_R = crate::FieldReader<u16, HALF_A>;
#[doc = "Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HALF_A(u16);
impl From<HALF_A> for u16 {
    #[inline(always)]
    fn from(val: HALF_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `HALF` writer - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels."]
pub type HALF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCON_TIM_SPEC, u16, HALF_A, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<0> {
        OFFSET_W::new(self)
    }
    #[doc = "Bits 16:26 - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<16> {
        HALF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Reference Timing Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcon_tim](index.html) module"]
pub struct TCON_TIM_SPEC;
impl crate::RegisterSpec for TCON_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcon_tim::R](R) reader structure"]
impl crate::Readable for TCON_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcon_tim::W](W) writer structure"]
impl crate::Writable for TCON_TIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCON_TIM to value 0"]
impl crate::Resettable for TCON_TIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
