#[doc = "Register `UCKSEL` reader"]
pub struct R(crate::R<UCKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCKSEL` writer"]
pub struct W(crate::W<UCKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCKSEL_SPEC>;
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
impl From<crate::W<UCKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCKSELC` reader - USB Clock Selection"]
pub type UCKSELC_R = crate::BitReader<UCKSELC_A>;
#[doc = "USB Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCKSELC_A {
    #[doc = "0: High-speed on-chip oscillator clock (HOCO) is not selected as USB clock"]
    _0 = 0,
    #[doc = "1: High-speed on-chip oscillator clock (HOCO) is selected as USB clock"]
    _1 = 1,
}
impl From<UCKSELC_A> for bool {
    #[inline(always)]
    fn from(variant: UCKSELC_A) -> Self {
        variant as u8 != 0
    }
}
impl UCKSELC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCKSELC_A {
        match self.bits {
            false => UCKSELC_A::_0,
            true => UCKSELC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UCKSELC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UCKSELC_A::_1
    }
}
#[doc = "Field `UCKSELC` writer - USB Clock Selection"]
pub type UCKSELC_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCKSEL_SPEC, UCKSELC_A, O>;
impl<'a, const O: u8> UCKSELC_W<'a, O> {
    #[doc = "High-speed on-chip oscillator clock (HOCO) is not selected as USB clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UCKSELC_A::_0)
    }
    #[doc = "High-speed on-chip oscillator clock (HOCO) is selected as USB clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UCKSELC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Selection"]
    #[inline(always)]
    pub fn uckselc(&self) -> UCKSELC_R {
        UCKSELC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn uckselc(&mut self) -> UCKSELC_W<0> {
        UCKSELC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucksel](index.html) module"]
pub struct UCKSEL_SPEC;
impl crate::RegisterSpec for UCKSEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucksel::R](R) reader structure"]
impl crate::Readable for UCKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucksel::W](W) writer structure"]
impl crate::Writable for UCKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCKSEL to value 0"]
impl crate::Resettable for UCKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
