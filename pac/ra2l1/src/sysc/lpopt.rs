#[doc = "Register `LPOPT` reader"]
pub struct R(crate::R<LPOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPOPT` writer"]
pub struct W(crate::W<LPOPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPOPT_SPEC>;
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
impl From<crate::W<LPOPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPOPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUDIS` reader - MPU Clock Disable Control"]
pub type MPUDIS_R = crate::BitReader<MPUDIS_A>;
#[doc = "MPU Clock Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPUDIS_A {
    #[doc = "0: MPU operates as normal"]
    _0 = 0,
    #[doc = "1: MPU operate clock stops (MPU function disable)."]
    _1 = 1,
}
impl From<MPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MPUDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUDIS_A {
        match self.bits {
            false => MPUDIS_A::_0,
            true => MPUDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPUDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPUDIS_A::_1
    }
}
#[doc = "Field `MPUDIS` writer - MPU Clock Disable Control"]
pub type MPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, LPOPT_SPEC, MPUDIS_A, O>;
impl<'a, const O: u8> MPUDIS_W<'a, O> {
    #[doc = "MPU operates as normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPUDIS_A::_0)
    }
    #[doc = "MPU operate clock stops (MPU function disable)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPUDIS_A::_1)
    }
}
#[doc = "Field `DCLKDIS` reader - Debug Clock Disable Control"]
pub type DCLKDIS_R = crate::FieldReader<u8, DCLKDIS_A>;
#[doc = "Debug Clock Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCLKDIS_A {
    #[doc = "0: Debug clock does not stop"]
    _00 = 0,
}
impl From<DCLKDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: DCLKDIS_A) -> Self {
        variant as _
    }
}
impl DCLKDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCLKDIS_A> {
        match self.bits {
            0 => Some(DCLKDIS_A::_00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DCLKDIS_A::_00
    }
}
#[doc = "Field `DCLKDIS` writer - Debug Clock Disable Control"]
pub type DCLKDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LPOPT_SPEC, u8, DCLKDIS_A, 2, O>;
impl<'a, const O: u8> DCLKDIS_W<'a, O> {
    #[doc = "Debug clock does not stop"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DCLKDIS_A::_00)
    }
}
#[doc = "Field `BPFCLKDIS` reader - BPF Clock Disable Control"]
pub type BPFCLKDIS_R = crate::BitReader<BPFCLKDIS_A>;
#[doc = "BPF Clock Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPFCLKDIS_A {
    #[doc = "0: Flash register R/W clock operates as normal"]
    _0 = 0,
    #[doc = "1: Flash register R/W clock stops."]
    _1 = 1,
}
impl From<BPFCLKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: BPFCLKDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl BPFCLKDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPFCLKDIS_A {
        match self.bits {
            false => BPFCLKDIS_A::_0,
            true => BPFCLKDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPFCLKDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPFCLKDIS_A::_1
    }
}
#[doc = "Field `BPFCLKDIS` writer - BPF Clock Disable Control"]
pub type BPFCLKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, LPOPT_SPEC, BPFCLKDIS_A, O>;
impl<'a, const O: u8> BPFCLKDIS_W<'a, O> {
    #[doc = "Flash register R/W clock operates as normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPFCLKDIS_A::_0)
    }
    #[doc = "Flash register R/W clock stops."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPFCLKDIS_A::_1)
    }
}
#[doc = "Field `LPOPTEN` reader - Lower Power Operation Enable"]
pub type LPOPTEN_R = crate::BitReader<LPOPTEN_A>;
#[doc = "Lower Power Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPOPTEN_A {
    #[doc = "0: All lower power counter measure disable"]
    _0 = 0,
    #[doc = "1: All lower power counter measure enable"]
    _1 = 1,
}
impl From<LPOPTEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPOPTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPOPTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOPTEN_A {
        match self.bits {
            false => LPOPTEN_A::_0,
            true => LPOPTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPOPTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPOPTEN_A::_1
    }
}
#[doc = "Field `LPOPTEN` writer - Lower Power Operation Enable"]
pub type LPOPTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, LPOPT_SPEC, LPOPTEN_A, O>;
impl<'a, const O: u8> LPOPTEN_W<'a, O> {
    #[doc = "All lower power counter measure disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPOPTEN_A::_0)
    }
    #[doc = "All lower power counter measure enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPOPTEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - MPU Clock Disable Control"]
    #[inline(always)]
    pub fn mpudis(&self) -> MPUDIS_R {
        MPUDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Debug Clock Disable Control"]
    #[inline(always)]
    pub fn dclkdis(&self) -> DCLKDIS_R {
        DCLKDIS_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - BPF Clock Disable Control"]
    #[inline(always)]
    pub fn bpfclkdis(&self) -> BPFCLKDIS_R {
        BPFCLKDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Lower Power Operation Enable"]
    #[inline(always)]
    pub fn lpopten(&self) -> LPOPTEN_R {
        LPOPTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Clock Disable Control"]
    #[inline(always)]
    #[must_use]
    pub fn mpudis(&mut self) -> MPUDIS_W<0> {
        MPUDIS_W::new(self)
    }
    #[doc = "Bits 1:2 - Debug Clock Disable Control"]
    #[inline(always)]
    #[must_use]
    pub fn dclkdis(&mut self) -> DCLKDIS_W<1> {
        DCLKDIS_W::new(self)
    }
    #[doc = "Bit 3 - BPF Clock Disable Control"]
    #[inline(always)]
    #[must_use]
    pub fn bpfclkdis(&mut self) -> BPFCLKDIS_W<3> {
        BPFCLKDIS_W::new(self)
    }
    #[doc = "Bit 7 - Lower Power Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpopten(&mut self) -> LPOPTEN_W<7> {
        LPOPTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower Power Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpopt](index.html) module"]
pub struct LPOPT_SPEC;
impl crate::RegisterSpec for LPOPT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpopt::R](R) reader structure"]
impl crate::Readable for LPOPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpopt::W](W) writer structure"]
impl crate::Writable for LPOPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPOPT to value 0"]
impl crate::Resettable for LPOPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
