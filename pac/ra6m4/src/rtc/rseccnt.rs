#[doc = "Register `RSECCNT` reader"]
pub struct R(crate::R<RSECCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSECCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSECCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSECCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSECCNT` writer"]
pub struct W(crate::W<RSECCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSECCNT_SPEC>;
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
impl From<crate::W<RSECCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSECCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC1` reader - 1-Second Count"]
pub type SEC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC1` writer - 1-Second Count"]
pub type SEC1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RSECCNT_SPEC, u8, u8, 4, O>;
#[doc = "Field `SEC10` reader - 10-Second Count"]
pub type SEC10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC10` writer - 10-Second Count"]
pub type SEC10_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RSECCNT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Second Count"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Second Count"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Second Count"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<0> {
        SEC1_W::new(self)
    }
    #[doc = "Bits 4:6 - 10-Second Count"]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<4> {
        SEC10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second Counter (in Calendar Count Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rseccnt](index.html) module"]
pub struct RSECCNT_SPEC;
impl crate::RegisterSpec for RSECCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rseccnt::R](R) reader structure"]
impl crate::Readable for RSECCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rseccnt::W](W) writer structure"]
impl crate::Writable for RSECCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSECCNT to value 0"]
impl crate::Resettable for RSECCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
