#[doc = "Register `DYRPFR` reader"]
pub struct R(crate::R<DYRPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYRPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYRPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYRPFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYRPFR` writer"]
pub struct W(crate::W<DYRPFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYRPFR_SPEC>;
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
impl From<crate::W<DYRPFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYRPFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG8` reader - alternateMasterFlag"]
pub type FLAG8_R = crate::BitReader<FLAG8_A>;
#[doc = "alternateMasterFlag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG8_A {
    #[doc = "0: alternateMasterFlag is set to FALSE."]
    _0 = 0,
    #[doc = "1: alternateMasterFlag is set to TRUE."]
    _1 = 1,
}
impl From<FLAG8_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG8_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG8_A {
        match self.bits {
            false => FLAG8_A::_0,
            true => FLAG8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG8_A::_1
    }
}
#[doc = "Field `FLAG8` writer - alternateMasterFlag"]
pub type FLAG8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYRPFR_SPEC, FLAG8_A, O>;
impl<'a, const O: u8> FLAG8_W<'a, O> {
    #[doc = "alternateMasterFlag is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG8_A::_0)
    }
    #[doc = "alternateMasterFlag is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG8_A::_1)
    }
}
#[doc = "Field `FLAG9` reader - woStepFlag"]
pub type FLAG9_R = crate::BitReader<FLAG9_A>;
#[doc = "woStepFlag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG9_A {
    #[doc = "0: Set this bit to 0 (FALSE)."]
    _0 = 0,
    #[doc = "1: Setting prohibited"]
    _1 = 1,
}
impl From<FLAG9_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG9_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG9_A {
        match self.bits {
            false => FLAG9_A::_0,
            true => FLAG9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG9_A::_1
    }
}
#[doc = "Field `FLAG9` writer - woStepFlag"]
pub type FLAG9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYRPFR_SPEC, FLAG9_A, O>;
impl<'a, const O: u8> FLAG9_W<'a, O> {
    #[doc = "Set this bit to 0 (FALSE)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG9_A::_0)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG9_A::_1)
    }
}
#[doc = "Field `FLAG10` reader - unicastFlag"]
pub type FLAG10_R = crate::BitReader<FLAG10_A>;
#[doc = "unicastFlag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG10_A {
    #[doc = "0: unicastFlag is set to FALSE."]
    _0 = 0,
    #[doc = "1: unicastFlag is set to TRUE."]
    _1 = 1,
}
impl From<FLAG10_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG10_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG10_A {
        match self.bits {
            false => FLAG10_A::_0,
            true => FLAG10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG10_A::_1
    }
}
#[doc = "Field `FLAG10` writer - unicastFlag"]
pub type FLAG10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYRPFR_SPEC, FLAG10_A, O>;
impl<'a, const O: u8> FLAG10_W<'a, O> {
    #[doc = "unicastFlag is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG10_A::_0)
    }
    #[doc = "unicastFlag is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG10_A::_1)
    }
}
#[doc = "Field `FLAG13` reader - PTP profile Specific 1"]
pub type FLAG13_R = crate::BitReader<FLAG13_A>;
#[doc = "PTP profile Specific 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG13_A {
    #[doc = "0: PTP profile Specific 1 is set to FALSE."]
    _0 = 0,
    #[doc = "1: PTP profile Specific 1 is set to TRUE."]
    _1 = 1,
}
impl From<FLAG13_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG13_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG13_A {
        match self.bits {
            false => FLAG13_A::_0,
            true => FLAG13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG13_A::_1
    }
}
#[doc = "Field `FLAG13` writer - PTP profile Specific 1"]
pub type FLAG13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYRPFR_SPEC, FLAG13_A, O>;
impl<'a, const O: u8> FLAG13_W<'a, O> {
    #[doc = "PTP profile Specific 1 is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG13_A::_0)
    }
    #[doc = "PTP profile Specific 1 is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG13_A::_1)
    }
}
#[doc = "Field `FLAG14` reader - PTP profile Specific 2"]
pub type FLAG14_R = crate::BitReader<FLAG14_A>;
#[doc = "PTP profile Specific 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG14_A {
    #[doc = "0: PTP profile Specific 2 is set to FALSE."]
    _0 = 0,
    #[doc = "1: PTP profile Specific 2 is set to TRUE."]
    _1 = 1,
}
impl From<FLAG14_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG14_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG14_A {
        match self.bits {
            false => FLAG14_A::_0,
            true => FLAG14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG14_A::_1
    }
}
#[doc = "Field `FLAG14` writer - PTP profile Specific 2"]
pub type FLAG14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYRPFR_SPEC, FLAG14_A, O>;
impl<'a, const O: u8> FLAG14_W<'a, O> {
    #[doc = "PTP profile Specific 2 is set to FALSE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLAG14_A::_0)
    }
    #[doc = "PTP profile Specific 2 is set to TRUE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLAG14_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - alternateMasterFlag"]
    #[inline(always)]
    pub fn flag8(&self) -> FLAG8_R {
        FLAG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - woStepFlag"]
    #[inline(always)]
    pub fn flag9(&self) -> FLAG9_R {
        FLAG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - unicastFlag"]
    #[inline(always)]
    pub fn flag10(&self) -> FLAG10_R {
        FLAG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - PTP profile Specific 1"]
    #[inline(always)]
    pub fn flag13(&self) -> FLAG13_R {
        FLAG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PTP profile Specific 2"]
    #[inline(always)]
    pub fn flag14(&self) -> FLAG14_R {
        FLAG14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - alternateMasterFlag"]
    #[inline(always)]
    #[must_use]
    pub fn flag8(&mut self) -> FLAG8_W<8> {
        FLAG8_W::new(self)
    }
    #[doc = "Bit 9 - woStepFlag"]
    #[inline(always)]
    #[must_use]
    pub fn flag9(&mut self) -> FLAG9_W<9> {
        FLAG9_W::new(self)
    }
    #[doc = "Bit 10 - unicastFlag"]
    #[inline(always)]
    #[must_use]
    pub fn flag10(&mut self) -> FLAG10_W<10> {
        FLAG10_W::new(self)
    }
    #[doc = "Bit 13 - PTP profile Specific 1"]
    #[inline(always)]
    #[must_use]
    pub fn flag13(&mut self) -> FLAG13_W<13> {
        FLAG13_W::new(self)
    }
    #[doc = "Bit 14 - PTP profile Specific 2"]
    #[inline(always)]
    #[must_use]
    pub fn flag14(&mut self) -> FLAG14_W<14> {
        FLAG14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay_Resp Message Flag Field Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dyrpfr](index.html) module"]
pub struct DYRPFR_SPEC;
impl crate::RegisterSpec for DYRPFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dyrpfr::R](R) reader structure"]
impl crate::Readable for DYRPFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dyrpfr::W](W) writer structure"]
impl crate::Writable for DYRPFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DYRPFR to value 0"]
impl crate::Resettable for DYRPFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
