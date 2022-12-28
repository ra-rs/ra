#[doc = "Register `DASYMRU` reader"]
pub struct R(crate::R<DASYMRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DASYMRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DASYMRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DASYMRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DASYMRU` writer"]
pub struct W(crate::W<DASYMRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DASYMRU_SPEC>;
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
impl From<crate::W<DASYMRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DASYMRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DASYMRU` reader - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value."]
pub type DASYMRU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DASYMRU` writer - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value."]
pub type DASYMRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DASYMRU_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value."]
    #[inline(always)]
    pub fn dasymru(&self) -> DASYMRU_R {
        DASYMRU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value."]
    #[inline(always)]
    #[must_use]
    pub fn dasymru(&mut self) -> DASYMRU_W<0> {
        DASYMRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asymmetric Delay Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dasymru](index.html) module"]
pub struct DASYMRU_SPEC;
impl crate::RegisterSpec for DASYMRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dasymru::R](R) reader structure"]
impl crate::Readable for DASYMRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dasymru::W](W) writer structure"]
impl crate::Writable for DASYMRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DASYMRU to value 0"]
impl crate::Resettable for DASYMRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
