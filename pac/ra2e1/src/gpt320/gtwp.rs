#[doc = "Register `GTWP` reader"]
pub struct R(crate::R<GTWP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTWP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTWP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTWP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTWP` writer"]
pub struct W(crate::W<GTWP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTWP_SPEC>;
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
impl From<crate::W<GTWP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTWP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP` reader - Register Write Disable"]
pub type WP_R = crate::BitReader<WP_A>;
#[doc = "Register Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    #[doc = "0: Write to the register enabled"]
    _0 = 0,
    #[doc = "1: Write to the register disabled"]
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
#[doc = "Field `WP` writer - Register Write Disable"]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTWP_SPEC, WP_A, O>;
impl<'a, const O: u8> WP_W<'a, O> {
    #[doc = "Write to the register enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_A::_0)
    }
    #[doc = "Write to the register disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_A::_1)
    }
}
#[doc = "Field `PRKEY` writer - GTWP Key Code"]
pub type PRKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTWP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<0> {
        WP_W::new(self)
    }
    #[doc = "Bits 8:15 - GTWP Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn prkey(&mut self) -> PRKEY_W<8> {
        PRKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Write-Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtwp](index.html) module"]
pub struct GTWP_SPEC;
impl crate::RegisterSpec for GTWP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtwp::R](R) reader structure"]
impl crate::Readable for GTWP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtwp::W](W) writer structure"]
impl crate::Writable for GTWP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTWP to value 0"]
impl crate::Resettable for GTWP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
