#[doc = "Register `RSECAR` reader"]
pub struct R(crate::R<RSECAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSECAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSECAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSECAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSECAR` writer"]
pub struct W(crate::W<RSECAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSECAR_SPEC>;
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
impl From<crate::W<RSECAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSECAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC1` reader - 1 Second"]
pub type SEC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC1` writer - 1 Second"]
pub type SEC1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RSECAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SEC10` reader - 10 Seconds"]
pub type SEC10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC10` writer - 10 Seconds"]
pub type SEC10_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RSECAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ENB` reader - ENB"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "ENB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: Do not compare register value with RSECCNT counter value"]
    _0 = 0,
    #[doc = "1: Compare register value with RSECCNT counter value"]
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
#[doc = "Field `ENB` writer - ENB"]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSECAR_SPEC, ENB_A, O>;
impl<'a, const O: u8> ENB_W<'a, O> {
    #[doc = "Do not compare register value with RSECCNT counter value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENB_A::_0)
    }
    #[doc = "Compare register value with RSECCNT counter value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1 Second"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10 Seconds"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - ENB"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Second"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<0> {
        SEC1_W::new(self)
    }
    #[doc = "Bits 4:6 - 10 Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<4> {
        SEC10_W::new(self)
    }
    #[doc = "Bit 7 - ENB"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<7> {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second Alarm Register (in Calendar Count Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsecar](index.html) module"]
pub struct RSECAR_SPEC;
impl crate::RegisterSpec for RSECAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rsecar::R](R) reader structure"]
impl crate::Readable for RSECAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsecar::W](W) writer structure"]
impl crate::Writable for RSECAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSECAR to value 0"]
impl crate::Resettable for RSECAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
