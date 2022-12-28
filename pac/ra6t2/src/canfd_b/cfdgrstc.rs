#[doc = "Register `CFDGRSTC` reader"]
pub struct R(crate::R<CFDGRSTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGRSTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGRSTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGRSTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGRSTC` writer"]
pub struct W(crate::W<CFDGRSTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGRSTC_SPEC>;
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
impl From<crate::W<CFDGRSTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGRSTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRST` reader - SW Reset"]
pub type SRST_R = crate::BitReader<SRST_A>;
#[doc = "SW Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRST_A {
    #[doc = "0: Normal state"]
    _0 = 0,
    #[doc = "1: SW reset state"]
    _1 = 1,
}
impl From<SRST_A> for bool {
    #[inline(always)]
    fn from(variant: SRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRST_A {
        match self.bits {
            false => SRST_A::_0,
            true => SRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRST_A::_1
    }
}
#[doc = "Field `SRST` writer - SW Reset"]
pub type SRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGRSTC_SPEC, SRST_A, O>;
impl<'a, const O: u8> SRST_W<'a, O> {
    #[doc = "Normal state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRST_A::_0)
    }
    #[doc = "SW reset state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRST_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGRSTC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - SW Reset"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW Reset"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SRST_W<0> {
        SRST_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global SW reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgrstc](index.html) module"]
pub struct CFDGRSTC_SPEC;
impl crate::RegisterSpec for CFDGRSTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgrstc::R](R) reader structure"]
impl crate::Readable for CFDGRSTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgrstc::W](W) writer structure"]
impl crate::Writable for CFDGRSTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGRSTC to value 0"]
impl crate::Resettable for CFDGRSTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
