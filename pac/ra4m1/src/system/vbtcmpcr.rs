#[doc = "Register `VBTCMPCR` reader"]
pub struct R(crate::R<VBTCMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTCMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTCMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTCMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTCMPCR` writer"]
pub struct W(crate::W<VBTCMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTCMPCR_SPEC>;
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
impl From<crate::W<VBTCMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTCMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBTCMPE` reader - VBATT pin low voltage detect circuit output enable"]
pub type VBTCMPE_R = crate::BitReader<VBTCMPE_A>;
#[doc = "VBATT pin low voltage detect circuit output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTCMPE_A {
    #[doc = "0: VBATT pin low voltage detect circuit output disabled"]
    _0 = 0,
    #[doc = "1: VBATT pin low voltage detect circuit output enabled"]
    _1 = 1,
}
impl From<VBTCMPE_A> for bool {
    #[inline(always)]
    fn from(variant: VBTCMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTCMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBTCMPE_A {
        match self.bits {
            false => VBTCMPE_A::_0,
            true => VBTCMPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTCMPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTCMPE_A::_1
    }
}
#[doc = "Field `VBTCMPE` writer - VBATT pin low voltage detect circuit output enable"]
pub type VBTCMPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTCMPCR_SPEC, VBTCMPE_A, O>;
impl<'a, const O: u8> VBTCMPE_W<'a, O> {
    #[doc = "VBATT pin low voltage detect circuit output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBTCMPE_A::_0)
    }
    #[doc = "VBATT pin low voltage detect circuit output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBTCMPE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    pub fn vbtcmpe(&self) -> VBTCMPE_R {
        VBTCMPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbtcmpe(&mut self) -> VBTCMPE_W<0> {
        VBTCMPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Comparator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtcmpcr](index.html) module"]
pub struct VBTCMPCR_SPEC;
impl crate::RegisterSpec for VBTCMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtcmpcr::R](R) reader structure"]
impl crate::Readable for VBTCMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtcmpcr::W](W) writer structure"]
impl crate::Writable for VBTCMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTCMPCR to value 0"]
impl crate::Resettable for VBTCMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
