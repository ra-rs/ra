#[doc = "Register `CFDGFDCFG` reader"]
pub struct R(crate::R<CFDGFDCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGFDCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGFDCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGFDCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGFDCFG` writer"]
pub struct W(crate::W<CFDGFDCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGFDCFG_SPEC>;
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
impl From<crate::W<CFDGFDCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGFDCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPED` reader - RES Bit Protocol Exception Disable"]
pub type RPED_R = crate::BitReader<RPED_A>;
#[doc = "RES Bit Protocol Exception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPED_A {
    #[doc = "0: Protocol exception event detection enabled"]
    _0 = 0,
    #[doc = "1: Protocol exception event detection disabled"]
    _1 = 1,
}
impl From<RPED_A> for bool {
    #[inline(always)]
    fn from(variant: RPED_A) -> Self {
        variant as u8 != 0
    }
}
impl RPED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPED_A {
        match self.bits {
            false => RPED_A::_0,
            true => RPED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPED_A::_1
    }
}
#[doc = "Field `RPED` writer - RES Bit Protocol Exception Disable"]
pub type RPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGFDCFG_SPEC, RPED_A, O>;
impl<'a, const O: u8> RPED_W<'a, O> {
    #[doc = "Protocol exception event detection enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPED_A::_0)
    }
    #[doc = "Protocol exception event detection disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPED_A::_1)
    }
}
#[doc = "Field `TSCCFG` reader - Timestamp Capture Configuration"]
pub type TSCCFG_R = crate::FieldReader<u8, TSCCFG_A>;
#[doc = "Timestamp Capture Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSCCFG_A {
    #[doc = "0: Timestamp capture at the sample point of SOF (start of frame)"]
    _00 = 0,
    #[doc = "1: Timestamp capture at frame valid indication"]
    _01 = 1,
    #[doc = "2: Timestamp capture at the sample point of RES bit"]
    _10 = 2,
    #[doc = "3: Reserved"]
    _11 = 3,
}
impl From<TSCCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TSCCFG_A) -> Self {
        variant as _
    }
}
impl TSCCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCCFG_A {
        match self.bits {
            0 => TSCCFG_A::_00,
            1 => TSCCFG_A::_01,
            2 => TSCCFG_A::_10,
            3 => TSCCFG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TSCCFG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TSCCFG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TSCCFG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TSCCFG_A::_11
    }
}
#[doc = "Field `TSCCFG` writer - Timestamp Capture Configuration"]
pub type TSCCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDGFDCFG_SPEC, u8, TSCCFG_A, 2, O>;
impl<'a, const O: u8> TSCCFG_W<'a, O> {
    #[doc = "Timestamp capture at the sample point of SOF (start of frame)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TSCCFG_A::_00)
    }
    #[doc = "Timestamp capture at frame valid indication"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TSCCFG_A::_01)
    }
    #[doc = "Timestamp capture at the sample point of RES bit"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TSCCFG_A::_10)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TSCCFG_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - RES Bit Protocol Exception Disable"]
    #[inline(always)]
    pub fn rped(&self) -> RPED_R {
        RPED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Timestamp Capture Configuration"]
    #[inline(always)]
    pub fn tsccfg(&self) -> TSCCFG_R {
        TSCCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RES Bit Protocol Exception Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rped(&mut self) -> RPED_W<0> {
        RPED_W::new(self)
    }
    #[doc = "Bits 8:9 - Timestamp Capture Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn tsccfg(&mut self) -> TSCCFG_W<8> {
        TSCCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global FD Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgfdcfg](index.html) module"]
pub struct CFDGFDCFG_SPEC;
impl crate::RegisterSpec for CFDGFDCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgfdcfg::R](R) reader structure"]
impl crate::Readable for CFDGFDCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgfdcfg::W](W) writer structure"]
impl crate::Writable for CFDGFDCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGFDCFG to value 0"]
impl crate::Resettable for CFDGFDCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
