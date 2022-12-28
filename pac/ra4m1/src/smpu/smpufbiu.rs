#[doc = "Register `SMPUFBIU` reader"]
pub struct R(crate::R<SMPUFBIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPUFBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPUFBIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPUFBIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPUFBIU` writer"]
pub struct W(crate::W<SMPUFBIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPUFBIU_SPEC>;
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
impl From<crate::W<SMPUFBIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPUFBIU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPCPU` reader - CPU Read protection"]
pub type RPCPU_R = crate::BitReader<RPCPU_A>;
#[doc = "CPU Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPCPU_A {
    #[doc = "0: CPU read of memory protection disabled."]
    _0 = 0,
    #[doc = "1: CPU read of memory protection enabled."]
    _1 = 1,
}
impl From<RPCPU_A> for bool {
    #[inline(always)]
    fn from(variant: RPCPU_A) -> Self {
        variant as u8 != 0
    }
}
impl RPCPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPCPU_A {
        match self.bits {
            false => RPCPU_A::_0,
            true => RPCPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPCPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPCPU_A::_1
    }
}
#[doc = "Field `RPCPU` writer - CPU Read protection"]
pub type RPCPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUFBIU_SPEC, RPCPU_A, O>;
impl<'a, const O: u8> RPCPU_W<'a, O> {
    #[doc = "CPU read of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPCPU_A::_0)
    }
    #[doc = "CPU read of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPCPU_A::_1)
    }
}
#[doc = "Field `WPCPU` reader - CPU Write protection"]
pub type WPCPU_R = crate::BitReader<WPCPU_A>;
#[doc = "CPU Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPCPU_A {
    #[doc = "0: CPU write of memory protection disabled."]
    _0 = 0,
    #[doc = "1: CPU write of memory protection enabled."]
    _1 = 1,
}
impl From<WPCPU_A> for bool {
    #[inline(always)]
    fn from(variant: WPCPU_A) -> Self {
        variant as u8 != 0
    }
}
impl WPCPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPCPU_A {
        match self.bits {
            false => WPCPU_A::_0,
            true => WPCPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPCPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPCPU_A::_1
    }
}
#[doc = "Field `WPCPU` writer - CPU Write protection"]
pub type WPCPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUFBIU_SPEC, WPCPU_A, O>;
impl<'a, const O: u8> WPCPU_W<'a, O> {
    #[doc = "CPU write of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPCPU_A::_0)
    }
    #[doc = "CPU write of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPCPU_A::_1)
    }
}
#[doc = "Field `RPGRPA` reader - Master Group A Read protection"]
pub type RPGRPA_R = crate::BitReader<RPGRPA_A>;
#[doc = "Master Group A Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPA_A {
    #[doc = "0: Master group A read of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A read of memory protection enabled."]
    _1 = 1,
}
impl From<RPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl RPGRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPGRPA_A {
        match self.bits {
            false => RPGRPA_A::_0,
            true => RPGRPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPA_A::_1
    }
}
#[doc = "Field `RPGRPA` writer - Master Group A Read protection"]
pub type RPGRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUFBIU_SPEC, RPGRPA_A, O>;
impl<'a, const O: u8> RPGRPA_W<'a, O> {
    #[doc = "Master group A read of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPGRPA_A::_0)
    }
    #[doc = "Master group A read of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPGRPA_A::_1)
    }
}
#[doc = "Field `WPGRPA` reader - Master Group A Write protection"]
pub type WPGRPA_R = crate::BitReader<WPGRPA_A>;
#[doc = "Master Group A Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPA_A {
    #[doc = "0: Master group A write of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A write of memory protection enabled."]
    _1 = 1,
}
impl From<WPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl WPGRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPGRPA_A {
        match self.bits {
            false => WPGRPA_A::_0,
            true => WPGRPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPA_A::_1
    }
}
#[doc = "Field `WPGRPA` writer - Master Group A Write protection"]
pub type WPGRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUFBIU_SPEC, WPGRPA_A, O>;
impl<'a, const O: u8> WPGRPA_W<'a, O> {
    #[doc = "Master group A write of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPGRPA_A::_0)
    }
    #[doc = "Master group A write of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPGRPA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(&self) -> RPCPU_R {
        RPCPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(&self) -> WPCPU_R {
        WPCPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(&self) -> RPGRPA_R {
        RPGRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(&self) -> WPGRPA_R {
        WPGRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpcpu(&mut self) -> RPCPU_W<0> {
        RPCPU_W::new(self)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpcpu(&mut self) -> WPCPU_W<1> {
        WPCPU_W::new(self)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<2> {
        RPGRPA_W::new(self)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpgrpa(&mut self) -> WPGRPA_W<3> {
        WPGRPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Control Register for FBIU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpufbiu](index.html) module"]
pub struct SMPUFBIU_SPEC;
impl crate::RegisterSpec for SMPUFBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smpufbiu::R](R) reader structure"]
impl crate::Readable for SMPUFBIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpufbiu::W](W) writer structure"]
impl crate::Writable for SMPUFBIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUFBIU to value 0"]
impl crate::Resettable for SMPUFBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
