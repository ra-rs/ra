#[doc = "Register `ECCPRCR` reader"]
pub struct R(crate::R<ECCPRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCPRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCPRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCPRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCPRCR` writer"]
pub struct W(crate::W<ECCPRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCPRCR_SPEC>;
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
impl From<crate::W<ECCPRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCPRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCPRCR` reader - Register Write Control"]
pub type ECCPRCR_R = crate::BitReader<ECCPRCR_A>;
#[doc = "Register Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCPRCR_A {
    #[doc = "0: Writing to the protect register is disabled."]
    _0 = 0,
    #[doc = "1: Writing to the protect register is enabled."]
    _1 = 1,
}
impl From<ECCPRCR_A> for bool {
    #[inline(always)]
    fn from(variant: ECCPRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCPRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCPRCR_A {
        match self.bits {
            false => ECCPRCR_A::_0,
            true => ECCPRCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCPRCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCPRCR_A::_1
    }
}
#[doc = "Field `ECCPRCR` writer - Register Write Control"]
pub type ECCPRCR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ECCPRCR_SPEC, ECCPRCR_A, O>;
impl<'a, const O: u8> ECCPRCR_W<'a, O> {
    #[doc = "Writing to the protect register is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECCPRCR_A::_0)
    }
    #[doc = "Writing to the protect register is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECCPRCR_A::_1)
    }
}
#[doc = "Write Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KW_AW {
    #[doc = "120: Writing to the ECCPRCR bit is valid, when the KW bits are written 1111000b."]
    _1111000 = 120,
}
impl From<KW_AW> for u8 {
    #[inline(always)]
    fn from(variant: KW_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KW` writer - Write Key Code"]
pub type KW_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ECCPRCR_SPEC, u8, KW_AW, 7, O>;
impl<'a, const O: u8> KW_W<'a, O> {
    #[doc = "Writing to the ECCPRCR bit is valid, when the KW bits are written 1111000b."]
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut W {
        self.variant(KW_AW::_1111000)
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn eccprcr(&self) -> ECCPRCR_R {
        ECCPRCR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn eccprcr(&mut self) -> ECCPRCR_W<0> {
        ECCPRCR_W::new(self)
    }
    #[doc = "Bits 1:7 - Write Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn kw(&mut self) -> KW_W<1> {
        KW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccprcr](index.html) module"]
pub struct ECCPRCR_SPEC;
impl crate::RegisterSpec for ECCPRCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eccprcr::R](R) reader structure"]
impl crate::Readable for ECCPRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccprcr::W](W) writer structure"]
impl crate::Writable for ECCPRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCPRCR to value 0"]
impl crate::Resettable for ECCPRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
