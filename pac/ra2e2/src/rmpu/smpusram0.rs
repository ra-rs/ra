#[doc = "Register `SMPUSRAM0` reader"]
pub struct R(crate::R<SMPUSRAM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPUSRAM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPUSRAM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPUSRAM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPUSRAM0` writer"]
pub struct W(crate::W<SMPUSRAM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPUSRAM0_SPEC>;
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
impl From<crate::W<SMPUSRAM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPUSRAM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPCPU` reader - CPU Read Protection"]
pub type RPCPU_R = crate::BitReader<RPCPU_A>;
#[doc = "CPU Read Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPCPU_A {
    #[doc = "0: Memory protection for CPU read disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for CPU read enabled"]
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
#[doc = "Field `RPCPU` writer - CPU Read Protection"]
pub type RPCPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUSRAM0_SPEC, RPCPU_A, O>;
impl<'a, const O: u8> RPCPU_W<'a, O> {
    #[doc = "Memory protection for CPU read disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPCPU_A::_0)
    }
    #[doc = "Memory protection for CPU read enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPCPU_A::_1)
    }
}
#[doc = "Field `WPCPU` reader - CPU Write Protection"]
pub type WPCPU_R = crate::BitReader<WPCPU_A>;
#[doc = "CPU Write Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPCPU_A {
    #[doc = "0: Memory protection for CPU write disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for CPU write enabled"]
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
#[doc = "Field `WPCPU` writer - CPU Write Protection"]
pub type WPCPU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUSRAM0_SPEC, WPCPU_A, O>;
impl<'a, const O: u8> WPCPU_W<'a, O> {
    #[doc = "Memory protection for CPU write disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPCPU_A::_0)
    }
    #[doc = "Memory protection for CPU write enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPCPU_A::_1)
    }
}
#[doc = "Field `RPGRPA` reader - Master MPU Group A Read Protection"]
pub type RPGRPA_R = crate::BitReader<RPGRPA_A>;
#[doc = "Master MPU Group A Read Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPA_A {
    #[doc = "0: Memory protection for master MPU group A read disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master MPU group A read enabled"]
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
#[doc = "Field `RPGRPA` writer - Master MPU Group A Read Protection"]
pub type RPGRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUSRAM0_SPEC, RPGRPA_A, O>;
impl<'a, const O: u8> RPGRPA_W<'a, O> {
    #[doc = "Memory protection for master MPU group A read disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPGRPA_A::_0)
    }
    #[doc = "Memory protection for master MPU group A read enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPGRPA_A::_1)
    }
}
#[doc = "Field `WPGRPA` reader - Master MPU Group A Write Protection"]
pub type WPGRPA_R = crate::BitReader<WPGRPA_A>;
#[doc = "Master MPU Group A Write Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPA_A {
    #[doc = "0: Memory protection for master MPU group A write disabled"]
    _0 = 0,
    #[doc = "1: Memory protection for master MPU group A write enabled"]
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
#[doc = "Field `WPGRPA` writer - Master MPU Group A Write Protection"]
pub type WPGRPA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUSRAM0_SPEC, WPGRPA_A, O>;
impl<'a, const O: u8> WPGRPA_W<'a, O> {
    #[doc = "Memory protection for master MPU group A write disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPGRPA_A::_0)
    }
    #[doc = "Memory protection for master MPU group A write enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPGRPA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Read Protection"]
    #[inline(always)]
    pub fn rpcpu(&self) -> RPCPU_R {
        RPCPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Write Protection"]
    #[inline(always)]
    pub fn wpcpu(&self) -> WPCPU_R {
        WPCPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master MPU Group A Read Protection"]
    #[inline(always)]
    pub fn rpgrpa(&self) -> RPGRPA_R {
        RPGRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master MPU Group A Write Protection"]
    #[inline(always)]
    pub fn wpgrpa(&self) -> WPGRPA_R {
        WPGRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Read Protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpcpu(&mut self) -> RPCPU_W<0> {
        RPCPU_W::new(self)
    }
    #[doc = "Bit 1 - CPU Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpcpu(&mut self) -> WPCPU_W<1> {
        WPCPU_W::new(self)
    }
    #[doc = "Bit 2 - Master MPU Group A Read Protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<2> {
        RPGRPA_W::new(self)
    }
    #[doc = "Bit 3 - Master MPU Group A Write Protection"]
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
#[doc = "Access Control Register for Memory Bus 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpusram0](index.html) module"]
pub struct SMPUSRAM0_SPEC;
impl crate::RegisterSpec for SMPUSRAM0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smpusram0::R](R) reader structure"]
impl crate::Readable for SMPUSRAM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpusram0::W](W) writer structure"]
impl crate::Writable for SMPUSRAM0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUSRAM0 to value 0"]
impl crate::Resettable for SMPUSRAM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
