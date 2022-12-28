#[doc = "Register `SMPUEXBIU` reader"]
pub struct R(crate::R<SMPUEXBIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPUEXBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPUEXBIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPUEXBIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPUEXBIU` writer"]
pub struct W(crate::W<SMPUEXBIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPUEXBIU_SPEC>;
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
impl From<crate::W<SMPUEXBIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPUEXBIU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RP_CPU` reader - CPU Read protection"]
pub type RP_CPU_R = crate::BitReader<RP_CPU_A>;
#[doc = "CPU Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_CPU_A {
    #[doc = "0: CPU read of memory protection is disabled."]
    _0 = 0,
    #[doc = "1: CPU read of memory protection is enabled."]
    _1 = 1,
}
impl From<RP_CPU_A> for bool {
    #[inline(always)]
    fn from(variant: RP_CPU_A) -> Self {
        variant as u8 != 0
    }
}
impl RP_CPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_CPU_A {
        match self.bits {
            false => RP_CPU_A::_0,
            true => RP_CPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_CPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_CPU_A::_1
    }
}
#[doc = "Field `RP_CPU` writer - CPU Read protection"]
pub type RP_CPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUEXBIU_SPEC, RP_CPU_A, O>;
impl<'a, const O: u8> RP_CPU_W<'a, O> {
    #[doc = "CPU read of memory protection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RP_CPU_A::_0)
    }
    #[doc = "CPU read of memory protection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RP_CPU_A::_1)
    }
}
#[doc = "Field `WP_CPU` reader - CPU Write protection"]
pub type WP_CPU_R = crate::BitReader<WP_CPU_A>;
#[doc = "CPU Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_CPU_A {
    #[doc = "0: CPU write of memory protection is disabled."]
    _0 = 0,
    #[doc = "1: CPU write of memory protection is enabled."]
    _1 = 1,
}
impl From<WP_CPU_A> for bool {
    #[inline(always)]
    fn from(variant: WP_CPU_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_CPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_CPU_A {
        match self.bits {
            false => WP_CPU_A::_0,
            true => WP_CPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_CPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_CPU_A::_1
    }
}
#[doc = "Field `WP_CPU` writer - CPU Write protection"]
pub type WP_CPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUEXBIU_SPEC, WP_CPU_A, O>;
impl<'a, const O: u8> WP_CPU_W<'a, O> {
    #[doc = "CPU write of memory protection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_CPU_A::_0)
    }
    #[doc = "CPU write of memory protection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_CPU_A::_1)
    }
}
#[doc = "Field `RP_GRPA` reader - Master Group A Read protection"]
pub type RP_GRPA_R = crate::BitReader<RP_GRPA_A>;
#[doc = "Master Group A Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_GRPA_A {
    #[doc = "0: Master group A read of memory protection is disabled."]
    _0 = 0,
    #[doc = "1: Master group A read of memory protection is enabled."]
    _1 = 1,
}
impl From<RP_GRPA_A> for bool {
    #[inline(always)]
    fn from(variant: RP_GRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl RP_GRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_GRPA_A {
        match self.bits {
            false => RP_GRPA_A::_0,
            true => RP_GRPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_GRPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_GRPA_A::_1
    }
}
#[doc = "Field `RP_GRPA` writer - Master Group A Read protection"]
pub type RP_GRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUEXBIU_SPEC, RP_GRPA_A, O>;
impl<'a, const O: u8> RP_GRPA_W<'a, O> {
    #[doc = "Master group A read of memory protection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RP_GRPA_A::_0)
    }
    #[doc = "Master group A read of memory protection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RP_GRPA_A::_1)
    }
}
#[doc = "Field `WP_GRPA` reader - Master Group A Write protection"]
pub type WP_GRPA_R = crate::BitReader<WP_GRPA_A>;
#[doc = "Master Group A Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_GRPA_A {
    #[doc = "0: Master group A write of memory protection is disabled."]
    _0 = 0,
    #[doc = "1: Master group A write of memory protection is enabled."]
    _1 = 1,
}
impl From<WP_GRPA_A> for bool {
    #[inline(always)]
    fn from(variant: WP_GRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_GRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_GRPA_A {
        match self.bits {
            false => WP_GRPA_A::_0,
            true => WP_GRPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_GRPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_GRPA_A::_1
    }
}
#[doc = "Field `WP_GRPA` writer - Master Group A Write protection"]
pub type WP_GRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUEXBIU_SPEC, WP_GRPA_A, O>;
impl<'a, const O: u8> WP_GRPA_W<'a, O> {
    #[doc = "Master group A write of memory protection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_GRPA_A::_0)
    }
    #[doc = "Master group A write of memory protection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_GRPA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    pub fn rp_cpu(&self) -> RP_CPU_R {
        RP_CPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    pub fn wp_cpu(&self) -> WP_CPU_R {
        WP_CPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rp_grpa(&self) -> RP_GRPA_R {
        RP_GRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wp_grpa(&self) -> WP_GRPA_R {
        WP_GRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rp_cpu(&mut self) -> RP_CPU_W<0> {
        RP_CPU_W::new(self)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wp_cpu(&mut self) -> WP_CPU_W<1> {
        WP_CPU_W::new(self)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rp_grpa(&mut self) -> RP_GRPA_W<2> {
        RP_GRPA_W::new(self)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wp_grpa(&mut self) -> WP_GRPA_W<3> {
        WP_GRPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Control Register for EXBIU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpuexbiu](index.html) module"]
pub struct SMPUEXBIU_SPEC;
impl crate::RegisterSpec for SMPUEXBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smpuexbiu::R](R) reader structure"]
impl crate::Readable for SMPUEXBIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpuexbiu::W](W) writer structure"]
impl crate::Writable for SMPUEXBIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUEXBIU to value 0"]
impl crate::Resettable for SMPUEXBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
