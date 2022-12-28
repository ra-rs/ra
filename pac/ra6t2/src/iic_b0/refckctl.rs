#[doc = "Register `REFCKCTL` reader"]
pub struct R(crate::R<REFCKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCKCTL` writer"]
pub struct W(crate::W<REFCKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCKCTL_SPEC>;
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
impl From<crate::W<REFCKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREFCKS` reader - Internal Reference Clock Selection"]
pub type IREFCKS_R = crate::FieldReader<u8, IREFCKS_A>;
#[doc = "Internal Reference Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IREFCKS_A {
    #[doc = "0: IICCLK/1 clock"]
    _000 = 0,
    #[doc = "1: IICCLK/2 clock"]
    _001 = 1,
    #[doc = "2: IICCLK/4 clock"]
    _010 = 2,
    #[doc = "3: IICCLK/8 clock"]
    _011 = 3,
    #[doc = "4: IICCLK/16 clock"]
    _100 = 4,
    #[doc = "5: IICCLK/32 clock"]
    _101 = 5,
    #[doc = "6: IICCLK/64 clock"]
    _110 = 6,
    #[doc = "7: IICCLK/128 clock"]
    _111 = 7,
}
impl From<IREFCKS_A> for u8 {
    #[inline(always)]
    fn from(variant: IREFCKS_A) -> Self {
        variant as _
    }
}
impl IREFCKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFCKS_A {
        match self.bits {
            0 => IREFCKS_A::_000,
            1 => IREFCKS_A::_001,
            2 => IREFCKS_A::_010,
            3 => IREFCKS_A::_011,
            4 => IREFCKS_A::_100,
            5 => IREFCKS_A::_101,
            6 => IREFCKS_A::_110,
            7 => IREFCKS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == IREFCKS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == IREFCKS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == IREFCKS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == IREFCKS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == IREFCKS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == IREFCKS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == IREFCKS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == IREFCKS_A::_111
    }
}
#[doc = "Field `IREFCKS` writer - Internal Reference Clock Selection"]
pub type IREFCKS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REFCKCTL_SPEC, u8, IREFCKS_A, 3, O>;
impl<'a, const O: u8> IREFCKS_W<'a, O> {
    #[doc = "IICCLK/1 clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(IREFCKS_A::_000)
    }
    #[doc = "IICCLK/2 clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(IREFCKS_A::_001)
    }
    #[doc = "IICCLK/4 clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(IREFCKS_A::_010)
    }
    #[doc = "IICCLK/8 clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(IREFCKS_A::_011)
    }
    #[doc = "IICCLK/16 clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(IREFCKS_A::_100)
    }
    #[doc = "IICCLK/32 clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(IREFCKS_A::_101)
    }
    #[doc = "IICCLK/64 clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(IREFCKS_A::_110)
    }
    #[doc = "IICCLK/128 clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(IREFCKS_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - Internal Reference Clock Selection"]
    #[inline(always)]
    pub fn irefcks(&self) -> IREFCKS_R {
        IREFCKS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Internal Reference Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn irefcks(&mut self) -> IREFCKS_W<0> {
        IREFCKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refckctl](index.html) module"]
pub struct REFCKCTL_SPEC;
impl crate::RegisterSpec for REFCKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refckctl::R](R) reader structure"]
impl crate::Readable for REFCKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refckctl::W](W) writer structure"]
impl crate::Writable for REFCKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFCKCTL to value 0"]
impl crate::Resettable for REFCKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
