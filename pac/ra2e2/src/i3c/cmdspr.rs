#[doc = "Register `CMDSPR` reader"]
pub struct R(crate::R<CMDSPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDSPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDSPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDSPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDSPR` writer"]
pub struct W(crate::W<CMDSPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDSPR_SPEC>;
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
impl From<crate::W<CMDSPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDSPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSRDR` reader - Maximum Sustained Read Data Rate"]
pub type MSRDR_R = crate::FieldReader<u8, MSRDR_A>;
#[doc = "Maximum Sustained Read Data Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSRDR_A {
    #[doc = "0: fscl Max (default value)"]
    _000 = 0,
    #[doc = "1: 8 MHz"]
    _001 = 1,
    #[doc = "2: 6 MHz"]
    _010 = 2,
    #[doc = "3: 4 MHz"]
    _011 = 3,
    #[doc = "4: 2 MHz"]
    _100 = 4,
}
impl From<MSRDR_A> for u8 {
    #[inline(always)]
    fn from(variant: MSRDR_A) -> Self {
        variant as _
    }
}
impl MSRDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSRDR_A> {
        match self.bits {
            0 => Some(MSRDR_A::_000),
            1 => Some(MSRDR_A::_001),
            2 => Some(MSRDR_A::_010),
            3 => Some(MSRDR_A::_011),
            4 => Some(MSRDR_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MSRDR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MSRDR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MSRDR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MSRDR_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MSRDR_A::_100
    }
}
#[doc = "Field `MSRDR` writer - Maximum Sustained Read Data Rate"]
pub type MSRDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDSPR_SPEC, u8, MSRDR_A, 3, O>;
impl<'a, const O: u8> MSRDR_W<'a, O> {
    #[doc = "fscl Max (default value)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSRDR_A::_000)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSRDR_A::_001)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSRDR_A::_010)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSRDR_A::_011)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSRDR_A::_100)
    }
}
#[doc = "Field `CDTTIM` reader - Clock to Data Turnaround Time (TSCO)"]
pub type CDTTIM_R = crate::FieldReader<u8, CDTTIM_A>;
#[doc = "Clock to Data Turnaround Time (TSCO)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDTTIM_A {
    #[doc = "0: 8 ns or less (default value)"]
    _000 = 0,
    #[doc = "1: 9 ns or less"]
    _001 = 1,
    #[doc = "2: 10 ns or less"]
    _010 = 2,
    #[doc = "3: 11 ns or less"]
    _011 = 3,
    #[doc = "4: 12 ns or less"]
    _100 = 4,
    #[doc = "7: TSCO is more than 12 ns, and is reported by private agreement."]
    _111 = 7,
}
impl From<CDTTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTTIM_A) -> Self {
        variant as _
    }
}
impl CDTTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTTIM_A> {
        match self.bits {
            0 => Some(CDTTIM_A::_000),
            1 => Some(CDTTIM_A::_001),
            2 => Some(CDTTIM_A::_010),
            3 => Some(CDTTIM_A::_011),
            4 => Some(CDTTIM_A::_100),
            7 => Some(CDTTIM_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CDTTIM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CDTTIM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CDTTIM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CDTTIM_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CDTTIM_A::_100
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CDTTIM_A::_111
    }
}
#[doc = "Field `CDTTIM` writer - Clock to Data Turnaround Time (TSCO)"]
pub type CDTTIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDSPR_SPEC, u8, CDTTIM_A, 3, O>;
impl<'a, const O: u8> CDTTIM_W<'a, O> {
    #[doc = "8 ns or less (default value)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CDTTIM_A::_000)
    }
    #[doc = "9 ns or less"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CDTTIM_A::_001)
    }
    #[doc = "10 ns or less"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CDTTIM_A::_010)
    }
    #[doc = "11 ns or less"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CDTTIM_A::_011)
    }
    #[doc = "12 ns or less"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CDTTIM_A::_100)
    }
    #[doc = "TSCO is more than 12 ns, and is reported by private agreement."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CDTTIM_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - Maximum Sustained Read Data Rate"]
    #[inline(always)]
    pub fn msrdr(&self) -> MSRDR_R {
        MSRDR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Clock to Data Turnaround Time (TSCO)"]
    #[inline(always)]
    pub fn cdttim(&self) -> CDTTIM_R {
        CDTTIM_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Maximum Sustained Read Data Rate"]
    #[inline(always)]
    #[must_use]
    pub fn msrdr(&mut self) -> MSRDR_W<0> {
        MSRDR_W::new(self)
    }
    #[doc = "Bits 3:5 - Clock to Data Turnaround Time (TSCO)"]
    #[inline(always)]
    #[must_use]
    pub fn cdttim(&mut self) -> CDTTIM_W<3> {
        CDTTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Max Data Speed R (Read) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdspr](index.html) module"]
pub struct CMDSPR_SPEC;
impl crate::RegisterSpec for CMDSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdspr::R](R) reader structure"]
impl crate::Readable for CMDSPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdspr::W](W) writer structure"]
impl crate::Writable for CMDSPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDSPR to value 0"]
impl crate::Resettable for CMDSPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
