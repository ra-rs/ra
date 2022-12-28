#[doc = "Register `SMPUP%sBIU` reader"]
pub struct R(crate::R<SMPUPBIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPUPBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPUPBIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPUPBIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPUP%sBIU` writer"]
pub struct W(crate::W<SMPUPBIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPUPBIU_SPEC>;
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
impl From<crate::W<SMPUPBIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPUPBIU_SPEC>) -> Self {
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
pub type RP_CPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, RP_CPU_A, O>;
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
pub type WP_CPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, WP_CPU_A, O>;
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
pub type RP_GRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, RP_GRPA_A, O>;
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
pub type WP_GRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, WP_GRPA_A, O>;
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
#[doc = "Field `RP_GRPB` reader - Master Group B Read protection"]
pub type RP_GRPB_R = crate::BitReader<RP_GRPB_A>;
#[doc = "Master Group B Read protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_GRPB_A {
    #[doc = "0: Setting prohibited"]
    _0 = 0,
    #[doc = "1: Master group B read of memory protection is enabled. The write value should always be 1."]
    _1 = 1,
}
impl From<RP_GRPB_A> for bool {
    #[inline(always)]
    fn from(variant: RP_GRPB_A) -> Self {
        variant as u8 != 0
    }
}
impl RP_GRPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_GRPB_A {
        match self.bits {
            false => RP_GRPB_A::_0,
            true => RP_GRPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_GRPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_GRPB_A::_1
    }
}
#[doc = "Field `RP_GRPB` writer - Master Group B Read protection"]
pub type RP_GRPB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, RP_GRPB_A, O>;
impl<'a, const O: u8> RP_GRPB_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RP_GRPB_A::_0)
    }
    #[doc = "Master group B read of memory protection is enabled. The write value should always be 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RP_GRPB_A::_1)
    }
}
#[doc = "Field `WP_GRPB` reader - Master Group B Write protection"]
pub type WP_GRPB_R = crate::BitReader<WP_GRPB_A>;
#[doc = "Master Group B Write protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_GRPB_A {
    #[doc = "0: Setting prohibited"]
    _0 = 0,
    #[doc = "1: Master group B write of memory protection is enabled. The write value should always be 1."]
    _1 = 1,
}
impl From<WP_GRPB_A> for bool {
    #[inline(always)]
    fn from(variant: WP_GRPB_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_GRPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_GRPB_A {
        match self.bits {
            false => WP_GRPB_A::_0,
            true => WP_GRPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_GRPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_GRPB_A::_1
    }
}
#[doc = "Field `WP_GRPB` writer - Master Group B Write protection"]
pub type WP_GRPB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, WP_GRPB_A, O>;
impl<'a, const O: u8> WP_GRPB_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_GRPB_A::_0)
    }
    #[doc = "Master group B write of memory protection is enabled. The write value should always be 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_GRPB_A::_1)
    }
}
#[doc = "Field `RP_GRPC` reader - Master Group C Read protection"]
pub type RP_GRPC_R = crate::BitReader<RP_GRPC_A>;
#[doc = "Master Group C Read protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_GRPC_A {
    #[doc = "0: Setting prohibited"]
    _0 = 0,
    #[doc = "1: Master group C read of memory protection is enabled. The write value should always be 1."]
    _1 = 1,
}
impl From<RP_GRPC_A> for bool {
    #[inline(always)]
    fn from(variant: RP_GRPC_A) -> Self {
        variant as u8 != 0
    }
}
impl RP_GRPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_GRPC_A {
        match self.bits {
            false => RP_GRPC_A::_0,
            true => RP_GRPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_GRPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_GRPC_A::_1
    }
}
#[doc = "Field `RP_GRPC` writer - Master Group C Read protection"]
pub type RP_GRPC_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, RP_GRPC_A, O>;
impl<'a, const O: u8> RP_GRPC_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RP_GRPC_A::_0)
    }
    #[doc = "Master group C read of memory protection is enabled. The write value should always be 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RP_GRPC_A::_1)
    }
}
#[doc = "Field `WP_GRPC` reader - Master Group C Write protection"]
pub type WP_GRPC_R = crate::BitReader<WP_GRPC_A>;
#[doc = "Master Group C Write protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_GRPC_A {
    #[doc = "0: Setting prohibited"]
    _0 = 0,
    #[doc = "1: Master group C write of memory protection is enabled. The write value should always be 1."]
    _1 = 1,
}
impl From<WP_GRPC_A> for bool {
    #[inline(always)]
    fn from(variant: WP_GRPC_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_GRPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_GRPC_A {
        match self.bits {
            false => WP_GRPC_A::_0,
            true => WP_GRPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_GRPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_GRPC_A::_1
    }
}
#[doc = "Field `WP_GRPC` writer - Master Group C Write protection"]
pub type WP_GRPC_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUPBIU_SPEC, WP_GRPC_A, O>;
impl<'a, const O: u8> WP_GRPC_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_GRPC_A::_0)
    }
    #[doc = "Master group C write of memory protection is enabled. The write value should always be 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_GRPC_A::_1)
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
    #[doc = "Bit 4 - Master Group B Read protection"]
    #[inline(always)]
    pub fn rp_grpb(&self) -> RP_GRPB_R {
        RP_GRPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Group B Write protection"]
    #[inline(always)]
    pub fn wp_grpb(&self) -> WP_GRPB_R {
        WP_GRPB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Group C Read protection"]
    #[inline(always)]
    pub fn rp_grpc(&self) -> RP_GRPC_R {
        RP_GRPC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master Group C Write protection"]
    #[inline(always)]
    pub fn wp_grpc(&self) -> WP_GRPC_R {
        WP_GRPC_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 4 - Master Group B Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rp_grpb(&mut self) -> RP_GRPB_W<4> {
        RP_GRPB_W::new(self)
    }
    #[doc = "Bit 5 - Master Group B Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wp_grpb(&mut self) -> WP_GRPB_W<5> {
        WP_GRPB_W::new(self)
    }
    #[doc = "Bit 6 - Master Group C Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rp_grpc(&mut self) -> RP_GRPC_W<6> {
        RP_GRPC_W::new(self)
    }
    #[doc = "Bit 7 - Master Group C Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wp_grpc(&mut self) -> WP_GRPC_W<7> {
        WP_GRPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Control Register for P%sBIU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpupbiu](index.html) module"]
pub struct SMPUPBIU_SPEC;
impl crate::RegisterSpec for SMPUPBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smpupbiu::R](R) reader structure"]
impl crate::Readable for SMPUPBIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpupbiu::W](W) writer structure"]
impl crate::Writable for SMPUPBIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUP%sBIU to value 0xf0"]
impl crate::Resettable for SMPUPBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
