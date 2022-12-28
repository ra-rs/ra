#[doc = "Register `ACSR` reader"]
pub struct R(crate::R<ACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSR` writer"]
pub struct W(crate::W<ACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSR_SPEC>;
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
impl From<crate::W<ACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACSR0` reader - Auto-calibration status of device 0"]
pub type ACSR0_R = crate::FieldReader<u8, ACSR0_A>;
#[doc = "Auto-calibration status of device 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACSR0_A {
    #[doc = "0: Initial state"]
    _000 = 0,
    #[doc = "1: Reserved"]
    _001 = 1,
    #[doc = "2: Reserved"]
    _010 = 2,
    #[doc = "3: Normal end"]
    _011 = 3,
    #[doc = "4: Error end"]
    _100 = 4,
}
impl From<ACSR0_A> for u8 {
    #[inline(always)]
    fn from(variant: ACSR0_A) -> Self {
        variant as _
    }
}
impl ACSR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACSR0_A> {
        match self.bits {
            0 => Some(ACSR0_A::_000),
            1 => Some(ACSR0_A::_001),
            2 => Some(ACSR0_A::_010),
            3 => Some(ACSR0_A::_011),
            4 => Some(ACSR0_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ACSR0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ACSR0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ACSR0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ACSR0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ACSR0_A::_100
    }
}
#[doc = "Field `ACSR0` writer - Auto-calibration status of device 0"]
pub type ACSR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACSR_SPEC, u8, ACSR0_A, 3, O>;
impl<'a, const O: u8> ACSR0_W<'a, O> {
    #[doc = "Initial state"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ACSR0_A::_000)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ACSR0_A::_001)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ACSR0_A::_010)
    }
    #[doc = "Normal end"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ACSR0_A::_011)
    }
    #[doc = "Error end"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ACSR0_A::_100)
    }
}
#[doc = "Field `ACSR1` reader - Auto-calibration status of device 1"]
pub type ACSR1_R = crate::FieldReader<u8, ACSR1_A>;
#[doc = "Auto-calibration status of device 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACSR1_A {
    #[doc = "0: Initial state"]
    _000 = 0,
    #[doc = "1: Reserved"]
    _001 = 1,
    #[doc = "2: Reserved"]
    _010 = 2,
    #[doc = "3: Normal end"]
    _011 = 3,
    #[doc = "4: Error end"]
    _100 = 4,
}
impl From<ACSR1_A> for u8 {
    #[inline(always)]
    fn from(variant: ACSR1_A) -> Self {
        variant as _
    }
}
impl ACSR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACSR1_A> {
        match self.bits {
            0 => Some(ACSR1_A::_000),
            1 => Some(ACSR1_A::_001),
            2 => Some(ACSR1_A::_010),
            3 => Some(ACSR1_A::_011),
            4 => Some(ACSR1_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ACSR1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ACSR1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ACSR1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ACSR1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ACSR1_A::_100
    }
}
#[doc = "Field `ACSR1` writer - Auto-calibration status of device 1"]
pub type ACSR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACSR_SPEC, u8, ACSR1_A, 3, O>;
impl<'a, const O: u8> ACSR1_W<'a, O> {
    #[doc = "Initial state"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ACSR1_A::_000)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ACSR1_A::_001)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ACSR1_A::_010)
    }
    #[doc = "Normal end"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ACSR1_A::_011)
    }
    #[doc = "Error end"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ACSR1_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - Auto-calibration status of device 0"]
    #[inline(always)]
    pub fn acsr0(&self) -> ACSR0_R {
        ACSR0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Auto-calibration status of device 1"]
    #[inline(always)]
    pub fn acsr1(&self) -> ACSR1_R {
        ACSR1_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Auto-calibration status of device 0"]
    #[inline(always)]
    #[must_use]
    pub fn acsr0(&mut self) -> ACSR0_W<0> {
        ACSR0_W::new(self)
    }
    #[doc = "Bits 3:5 - Auto-calibration status of device 1"]
    #[inline(always)]
    #[must_use]
    pub fn acsr1(&mut self) -> ACSR1_W<3> {
        ACSR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto-Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr](index.html) module"]
pub struct ACSR_SPEC;
impl crate::RegisterSpec for ACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acsr::R](R) reader structure"]
impl crate::Readable for ACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsr::W](W) writer structure"]
impl crate::Writable for ACSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR to value 0"]
impl crate::Resettable for ACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
