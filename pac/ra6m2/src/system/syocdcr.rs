#[doc = "Register `SYOCDCR` reader"]
pub struct R(crate::R<SYOCDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYOCDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYOCDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYOCDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYOCDCR` writer"]
pub struct W(crate::W<SYOCDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYOCDCR_SPEC>;
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
impl From<crate::W<SYOCDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYOCDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOCDF` reader - Deep Standby OCD flag\n\nThe field is **modified** in some way after a read operation."]
pub type DOCDF_R = crate::BitReader<DOCDF_A>;
#[doc = "Deep Standby OCD flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOCDF_A {
    #[doc = "0: On-chip debugger is disabled"]
    _0 = 0,
    #[doc = "1: On-chip debugger is enabled"]
    _1 = 1,
}
impl From<DOCDF_A> for bool {
    #[inline(always)]
    fn from(variant: DOCDF_A) -> Self {
        variant as u8 != 0
    }
}
impl DOCDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOCDF_A {
        match self.bits {
            false => DOCDF_A::_0,
            true => DOCDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOCDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOCDF_A::_1
    }
}
#[doc = "Field `DOCDF` writer - Deep Standby OCD flag"]
pub type DOCDF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SYOCDCR_SPEC, DOCDF_A, O>;
impl<'a, const O: u8> DOCDF_W<'a, O> {
    #[doc = "On-chip debugger is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOCDF_A::_0)
    }
    #[doc = "On-chip debugger is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOCDF_A::_1)
    }
}
#[doc = "Field `DBGEN` reader - Debugger Enable bit"]
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
#[doc = "Debugger Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    #[doc = "0: On-chip debugger is disabled"]
    _0 = 0,
    #[doc = "1: On-chip debugger is enabled"]
    _1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::_0,
            true => DBGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGEN_A::_1
    }
}
#[doc = "Field `DBGEN` writer - Debugger Enable bit"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SYOCDCR_SPEC, DBGEN_A, O>;
impl<'a, const O: u8> DBGEN_W<'a, O> {
    #[doc = "On-chip debugger is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEN_A::_0)
    }
    #[doc = "On-chip debugger is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Deep Standby OCD flag"]
    #[inline(always)]
    pub fn docdf(&self) -> DOCDF_R {
        DOCDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deep Standby OCD flag"]
    #[inline(always)]
    #[must_use]
    pub fn docdf(&mut self) -> DOCDF_W<0> {
        DOCDF_W::new(self)
    }
    #[doc = "Bit 7 - Debugger Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<7> {
        DBGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control OCD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syocdcr](index.html) module"]
pub struct SYOCDCR_SPEC;
impl crate::RegisterSpec for SYOCDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syocdcr::R](R) reader structure"]
impl crate::Readable for SYOCDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syocdcr::W](W) writer structure"]
impl crate::Writable for SYOCDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYOCDCR to value 0"]
impl crate::Resettable for SYOCDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
