#[doc = "Register `FEARH` reader"]
pub struct R(crate::R<FEARH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEARH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEARH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEARH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEARH` writer"]
pub struct W(crate::W<FEARH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEARH_SPEC>;
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
impl From<crate::W<FEARH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEARH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEARH` reader - Flash Processing End Address H"]
pub type FEARH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FEARH` writer - Flash Processing End Address H"]
pub type FEARH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FEARH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing End Address H"]
    #[inline(always)]
    pub fn fearh(&self) -> FEARH_R {
        FEARH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing End Address H"]
    #[inline(always)]
    #[must_use]
    pub fn fearh(&mut self) -> FEARH_W<0> {
        FEARH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Processing End Address Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fearh](index.html) module"]
pub struct FEARH_SPEC;
impl crate::RegisterSpec for FEARH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fearh::R](R) reader structure"]
impl crate::Readable for FEARH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fearh::W](W) writer structure"]
impl crate::Writable for FEARH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEARH to value 0"]
impl crate::Resettable for FEARH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
