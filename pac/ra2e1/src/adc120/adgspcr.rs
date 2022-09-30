#[doc = "Register `ADGSPCR` reader"]
pub struct R(crate::R<ADGSPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADGSPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADGSPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADGSPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADGSPCR` writer"]
pub struct W(crate::W<ADGSPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADGSPCR_SPEC>;
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
impl From<crate::W<ADGSPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADGSPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGS` reader - Group Priority Operation Setting"]
pub type PGS_R = crate::BitReader<PGS_A>;
#[doc = "Group Priority Operation Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGS_A {
    #[doc = "0: Operate without group priority control."]
    _0 = 0,
    #[doc = "1: Operate with group priority control."]
    _1 = 1,
}
impl From<PGS_A> for bool {
    #[inline(always)]
    fn from(variant: PGS_A) -> Self {
        variant as u8 != 0
    }
}
impl PGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGS_A {
        match self.bits {
            false => PGS_A::_0,
            true => PGS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGS_A::_1
    }
}
#[doc = "Field `PGS` writer - Group Priority Operation Setting"]
pub type PGS_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADGSPCR_SPEC, PGS_A, O>;
impl<'a, const O: u8> PGS_W<'a, O> {
    #[doc = "Operate without group priority control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGS_A::_0)
    }
    #[doc = "Operate with group priority control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGS_A::_1)
    }
}
#[doc = "Field `GBRSCN` reader - Lower-Priority Group Restart Setting"]
pub type GBRSCN_R = crate::BitReader<GBRSCN_A>;
#[doc = "Lower-Priority Group Restart Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GBRSCN_A {
    #[doc = "0: Disable rescanning of the group that was stopped in group priority operation"]
    _0 = 0,
    #[doc = "1: Enable rescanning of the group that was stopped in group priority operation."]
    _1 = 1,
}
impl From<GBRSCN_A> for bool {
    #[inline(always)]
    fn from(variant: GBRSCN_A) -> Self {
        variant as u8 != 0
    }
}
impl GBRSCN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GBRSCN_A {
        match self.bits {
            false => GBRSCN_A::_0,
            true => GBRSCN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBRSCN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBRSCN_A::_1
    }
}
#[doc = "Field `GBRSCN` writer - Lower-Priority Group Restart Setting"]
pub type GBRSCN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADGSPCR_SPEC, GBRSCN_A, O>;
impl<'a, const O: u8> GBRSCN_W<'a, O> {
    #[doc = "Disable rescanning of the group that was stopped in group priority operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GBRSCN_A::_0)
    }
    #[doc = "Enable rescanning of the group that was stopped in group priority operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GBRSCN_A::_1)
    }
}
#[doc = "Field `GBRP` reader - Single Scan Continuous Start"]
pub type GBRP_R = crate::BitReader<GBRP_A>;
#[doc = "Single Scan Continuous Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GBRP_A {
    #[doc = "0: Single scan is not continuously activated."]
    _0 = 0,
    #[doc = "1: Single scan for the group with the lower-priority is continuously activated."]
    _1 = 1,
}
impl From<GBRP_A> for bool {
    #[inline(always)]
    fn from(variant: GBRP_A) -> Self {
        variant as u8 != 0
    }
}
impl GBRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GBRP_A {
        match self.bits {
            false => GBRP_A::_0,
            true => GBRP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBRP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBRP_A::_1
    }
}
#[doc = "Field `GBRP` writer - Single Scan Continuous Start"]
pub type GBRP_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADGSPCR_SPEC, GBRP_A, O>;
impl<'a, const O: u8> GBRP_W<'a, O> {
    #[doc = "Single scan is not continuously activated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GBRP_A::_0)
    }
    #[doc = "Single scan for the group with the lower-priority is continuously activated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GBRP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Group Priority Operation Setting"]
    #[inline(always)]
    pub fn pgs(&self) -> PGS_R {
        PGS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lower-Priority Group Restart Setting"]
    #[inline(always)]
    pub fn gbrscn(&self) -> GBRSCN_R {
        GBRSCN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - Single Scan Continuous Start"]
    #[inline(always)]
    pub fn gbrp(&self) -> GBRP_R {
        GBRP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Group Priority Operation Setting"]
    #[inline(always)]
    pub fn pgs(&mut self) -> PGS_W<0> {
        PGS_W::new(self)
    }
    #[doc = "Bit 1 - Lower-Priority Group Restart Setting"]
    #[inline(always)]
    pub fn gbrscn(&mut self) -> GBRSCN_W<1> {
        GBRSCN_W::new(self)
    }
    #[doc = "Bit 15 - Single Scan Continuous Start"]
    #[inline(always)]
    pub fn gbrp(&mut self) -> GBRP_W<15> {
        GBRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Group Scan Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adgspcr](index.html) module"]
pub struct ADGSPCR_SPEC;
impl crate::RegisterSpec for ADGSPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adgspcr::R](R) reader structure"]
impl crate::Readable for ADGSPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adgspcr::W](W) writer structure"]
impl crate::Writable for ADGSPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADGSPCR to value 0"]
impl crate::Resettable for ADGSPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
