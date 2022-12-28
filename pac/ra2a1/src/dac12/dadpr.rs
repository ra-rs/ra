#[doc = "Register `DADPR` reader"]
pub struct R(crate::R<DADPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADPR` writer"]
pub struct W(crate::W<DADPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADPR_SPEC>;
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
impl From<crate::W<DADPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPSEL` reader - DADRm Format Select"]
pub type DPSEL_R = crate::BitReader<DPSEL_A>;
#[doc = "DADRm Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSEL_A {
    #[doc = "0: Right justified format."]
    _0 = 0,
    #[doc = "1: Left justified format."]
    _1 = 1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::_0,
            true => DPSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSEL_A::_1
    }
}
#[doc = "Field `DPSEL` writer - DADRm Format Select"]
pub type DPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, DADPR_SPEC, DPSEL_A, O>;
impl<'a, const O: u8> DPSEL_W<'a, O> {
    #[doc = "Right justified format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPSEL_A::_0)
    }
    #[doc = "Left justified format."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - DADRm Format Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - DADRm Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn dpsel(&mut self) -> DPSEL_W<7> {
        DPSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DADR0 Format Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dadpr](index.html) module"]
pub struct DADPR_SPEC;
impl crate::RegisterSpec for DADPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dadpr::R](R) reader structure"]
impl crate::Readable for DADPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dadpr::W](W) writer structure"]
impl crate::Writable for DADPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADPR to value 0"]
impl crate::Resettable for DADPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
