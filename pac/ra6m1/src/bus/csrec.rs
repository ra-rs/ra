#[doc = "Register `CS%sREC` reader"]
pub struct R(crate::R<CSREC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSREC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSREC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSREC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS%sREC` writer"]
pub struct W(crate::W<CSREC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSREC_SPEC>;
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
impl From<crate::W<CSREC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSREC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRCV` reader - Read Recovery"]
pub type RRCV_R = crate::FieldReader<u8, RRCV_A>;
#[doc = "Read Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RRCV_A {
    #[doc = "0: No recovery cycle is inserted."]
    _0X0 = 0,
}
impl From<RRCV_A> for u8 {
    #[inline(always)]
    fn from(variant: RRCV_A) -> Self {
        variant as _
    }
}
impl RRCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RRCV_A> {
        match self.bits {
            0 => Some(RRCV_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == RRCV_A::_0X0
    }
}
#[doc = "Field `RRCV` writer - Read Recovery"]
pub type RRCV_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSREC_SPEC, u8, RRCV_A, 4, O>;
impl<'a, const O: u8> RRCV_W<'a, O> {
    #[doc = "No recovery cycle is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(RRCV_A::_0X0)
    }
}
#[doc = "Field `WRCV` reader - Write Recovery"]
pub type WRCV_R = crate::FieldReader<u8, WRCV_A>;
#[doc = "Write Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRCV_A {
    #[doc = "0: No recovery cycle is inserted."]
    _0X0 = 0,
}
impl From<WRCV_A> for u8 {
    #[inline(always)]
    fn from(variant: WRCV_A) -> Self {
        variant as _
    }
}
impl WRCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRCV_A> {
        match self.bits {
            0 => Some(WRCV_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WRCV_A::_0X0
    }
}
#[doc = "Field `WRCV` writer - Write Recovery"]
pub type WRCV_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSREC_SPEC, u8, WRCV_A, 4, O>;
impl<'a, const O: u8> WRCV_W<'a, O> {
    #[doc = "No recovery cycle is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(WRCV_A::_0X0)
    }
}
impl R {
    #[doc = "Bits 0:3 - Read Recovery"]
    #[inline(always)]
    pub fn rrcv(&self) -> RRCV_R {
        RRCV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Write Recovery"]
    #[inline(always)]
    pub fn wrcv(&self) -> WRCV_R {
        WRCV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read Recovery"]
    #[inline(always)]
    #[must_use]
    pub fn rrcv(&mut self) -> RRCV_W<0> {
        RRCV_W::new(self)
    }
    #[doc = "Bits 8:11 - Write Recovery"]
    #[inline(always)]
    #[must_use]
    pub fn wrcv(&mut self) -> WRCV_W<8> {
        WRCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS%s Recovery Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrec](index.html) module"]
pub struct CSREC_SPEC;
impl crate::RegisterSpec for CSREC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csrec::R](R) reader structure"]
impl crate::Readable for CSREC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csrec::W](W) writer structure"]
impl crate::Writable for CSREC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS%sREC to value 0"]
impl crate::Resettable for CSREC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
