#[doc = "Register `SPSCR` reader"]
pub struct R(crate::R<SPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPSCR` writer"]
pub struct W(crate::W<SPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPSCR_SPEC>;
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
impl From<crate::W<SPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPSLN` reader - SPI Sequence Length Specification"]
pub type SPSLN_R = crate::FieldReader<u8, SPSLN_A>;
#[doc = "SPI Sequence Length Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPSLN_A {
    #[doc = "0: Sequence Length is 1 (Referenced SPCMDn, n = 0→0→…)"]
    _000 = 0,
    #[doc = "1: Sequence Length is 2 (Referenced SPCMDn, n = 0→1→0→…)"]
    _001 = 1,
    #[doc = "2: Sequence Length is 3 (Referenced SPCMDn, n = 0→1→2→0→…)"]
    _010 = 2,
    #[doc = "3: Sequence Length is 4 (Referenced SPCMDn, n = 0→1→2→3→0→…)"]
    _011 = 3,
    #[doc = "4: Sequence Length is 5 (Referenced SPCMDn, n = 0→1→2→3→4→0→…)"]
    _100 = 4,
    #[doc = "5: Sequence Length is 6 (Referenced SPCMDn, n = 0→1→2→3→4→5→0→…)"]
    _101 = 5,
    #[doc = "6: Sequence Length is 7 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→0→…)"]
    _110 = 6,
    #[doc = "7: Sequence Length is 8 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→7→0→…)"]
    _111 = 7,
}
impl From<SPSLN_A> for u8 {
    #[inline(always)]
    fn from(variant: SPSLN_A) -> Self {
        variant as _
    }
}
impl SPSLN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPSLN_A {
        match self.bits {
            0 => SPSLN_A::_000,
            1 => SPSLN_A::_001,
            2 => SPSLN_A::_010,
            3 => SPSLN_A::_011,
            4 => SPSLN_A::_100,
            5 => SPSLN_A::_101,
            6 => SPSLN_A::_110,
            7 => SPSLN_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPSLN_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPSLN_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPSLN_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPSLN_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPSLN_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPSLN_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPSLN_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPSLN_A::_111
    }
}
#[doc = "Field `SPSLN` writer - SPI Sequence Length Specification"]
pub type SPSLN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SPSCR_SPEC, u8, SPSLN_A, 3, O>;
impl<'a, const O: u8> SPSLN_W<'a, O> {
    #[doc = "Sequence Length is 1 (Referenced SPCMDn, n = 0→0→…)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SPSLN_A::_000)
    }
    #[doc = "Sequence Length is 2 (Referenced SPCMDn, n = 0→1→0→…)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SPSLN_A::_001)
    }
    #[doc = "Sequence Length is 3 (Referenced SPCMDn, n = 0→1→2→0→…)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SPSLN_A::_010)
    }
    #[doc = "Sequence Length is 4 (Referenced SPCMDn, n = 0→1→2→3→0→…)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SPSLN_A::_011)
    }
    #[doc = "Sequence Length is 5 (Referenced SPCMDn, n = 0→1→2→3→4→0→…)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SPSLN_A::_100)
    }
    #[doc = "Sequence Length is 6 (Referenced SPCMDn, n = 0→1→2→3→4→5→0→…)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SPSLN_A::_101)
    }
    #[doc = "Sequence Length is 7 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→0→…)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SPSLN_A::_110)
    }
    #[doc = "Sequence Length is 8 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→7→0→…)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SPSLN_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI Sequence Length Specification"]
    #[inline(always)]
    pub fn spsln(&self) -> SPSLN_R {
        SPSLN_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI Sequence Length Specification"]
    #[inline(always)]
    #[must_use]
    pub fn spsln(&mut self) -> SPSLN_W<0> {
        SPSLN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spscr](index.html) module"]
pub struct SPSCR_SPEC;
impl crate::RegisterSpec for SPSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spscr::R](R) reader structure"]
impl crate::Readable for SPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spscr::W](W) writer structure"]
impl crate::Writable for SPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSCR to value 0"]
impl crate::Resettable for SPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
