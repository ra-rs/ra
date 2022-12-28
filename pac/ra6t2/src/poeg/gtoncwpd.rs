#[doc = "Register `GTONCWPD` reader"]
pub struct R(crate::R<GTONCWPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTONCWPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTONCWPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTONCWPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTONCWPD` writer"]
pub struct W(crate::W<GTONCWPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTONCWPD_SPEC>;
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
impl From<crate::W<GTONCWPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTONCWPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP` reader - Register Writing Disable"]
pub type WP_R = crate::BitReader<WP_A>;
#[doc = "Register Writing Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    #[doc = "0: Writing to the GTONCCRn register is enabled"]
    _0 = 0,
    #[doc = "1: Writing to the GTONCCRn register is disabled"]
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
#[doc = "Field `WP` writer - Register Writing Disable"]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTONCWPD_SPEC, WP_A, O>;
impl<'a, const O: u8> WP_W<'a, O> {
    #[doc = "Writing to the GTONCCRn register is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_A::_0)
    }
    #[doc = "Writing to the GTONCCRn register is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_A::_1)
    }
}
#[doc = "Field `PRKEY` reader - Key Code"]
pub type PRKEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRKEY` writer - Key Code"]
pub type PRKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, GTONCWPD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Register Writing Disable"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    pub fn prkey(&self) -> PRKEY_R {
        PRKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Register Writing Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<0> {
        WP_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn prkey(&mut self) -> PRKEY_W<8> {
        PRKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Output Stopping Control Group D Write Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtoncwpd](index.html) module"]
pub struct GTONCWPD_SPEC;
impl crate::RegisterSpec for GTONCWPD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gtoncwpd::R](R) reader structure"]
impl crate::Readable for GTONCWPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtoncwpd::W](W) writer structure"]
impl crate::Writable for GTONCWPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTONCWPD to value 0"]
impl crate::Resettable for GTONCWPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
