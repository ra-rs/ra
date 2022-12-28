#[doc = "Register `MKR[%s]` reader"]
pub struct R(crate::R<MKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MKR[%s]` writer"]
pub struct W(crate::W<MKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MKR_SPEC>;
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
impl From<crate::W<MKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EID` reader - Extended ID"]
pub type EID_R = crate::FieldReader<u32, EID_A>;
#[doc = "Extended ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EID_A {
    #[doc = "0: Do not compare associated EID\\[17:0\\]
bits"]
    _0 = 0,
    #[doc = "1: Compare associated EID\\[17:0\\]
bits"]
    _1 = 1,
}
impl From<EID_A> for u32 {
    #[inline(always)]
    fn from(variant: EID_A) -> Self {
        variant as _
    }
}
impl EID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EID_A> {
        match self.bits {
            0 => Some(EID_A::_0),
            1 => Some(EID_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EID_A::_1
    }
}
#[doc = "Field `EID` writer - Extended ID"]
pub type EID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MKR_SPEC, u32, EID_A, 18, O>;
impl<'a, const O: u8> EID_W<'a, O> {
    #[doc = "Do not compare associated EID\\[17:0\\]
bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EID_A::_0)
    }
    #[doc = "Compare associated EID\\[17:0\\]
bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EID_A::_1)
    }
}
#[doc = "Field `SID` reader - Standard ID"]
pub type SID_R = crate::FieldReader<u16, SID_A>;
#[doc = "Standard ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SID_A {
    #[doc = "0: Do not compare associated SID\\[10:0\\]
bits"]
    _0 = 0,
    #[doc = "1: Compare associated SID\\[10:0\\]
bits"]
    _1 = 1,
}
impl From<SID_A> for u16 {
    #[inline(always)]
    fn from(variant: SID_A) -> Self {
        variant as _
    }
}
impl SID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SID_A> {
        match self.bits {
            0 => Some(SID_A::_0),
            1 => Some(SID_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SID_A::_1
    }
}
#[doc = "Field `SID` writer - Standard ID"]
pub type SID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MKR_SPEC, u16, SID_A, 11, O>;
impl<'a, const O: u8> SID_W<'a, O> {
    #[doc = "Do not compare associated SID\\[10:0\\]
bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SID_A::_0)
    }
    #[doc = "Compare associated SID\\[10:0\\]
bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SID_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    #[must_use]
    pub fn eid(&mut self) -> EID_W<0> {
        EID_W::new(self)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SID_W<18> {
        SID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mkr](index.html) module"]
pub struct MKR_SPEC;
impl crate::RegisterSpec for MKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mkr::R](R) reader structure"]
impl crate::Readable for MKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mkr::W](W) writer structure"]
impl crate::Writable for MKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MKR[%s]
to value 0"]
impl crate::Resettable for MKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
