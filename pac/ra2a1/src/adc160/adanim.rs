#[doc = "Register `ADANIM` reader"]
pub struct R(crate::R<ADANIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANIM` writer"]
pub struct W(crate::W<ADANIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANIM_SPEC>;
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
impl From<crate::W<ADANIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANIM` reader - Analog Channel Input Mode Select"]
pub type ANIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANIM` writer - Analog Channel Input Mode Select"]
pub type ANIM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADANIM_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Analog Channel Input Mode Select"]
    #[inline(always)]
    pub fn anim(&self) -> ANIM_R {
        ANIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Analog Channel Input Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn anim(&mut self) -> ANIM_W<0> {
        ANIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Input Mode Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adanim](index.html) module"]
pub struct ADANIM_SPEC;
impl crate::RegisterSpec for ADANIM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adanim::R](R) reader structure"]
impl crate::Readable for ADANIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adanim::W](W) writer structure"]
impl crate::Writable for ADANIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANIM to value 0"]
impl crate::Resettable for ADANIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
