#[doc = "Register `SYNTOR` reader"]
pub struct R(crate::R<SYNTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNTOR` writer"]
pub struct W(crate::W<SYNTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNTOR_SPEC>;
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
impl From<crate::W<SYNTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNTOR` reader - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1."]
pub type SYNTOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNTOR` writer - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1."]
pub type SYNTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNTOR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1."]
    #[inline(always)]
    pub fn syntor(&self) -> SYNTOR_R {
        SYNTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn syntor(&mut self) -> SYNTOR_W<0> {
        SYNTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync Message Reception Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syntor](index.html) module"]
pub struct SYNTOR_SPEC;
impl crate::RegisterSpec for SYNTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syntor::R](R) reader structure"]
impl crate::Readable for SYNTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syntor::W](W) writer structure"]
impl crate::Writable for SYNTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNTOR to value 0"]
impl crate::Resettable for SYNTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
