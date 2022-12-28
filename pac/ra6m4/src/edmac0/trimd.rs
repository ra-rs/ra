#[doc = "Register `TRIMD` reader"]
pub struct R(crate::R<TRIMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIMD` writer"]
pub struct W(crate::W<TRIMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIMD_SPEC>;
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
impl From<crate::W<TRIMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIS` reader - Transmit Interrupt Enable"]
pub type TIS_R = crate::BitReader<TIS_A>;
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIS_A {
    #[doc = "0: Disable transmit interrupts"]
    _0 = 0,
    #[doc = "1: Enable transmit Interrupts."]
    _1 = 1,
}
impl From<TIS_A> for bool {
    #[inline(always)]
    fn from(variant: TIS_A) -> Self {
        variant as u8 != 0
    }
}
impl TIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIS_A {
        match self.bits {
            false => TIS_A::_0,
            true => TIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIS_A::_1
    }
}
#[doc = "Field `TIS` writer - Transmit Interrupt Enable"]
pub type TIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIMD_SPEC, TIS_A, O>;
impl<'a, const O: u8> TIS_W<'a, O> {
    #[doc = "Disable transmit interrupts"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIS_A::_0)
    }
    #[doc = "Enable transmit Interrupts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIS_A::_1)
    }
}
#[doc = "Field `TIM` reader - Transmit Interrupt Mode"]
pub type TIM_R = crate::BitReader<TIM_A>;
#[doc = "Transmit Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM_A {
    #[doc = "0: Select transmission complete interrupt mode, where an interrupt occurs when a frame is transmitted"]
    _0 = 0,
    #[doc = "1: Select write-back complete interrupt mode, where an interrupt occurs when write-back to the transmit descriptor is complete while the TWBI bit is 1."]
    _1 = 1,
}
impl From<TIM_A> for bool {
    #[inline(always)]
    fn from(variant: TIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM_A {
        match self.bits {
            false => TIM_A::_0,
            true => TIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIM_A::_1
    }
}
#[doc = "Field `TIM` writer - Transmit Interrupt Mode"]
pub type TIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIMD_SPEC, TIM_A, O>;
impl<'a, const O: u8> TIM_W<'a, O> {
    #[doc = "Select transmission complete interrupt mode, where an interrupt occurs when a frame is transmitted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIM_A::_0)
    }
    #[doc = "Select write-back complete interrupt mode, where an interrupt occurs when write-back to the transmit descriptor is complete while the TWBI bit is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tis(&self) -> TIS_R {
        TIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Interrupt Mode"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tis(&mut self) -> TIS_W<0> {
        TIS_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim(&mut self) -> TIM_W<4> {
        TIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Interrupt Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trimd](index.html) module"]
pub struct TRIMD_SPEC;
impl crate::RegisterSpec for TRIMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trimd::R](R) reader structure"]
impl crate::Readable for TRIMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trimd::W](W) writer structure"]
impl crate::Writable for TRIMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIMD to value 0"]
impl crate::Resettable for TRIMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
