#[doc = "Register `DCR` reader"]
pub struct R(crate::R<DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR` writer"]
pub struct W(crate::W<DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR_SPEC>;
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
impl From<crate::W<DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEPOL` reader - Driver effective polarity select"]
pub type DEPOL_R = crate::BitReader<DEPOL_A>;
#[doc = "Driver effective polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEPOL_A {
    #[doc = "0: The DEn signal is active high."]
    _0 = 0,
    #[doc = "1: The DEn signal is active low."]
    _1 = 1,
}
impl From<DEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DEPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl DEPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEPOL_A {
        match self.bits {
            false => DEPOL_A::_0,
            true => DEPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEPOL_A::_1
    }
}
#[doc = "Field `DEPOL` writer - Driver effective polarity select"]
pub type DEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR_SPEC, DEPOL_A, O>;
impl<'a, const O: u8> DEPOL_W<'a, O> {
    #[doc = "The DEn signal is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEPOL_A::_0)
    }
    #[doc = "The DEn signal is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEPOL_A::_1)
    }
}
#[doc = "Field `DEAST` reader - Driver Assertion Time"]
pub type DEAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEAST` writer - Driver Assertion Time"]
pub type DEAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DENGT` reader - Driver negate time"]
pub type DENGT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DENGT` writer - Driver negate time"]
pub type DENGT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Driver effective polarity select"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - Driver Assertion Time"]
    #[inline(always)]
    pub fn deast(&self) -> DEAST_R {
        DEAST_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Driver negate time"]
    #[inline(always)]
    pub fn dengt(&self) -> DENGT_R {
        DENGT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Driver effective polarity select"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<0> {
        DEPOL_W::new(self)
    }
    #[doc = "Bits 8:12 - Driver Assertion Time"]
    #[inline(always)]
    #[must_use]
    pub fn deast(&mut self) -> DEAST_W<8> {
        DEAST_W::new(self)
    }
    #[doc = "Bits 16:20 - Driver negate time"]
    #[inline(always)]
    #[must_use]
    pub fn dengt(&mut self) -> DENGT_W<16> {
        DENGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Driver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](index.html) module"]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr::R](R) reader structure"]
impl crate::Readable for DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr::W](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
