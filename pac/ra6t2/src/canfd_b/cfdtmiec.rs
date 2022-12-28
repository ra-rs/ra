#[doc = "Register `CFDTMIEC` reader"]
pub struct R(crate::R<CFDTMIEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMIEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMIEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMIEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTMIEC` writer"]
pub struct W(crate::W<CFDTMIEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTMIEC_SPEC>;
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
impl From<crate::W<CFDTMIEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTMIEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMIEg` reader - TX Message Buffer Interrupt Enable"]
pub type TMIEG_R = crate::FieldReader<u8, TMIEG_A>;
#[doc = "TX Message Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMIEG_A {
    #[doc = "0: TX message buffer interrupt disabled for corresponding TX message buffer"]
    _0 = 0,
    #[doc = "1: TX message buffer interrupt enabled for corresponding TX message buffer"]
    _1 = 1,
}
impl From<TMIEG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMIEG_A) -> Self {
        variant as _
    }
}
impl TMIEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMIEG_A> {
        match self.bits {
            0 => Some(TMIEG_A::_0),
            1 => Some(TMIEG_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMIEG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMIEG_A::_1
    }
}
#[doc = "Field `TMIEg` writer - TX Message Buffer Interrupt Enable"]
pub type TMIEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMIEC_SPEC, u8, TMIEG_A, 4, O>;
impl<'a, const O: u8> TMIEG_W<'a, O> {
    #[doc = "TX message buffer interrupt disabled for corresponding TX message buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMIEG_A::_0)
    }
    #[doc = "TX message buffer interrupt enabled for corresponding TX message buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMIEG_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - TX Message Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn tmieg(&self) -> TMIEG_R {
        TMIEG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TX Message Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmieg(&mut self) -> TMIEG_W<0> {
        TMIEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Message Buffer Interrupt Enable Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmiec](index.html) module"]
pub struct CFDTMIEC_SPEC;
impl crate::RegisterSpec for CFDTMIEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmiec::R](R) reader structure"]
impl crate::Readable for CFDTMIEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtmiec::W](W) writer structure"]
impl crate::Writable for CFDTMIEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTMIEC to value 0"]
impl crate::Resettable for CFDTMIEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
