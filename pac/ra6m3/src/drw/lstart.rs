#[doc = "Register `L%sSTART` writer"]
pub struct W(crate::W<LSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTART_SPEC>;
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
impl From<crate::W<LSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTART` writer - Start value of the n'th limiter(n=1-6)"]
pub type LSTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LSTART_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Start value of the n'th limiter(n=1-6)"]
    #[inline(always)]
    #[must_use]
    pub fn lstart(&mut self) -> LSTART_W<0> {
        LSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limiter %s Start Value Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstart](index.html) module"]
pub struct LSTART_SPEC;
impl crate::RegisterSpec for LSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lstart::W](W) writer structure"]
impl crate::Writable for LSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L%sSTART to value 0"]
impl crate::Resettable for LSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
