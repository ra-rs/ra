#[doc = "Register `GTPC` reader"]
pub struct R(crate::R<GTPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTPC` writer"]
pub struct W(crate::W<GTPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTPC_SPEC>;
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
impl From<crate::W<GTPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCEN` reader - Period Count Function Enable"]
pub type PCEN_R = crate::BitReader<PCEN_A>;
#[doc = "Period Count Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCEN_A {
    #[doc = "0: Period count function is disabled"]
    _0 = 0,
    #[doc = "1: Period count function is enabled"]
    _1 = 1,
}
impl From<PCEN_A> for bool {
    #[inline(always)]
    fn from(variant: PCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCEN_A {
        match self.bits {
            false => PCEN_A::_0,
            true => PCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCEN_A::_1
    }
}
#[doc = "Field `PCEN` writer - Period Count Function Enable"]
pub type PCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTPC_SPEC, PCEN_A, O>;
impl<'a, const O: u8> PCEN_W<'a, O> {
    #[doc = "Period count function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCEN_A::_0)
    }
    #[doc = "Period count function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCEN_A::_1)
    }
}
#[doc = "Field `ASTP` reader - Automatic Stop Function Enable"]
pub type ASTP_R = crate::BitReader<ASTP_A>;
#[doc = "Automatic Stop Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASTP_A {
    #[doc = "0: Automatic stop function is disabled"]
    _0 = 0,
    #[doc = "1: Automatic stop function is enabled"]
    _1 = 1,
}
impl From<ASTP_A> for bool {
    #[inline(always)]
    fn from(variant: ASTP_A) -> Self {
        variant as u8 != 0
    }
}
impl ASTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASTP_A {
        match self.bits {
            false => ASTP_A::_0,
            true => ASTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASTP_A::_1
    }
}
#[doc = "Field `ASTP` writer - Automatic Stop Function Enable"]
pub type ASTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTPC_SPEC, ASTP_A, O>;
impl<'a, const O: u8> ASTP_W<'a, O> {
    #[doc = "Automatic stop function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASTP_A::_0)
    }
    #[doc = "Automatic stop function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASTP_A::_1)
    }
}
#[doc = "Field `PCNT` reader - Period Counter"]
pub type PCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCNT` writer - Period Counter"]
pub type PCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTPC_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 0 - Period Count Function Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Automatic Stop Function Enable"]
    #[inline(always)]
    pub fn astp(&self) -> ASTP_R {
        ASTP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Period Counter"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Period Count Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<0> {
        PCEN_W::new(self)
    }
    #[doc = "Bit 8 - Automatic Stop Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn astp(&mut self) -> ASTP_W<8> {
        ASTP_W::new(self)
    }
    #[doc = "Bits 16:27 - Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PCNT_W<16> {
        PCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Period Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtpc](index.html) module"]
pub struct GTPC_SPEC;
impl crate::RegisterSpec for GTPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtpc::R](R) reader structure"]
impl crate::Readable for GTPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtpc::W](W) writer structure"]
impl crate::Writable for GTPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTPC to value 0"]
impl crate::Resettable for GTPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
