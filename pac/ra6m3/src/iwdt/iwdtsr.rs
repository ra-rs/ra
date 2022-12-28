#[doc = "Register `IWDTSR` reader"]
pub struct R(crate::R<IWDTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWDTSR` writer"]
pub struct W(crate::W<IWDTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDTSR_SPEC>;
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
impl From<crate::W<IWDTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTVAL` reader - Counter ValueValue counted by the counter"]
pub type CNTVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UNDFF` reader - Underflow Flag\n\nThe field is **modified** in some way after a read operation."]
pub type UNDFF_R = crate::BitReader<UNDFF_A>;
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDFF_A {
    #[doc = "0: Underflow not occurred"]
    _0 = 0,
    #[doc = "1: Underflow occurred"]
    _1 = 1,
}
impl From<UNDFF_A> for bool {
    #[inline(always)]
    fn from(variant: UNDFF_A) -> Self {
        variant as u8 != 0
    }
}
impl UNDFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDFF_A {
        match self.bits {
            false => UNDFF_A::_0,
            true => UNDFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNDFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNDFF_A::_1
    }
}
#[doc = "Field `UNDFF` writer - Underflow Flag"]
pub type UNDFF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, IWDTSR_SPEC, UNDFF_A, O>;
impl<'a, const O: u8> UNDFF_W<'a, O> {
    #[doc = "Underflow not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNDFF_A::_0)
    }
    #[doc = "Underflow occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNDFF_A::_1)
    }
}
#[doc = "Field `REFEF` reader - Refresh Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type REFEF_R = crate::BitReader<REFEF_A>;
#[doc = "Refresh Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFEF_A {
    #[doc = "0: Refresh error not occurred"]
    _0 = 0,
    #[doc = "1: Refresh error occurred"]
    _1 = 1,
}
impl From<REFEF_A> for bool {
    #[inline(always)]
    fn from(variant: REFEF_A) -> Self {
        variant as u8 != 0
    }
}
impl REFEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFEF_A {
        match self.bits {
            false => REFEF_A::_0,
            true => REFEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REFEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REFEF_A::_1
    }
}
#[doc = "Field `REFEF` writer - Refresh Error Flag"]
pub type REFEF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, IWDTSR_SPEC, REFEF_A, O>;
impl<'a, const O: u8> REFEF_W<'a, O> {
    #[doc = "Refresh error not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REFEF_A::_0)
    }
    #[doc = "Refresh error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REFEF_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:13 - Counter ValueValue counted by the counter"]
    #[inline(always)]
    pub fn cntval(&self) -> CNTVAL_R {
        CNTVAL_R::new(self.bits & 0x3fff)
    }
    #[doc = "Bit 14 - Underflow Flag"]
    #[inline(always)]
    pub fn undff(&self) -> UNDFF_R {
        UNDFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Refresh Error Flag"]
    #[inline(always)]
    pub fn refef(&self) -> REFEF_R {
        REFEF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn undff(&mut self) -> UNDFF_W<14> {
        UNDFF_W::new(self)
    }
    #[doc = "Bit 15 - Refresh Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn refef(&mut self) -> REFEF_W<15> {
        REFEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWDT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdtsr](index.html) module"]
pub struct IWDTSR_SPEC;
impl crate::RegisterSpec for IWDTSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [iwdtsr::R](R) reader structure"]
impl crate::Readable for IWDTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwdtsr::W](W) writer structure"]
impl crate::Writable for IWDTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xc000;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWDTSR to value 0"]
impl crate::Resettable for IWDTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
