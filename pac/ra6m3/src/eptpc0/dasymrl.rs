#[doc = "Register `DASYMRL` reader"]
pub struct R(crate::R<DASYMRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DASYMRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DASYMRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DASYMRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DASYMRL` writer"]
pub struct W(crate::W<DASYMRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DASYMRL_SPEC>;
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
impl From<crate::W<DASYMRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DASYMRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DASYMRL` reader - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value."]
pub type DASYMRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DASYMRL` writer - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value."]
pub type DASYMRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DASYMRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value."]
    #[inline(always)]
    pub fn dasymrl(&self) -> DASYMRL_R {
        DASYMRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value."]
    #[inline(always)]
    #[must_use]
    pub fn dasymrl(&mut self) -> DASYMRL_W<0> {
        DASYMRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asymmetric Delay Setting Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dasymrl](index.html) module"]
pub struct DASYMRL_SPEC;
impl crate::RegisterSpec for DASYMRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dasymrl::R](R) reader structure"]
impl crate::Readable for DASYMRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dasymrl::W](W) writer structure"]
impl crate::Writable for DASYMRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DASYMRL to value 0"]
impl crate::Resettable for DASYMRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
