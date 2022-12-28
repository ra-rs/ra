#[doc = "Register `GTINTAD` reader"]
pub struct R(crate::R<GTINTAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTINTAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTINTAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTINTAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTINTAD` writer"]
pub struct W(crate::W<GTINTAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTINTAD_SPEC>;
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
impl From<crate::W<GTINTAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTINTAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GRP` reader - Output Disable Source Select"]
pub type GRP_R = crate::FieldReader<u8, GRP_A>;
#[doc = "Output Disable Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GRP_A {
    #[doc = "0: Group A output disable request"]
    _00 = 0,
    #[doc = "1: Group B output disable request"]
    _01 = 1,
}
impl From<GRP_A> for u8 {
    #[inline(always)]
    fn from(variant: GRP_A) -> Self {
        variant as _
    }
}
impl GRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GRP_A> {
        match self.bits {
            0 => Some(GRP_A::_00),
            1 => Some(GRP_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GRP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GRP_A::_01
    }
}
#[doc = "Field `GRP` writer - Output Disable Source Select"]
pub type GRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTINTAD_SPEC, u8, GRP_A, 2, O>;
impl<'a, const O: u8> GRP_W<'a, O> {
    #[doc = "Group A output disable request"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GRP_A::_00)
    }
    #[doc = "Group B output disable request"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GRP_A::_01)
    }
}
#[doc = "Field `GRPABH` reader - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_R = crate::BitReader<GRPABH_A>;
#[doc = "Same Time Output Level High Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABH_A {
    #[doc = "0: Same time output level high disable request is disabled."]
    _0 = 0,
    #[doc = "1: Same time output level high disable request is enabled."]
    _1 = 1,
}
impl From<GRPABH_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABH_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPABH_A {
        match self.bits {
            false => GRPABH_A::_0,
            true => GRPABH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABH_A::_1
    }
}
#[doc = "Field `GRPABH` writer - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPABH_A, O>;
impl<'a, const O: u8> GRPABH_W<'a, O> {
    #[doc = "Same time output level high disable request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPABH_A::_0)
    }
    #[doc = "Same time output level high disable request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPABH_A::_1)
    }
}
#[doc = "Field `GRPABL` reader - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_R = crate::BitReader<GRPABL_A>;
#[doc = "Same Time Output Level Low Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABL_A {
    #[doc = "0: Same time output level low disable request is disabled."]
    _0 = 0,
    #[doc = "1: Same time output level low disable request is enabled."]
    _1 = 1,
}
impl From<GRPABL_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABL_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPABL_A {
        match self.bits {
            false => GRPABL_A::_0,
            true => GRPABL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABL_A::_1
    }
}
#[doc = "Field `GRPABL` writer - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPABL_A, O>;
impl<'a, const O: u8> GRPABL_W<'a, O> {
    #[doc = "Same time output level low disable request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPABL_A::_0)
    }
    #[doc = "Same time output level low disable request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPABL_A::_1)
    }
}
impl R {
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(&self) -> GRPABH_R {
        GRPABH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(&self) -> GRPABL_R {
        GRPABL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn grp(&mut self) -> GRP_W<24> {
        GRP_W::new(self)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabh(&mut self) -> GRPABH_W<29> {
        GRPABH_W::new(self)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabl(&mut self) -> GRPABL_W<30> {
        GRPABL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Interrupt Output Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtintad](index.html) module"]
pub struct GTINTAD_SPEC;
impl crate::RegisterSpec for GTINTAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtintad::R](R) reader structure"]
impl crate::Readable for GTINTAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtintad::W](W) writer structure"]
impl crate::Writable for GTINTAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTINTAD to value 0"]
impl crate::Resettable for GTINTAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
