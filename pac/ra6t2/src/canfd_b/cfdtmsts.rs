#[doc = "Register `CFDTMSTS%s` reader"]
pub struct R(crate::R<CFDTMSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTMSTS%s` writer"]
pub struct W(crate::W<CFDTMSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTMSTS_SPEC>;
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
impl From<crate::W<CFDTMSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTMSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMTSTS` reader - TX Message Buffer Transmission Status"]
pub type TMTSTS_R = crate::BitReader<TMTSTS_A>;
#[doc = "TX Message Buffer Transmission Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMTSTS_A {
    #[doc = "0: No on-going transmission"]
    _0 = 0,
    #[doc = "1: On-going transmission"]
    _1 = 1,
}
impl From<TMTSTS_A> for bool {
    #[inline(always)]
    fn from(variant: TMTSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMTSTS_A {
        match self.bits {
            false => TMTSTS_A::_0,
            true => TMTSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMTSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMTSTS_A::_1
    }
}
#[doc = "Field `TMTRF` reader - TX Message Buffer Transmission Result Flag"]
pub type TMTRF_R = crate::FieldReader<u8, TMTRF_A>;
#[doc = "TX Message Buffer Transmission Result Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMTRF_A {
    #[doc = "0: No result"]
    _00 = 0,
    #[doc = "1: Transmission aborted from the TX message buffer"]
    _01 = 1,
    #[doc = "2: Transmission successful from the TX message buffer and transmission abort was not requested"]
    _10 = 2,
    #[doc = "3: Transmission successful from the TX message buffer and transmission abort was requested"]
    _11 = 3,
}
impl From<TMTRF_A> for u8 {
    #[inline(always)]
    fn from(variant: TMTRF_A) -> Self {
        variant as _
    }
}
impl TMTRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMTRF_A {
        match self.bits {
            0 => TMTRF_A::_00,
            1 => TMTRF_A::_01,
            2 => TMTRF_A::_10,
            3 => TMTRF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TMTRF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TMTRF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TMTRF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TMTRF_A::_11
    }
}
#[doc = "Field `TMTRF` writer - TX Message Buffer Transmission Result Flag"]
pub type TMTRF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CFDTMSTS_SPEC, u8, TMTRF_A, 2, O>;
impl<'a, const O: u8> TMTRF_W<'a, O> {
    #[doc = "No result"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TMTRF_A::_00)
    }
    #[doc = "Transmission aborted from the TX message buffer"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TMTRF_A::_01)
    }
    #[doc = "Transmission successful from the TX message buffer and transmission abort was not requested"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TMTRF_A::_10)
    }
    #[doc = "Transmission successful from the TX message buffer and transmission abort was requested"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TMTRF_A::_11)
    }
}
#[doc = "Field `TMTRM` reader - TX Message Buffer Transmission Request Mirrored"]
pub type TMTRM_R = crate::BitReader<TMTRM_A>;
#[doc = "TX Message Buffer Transmission Request Mirrored\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMTRM_A {
    #[doc = "0: TX message buffer transmission not requested"]
    _0 = 0,
    #[doc = "1: TX message buffer transmission requested"]
    _1 = 1,
}
impl From<TMTRM_A> for bool {
    #[inline(always)]
    fn from(variant: TMTRM_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMTRM_A {
        match self.bits {
            false => TMTRM_A::_0,
            true => TMTRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMTRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMTRM_A::_1
    }
}
#[doc = "Field `TMTARM` reader - TX Message Buffer Transmission Abort Request Mirrored"]
pub type TMTARM_R = crate::BitReader<TMTARM_A>;
#[doc = "TX Message Buffer Transmission Abort Request Mirrored\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMTARM_A {
    #[doc = "0: TX message buffer transmission request abort not requested"]
    _0 = 0,
    #[doc = "1: TX message buffer transmission request abort requested"]
    _1 = 1,
}
impl From<TMTARM_A> for bool {
    #[inline(always)]
    fn from(variant: TMTARM_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMTARM_A {
        match self.bits {
            false => TMTARM_A::_0,
            true => TMTARM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMTARM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMTARM_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - TX Message Buffer Transmission Status"]
    #[inline(always)]
    pub fn tmtsts(&self) -> TMTSTS_R {
        TMTSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TX Message Buffer Transmission Result Flag"]
    #[inline(always)]
    pub fn tmtrf(&self) -> TMTRF_R {
        TMTRF_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - TX Message Buffer Transmission Request Mirrored"]
    #[inline(always)]
    pub fn tmtrm(&self) -> TMTRM_R {
        TMTRM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Message Buffer Transmission Abort Request Mirrored"]
    #[inline(always)]
    pub fn tmtarm(&self) -> TMTARM_R {
        TMTARM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - TX Message Buffer Transmission Result Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tmtrf(&mut self) -> TMTRF_W<1> {
        TMTRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Message Buffer Status Registers %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmsts](index.html) module"]
pub struct CFDTMSTS_SPEC;
impl crate::RegisterSpec for CFDTMSTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfdtmsts::R](R) reader structure"]
impl crate::Readable for CFDTMSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtmsts::W](W) writer structure"]
impl crate::Writable for CFDTMSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTMSTS%s to value 0"]
impl crate::Resettable for CFDTMSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
