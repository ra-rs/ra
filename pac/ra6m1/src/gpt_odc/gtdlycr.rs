#[doc = "Register `GTDLYCR` reader"]
pub struct R(crate::R<GTDLYCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDLYCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDLYCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDLYCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDLYCR` writer"]
pub struct W(crate::W<GTDLYCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDLYCR_SPEC>;
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
impl From<crate::W<GTDLYCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDLYCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLLEN` reader - DLL Operation Enable"]
pub type DLLEN_R = crate::BitReader<DLLEN_A>;
#[doc = "DLL Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLEN_A {
    #[doc = "0: Disable DLL operation"]
    _0 = 0,
    #[doc = "1: Enable DLL operation"]
    _1 = 1,
}
impl From<DLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLEN_A {
        match self.bits {
            false => DLLEN_A::_0,
            true => DLLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLLEN_A::_1
    }
}
#[doc = "Field `DLLEN` writer - DLL Operation Enable"]
pub type DLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR_SPEC, DLLEN_A, O>;
impl<'a, const O: u8> DLLEN_W<'a, O> {
    #[doc = "Disable DLL operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLLEN_A::_0)
    }
    #[doc = "Enable DLL operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLLEN_A::_1)
    }
}
#[doc = "Field `DLYRST` reader - PWM Delay Generation Circuit Reset"]
pub type DLYRST_R = crate::BitReader<DLYRST_A>;
#[doc = "PWM Delay Generation Circuit Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYRST_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Reset"]
    _1 = 1,
}
impl From<DLYRST_A> for bool {
    #[inline(always)]
    fn from(variant: DLYRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYRST_A {
        match self.bits {
            false => DLYRST_A::_0,
            true => DLYRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYRST_A::_1
    }
}
#[doc = "Field `DLYRST` writer - PWM Delay Generation Circuit Reset"]
pub type DLYRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTDLYCR_SPEC, DLYRST_A, O>;
impl<'a, const O: u8> DLYRST_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLYRST_A::_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLYRST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DLL Operation Enable"]
    #[inline(always)]
    pub fn dllen(&self) -> DLLEN_R {
        DLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Delay Generation Circuit Reset"]
    #[inline(always)]
    pub fn dlyrst(&self) -> DLYRST_R {
        DLYRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DLL Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dllen(&mut self) -> DLLEN_W<0> {
        DLLEN_W::new(self)
    }
    #[doc = "Bit 1 - PWM Delay Generation Circuit Reset"]
    #[inline(always)]
    #[must_use]
    pub fn dlyrst(&mut self) -> DLYRST_W<1> {
        DLYRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdlycr](index.html) module"]
pub struct GTDLYCR_SPEC;
impl crate::RegisterSpec for GTDLYCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gtdlycr::R](R) reader structure"]
impl crate::Readable for GTDLYCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdlycr::W](W) writer structure"]
impl crate::Writable for GTDLYCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDLYCR to value 0"]
impl crate::Resettable for GTDLYCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
